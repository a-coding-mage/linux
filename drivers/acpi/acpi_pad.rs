// SPDX-License-Identifier: GPL-2.0-only
/*
 * acpi_pad.c ACPI Processor Aggregator Driver
 *
 * Copyright (c) 2009, Intel Corporation.
 */

// Dependencies supplied by the kernel translation environment.

const ACPI_PROCESSOR_AGGREGATOR_NOTIFY: u32 = 0x80;
const ACPI_PROCESSOR_AGGREGATOR_STATUS_SUCCESS: u32 = 0;
const ACPI_PROCESSOR_AGGREGATOR_STATUS_NO_ACTION: u32 = 1;

static mut isolated_cpus_lock: Mutex = DEFINE_MUTEX!();
static mut round_robin_lock: Mutex = DEFINE_MUTEX!();

static mut acpi_pad_teardown: bool = false;
static mut power_saving_mwait_eax: u32 = 0;
static mut tsc_detected_unstable: u8 = 0;
static mut tsc_marked_unstable: u8 = 0;

unsafe fn power_saving_mwait_init() {
    let (mut eax, mut ebx, mut ecx, mut edx): (u32, u32, u32, u32);
    let mut highest_cstate: u32 = 0;
    let mut highest_subcstate: u32 = 0;
    let mut i: i32;

    if !boot_cpu_has(X86_FEATURE_MWAIT) { return; }

    cpuid(CPUID_LEAF_MWAIT, &mut eax, &mut ebx, &mut ecx, &mut edx);
    if (ecx & CPUID5_ECX_EXTENSIONS_SUPPORTED) == 0 ||
       (ecx & CPUID5_ECX_INTERRUPT_BREAK) == 0 { return; }

    edx >>= MWAIT_SUBSTATE_SIZE;
    i = 0;
    while i < 7 && edx != 0 {
        if (edx & MWAIT_SUBSTATE_MASK) != 0 {
            highest_cstate = i as u32;
            highest_subcstate = edx & MWAIT_SUBSTATE_MASK;
        }
        i += 1;
        edx >>= MWAIT_SUBSTATE_SIZE;
    }
    power_saving_mwait_eax = (highest_cstate << MWAIT_SUBSTATE_SIZE) |
        (highest_subcstate - 1);

    #[cfg(CONFIG_X86)]
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_HYGON | X86_VENDOR_AMD | X86_VENDOR_INTEL |
        X86_VENDOR_ZHAOXIN | X86_VENDOR_CENTAUR => {
            /* AMD Fam10h TSC will tick in all C/P/S0/S1 states when this bit is set. */
            if !boot_cpu_has(X86_FEATURE_NONSTOP_TSC) { tsc_detected_unstable = 1; }
        }
        _ => { /* TSC could halt in idle */ tsc_detected_unstable = 1; }
    }
}

static mut cpu_weight: [c_ulong; NR_CPUS] = [0; NR_CPUS];
static mut tsk_in_cpu: [i32; NR_CPUS] = [-1; NR_CPUS];
static mut pad_busy_cpus_bits: Bitmap<NR_CPUS> = DECLARE_BITMAP!();

unsafe fn round_robin_cpu(tsk_index: u32) {
    let pad_busy_cpus: *mut cpumask = to_cpumask(&mut pad_busy_cpus_bits);
    let mut tmp: cpumask_var_t = core::ptr::null_mut();
    let mut cpu: i32;
    let mut min_weight: c_ulong = !0;
    let mut preferred_cpu: i32 = 0;

    if !alloc_cpumask_var(&mut tmp, GFP_KERNEL) { return; }
    mutex_lock(&mut round_robin_lock);
    cpumask_clear(tmp);
    for_each_cpu!(cpu, pad_busy_cpus) {
        cpumask_or(tmp, tmp, topology_sibling_cpumask(cpu));
    }
    cpumask_andnot(tmp, cpu_online_mask, tmp);
    /* avoid HT siblings if possible */
    if cpumask_empty(tmp) { cpumask_andnot(tmp, cpu_online_mask, pad_busy_cpus); }
    if cpumask_empty(tmp) {
        mutex_unlock(&mut round_robin_lock);
        free_cpumask_var(tmp);
        return;
    }
    for_each_cpu!(cpu, tmp) {
        if cpu_weight[cpu as usize] < min_weight {
            min_weight = cpu_weight[cpu as usize];
            preferred_cpu = cpu;
        }
    }
    if tsk_in_cpu[tsk_index as usize] != -1 {
        cpumask_clear_cpu(tsk_in_cpu[tsk_index as usize], pad_busy_cpus);
    }
    tsk_in_cpu[tsk_index as usize] = preferred_cpu;
    cpumask_set_cpu(preferred_cpu, pad_busy_cpus);
    cpu_weight[preferred_cpu as usize] = cpu_weight[preferred_cpu as usize].wrapping_add(1);
    mutex_unlock(&mut round_robin_lock);
    set_cpus_allowed_ptr(current, cpumask_of(preferred_cpu));
    free_cpumask_var(tmp);
}

