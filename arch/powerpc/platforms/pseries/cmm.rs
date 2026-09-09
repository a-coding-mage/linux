// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Collaborative memory management interface.
 *
 * Copyright (C) 2008 IBM Corporation
 * Author(s): Brian King (brking@linux.vnet.ibm.com),
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

const CMM_DRIVER_VERSION: &str = "1.0.0";
const CMM_DEFAULT_DELAY: u32 = 1;
const CMM_HOTPLUG_DELAY: u32 = 5;
const CMM_DEBUG: u32 = 0;
const CMM_DISABLE: u32 = 0;
const CMM_OOM_KB: u32 = 1024;
const CMM_MIN_MEM_MB: u64 = 256;
const CMM_MEM_HOTPLUG_PRI: i32 = 1;

#[inline]
fn kb2pages(p: u64) -> u64 { p >> (PAGE_SHIFT - 10) }
#[inline]
fn pages2kb(p: u64) -> u64 { p << (PAGE_SHIFT - 10) }

static mut delay: u32 = CMM_DEFAULT_DELAY;
static mut hotplug_delay: u32 = CMM_HOTPLUG_DELAY;
static mut oom_kb: u32 = CMM_OOM_KB;
static mut cmm_debug: u32 = CMM_DEBUG;
static mut cmm_disabled: u32 = CMM_DISABLE;
static mut min_mem_mb: u64 = CMM_MIN_MEM_MB;
static mut simulate: bool = false;
static mut simulate_loan_target_kb: u64 = 0;
static mut cmm_dev: device = unsafe { core::mem::zeroed() };

static mut loaned_pages: atomic_long_t = unsafe { core::mem::zeroed() };
static mut loaned_pages_target: u64 = 0;
static mut oom_freed_pages: u64 = 0;
static mut hotplug_mutex: mutex = unsafe { core::mem::zeroed() };
static mut hotplug_occurred: i32 = 0;
static mut cmm_thread_ptr: *mut task_struct = core::ptr::null_mut();
static mut b_dev_info: balloon_dev_info = unsafe { core::mem::zeroed() };

unsafe fn plpar_page_set_loaned(page: *mut page) -> i64 {
    let vpa = page_to_phys(page);
    let cmo_page_sz = cmo_get_page_size();
    let mut rc: i64 = 0;
    let mut i: i32 = 0;
    if simulate { return 0; }
    while rc == 0 && i < PAGE_SIZE as i32 {
        rc = plpar_hcall_norets(H_PAGE_INIT, H_PAGE_SET_LOANED, vpa + i as u64, 0);
        i += cmo_page_sz as i32;
    }
    i -= cmo_page_sz as i32;
    while rc != 0 && i != 0 {
        plpar_hcall_norets(H_PAGE_INIT, H_PAGE_SET_ACTIVE, vpa + (i - cmo_page_sz as i32) as u64, 0);
        i -= cmo_page_sz as i32;
    }
    rc
}

unsafe fn plpar_page_set_active(page: *mut page) -> i64 {
    let vpa = page_to_phys(page);
    let cmo_page_sz = cmo_get_page_size();
    let mut rc: i64 = 0;
    let mut i: i32 = 0;
    if simulate { return 0; }
    while rc == 0 && i < PAGE_SIZE as i32 {
        rc = plpar_hcall_norets(H_PAGE_INIT, H_PAGE_SET_ACTIVE, vpa + i as u64, 0);
        i += cmo_page_sz as i32;
    }
    i -= cmo_page_sz as i32;
    while rc != 0 && i != 0 {
        plpar_hcall_norets(H_PAGE_INIT, H_PAGE_SET_LOANED, vpa + (i - cmo_page_sz as i32) as u64, 0);
        i -= cmo_page_sz as i32;
    }
    rc
}

unsafe fn cmm_alloc_pages(mut nr: i64) -> i64 {
    while nr != 0 {
        if mutex_trylock(&mut hotplug_mutex) != 0 {
            if hotplug_occurred != 0 { mutex_unlock(&mut hotplug_mutex); break; }
            mutex_unlock(&mut hotplug_mutex);
        } else { break; }
        let page = balloon_page_alloc();
        if page.is_null() { break; }
        if plpar_page_set_loaned(page) != 0 { __free_page(page); break; }
        balloon_page_enqueue(&mut b_dev_info, page);
        atomic_long_inc(&mut loaned_pages);
        nr -= 1;
    }
    nr
}

unsafe fn cmm_free_pages(mut nr: i64) -> i64 {
    while nr != 0 {
        let page = balloon_page_dequeue(&mut b_dev_info);
        if page.is_null() { break; }
        plpar_page_set_active(page);
        __free_page(page);
        atomic_long_dec(&mut loaned_pages);
        nr -= 1;
    }
    nr
}

unsafe fn cmm_oom_notify(_self: *mut notifier_block, _dummy: u64, parm: *mut core::ffi::c_void) -> i32 {
    let freed = parm as *mut u64;
    let nr = cmm_free_pages(kb2pages(oom_kb as u64) as i64);
    loaned_pages_target = atomic_long_read(&loaned_pages) as u64;
    *freed += kb2pages(oom_kb as u64) - nr as u64;
    oom_freed_pages += kb2pages(oom_kb as u64) - nr as u64;
    NOTIFY_OK
}

