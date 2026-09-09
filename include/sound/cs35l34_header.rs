/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/cs35l34.h -- Platform data for CS35l34
 *
 * Copyright (c) 2016 Cirrus Logic Inc.
 */

// __CS35L34_H

#[repr(C)]
pub struct cs35l34_platform_data {
    /* Set AIF to half drive strength */
    pub aif_half_drv: bool,
    /* Digital Soft Ramp Disable */
    pub digsft_disable: bool,
    /* Amplifier Invert */
    pub amp_inv: bool,
    /* Peak current (mA) */
    pub boost_peak: ::core::ffi::c_uint,
    /* Boost inductor value (nH) */
    pub boost_ind: ::core::ffi::c_uint,
    /* Boost Controller Voltage Setting (mV) */
    pub boost_vtge: ::core::ffi::c_uint,
    /* Gain Change Zero Cross */
    pub gain_zc_disable: bool,
    /* SDIN Left/Right Selection */
    pub i2s_sdinloc: ::core::ffi::c_uint,
    /* TDM Rising Edge */
    pub tdm_rising_edge: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
