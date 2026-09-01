/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022, The Linux Foundation. All rights reserved.
 */

use core::ffi::{c_char, c_int, c_void};

/* NPL clock is expected */
pub const LPASS_MACRO_FLAG_HAS_NPL_CLOCK: u32 = 1u32 << 0;
/* The soundwire block should be internally reset at probe */
pub const LPASS_MACRO_FLAG_RESET_SWR: u32 = 1u32 << 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum lpass_version {
    LPASS_VER_9_0_0,
    LPASS_VER_9_2_0,
    LPASS_VER_10_0_0,
    LPASS_VER_11_0_0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum lpass_codec_version {
    LPASS_CODEC_VERSION_UNKNOWN,
    LPASS_CODEC_VERSION_1_0,
    LPASS_CODEC_VERSION_1_1,
    LPASS_CODEC_VERSION_1_2,
    LPASS_CODEC_VERSION_2_0,
    LPASS_CODEC_VERSION_2_1,
    LPASS_CODEC_VERSION_2_5,
    LPASS_CODEC_VERSION_2_6,
    LPASS_CODEC_VERSION_2_7,
    LPASS_CODEC_VERSION_2_8,
    LPASS_CODEC_VERSION_2_9,
}

#[repr(C)]
pub struct lpass_macro {
    pub macro_pd: *mut device,
    pub dcodec_pd: *mut device,
}

unsafe extern "C" {
    pub fn lpass_macro_pds_init(dev: *mut device) -> *mut lpass_macro;
    pub fn lpass_macro_pds_exit(pds: *mut lpass_macro);
    pub fn lpass_macro_set_codec_version(version: lpass_codec_version);
    pub fn lpass_macro_get_codec_version() -> lpass_codec_version;
}

#[inline]
pub unsafe fn lpass_macro_pds_exit_action(pds: *mut c_void) {
    unsafe {
        lpass_macro_pds_exit(pds as *mut lpass_macro);
    }
}

#[inline]
pub unsafe fn lpass_macro_get_codec_version_string(version: c_int) -> *const c_char {
    match version {
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_1_0 as c_int => c"v1.0".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_1_1 as c_int => c"v1.1".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_1_2 as c_int => c"v1.2".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_0 as c_int => c"v2.0".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_1 as c_int => c"v2.1".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_5 as c_int => c"v2.5".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_6 as c_int => c"v2.6".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_7 as c_int => c"v2.7".as_ptr(),
        x if x == lpass_codec_version::LPASS_CODEC_VERSION_2_8 as c_int => c"v2.8".as_ptr(),
        _ => c"NA".as_ptr(),
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
