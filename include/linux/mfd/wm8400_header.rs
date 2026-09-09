/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8400 client interface
 *
 * Copyright 2008 Wolfson Microelectronics plc
 */

// Dependency intent: equivalent of <linux/regulator/machine.h>.
use core::ffi::c_int;

pub const WM8400_LDO1: c_int = 0;
pub const WM8400_LDO2: c_int = 1;
pub const WM8400_LDO3: c_int = 2;
pub const WM8400_LDO4: c_int = 3;
pub const WM8400_DCDC1: c_int = 4;
pub const WM8400_DCDC2: c_int = 5;

// External types supplied by other dependencies.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm8400_platform_data {
    pub platform_init: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

unsafe extern "C" {
    pub fn wm8400_register_regulator(
        dev: *mut device,
        reg: c_int,
        initdata: *mut regulator_init_data,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
