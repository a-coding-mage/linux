/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * max8973-regulator.h -- MAXIM 8973 regulator
 *
 * Interface for regulator driver for MAXIM 8973 DC-DC step-down
 * switching regulator.
 *
 * Copyright (C) 2012 NVIDIA Corporation

 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

/* Control flags for configuration of the device.
 * Client need to pass this information with ORed.
 */
pub const MAX8973_CONTROL_REMOTE_SENSE_ENABLE: u32 = 0x00000001;
pub const MAX8973_CONTROL_FALLING_SLEW_RATE_ENABLE: u32 = 0x00000002;
pub const MAX8973_CONTROL_OUTPUT_ACTIVE_DISCH_ENABLE: u32 = 0x00000004;
pub const MAX8973_CONTROL_BIAS_ENABLE: u32 = 0x00000008;
pub const MAX8973_CONTROL_PULL_DOWN_ENABLE: u32 = 0x00000010;
pub const MAX8973_CONTROL_FREQ_SHIFT_9PER_ENABLE: u32 = 0x00000020;

pub const MAX8973_CONTROL_CLKADV_TRIP_DISABLED: u32 = 0x00000000;
pub const MAX8973_CONTROL_CLKADV_TRIP_75mV_PER_US: u32 = 0x00010000;
pub const MAX8973_CONTROL_CLKADV_TRIP_150mV_PER_US: u32 = 0x00020000;
pub const MAX8973_CONTROL_CLKADV_TRIP_75mV_PER_US_HIST_DIS: u32 = 0x00030000;

pub const MAX8973_CONTROL_INDUCTOR_VALUE_NOMINAL: u32 = 0x00000000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_MINUS_30_PER: u32 = 0x00100000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_PLUS_30_PER: u32 = 0x00200000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_PLUS_60_PER: u32 = 0x00300000;

/*
 * struct max8973_regulator_platform_data - max8973 regulator platform data.
 *
 * @reg_init_data: The regulator init data.
 * @control_flags: Control flags which are ORed value of above flags to
 *	 configure device.
 * @junction_temp_warning: Junction temp in millicelcius on which warning need
 *			   to be set. Thermal functionality is only supported on
 *			   MAX77621. The threshold warning supported by MAX77621
 *			   are 120C and 140C.
 * @enable_ext_control: Enable the voltage enable/disable through external
 *	 control signal from EN input pin. If it is false then
 *	 voltage output will be enabled/disabled through EN bit of
 *	 device register.
 * @dvs_def_state: Default state of dvs. 1 if it is high else 0.
 */
#[repr(C)]
pub struct max8973_regulator_platform_data {
    pub reg_init_data: *mut regulator_init_data,
    pub control_flags: ::core::ffi::c_ulong,
    pub junction_temp_warning: ::core::ffi::c_ulong,
    pub enable_ext_control: bool,
    /* C unsigned bit-field: dvs_def_state:1; stored in its unsigned-int unit. */
    pub dvs_def_state: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
