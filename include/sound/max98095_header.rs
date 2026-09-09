/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for MAX98095
 *
 * Copyright 2011 Maxim Integrated Products
 */

/* Equalizer filter response configuration */
#[repr(C)]
pub struct max98095_eq_cfg {
    pub name: *const core::ffi::c_char,
    pub rate: u32,
    pub band1: [u16; 5],
    pub band2: [u16; 5],
    pub band3: [u16; 5],
    pub band4: [u16; 5],
    pub band5: [u16; 5],
}

/* Biquad filter response configuration */
#[repr(C)]
pub struct max98095_biquad_cfg {
    pub name: *const core::ffi::c_char,
    pub rate: u32,
    pub band1: [u16; 5],
    pub band2: [u16; 5],
}

/* codec platform data */
#[repr(C)]
pub struct max98095_pdata {
    /* Equalizers for DAI1 and DAI2 */
    pub eq_cfg: *mut max98095_eq_cfg,
    pub eq_cfgcnt: u32,

    /* Biquad filter for DAI1 and DAI2 */
    pub bq_cfg: *mut max98095_biquad_cfg,
    pub bq_cfgcnt: u32,

    /* Analog/digital microphone configuration:
     * 0 = analog microphone input (normal setting)
     * 1 = digital microphone input
     *
     * These fields correspond to one-bit C bit-fields.
     */
    pub digmic_left_mode: u32,
    pub digmic_right_mode: u32,

    /* Pin5 is the mechanical method of sensing jack insertion
     * but it is something that might not be supported.
     * 0 = PIN5 not supported
     * 1 = PIN5 supported
     *
     * This field corresponds to a one-bit C bit-field.
     */
    pub jack_detect_pin5en: u32,

    /* Slew amount for jack detection. Calculated as 4 * (delay + 1).
     * Default delay is 24 to get a time of 100ms.
     */
    pub jack_detect_delay: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
