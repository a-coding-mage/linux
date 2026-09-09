// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2022-23 IBM Corp.
 */

// #define pr_fmt(fmt) "vas: " fmt
// C dependencies: linux/module.h, linux/kernel.h, linux/miscdevice.h,
// linux/kobject.h, linux/slab.h, linux/sysfs.h, linux/mm.h, and "vas.h".

#[cfg(CONFIG_SYSFS)]
static mut PSERIES_VAS_KOBJ: *mut kobject = core::ptr::null_mut();
#[cfg(CONFIG_SYSFS)]
static mut GZIP_CAPS_KOBJ: *mut kobject = core::ptr::null_mut();

#[cfg(CONFIG_SYSFS)]
#[repr(C)]
struct vas_caps_entry {
    kobj: kobject,
    caps: *mut vas_cop_feat_caps,
}

#[cfg(CONFIG_SYSFS)]
#[repr(C)]
struct vas_sysfs_entry {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut vas_cop_feat_caps, *mut core::ffi::c_char) -> isize>,
    store: Option<unsafe extern "C" fn(*mut vas_cop_feat_caps, *const core::ffi::c_char, usize) -> isize>,
}

#[cfg(CONFIG_SYSFS)]
unsafe fn update_total_credits_store(
    caps: *mut vas_cop_feat_caps,
    buf: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let mut creds: u16 = 0;
    let mut err = kstrtou16(buf, 0, &mut creds);
    /*
     * The user space interface from the management console
     * notifies OS with the new QoS credits and then the
     * hypervisor. So OS has to use this new credits value
     * and reconfigure VAS windows (close or reopen depends
     * on the credits available) instead of depending on VAS
     * QoS capabilities from the hypervisor.
     */
    if err == 0 {
        err = vas_reconfig_capabilties((*caps).win_type, creds);
    }
    if err != 0 {
        return -EINVAL;
    }
    pr_info!("Set QoS total credits {}\n", creds);
    count as isize
}

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn nr_total_credits_show(caps: *mut vas_cop_feat_caps, buf: *mut core::ffi::c_char) -> isize {
    sysfs_emit(buf, "%d\n", atomic_read(&(*caps).nr_total_credits))
}

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn nr_used_credits_show(caps: *mut vas_cop_feat_caps, buf: *mut core::ffi::c_char) -> isize {
    sysfs_emit(buf, "%d\n", atomic_read(&(*caps).nr_used_credits))
}

#[cfg(CONFIG_SYSFS)]
static mut NR_TOTAL_CREDITS_ATTRIBUTE: vas_sysfs_entry = vas_sysfs_entry {
    attr: __ATTR!("nr_total_credits", 0o444, Some(nr_total_credits_show), None),
    show: Some(nr_total_credits_show),
    store: None,
};

#[cfg(CONFIG_SYSFS)]
static mut NR_USED_CREDITS_ATTRIBUTE: vas_sysfs_entry = vas_sysfs_entry {
    attr: __ATTR!("nr_used_credits", 0o444, Some(nr_used_credits_show), None),
    show: Some(nr_used_credits_show),
    store: None,
};

#[cfg(CONFIG_SYSFS)]
static mut UPDATE_TOTAL_CREDITS_ATTRIBUTE: vas_sysfs_entry = vas_sysfs_entry {
    attr: __ATTR!("update_total_credits", 0o200, None, Some(update_total_credits_store)),
    show: None,
    store: Some(update_total_credits_store),
};

#[cfg(CONFIG_SYSFS)]
static mut VAS_DEF_CAPAB_ATTRS: [*mut attribute; 3] = [
    unsafe { &mut NR_TOTAL_CREDITS_ATTRIBUTE.attr },
    unsafe { &mut NR_USED_CREDITS_ATTRIBUTE.attr },
    core::ptr::null_mut(),
];

#[cfg(CONFIG_SYSFS)]
static mut VAS_QOS_CAPAB_ATTRS: [*mut attribute; 4] = [
    unsafe { &mut NR_TOTAL_CREDITS_ATTRIBUTE.attr },
    unsafe { &mut NR_USED_CREDITS_ATTRIBUTE.attr },
    unsafe { &mut UPDATE_TOTAL_CREDITS_ATTRIBUTE.attr },
    core::ptr::null_mut(),
];

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn vas_type_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut core::ffi::c_char) -> isize {
    let centry = container_of!(kobj, vas_caps_entry, kobj);
    let entry = container_of!(attr, vas_sysfs_entry, attr);
    match (*entry).show {
        Some(show) => show((*centry).caps, buf),
        None => -EIO,
    }
}

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn vas_type_store(kobj: *mut kobject, attr: *mut attribute, buf: *const core::ffi::c_char, count: usize) -> isize {
    let centry = container_of!(kobj, vas_caps_entry, kobj);
    let entry = container_of!(attr, vas_sysfs_entry, attr);
    match (*entry).store {
        Some(store) => store((*centry).caps, buf, count),
        None => -EIO,
    }
}

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn vas_type_release(kobj: *mut kobject) {
    let centry = container_of!(kobj, vas_caps_entry, kobj);
    kfree(centry as *mut core::ffi::c_void);
}

