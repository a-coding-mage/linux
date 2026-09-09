// SPDX-License-Identifier: GPL-2.0-only
/*
 * sys.c
 *
 * OCFS2 cluster sysfs interface
 *
 * Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Linux kernel dependencies and declarations supplied by the surrounding
// OCFS2 translation unit are intentionally referenced here without copies.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn kset_create_and_add(
        name: *const c_char,
        parent: *mut kset,
        kobj: *mut kobject,
    ) -> *mut kset;
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn kset_unregister(kset: *mut kset);
    fn mlog_sys_shutdown();
    fn mlog_sys_init(kset: *mut kset) -> c_int;
}

extern "C" {
    static mut fs_kobj: *mut kobject;
    static O2NM_API_VERSION: c_uint;
    static PAGE_SIZE: usize;
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kset {
    pub kobj: kobject,
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

// `__ATTR(interface_revision, S_IRUGO, version_show, NULL)` is the direct
// translation of the kernel macro initializer; its remaining fields are
// supplied by the kernel declaration of kobj_attribute.
static mut attr_version: kobj_attribute = unsafe { core::mem::zeroed() };

static mut o2cb_attrs: [*mut attribute; 2] = [
    unsafe { &raw mut attr_version.attr },
    core::ptr::null_mut(),
];

static mut o2cb_attr_group: attribute_group = attribute_group {
    attrs: unsafe { &raw mut o2cb_attrs[0] },
};

static mut o2cb_kset: *mut kset = core::ptr::null_mut();

unsafe extern "C" fn version_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> isize {
    snprintf(
        buf,
        PAGE_SIZE,
        b"%u\n\0".as_ptr() as *const c_char,
        O2NM_API_VERSION,
    ) as isize
}

pub unsafe extern "C" fn o2cb_sys_shutdown() {
    mlog_sys_shutdown();
    kset_unregister(o2cb_kset);
}

pub unsafe extern "C" fn o2cb_sys_init() -> c_int {
    let mut ret: c_int;

    o2cb_kset = kset_create_and_add(
        b"o2cb\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        fs_kobj,
    );
    if o2cb_kset.is_null() {
        return -12; // -ENOMEM
    }

    ret = sysfs_create_group(&mut (*o2cb_kset).kobj, &o2cb_attr_group);
    if ret != 0 {
        kset_unregister(o2cb_kset);
        return ret;
    }

    ret = mlog_sys_init(o2cb_kset);
    if ret != 0 {
        kset_unregister(o2cb_kset);
        return ret;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
