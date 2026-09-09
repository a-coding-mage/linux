/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * LP8755 High Performance Power Management Unit Driver:System Interface Driver
 *
 *			Copyright (C) 2012 Texas Instruments
 *
 * Author: Daniel(Geon Si) Jeong <daniel.jeong@ti.com>
 *             G.Shark Jeong <gshark.jeong@gmail.com>
 */

// Dependency supplied by the regulator consumer subsystem.

pub const LP8755_NAME: &str = "lp8755-regulator";
/*
 *PWR FAULT : power fault detected
 *OCP : over current protect activated
 *OVP : over voltage protect activated
 *TEMP_WARN : thermal warning
 *TEMP_SHDN : thermal shutdonw detected
 *I_LOAD : current measured
 */
pub const LP8755_EVENT_PWR_FAULT: u32 = REGULATOR_EVENT_FAIL;
pub const LP8755_EVENT_OCP: u32 = REGULATOR_EVENT_OVER_CURRENT;
pub const LP8755_EVENT_OVP: u32 = 0x10000;
pub const LP8755_EVENT_TEMP_WARN: u32 = 0x2000;
pub const LP8755_EVENT_TEMP_SHDN: u32 = REGULATOR_EVENT_OVER_TEMP;
pub const LP8755_EVENT_I_LOAD: u32 = 0x40000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lp8755_bucks {
	LP8755_BUCK0 = 0,
	LP8755_BUCK1,
	LP8755_BUCK2,
	LP8755_BUCK3,
	LP8755_BUCK4,
	LP8755_BUCK5,
	LP8755_BUCK_MAX,
}

/**
 * multiphase configuration options
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lp8755_mphase_config {
	MPHASE_CONF0,
	MPHASE_CONF1,
	MPHASE_CONF2,
	MPHASE_CONF3,
	MPHASE_CONF4,
	MPHASE_CONF5,
	MPHASE_CONF6,
	MPHASE_CONF7,
	MPHASE_CONF8,
	MPHASE_CONF_MAX,
}

/**
 * struct lp8755_platform_data
 * @mphase_type : Multiphase Switcher Configurations.
 * @buck_data   : buck0~6 init voltage in uV
 */
#[repr(C)]
pub struct lp8755_platform_data {
	pub mphase: ::core::ffi::c_int,
	pub buck_data: [*mut regulator_init_data; LP8755_BUCK_MAX as usize],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
