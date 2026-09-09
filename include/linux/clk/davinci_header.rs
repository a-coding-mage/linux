// SPDX-License-Identifier: GPL-2.0
/*
 * Clock drivers for TI DaVinci PLL and PSC controllers
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// C dependencies: <linux/device.h> and <linux/regmap.h>

use core::ffi::c_void;

/* Opaque types supplied by the corresponding Linux dependencies. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

/* function for registering clocks in early boot */
unsafe extern "C" {
    pub fn da850_pll0_init(
        dev: *mut device,
        base: *mut c_void,
        cfgchip: *mut regmap,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
