/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for MAX98088
 *
 * Copyright 2010 Maxim Integrated Products
 */

/* Equalizer filter response configuration */
#[repr(C)]
pub struct max98088_eq_cfg {
    pub name: *const core::ffi::c_char,
    pub rate: u32,
    pub band1: [u16; 5],
    pub band2: [u16; 5],
    pub band3: [u16; 5],
    pub band4: [u16; 5],
    pub band5: [u16; 5],
}

/* codec platform data */
#[repr(C)]
pub struct max98088_pdata {
    /* Equalizers for DAI1 and DAI2 */
    pub eq_cfg: *mut max98088_eq_cfg,
    pub eq_cfgcnt: u32,

    /* Receiver output can be configured as power amplifier or LINE out */
    /* Set receiver_mode to:
     * 0 = amplifier output, or
     * 1 = LINE level output
     *
     * C bit-field: unsigned int receiver_mode:1
     */
    pub receiver_mode: u32,

    /* Analog/digital microphone configuration:
     * 0 = analog microphone input (normal setting)
     * 1 = digital microphone input
     *
     * C bit-fields: unsigned int digmic_left_mode:1;
     *                unsigned int digmic_right_mode:1;
     */
    pub digmic_left_mode: u32,
    pub digmic_right_mode: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
