/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5514.h -- Platform data for RT5514
 *
 * Copyright 2016 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

use core::ffi::c_char;

#[repr(C)]
pub struct rt5514_platform_data {
    pub dmic_init_delay: core::ffi::c_uint,
    pub dsp_calib_clk_name: *const c_char,
    pub dsp_calib_clk_rate: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
