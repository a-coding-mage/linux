/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Endless Mobile, Inc.
 * Author: Carlo Caione <carlo@endlessm.com>
 */

// Translated from the C header `meson_sm.h`.

pub const SM_EFUSE_READ: i32 = 0;
pub const SM_EFUSE_WRITE: i32 = 1;
pub const SM_EFUSE_USER_MAX: i32 = 2;
pub const SM_GET_CHIP_ID: i32 = 3;
pub const SM_THERMAL_CALIB_READ: i32 = 4;
pub const SM_A1_PWRC_SET: i32 = 5;
pub const SM_A1_PWRC_GET: i32 = 6;

#[repr(C)]
pub struct meson_sm_firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn meson_sm_call(
        fw: *mut meson_sm_firmware,
        cmd_index: u32,
        ret: *mut i32,
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
    ) -> i32;

    pub fn meson_sm_call_write(
        fw: *mut meson_sm_firmware,
        buffer: *mut core::ffi::c_void,
        b_size: u32,
        cmd_index: u32,
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
    ) -> i32;

    pub fn meson_sm_call_read(
        fw: *mut meson_sm_firmware,
        buffer: *mut core::ffi::c_void,
        bsize: u32,
        cmd_index: u32,
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
    ) -> i32;

    pub fn meson_sm_get(firmware_node: *mut device_node) -> *mut meson_sm_firmware;

    pub fn meson_sm_get_thermal_calib(
        fw: *mut meson_sm_firmware,
        trim_info: *mut u32,
        tsensor_id: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