unsafe fn cmm_get_mpp() {
    let loaned = atomic_long_read(&loaned_pages);
    let total_pages = totalram_pages() + loaned as u64;
    let mut mpp_data: hvcall_mpp_data = core::mem::zeroed();
    let min_mem_pages = (min_mem_mb * 1024 * 1024) / PAGE_SIZE as u64;
    let (mut target, _page_loan_request): (i64, i64);
    if !simulate {
        if h_get_mpp(&mut mpp_data) != H_SUCCESS { return; }
        let request = div_s64(mpp_data.loan_request as i64, PAGE_SIZE as i64);
        target = request + loaned;
        _page_loan_request = request;
    } else {
        target = kb2pages(simulate_loan_target_kb) as i64 - loaned;
        _page_loan_request = target;
        target += loaned;
    }
    if target < 0 || total_pages < min_mem_pages { target = 0; }
    target = if target > oom_freed_pages as i64 { target - oom_freed_pages as i64 } else { 0 };
    let active_pages_target = total_pages as i64 - target;
    if min_mem_pages as i64 > active_pages_target { target = total_pages as i64 - min_mem_pages as i64; }
    if target < 0 { target = 0; }
    loaned_pages_target = target as u64;
}

unsafe fn cmm_thread(_dummy: *mut core::ffi::c_void) -> i32 {
    loop {
        let timeleft = msleep_interruptible(delay * 1000);
        if kthread_should_stop() || timeleft != 0 { break; }
        if mutex_trylock(&mut hotplug_mutex) != 0 {
            if hotplug_occurred != 0 {
                hotplug_occurred = 0; mutex_unlock(&mut hotplug_mutex);
                let timeleft = msleep_interruptible(hotplug_delay * 1000);
                if kthread_should_stop() || timeleft != 0 { break; }
                continue;
            }
            mutex_unlock(&mut hotplug_mutex);
        } else { continue; }
        cmm_get_mpp();
        let loaned = atomic_long_read(&loaned_pages) as u64;
        if loaned_pages_target > loaned {
            if cmm_alloc_pages((loaned_pages_target - loaned) as i64) != 0 { loaned_pages_target = loaned; }
        } else if loaned_pages_target < loaned { cmm_free_pages((loaned - loaned_pages_target) as i64); }
    }
    0
}

unsafe fn cmm_reboot_notifier(_nb: *mut notifier_block, action: u64, _unused: *mut core::ffi::c_void) -> i32 {
    if action == SYS_RESTART {
        if !cmm_thread_ptr.is_null() { kthread_stop(cmm_thread_ptr); }
        cmm_thread_ptr = core::ptr::null_mut();
        cmm_free_pages(atomic_long_read(&loaned_pages));
    }
    NOTIFY_DONE
}

unsafe fn cmm_memory_cb(_self: *mut notifier_block, action: u64, _arg: *mut core::ffi::c_void) -> i32 {
    match action {
        MEM_GOING_OFFLINE => { mutex_lock(&mut hotplug_mutex); hotplug_occurred = 1; }
        MEM_OFFLINE | MEM_CANCEL_OFFLINE => { mutex_unlock(&mut hotplug_mutex); }
        MEM_GOING_ONLINE | MEM_ONLINE | MEM_CANCEL_ONLINE => {}
        _ => {}
    }
    NOTIFY_OK
}

#[cfg(feature = "CONFIG_BALLOON_MIGRATION")]
unsafe fn cmm_migratepage(_info: *mut balloon_dev_info, newpage: *mut page, page: *mut page, _mode: migrate_mode) -> i32 {
    if plpar_page_set_loaned(newpage) != 0 { return -EBUSY; }
    plpar_page_set_active(page);
    0
}

// The CONFIG_BALLOON_MIGRATION-disabled branch is an external declaration.
#[cfg(not(feature = "CONFIG_BALLOON_MIGRATION"))]
unsafe extern "C" { fn cmm_migratepage(info: *mut balloon_dev_info, newpage: *mut page, page: *mut page, mode: migrate_mode) -> i32; }

unsafe fn cmm_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_CMO) && !simulate { return -EOPNOTSUPP; }
    balloon_devinfo_init(&mut b_dev_info);
    b_dev_info.adjust_managed_page_count = true;
    if IS_ENABLED_CONFIG_BALLOON_MIGRATION { b_dev_info.migratepage = Some(cmm_migratepage); }
    let mut rc = register_oom_notifier(&mut cmm_oom_nb);
    if rc < 0 { return rc; }
    rc = register_reboot_notifier(&mut cmm_reboot_nb);
    if rc != 0 { unregister_oom_notifier(&mut cmm_oom_nb); return rc; }
    rc = cmm_sysfs_register(&mut cmm_dev);
    if rc != 0 { unregister_reboot_notifier(&mut cmm_reboot_nb); unregister_oom_notifier(&mut cmm_oom_nb); return rc; }
    rc = register_memory_notifier(&mut cmm_mem_nb);
    if rc != 0 { cmm_unregister_sysfs(&mut cmm_dev); unregister_reboot_notifier(&mut cmm_reboot_nb); unregister_oom_notifier(&mut cmm_oom_nb); return rc; }
    if cmm_disabled != 0 { return 0; }
    cmm_thread_ptr = kthread_run(cmm_thread, core::ptr::null_mut(), "cmmthread");
    if IS_ERR(cmm_thread_ptr) { rc = PTR_ERR(cmm_thread_ptr); unregister_memory_notifier(&mut cmm_mem_nb); cmm_unregister_sysfs(&mut cmm_dev); unregister_reboot_notifier(&mut cmm_reboot_nb); unregister_oom_notifier(&mut cmm_oom_nb); return rc; }
    0
}

