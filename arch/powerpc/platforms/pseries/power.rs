// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Interface for power-management for ppc64 compliant platform
 *
 *  Manish Ahuja <mahuja@us.ibm.com>
 *
 *  Feb 2007
 *
 *  Copyright (C) 2007 IBM Corporation.
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sscanf(buf: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn sysfs_create_file(kobj: *mut kobject, attr: *mut attribute) -> c_int;
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
}

pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;

pub static mut rtas_poweron_auto: c_ulong = 0; /* default and normal state is 0 */

unsafe extern "C" fn auto_poweron_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%lu\0".as_ptr() as *const c_char, rtas_poweron_auto) as isize
}

unsafe extern "C" fn auto_poweron_store(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    n: usize,
) -> isize {
    let mut ret: c_int;
    let mut ups_restart: c_ulong = 0;
    ret = sscanf(
        buf,
        b"%lu\0".as_ptr() as *const c_char,
        &mut ups_restart as *mut c_ulong,
    );

    if (ret == 1) && ((ups_restart == 1) || (ups_restart == 0)) {
        rtas_poweron_auto = ups_restart;
        return n as isize;
    }
    -(EINVAL as isize)
}

// __ATTR(auto_poweron, 0644, auto_poweron_show, auto_poweron_store)
static mut auto_poweron_attr: kobj_attribute = kobj_attribute {
    attr: attribute { _private: [] },
    _private: [],
};

// The following block corresponds to the !CONFIG_PM build-time condition.
#[cfg(not(CONFIG_PM))]
pub static mut power_kobj: *mut kobject = core::ptr::null_mut();

#[cfg(not(CONFIG_PM))]
static mut g: [*mut attribute; 2] = [
    unsafe { &mut auto_poweron_attr.attr as *mut attribute },
    core::ptr::null_mut(),
];

#[cfg(not(CONFIG_PM))]
static attr_group: attribute_group = attribute_group {
    attrs: unsafe { g.as_ptr() as *mut *mut attribute },
};

#[cfg(not(CONFIG_PM))]
unsafe extern "C" fn pm_init() -> c_int {
    power_kobj = kobject_create_and_add(b"power\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if power_kobj.is_null() {
        return -(ENOMEM as c_int);
    }
    sysfs_create_group(power_kobj, &attr_group)
}

// machine_core_initcall(pseries, pm_init);

// The following block corresponds to the CONFIG_PM build-time condition.
#[cfg(CONFIG_PM)]
unsafe extern "C" fn apo_pm_init() -> c_int {
    sysfs_create_file(power_kobj, &mut auto_poweron_attr.attr)
}

// machine_device_initcall(pseries, apo_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
