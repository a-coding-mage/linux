/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/cs35l35.h -- Platform data for CS35l35
 *
 * Copyright (c) 2016 Cirrus Logic Inc.
 */

#[repr(C)]
pub struct classh_cfg {
	/*
	 * Class H Algorithm Control Variables
	 * You can either have it done
	 * automatically or you can adjust
	 * these variables for tuning
	 *
	 * if you do not enable the internal algorithm
	 * you will get a set of mixer controls for
	 * Class H tuning
	 *
	 * Section 4.3 of the datasheet
	 */
	pub classh_bst_override: bool,
	pub classh_algo_enable: bool,
	pub classh_bst_max_limit: i32,
	pub classh_mem_depth: i32,
	pub classh_release_rate: i32,
	pub classh_headroom: i32,
	pub classh_wk_fet_disable: i32,
	pub classh_wk_fet_delay: i32,
	pub classh_wk_fet_thld: i32,
	pub classh_vpch_auto: i32,
	pub classh_vpch_rate: i32,
	pub classh_vpch_man: i32,
}

#[repr(C)]
pub struct monitor_cfg {
	/*
	 * Signal Monitor Data
	 * highly configurable signal monitoring
	 * data positioning and different types of
	 * monitoring data.
	 *
	 * Section 4.8.2 - 4.8.4 of the datasheet
	 */
	pub is_present: bool,
	pub imon_specs: bool,
	pub vmon_specs: bool,
	pub vpmon_specs: bool,
	pub vbstmon_specs: bool,
	pub vpbrstat_specs: bool,
	pub zerofill_specs: bool,
	pub imon_dpth: u8,
	pub imon_loc: u8,
	pub imon_frm: u8,
	pub imon_scale: u8,
	pub vmon_dpth: u8,
	pub vmon_loc: u8,
	pub vmon_frm: u8,
	pub vpmon_dpth: u8,
	pub vpmon_loc: u8,
	pub vpmon_frm: u8,
	pub vbstmon_dpth: u8,
	pub vbstmon_loc: u8,
	pub vbstmon_frm: u8,
	pub vpbrstat_dpth: u8,
	pub vpbrstat_loc: u8,
	pub vpbrstat_frm: u8,
	pub zerofill_dpth: u8,
	pub zerofill_loc: u8,
	pub zerofill_frm: u8,
}

#[repr(C)]
pub struct cs35l35_platform_data {
	/* Stereo (2 Device) */
	pub stereo: bool,
	/* serial port drive strength */
	pub sp_drv_str: i32,
	/* serial port drive in unused slots */
	pub sp_drv_unused: i32,
	/* Boost Power Down with FET */
	pub bst_pdn_fet_on: bool,
	/* Boost Voltage : used if ClassH Algo Enabled */
	pub bst_vctl: i32,
	/* Boost Converter Peak Current CTRL */
	pub bst_ipk: i32,
	/* Amp Gain Zero Cross */
	pub gain_zc: bool,
	/* Audio Input Location */
	pub aud_channel: i32,
	/* Advisory Input Location */
	pub adv_channel: i32,
	/* Shared Boost for stereo */
	pub shared_bst: bool,
	/* Specifies this amp is using an external boost supply */
	pub ext_bst: bool,
	/* Inductor Value */
	pub boost_ind: i32,
	/* ClassH Algorithm */
	pub classh_algo: classh_cfg,
	/* Monitor Config */
	pub mon_cfg: monitor_cfg,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
