// SPDX-License-Identifier: GPL-2.0+
/*
 * Marvell Armada AP and CP110 helper
 *
 * Copyright (C) 2018 Marvell
 *
 * Gregory Clement <gregory.clement@bootlin.com>
 *
 */

// Translated from armada_ap_cp_helper.h and the Linux device-tree/address APIs.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
    pub flags: c_ulonglong,
    pub desc: c_ulonglong,
}

// External symbols supplied by the Linux and platform headers.
unsafe extern "C" {
    pub static GFP_KERNEL: c_uint;

    pub fn of_address_to_resource(
        np: *mut device_node,
        index: c_int,
        res: *mut resource,
    ) -> c_int;

    pub fn devm_kasprintf(
        dev: *mut device,
        gfp: c_uint,
        fmt: *const c_char,
        ...,
    ) -> *mut c_char;
}

pub unsafe fn ap_cp_unique_name(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
) -> *mut c_char {
    let mut res: resource = core::mem::zeroed();

    /* Do not create a name if there is no clock */
    if name.is_null() {
        return core::ptr::null_mut();
    }

    of_address_to_resource(np, 0, &mut res);
    devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"%llx-%s\0".as_ptr() as *const c_char,
        res.start as c_ulonglong,
        name,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