#[cfg(CONFIG_SYSFS)]
static VAS_SYSFS_OPS: sysfs_ops = sysfs_ops { show: Some(vas_type_show), store: Some(vas_type_store) };

#[cfg(CONFIG_SYSFS)]
static VAS_DEF_ATTR_TYPE: kobj_type = kobj_type { release: Some(vas_type_release), sysfs_ops: &VAS_SYSFS_OPS, default_groups: vas_def_capab_groups };

#[cfg(CONFIG_SYSFS)]
static VAS_QOS_ATTR_TYPE: kobj_type = kobj_type { release: Some(vas_type_release), sysfs_ops: &VAS_SYSFS_OPS, default_groups: vas_qos_capab_groups };

#[cfg(CONFIG_SYSFS)]
unsafe fn vas_caps_kobj_name(centry: *mut vas_caps_entry, kobj: *mut *mut kobject) -> *const core::ffi::c_char {
    let caps = (*centry).caps;
    if (*caps).descriptor == VAS_GZIP_QOS_CAPABILITIES {
        kobject_init(&mut (*centry).kobj, &VAS_QOS_ATTR_TYPE);
        *kobj = GZIP_CAPS_KOBJ;
        b"qos_capabilities\0".as_ptr() as *const _
    } else if (*caps).descriptor == VAS_GZIP_DEFAULT_CAPABILITIES {
        kobject_init(&mut (*centry).kobj, &VAS_DEF_ATTR_TYPE);
        *kobj = GZIP_CAPS_KOBJ;
        b"default_capabilities\0".as_ptr() as *const _
    } else {
        b"Unknown\0".as_ptr() as *const _
    }
}

#[cfg(CONFIG_SYSFS)]
pub unsafe extern "C" fn sysfs_add_vas_caps(caps: *mut vas_cop_feat_caps) -> i32 {
    let centry = kzalloc_obj::<vas_caps_entry>();
    if centry.is_null() { return -ENOMEM; }
    (*centry).caps = caps;
    let mut kobj = core::ptr::null_mut();
    let name = vas_caps_kobj_name(centry, &mut kobj);
    if !kobj.is_null() {
        let ret = kobject_add(&mut (*centry).kobj, kobj, b"%s\0".as_ptr() as *const _, name);
        if ret != 0 { pr_err!("VAS: sysfs kobject add / event failed {}\n", ret); kobject_put(&mut (*centry).kobj); }
        return ret;
    }
    0
}

static mut VAS_MISCDEV: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"vas\0".as_ptr() as *const _ };

#[cfg(CONFIG_SYSFS)]
pub unsafe extern "C" fn sysfs_pseries_vas_init(vas_caps: *mut vas_all_caps) -> i32 {
    let ret = misc_register(&mut VAS_MISCDEV);
    if ret < 0 { pr_err!("{}: register vas misc device failed\n", __func__); return ret; }
    PSERIES_VAS_KOBJ = kobject_create_and_add(b"vas0\0".as_ptr() as *const _, (*VAS_MISCDEV.this_device).kobj.as_mut());
    if PSERIES_VAS_KOBJ.is_null() { misc_deregister(&mut VAS_MISCDEV); pr_err!("Failed to create VAS sysfs entry\n"); return -ENOMEM; }
    if ((*vas_caps).feat_type & VAS_GZIP_QOS_FEAT_BIT) != 0 || ((*vas_caps).feat_type & VAS_GZIP_DEF_FEAT_BIT) != 0 {
        GZIP_CAPS_KOBJ = kobject_create_and_add(b"gzip\0".as_ptr() as *const _, PSERIES_VAS_KOBJ);
        if GZIP_CAPS_KOBJ.is_null() { pr_err!("Failed to create VAS GZIP capability entry\n"); kobject_put(PSERIES_VAS_KOBJ); misc_deregister(&mut VAS_MISCDEV); return -ENOMEM; }
    }
    0
}

#[cfg(not(CONFIG_SYSFS))]
pub unsafe extern "C" fn sysfs_add_vas_caps(_caps: *mut vas_cop_feat_caps) -> i32 { 0 }

#[cfg(not(CONFIG_SYSFS))]
pub unsafe extern "C" fn sysfs_pseries_vas_init(_vas_caps: *mut vas_all_caps) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
