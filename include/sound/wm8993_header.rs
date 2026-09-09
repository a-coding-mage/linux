/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm8993.h -- Platform data for WM8993
 *
 * Copyright 2009 Wolfson Microelectronics. PLC.
 */

/* Note that EQ1 only contains the enable/disable bit so will be
   ignored but is included for simplicity.
 */
#[repr(C)]
pub struct wm8993_retune_mobile_setting {
    pub name: *const ::core::ffi::c_char,
    pub rate: u32,
    pub config: [u16; 24],
}

#[repr(C)]
pub struct wm8993_platform_data {
    pub retune_configs: *mut wm8993_retune_mobile_setting,
    pub num_retune_configs: i32,

    /* LINEOUT can be differential or single ended */
    pub lineout1_diff: u32,
    pub lineout2_diff: u32,

    /* Common mode feedback */
    pub lineout1fb: u32,
    pub lineout2fb: u32,

    /* Delay to add for microphones to stabalise after power up */
    pub micbias1_delay: i32,
    pub micbias2_delay: i32,

    /* Microphone biases: 0=0.9*AVDD1 1=0.65*AVVD1 */
    pub micbias1_lvl: u32,
    pub micbias2_lvl: u32,

    /* Jack detect threshold levels, see datasheet for values */
    pub jd_scthr: u32,
    pub jd_thr: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
