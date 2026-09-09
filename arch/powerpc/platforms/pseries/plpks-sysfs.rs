// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 IBM Corporation, Srish Srinivasan <ssrish@linux.ibm.com>
 *
 * This code exposes PLPKS config to user via sysfs
 */

// pr_fmt(fmt) = "plpks-sysfs: " fmt

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

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
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

extern "C" {
    static mut firmware_kobj: *mut kobject;

    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_create_link(from: *mut kobject, to: *mut kobject, name: *const c_char) -> c_int;
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn pr_err(fmt: *const c_char, ...);

    fn plpks_get_version() -> u32;
    fn plpks_get_maxobjectsize() -> u32;
    fn plpks_get_totalsize() -> u32;
    fn plpks_get_usedspace() -> u32;
    fn plpks_get_supportedpolicies() -> u32;
    fn plpks_get_signedupdatealgorithms() -> u64;
    fn plpks_get_wrappingfeatures() -> u64;
    fn plpks_is_available() -> bool;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

// PLPKS_CONFIG_ATTR(version, "%u\n", plpks_get_version)
unsafe extern "C" fn version_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, plpks_get_version())
}

// PLPKS_CONFIG_ATTR(max_object_size, "%u\n", plpks_get_maxobjectsize)
unsafe extern "C" fn max_object_size_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, plpks_get_maxobjectsize())
}

// PLPKS_CONFIG_ATTR(total_size, "%u\n", plpks_get_totalsize)
unsafe extern "C" fn total_size_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, plpks_get_totalsize())
}

// PLPKS_CONFIG_ATTR(used_space, "%u\n", plpks_get_usedspace)
unsafe extern "C" fn used_space_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, plpks_get_usedspace())
}

// PLPKS_CONFIG_ATTR(supported_policies, "%08x\n", plpks_get_supportedpolicies)
unsafe extern "C" fn supported_policies_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(buf, b"%08x\n\0".as_ptr() as *const c_char, plpks_get_supportedpolicies())
}

// PLPKS_CONFIG_ATTR(signed_update_algorithms, "%016llx\n", plpks_get_signedupdatealgorithms)
unsafe extern "C" fn signed_update_algorithms_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(
        buf,
        b"%016llx\n\0".as_ptr() as *const c_char,
        plpks_get_signedupdatealgorithms(),
    )
}

// PLPKS_CONFIG_ATTR(wrapping_features, "%016llx\n", plpks_get_wrappingfeatures)
unsafe extern "C" fn wrapping_features_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    sysfs_emit(
        buf,
        b"%016llx\n\0".as_ptr() as *const c_char,
        plpks_get_wrappingfeatures(),
    )
}

// The __ATTR_RO-generated attribute objects are supplied with their kernel metadata here.
static mut attr_version: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_max_object_size: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_total_size: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_used_space: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_supported_policies: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_signed_update_algorithms: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };
static mut attr_wrapping_features: kobj_attribute = kobj_attribute { attr: attribute { _private: [] }, _private: [] };

static mut config_attrs: [*const attribute; 8] = [
    unsafe { &attr_version.attr },
    unsafe { &attr_max_object_size.attr },
    unsafe { &attr_total_size.attr },
    unsafe { &attr_used_space.attr },
    unsafe { &attr_supported_policies.attr },
    unsafe { &attr_signed_update_algorithms.attr },
    unsafe { &attr_wrapping_features.attr },
    core::ptr::null(),
];

static mut plpks_kobj: *mut kobject = core::ptr::null_mut();
static mut plpks_config_kobj: *mut kobject = core::ptr::null_mut();

pub unsafe fn plpks_config_create_softlink(from: *mut kobject) -> c_int {
    if plpks_config_kobj.is_null() {
        return -EINVAL;
    }
    sysfs_create_link(from, plpks_config_kobj, b"config\0".as_ptr() as *const c_char)
}

unsafe fn plpks_sysfs_config(kobj: *mut kobject) -> c_int {
    let config_group = attribute_group {
        name: core::ptr::null(),
        attrs: config_attrs.as_mut_ptr() as *mut *mut attribute,
    };

    sysfs_create_group(kobj, &config_group)
}

unsafe fn plpks_sysfs_init() -> c_int {
    let rc: c_int;

    if !plpks_is_available() {
        return -ENODEV;
    }

    plpks_kobj = kobject_create_and_add(b"plpks\0".as_ptr() as *const c_char, firmware_kobj);
    if plpks_kobj.is_null() {
        pr_err(b"plpks-sysfs: Failed to create plpks kobj\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    plpks_config_kobj =
        kobject_create_and_add(b"config\0".as_ptr() as *const c_char, plpks_kobj);
    if plpks_config_kobj.is_null() {
        pr_err(b"plpks-sysfs: Failed to create plpks config kobj\n\0".as_ptr() as *const c_char);
        kobject_put(plpks_kobj);
        return -ENOMEM;
    }

    rc = plpks_sysfs_config(plpks_config_kobj);
    if rc != 0 {
        pr_err(
            b"plpks-sysfs: Failed to create attribute group for plpks config\n\0".as_ptr()
                as *const c_char,
        );
        kobject_put(plpks_config_kobj);
        kobject_put(plpks_kobj);
        return rc;
    }

    0
}

// machine_subsys_initcall(pseries, plpks_sysfs_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