unsafe fn exit_round_robin(tsk_index: u32) {
    let pad_busy_cpus = to_cpumask(&mut pad_busy_cpus_bits);
    if tsk_in_cpu[tsk_index as usize] != -1 {
        cpumask_clear_cpu(tsk_in_cpu[tsk_index as usize], pad_busy_cpus);
        tsk_in_cpu[tsk_index as usize] = -1;
    }
}

static mut idle_pct: u32 = 5; /* percentage */
static mut round_robin_time: u32 = 1; /* second */

unsafe extern "C" fn power_saving_thread(data: *mut c_void) -> i32 {
    let mut do_sleep: i32;
    let tsk_index = data as usize as u32;
    let mut last_jiffies: u64 = 0;
    sched_set_fifo_low(current);
    while !kthread_should_stop() {
        let mut expire_time: c_ulong;
        expire_time = last_jiffies.wrapping_add(round_robin_time as u64 * HZ as u64) as c_ulong;
        if time_before(expire_time, jiffies) {
            last_jiffies = jiffies as u64;
            round_robin_cpu(tsk_index);
        }
        do_sleep = 0;
        expire_time = jiffies.wrapping_add(HZ * (100 - idle_pct) / 100);
        while !need_resched() {
            if tsc_detected_unstable != 0 && tsc_marked_unstable == 0 {
                mark_tsc_unstable(c_str!("TSC halts in idle"));
                tsc_marked_unstable = 1;
            }
            local_irq_disable();
            perf_lopwr_cb(true);
            tick_broadcast_enable();
            tick_broadcast_enter();
            stop_critical_timings();
            mwait_idle_with_hints(power_saving_mwait_eax, 1);
            start_critical_timings();
            tick_broadcast_exit();
            perf_lopwr_cb(false);
            local_irq_enable();
            if time_before(expire_time, jiffies) { do_sleep = 1; break; }
        }
        if unlikely(do_sleep != 0) { schedule_timeout_killable(HZ * idle_pct / 100); }
        if unlikely(need_resched()) { schedule(); }
    }
    exit_round_robin(tsk_index);
    0
}

static mut ps_tsks: [*mut task_struct; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
static mut ps_tsk_num: u32 = 0;

unsafe fn create_power_saving_task() -> i32 {
    let index = ps_tsk_num as usize;
    ps_tsks[index] = kthread_run(power_saving_thread, index as *mut c_void,
                                 c_str!("acpi_pad/%d"), ps_tsk_num);
    if IS_ERR(ps_tsks[index]) {
        let rc = PTR_ERR(ps_tsks[index]); ps_tsks[index] = core::ptr::null_mut(); rc
    } else { ps_tsk_num += 1; 0 }
}

unsafe fn destroy_power_saving_task() {
    if ps_tsk_num > 0 { ps_tsk_num -= 1; kthread_stop(ps_tsks[ps_tsk_num as usize]); ps_tsks[ps_tsk_num as usize] = core::ptr::null_mut(); }
}

unsafe fn set_power_saving_task_num(num: u32) {
    if num > ps_tsk_num { while ps_tsk_num < num { if create_power_saving_task() != 0 { return; } } }
    else if num < ps_tsk_num { while ps_tsk_num > num { destroy_power_saving_task(); } }
}

unsafe fn acpi_pad_idle_cpus(mut num_cpus: u32) {
    cpus_read_lock();
    num_cpus = min_t!(u32, num_cpus, num_online_cpus());
    set_power_saving_task_num(num_cpus);
    cpus_read_unlock();
}

unsafe fn acpi_pad_idle_cpus_num() -> u32 { ps_tsk_num }

unsafe extern "C" fn rrtime_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut num: c_ulong = 0;
    if kstrtoul(buf, 0, &mut num) != 0 || num < 1 || num >= 100 { return -EINVAL; }
    mutex_lock(&mut isolated_cpus_lock); round_robin_time = num as u32; mutex_unlock(&mut isolated_cpus_lock); count as isize
}
unsafe extern "C" fn rrtime_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { sysfs_emit(buf, c_str!("%d\n"), round_robin_time) }
static DEVICE_ATTR_RW!(rrtime);

unsafe extern "C" fn idlepct_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut num: c_ulong = 0;
    if kstrtoul(buf, 0, &mut num) != 0 || num < 1 || num >= 100 { return -EINVAL; }
    mutex_lock(&mut isolated_cpus_lock); idle_pct = num as u32; mutex_unlock(&mut isolated_cpus_lock); count as isize
}
unsafe extern "C" fn idlepct_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { sysfs_emit(buf, c_str!("%d\n"), idle_pct) }
static DEVICE_ATTR_RW!(idlepct);

