/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI Wakeup M3 remote processor platform data
 *
 * Copyright (C) 2014-2015 Texas Instruments, Inc.
 *
 * Dave Gerlach <d-gerlach@ti.com>
 */

use core::ffi::c_char;

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wkup_m3_platform_data {
    pub reset_name: *const c_char,

    pub assert_reset:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, name: *const c_char) -> i32>,
    pub deassert_reset:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, name: *const c_char) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