unsafe fn cmm_exit() {
    if !cmm_thread_ptr.is_null() { kthread_stop(cmm_thread_ptr); }
    unregister_oom_notifier(&mut cmm_oom_nb);
    unregister_reboot_notifier(&mut cmm_reboot_nb);
    unregister_memory_notifier(&mut cmm_mem_nb);
    cmm_free_pages(atomic_long_read(&loaned_pages));
    cmm_unregister_sysfs(&mut cmm_dev);
}

unsafe fn show_loaned_kb(_dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    sysfs_emit(buf, "%lu\n", pages2kb(atomic_long_read(&loaned_pages) as u64));
}
unsafe fn show_loaned_target_kb(_dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    sysfs_emit(buf, "%lu\n", pages2kb(loaned_pages_target));
}
unsafe fn show_oom_pages(_dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    sysfs_emit(buf, "%lu\n", pages2kb(oom_freed_pages));
}
unsafe fn store_oom_pages(_dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> isize {
    let val = simple_strtoul(buf, core::ptr::null_mut(), 10);
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    if val != 0 { return -EBADMSG as isize; }
    oom_freed_pages = 0;
    count as isize
}

unsafe fn cmm_release_device(_dev: *mut device) {}

unsafe fn cmm_sysfs_register(dev: *mut device) -> i32 {
    let mut rc = subsys_system_register(&mut cmm_subsys, core::ptr::null());
    if rc != 0 { return rc; }
    (*dev).id = 0;
    (*dev).bus = &mut cmm_subsys;
    (*dev).release = Some(cmm_release_device);
    rc = device_register(dev);
    if rc != 0 { bus_unregister(&mut cmm_subsys); return rc; }
    let attrs = [&mut dev_attr_loaned_kb, &mut dev_attr_loaned_target_kb, &mut dev_attr_oom_freed_kb];
    let mut i = 0;
    while i < attrs.len() {
        rc = device_create_file(dev, attrs[i]);
        if rc != 0 {
            while i > 0 { i -= 1; device_remove_file(dev, attrs[i]); }
            device_unregister(dev); bus_unregister(&mut cmm_subsys); return rc;
        }
        i += 1;
    }
    if !simulate { return 0; }
    rc = device_create_file(dev, &mut dev_attr_simulate_loan_target_kb.attr);
    if rc != 0 {
        while i > 0 { i -= 1; device_remove_file(dev, attrs[i]); }
        device_unregister(dev); bus_unregister(&mut cmm_subsys);
    }
    rc
}

unsafe fn cmm_unregister_sysfs(dev: *mut device) {
    device_remove_file(dev, &mut dev_attr_loaned_kb);
    device_remove_file(dev, &mut dev_attr_loaned_target_kb);
    device_remove_file(dev, &mut dev_attr_oom_freed_kb);
    device_unregister(dev);
    bus_unregister(&mut cmm_subsys);
}

unsafe fn cmm_set_disable(val: *const i8, _kp: *const kernel_param) -> i32 {
    let disable = simple_strtoul(val, core::ptr::null_mut(), 10) as u32;
    if disable != 0 && disable != 1 { return -EINVAL; }
    if disable != 0 && cmm_disabled == 0 {
        if !cmm_thread_ptr.is_null() { kthread_stop(cmm_thread_ptr); }
        cmm_thread_ptr = core::ptr::null_mut();
        cmm_free_pages(atomic_long_read(&loaned_pages));
    } else if disable == 0 && cmm_disabled != 0 {
        cmm_thread_ptr = kthread_run(cmm_thread, core::ptr::null_mut(), "cmmthread");
        if IS_ERR(cmm_thread_ptr) { return PTR_ERR(cmm_thread_ptr); }
    }
    cmm_disabled = disable;
    0
}

static mut cmm_oom_nb: notifier_block = notifier_block { notifier_call: Some(cmm_oom_notify) };
static mut cmm_reboot_nb: notifier_block = notifier_block { notifier_call: Some(cmm_reboot_notifier) };
static mut cmm_mem_nb: notifier_block = notifier_block { notifier_call: Some(cmm_memory_cb), priority: CMM_MEM_HOTPLUG_PRI };
static mut cmm_subsys: bus_type = bus_type { name: "cmm", dev_name: "cmm" };
// DEVICE_ATTR and module metadata declarations are represented by surrounding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
