/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/cs35l33.h -- Platform data for CS35l33
 *
 * Copyright (c) 2016 Cirrus Logic Inc.
 */

// C header guard __CS35L33_H omitted; Rust item/module boundaries provide
// equivalent single-definition protection.

#[repr(C)]
pub struct Cs35l33Hg {
    pub enable_hg_algo: bool,
    pub mem_depth: u32,
    pub release_rate: u32,
    pub hd_rm: u32,
    pub ldo_thld: u32,
    pub ldo_path_disable: u32,
    pub ldo_entry_delay: u32,
    pub vp_hg_auto: bool,
    pub vp_hg: u32,
    pub vp_hg_rate: u32,
    pub vp_hg_va: u32,
}

#[repr(C)]
pub struct Cs35l33Pdata {
    /* Boost Controller Voltage Setting */
    pub boost_ctl: u32,

    /* Boost Controller Peak Current */
    pub boost_ipk: u32,

    /* Amplifier Drive Select */
    pub amp_drv_sel: u32,

    /* soft volume ramp */
    pub ramp_rate: u32,

    /* IMON adc scale */
    pub imon_adc_scale: u32,

    /* H/G algo configuration */
    pub hg_config: Cs35l33Hg,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
