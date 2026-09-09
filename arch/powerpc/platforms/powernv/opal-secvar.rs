// SPDX-License-Identifier: GPL-2.0
/*
 * PowerNV code for secure variables
 *
 * Copyright (C) 2019 IBM Corporation
 * Author: Claudio Carvalho
 *         Nayna Jain
 *
 * APIs to access secure variables managed by OPAL.
 */

// Kernel dependencies supplied by the surrounding Rust port.

use core::ffi::{c_char, c_int, c_void};

type __be64 = u64;
type u8_t = u8;
type u64_t = u64;
type ssize_t = isize;

extern "C" {
    fn opal_secvar_get(key: *const c_char, ksize: u64_t, data: *mut u8_t,
                       dsize: *mut u64_t) -> c_int;
    fn opal_secvar_get_next(key: *const c_char, keylen: *mut u64_t,
                            keybufsize: u64_t) -> c_int;
    fn opal_secvar_enqueue_update(key: *const c_char, ksize: u64_t,
                                  data: *mut u8_t, dsize: u64_t) -> c_int;
    fn opal_check_token(token: c_int) -> bool;
    fn set_secvar_ops(ops: *const secvar_operations) -> c_int;
    fn of_find_compatible_node(from: *mut device_node, type_: *const c_char,
                               compatible: *const c_char) -> *mut device_node;
    fn of_device_is_available(node: *mut device_node) -> bool;
    fn of_property_read_string(node: *mut device_node, name: *const c_char,
                               value: *mut *const c_char) -> c_int;
    fn of_property_read_u64(node: *mut device_node, name: *const c_char,
                            value: *mut u64_t) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn platform_driver_probe(driver: *mut platform_driver,
                             probe: unsafe extern "C" fn(*mut platform_device) -> c_int) -> c_int;
}

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }

#[repr(C)]
pub struct secvar_operations {
    pub get: Option<unsafe extern "C" fn(*const c_char, u64_t, *mut u8_t, *mut u64_t) -> c_int>,
    pub get_next: Option<unsafe extern "C" fn(*const c_char, *mut u64_t, u64_t) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*const c_char, u64_t, *mut u8_t, u64_t) -> c_int>,
    pub format: Option<unsafe extern "C" fn(*mut c_char, usize) -> ssize_t>,
    pub max_size: Option<unsafe extern "C" fn(*mut u64_t) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
}

const OPAL_SUCCESS: c_int = 0;
const OPAL_UNSUPPORTED: c_int = -1;
const OPAL_PARAMETER: c_int = -2;
const OPAL_RESOURCE: c_int = -3;
const OPAL_HARDWARE: c_int = -4;
const OPAL_NO_MEM: c_int = -5;
const OPAL_EMPTY: c_int = -6;
const OPAL_PARTIAL: c_int = -7;
const OPAL_SECVAR_GET: c_int = 0;
const OPAL_SECVAR_GET_NEXT: c_int = 0;
const OPAL_SECVAR_ENQUEUE_UPDATE: c_int = 0;
const ENXIO: c_int = 6;
const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EFBIG: c_int = 27;
const ENODEV: c_int = 19;

unsafe fn opal_status_to_err(rc: c_int) -> c_int {
    match rc {
        OPAL_SUCCESS => 0,
        OPAL_UNSUPPORTED => -ENXIO,
        OPAL_PARAMETER => -EINVAL,
        OPAL_RESOURCE => -ENOSPC,
        OPAL_HARDWARE => -EIO,
        OPAL_NO_MEM => -ENOMEM,
        OPAL_EMPTY => -ENOENT,
        OPAL_PARTIAL => -EFBIG,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn opal_get_variable(key: *const c_char, ksize: u64_t,
                                        data: *mut u8_t, dsize: *mut u64_t) -> c_int {
    if key.is_null() || dsize.is_null() { return -EINVAL; }
    *dsize = (*dsize).to_be();
    let rc = opal_secvar_get(key, ksize, data, dsize);
    *dsize = u64::from_be(*dsize);
    opal_status_to_err(rc)
}

unsafe extern "C" fn opal_get_next_variable(key: *const c_char, keylen: *mut u64_t,
                                             keybufsize: u64_t) -> c_int {
    if key.is_null() || keylen.is_null() { return -EINVAL; }
    *keylen = (*keylen).to_be();
    let rc = opal_secvar_get_next(key, keylen, keybufsize);
    *keylen = u64::from_be(*keylen);
    opal_status_to_err(rc)
}

unsafe extern "C" fn opal_set_variable(key: *const c_char, ksize: u64_t,
                                        data: *mut u8_t, dsize: u64_t) -> c_int {
    if key.is_null() || data.is_null() { return -EINVAL; }
    opal_status_to_err(opal_secvar_enqueue_update(key, ksize, data, dsize))
}

unsafe extern "C" fn opal_secvar_format(buf: *mut c_char, bufsize: usize) -> ssize_t {
    let mut rc: ssize_t = 0;
    let mut node;
    let mut format: *const c_char = core::ptr::null();
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ibm,secvar-backend".as_ptr());
    if !of_device_is_available(node) { rc = -ENODEV as ssize_t; } else {
        rc = of_property_read_string(node, c"format".as_ptr(), &mut format) as ssize_t;
        if rc == 0 { rc = snprintf(buf, bufsize, c"%s".as_ptr(), format) as ssize_t; }
    }
    of_node_put(node);
    rc
}

unsafe extern "C" fn opal_secvar_max_size(max_size: *mut u64_t) -> c_int {
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ibm,secvar-backend".as_ptr());
    if node.is_null() { return -ENODEV; }
    let rc = if !of_device_is_available(node) { -ENODEV } else {
        of_property_read_u64(node, c"max-var-size".as_ptr(), max_size)
    };
    of_node_put(node);
    rc
}

static OPAL_SECVAR_OPS: secvar_operations = secvar_operations {
    get: Some(opal_get_variable), get_next: Some(opal_get_next_variable),
    set: Some(opal_set_variable), format: Some(opal_secvar_format),
    max_size: Some(opal_secvar_max_size),
};

unsafe extern "C" fn opal_secvar_probe(_pdev: *mut platform_device) -> c_int {
    if !opal_check_token(OPAL_SECVAR_GET) || !opal_check_token(OPAL_SECVAR_GET_NEXT) ||
       !opal_check_token(OPAL_SECVAR_ENQUEUE_UPDATE) { return -ENODEV; }
    set_secvar_ops(&OPAL_SECVAR_OPS)
}

static OPAL_SECVAR_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"ibm,secvar-backend".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut OPAL_SECVAR_DRIVER: platform_driver = platform_driver { driver: device_driver {
    name: c"secvar".as_ptr(), of_match_table: OPAL_SECVAR_MATCH.as_ptr(),
} };

unsafe extern "C" fn opal_secvar_init() -> c_int {
    platform_driver_probe(&mut OPAL_SECVAR_DRIVER, opal_secvar_probe)
}

// device_initcall(opal_secvar_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
