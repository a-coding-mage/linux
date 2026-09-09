/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/sound/cs35l36.h -- Platform data for CS35L36
 *
 * Copyright 2018 Cirrus Logic, Inc.
 *
 * Author: James Schulman <james.schulman@cirrus.com>
 *
 */

#[repr(C)]
pub struct cs35l36_vpbr_cfg {
    pub is_present: bool,
    pub vpbr_en: bool,
    pub vpbr_thld: i32,
    pub vpbr_atk_rate: i32,
    pub vpbr_atk_vol: i32,
    pub vpbr_max_attn: i32,
    pub vpbr_wait: i32,
    pub vpbr_rel_rate: i32,
    pub vpbr_mute_en: i32,
}

#[repr(C)]
pub struct cs35l36_platform_data {
    pub multi_amp_mode: bool,
    pub dcm_mode: bool,
    pub amp_pcm_inv: bool,
    pub imon_pol_inv: bool,
    pub vmon_pol_inv: bool,
    pub boost_ind: i32,
    pub bst_vctl: i32,
    pub bst_vctl_sel: i32,
    pub bst_ipk: i32,
    pub extern_boost: bool,
    pub temp_warn_thld: i32,
    pub irq_drv_sel: i32,
    pub irq_gpio_sel: i32,
    pub vpbr_config: cs35l36_vpbr_cfg,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
