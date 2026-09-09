// SPDX-License-Identifier: GPL-2.0
/*
 * Ultravisor high level interfaces
 *
 * Copyright 2019, IBM Corporation.
 *
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct memcons {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bin_attribute {
    pub attr: attribute,
    pub size: usize,
    pub read: Option<unsafe extern "C" fn(
        file: *mut file,
        kobj: *mut kobject,
        bin_attr: *const bin_attribute,
        to: *mut c_char,
        pos: i64,
        count: usize,
    ) -> isize>,
}

#[repr(C)]
pub struct attribute {
    pub name: *const c_char,
    pub mode: u16,
}

extern "C" {
    static mut powerpc_firmware_features: u64;
    static mut firmware_kobj: *mut kobject;

    fn of_flat_dt_is_compatible(node: usize, compat: *const c_char) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn firmware_has_feature(feature: u64) -> c_int;
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn memcons_init(node: *mut device_node, name: *const c_char) -> *mut memcons;
    fn of_node_put(node: *mut device_node);
    fn memcons_get_size(memcons: *mut memcons) -> usize;
    fn memcons_copy(memcons: *mut memcons, to: *mut c_char, pos: i64, count: usize) -> isize;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *mut bin_attribute) -> c_int;
}

// FW_FEATURE_ULTRAVISOR is supplied by asm/firmware.h.
extern "C" {
    static FW_FEATURE_ULTRAVISOR: u64;
}

static mut ultravisor_kobj: *mut kobject = core::ptr::null_mut();
static mut uv_memcons: *mut memcons = core::ptr::null_mut();

static mut UV_MSGLOG_ATTR: bin_attribute = bin_attribute {
    attr: attribute {
        name: b"msglog\0".as_ptr() as *const c_char,
        mode: 0o400,
    },
    size: 0,
    read: Some(uv_msglog_read),
};

#[no_mangle]
pub unsafe extern "C" fn early_init_dt_scan_ultravisor(
    node: usize,
    _uname: *const c_char,
    _depth: c_int,
    _data: *mut c_void,
) -> c_int {
    if of_flat_dt_is_compatible(node, b"ibm,ultravisor\0".as_ptr() as *const c_char) == 0 {
        return 0;
    }

    powerpc_firmware_features |= FW_FEATURE_ULTRAVISOR;
    pr_debug(b"Ultravisor detected!\n\0".as_ptr() as *const c_char);
    1
}

unsafe extern "C" fn uv_msglog_read(
    _file: *mut file,
    _kobj: *mut kobject,
    _bin_attr: *const bin_attribute,
    to: *mut c_char,
    pos: i64,
    count: usize,
) -> isize {
    memcons_copy(uv_memcons, to, pos, count)
}

#[no_mangle]
pub unsafe extern "C" fn uv_init() -> c_int {
    let node: *mut device_node;

    if firmware_has_feature(FW_FEATURE_ULTRAVISOR) == 0 {
        return 0;
    }

    node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"ibm,uv-firmware\0".as_ptr() as *const c_char,
    );
    if node.is_null() {
        return -19; // -ENODEV
    }

    uv_memcons = memcons_init(node, b"memcons\0".as_ptr() as *const c_char);
    of_node_put(node);
    if uv_memcons.is_null() {
        return -2; // -ENOENT
    }

    UV_MSGLOG_ATTR.size = memcons_get_size(uv_memcons);

    ultravisor_kobj = kobject_create_and_add(
        b"ultravisor\0".as_ptr() as *const c_char,
        firmware_kobj,
    );
    if ultravisor_kobj.is_null() {
        return -12; // -ENOMEM
    }

    sysfs_create_bin_file(ultravisor_kobj, &mut UV_MSGLOG_ATTR)
}

// machine_subsys_initcall(powernv, uv_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