unsafe extern "C" fn idlecpus_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut num: c_ulong = 0;
    if kstrtoul(buf, 0, &mut num) != 0 { return -EINVAL; }
    mutex_lock(&mut isolated_cpus_lock); acpi_pad_idle_cpus(num as u32); mutex_unlock(&mut isolated_cpus_lock); count as isize
}
unsafe extern "C" fn idlecpus_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { sysfs_emit(buf, c_str!("%*pb\n"), cpumask_pr_args(to_cpumask(&mut pad_busy_cpus_bits))) }
static DEVICE_ATTR_RW!(idlecpus);

static acpi_pad_attrs: [*mut attribute; 4] = [
    &dev_attr_idlecpus.attr, &dev_attr_idlepct.attr, &dev_attr_rrtime.attr, core::ptr::null_mut(),
];
ATTRIBUTE_GROUPS!(acpi_pad);

/* Query firmware how many CPUs should be idle; return -1 on failure. */
unsafe fn acpi_pad_pur(handle: acpi_handle) -> i32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut num = -1;
    if acpi_pad_teardown || ACPI_FAILURE(acpi_evaluate_object(handle, c_str!("_PUR"), core::ptr::null_mut(), &mut buffer)) != 0 || buffer.length == 0 || buffer.pointer.is_null() { return num; }
    let package = buffer.pointer as *mut acpi_object;
    if (*package).type_ == ACPI_TYPE_PACKAGE && (*package).package.count == 2 && (*package).package.elements[0].integer.value == 1 { num = (*package).package.elements[1].integer.value as i32; }
    kfree(buffer.pointer); num
}

unsafe fn acpi_pad_handle_notify(handle: acpi_handle) {
    let mut idle_cpus: u32 = 0;
    let mut param = acpi_buffer { length: 4, pointer: &mut idle_cpus as *mut _ as *mut c_void };
    mutex_lock(&mut isolated_cpus_lock);
    let num_cpus = acpi_pad_pur(handle);
    let status = if num_cpus < 0 { ACPI_PROCESSOR_AGGREGATOR_STATUS_NO_ACTION } else { acpi_pad_idle_cpus(num_cpus as u32); ACPI_PROCESSOR_AGGREGATOR_STATUS_SUCCESS };
    idle_cpus = acpi_pad_idle_cpus_num();
    acpi_evaluate_ost(handle, ACPI_PROCESSOR_AGGREGATOR_NOTIFY, status, &mut param);
    mutex_unlock(&mut isolated_cpus_lock);
}

unsafe extern "C" fn acpi_pad_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    if event != ACPI_PROCESSOR_AGGREGATOR_NOTIFY { pr_warn!("Unsupported event [0x%x]\n", event); return; }
    acpi_pad_handle_notify(handle); acpi_bus_generate_netlink_event(c_str!("acpi_pad"), dev_name(data), event, 0);
}

unsafe extern "C" fn acpi_pad_probe(pdev: *mut platform_device) -> i32 { acpi_pad_teardown = false; devm_acpi_install_notify_handler(&mut (*pdev).dev, ACPI_DEVICE_NOTIFY, acpi_pad_notify, &mut (*pdev).dev as *mut _ as *mut c_void) }
unsafe extern "C" fn acpi_pad_remove(_pdev: *mut platform_device) { mutex_lock(&mut isolated_cpus_lock); acpi_pad_teardown = true; acpi_pad_idle_cpus(0); mutex_unlock(&mut isolated_cpus_lock); }

static pad_device_ids: [acpi_device_id; 2] = [acpi_device_id { name: c_str!("ACPI000C"), driver_data: 0 }, acpi_device_id { name: c_str!(""), driver_data: 0 }];
MODULE_DEVICE_TABLE!(acpi, pad_device_ids);
static mut acpi_pad_driver: platform_driver = platform_driver { probe: acpi_pad_probe, remove: acpi_pad_remove, driver: driver { dev_groups: acpi_pad_groups, name: c_str!("processor_aggregator"), acpi_match_table: pad_device_ids.as_ptr() } };

unsafe extern "C" fn acpi_pad_init() -> i32 {
    /* Xen ACPI PAD is used when running as Xen Dom0. */
    if xen_initial_domain() { return -ENODEV; }
    power_saving_mwait_init();
    if power_saving_mwait_eax == 0 { return -EINVAL; }
    platform_driver_register(&mut acpi_pad_driver)
}
unsafe extern "C" fn acpi_pad_exit() { platform_driver_unregister(&mut acpi_pad_driver); }
module_init!(acpi_pad_init);
module_exit!(acpi_pad_exit);
MODULE_AUTHOR!("Shaohua Li<shaohua.li@intel.com>");
MODULE_DESCRIPTION!("ACPI Processor Aggregator Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
