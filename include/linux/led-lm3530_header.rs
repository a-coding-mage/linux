/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 ST-Ericsson SA.
 * Copyright (C) 2009 Motorola, Inc.
 *
 * Simple driver for National Semiconductor LM35330 Backlight driver chip
 *
 * Author: Shreshtha Kumar SAHU <shreshthakumar.sahu@stericsson.com>
 * based on leds-lm3530.c by Dan Murphy <D.Murphy@motorola.com>
 */

pub const LM3530_FS_CURR_5mA: i32 = 0; /* Full Scale Current */
pub const LM3530_FS_CURR_8mA: i32 = 1;
pub const LM3530_FS_CURR_12mA: i32 = 2;
pub const LM3530_FS_CURR_15mA: i32 = 3;
pub const LM3530_FS_CURR_19mA: i32 = 4;
pub const LM3530_FS_CURR_22mA: i32 = 5;
pub const LM3530_FS_CURR_26mA: i32 = 6;
pub const LM3530_FS_CURR_29mA: i32 = 7;

pub const LM3530_ALS_AVRG_TIME_32ms: i32 = 0; /* ALS Averaging Time */
pub const LM3530_ALS_AVRG_TIME_64ms: i32 = 1;
pub const LM3530_ALS_AVRG_TIME_128ms: i32 = 2;
pub const LM3530_ALS_AVRG_TIME_256ms: i32 = 3;
pub const LM3530_ALS_AVRG_TIME_512ms: i32 = 4;
pub const LM3530_ALS_AVRG_TIME_1024ms: i32 = 5;
pub const LM3530_ALS_AVRG_TIME_2048ms: i32 = 6;
pub const LM3530_ALS_AVRG_TIME_4096ms: i32 = 7;

pub const LM3530_RAMP_TIME_1ms: i32 = 0; /* Brigtness Ramp Time */
pub const LM3530_RAMP_TIME_130ms: i32 = 1; /* Max to 0 and vice versa */
pub const LM3530_RAMP_TIME_260ms: i32 = 2;
pub const LM3530_RAMP_TIME_520ms: i32 = 3;
pub const LM3530_RAMP_TIME_1s: i32 = 4;
pub const LM3530_RAMP_TIME_2s: i32 = 5;
pub const LM3530_RAMP_TIME_4s: i32 = 6;
pub const LM3530_RAMP_TIME_8s: i32 = 7;

/* ALS Resistor Select */
pub const LM3530_ALS_IMPD_Z: i32 = 0x00; /* ALS Impedance */
pub const LM3530_ALS_IMPD_13_53kOhm: i32 = 0x01;
pub const LM3530_ALS_IMPD_9_01kOhm: i32 = 0x02;
pub const LM3530_ALS_IMPD_5_41kOhm: i32 = 0x03;
pub const LM3530_ALS_IMPD_2_27kOhm: i32 = 0x04;
pub const LM3530_ALS_IMPD_1_94kOhm: i32 = 0x05;
pub const LM3530_ALS_IMPD_1_81kOhm: i32 = 0x06;
pub const LM3530_ALS_IMPD_1_6kOhm: i32 = 0x07;
pub const LM3530_ALS_IMPD_1_138kOhm: i32 = 0x08;
pub const LM3530_ALS_IMPD_1_05kOhm: i32 = 0x09;
pub const LM3530_ALS_IMPD_1_011kOhm: i32 = 0x0A;
pub const LM3530_ALS_IMPD_941Ohm: i32 = 0x0B;
pub const LM3530_ALS_IMPD_759Ohm: i32 = 0x0C;
pub const LM3530_ALS_IMPD_719Ohm: i32 = 0x0D;
pub const LM3530_ALS_IMPD_700Ohm: i32 = 0x0E;
pub const LM3530_ALS_IMPD_667Ohm: i32 = 0x0F;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lm3530_mode {
    LM3530_BL_MODE_MANUAL = 0, /* "man" */
    LM3530_BL_MODE_ALS,        /* "als" */
    LM3530_BL_MODE_PWM,        /* "pwm" */
}

/* ALS input select */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lm3530_als_mode {
    LM3530_INPUT_AVRG = 0, /* ALS1 and ALS2 input average */
    LM3530_INPUT_ALS1,     /* ALS1 Input */
    LM3530_INPUT_ALS2,     /* ALS2 Input */
    LM3530_INPUT_CEIL,     /* Max of ALS1 and ALS2 */
}

/* PWM Platform Specific Data */
#[repr(C)]
pub struct lm3530_pwm_data {
    pub pwm_set_intensity: Option<unsafe extern "C" fn(brightness: i32, max_brightness: i32)>,
    pub pwm_get_intensity: Option<unsafe extern "C" fn(max_brightness: i32) -> i32>,
}

/**
 * struct lm3530_platform_data
 * @mode: mode of operation i.e. Manual, ALS or PWM
 * @als_input_mode: select source of ALS input - ALS1/2 or average
 * @max_current: full scale LED current
 * @pwm_pol_hi: PWM input polarity - active high/active low
 * @als_avrg_time: ALS input averaging time
 * @brt_ramp_law: brightness mapping mode - exponential/linear
 * @brt_ramp_fall: rate of fall of led current
 * @brt_ramp_rise: rate of rise of led current
 * @als1_resistor_sel: internal resistance from ALS1 input to ground
 * @als2_resistor_sel: internal resistance from ALS2 input to ground
 * @als_vmin: als input voltage calibrated for max brightness in mV
 * @als_vmax: als input voltage calibrated for min brightness in mV
 * @brt_val: brightness value (0-127)
 * @pwm_data: PWM control functions (only valid when the mode is PWM)
 */
#[repr(C)]
pub struct lm3530_platform_data {
    pub mode: lm3530_mode,
    pub als_input_mode: lm3530_als_mode,
    pub max_current: u8,
    pub pwm_pol_hi: bool,
    pub als_avrg_time: u8,
    pub brt_ramp_law: bool,
    pub brt_ramp_fall: u8,
    pub brt_ramp_rise: u8,
    pub als1_resistor_sel: u8,
    pub als2_resistor_sel: u8,
    pub als_vmin: u32,
    pub als_vmax: u32,
    pub brt_val: u8,
    pub pwm_data: lm3530_pwm_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
