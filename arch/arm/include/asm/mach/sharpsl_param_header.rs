/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Hardware parameter area specific to Sharp SL series devices
 *
 * Copyright (c) 2005 Richard Purdie
 *
 * Based on Sharp's 2.4 kernel patches
 */

#[repr(C, packed)]
pub struct sharpsl_param_info {
    pub comadj_keyword: ::core::ffi::c_uint,
    pub comadj: ::core::ffi::c_uint,

    pub uuid_keyword: ::core::ffi::c_uint,
    pub uuid: [u8; 16],

    pub touch_keyword: ::core::ffi::c_uint,
    pub touch_xp: ::core::ffi::c_uint,
    pub touch_yp: ::core::ffi::c_uint,
    pub touch_xd: ::core::ffi::c_uint,
    pub touch_yd: ::core::ffi::c_uint,

    pub adadj_keyword: ::core::ffi::c_uint,
    pub adadj: ::core::ffi::c_uint,

    pub phad_keyword: ::core::ffi::c_uint,
    pub phadadj: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub static mut sharpsl_param: sharpsl_param_info;
    pub fn sharpsl_save_param();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
