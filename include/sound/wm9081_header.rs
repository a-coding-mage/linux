/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm9081.h -- Platform data for WM9081
 *
 * Copyright 2009 Wolfson Microelectronics. PLC.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct wm9081_retune_mobile_setting {
    pub name: *const c_char,
    pub rate: u32,
    pub config: [u16; 20],
}

#[repr(C)]
pub struct wm9081_pdata {
    pub irq_high: bool,   /* IRQ is active high */
    pub irq_cmos: bool,   /* IRQ is in CMOS mode */

    pub retune_configs: *mut wm9081_retune_mobile_setting,
    pub num_retune_configs: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
