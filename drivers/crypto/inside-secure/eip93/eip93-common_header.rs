/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

use core::ffi::c_void;

// Opaque declarations corresponding to the C structures supplied by other files.
#[repr(C)]
pub struct eip93_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eip93_descriptor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sa_record {
    _private: [u8; 0],
}

extern "C" {
    pub fn eip93_get_descriptor(eip93: *mut eip93_device) -> *mut c_void;

    pub fn eip93_put_descriptor(
        eip93: *mut eip93_device,
        desc: *mut eip93_descriptor,
    ) -> i32;

    pub fn eip93_set_sa_record(
        sa_record: *mut sa_record,
        keylen: u32,
        flags: u32,
    );

    pub fn eip93_parse_ctrl_stat_err(eip93: *mut eip93_device, err: i32) -> i32;

    pub fn eip93_hmac_setkey(
        ctx_flags: u32,
        key: *const u8,
        keylen: u32,
        hashlen: u32,
        ipad: *mut u8,
        opad: *mut u8,
        skip_ipad: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
