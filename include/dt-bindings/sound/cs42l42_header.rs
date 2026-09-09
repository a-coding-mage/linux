/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs42l42.h -- CS42L42 ALSA SoC audio driver DT bindings header
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: James Schulman <james.schulman@cirrus.com>
 * Author: Brian Austin <brian.austin@cirrus.com>
 * Author: Michael White <michael.white@cirrus.com>
 */

/* HPOUT Load Capacity */
pub const CS42L42_HPOUT_LOAD_1NF: i32 = 0;
pub const CS42L42_HPOUT_LOAD_10NF: i32 = 1;

/* HPOUT Clamp to GND Override */
pub const CS42L42_HPOUT_CLAMP_EN: i32 = 0;
pub const CS42L42_HPOUT_CLAMP_DIS: i32 = 1;

/* Tip Sense Inversion */
pub const CS42L42_TS_INV_DIS: i32 = 0;
pub const CS42L42_TS_INV_EN: i32 = 1;

/* Tip Sense Debounce */
pub const CS42L42_TS_DBNCE_0: i32 = 0;
pub const CS42L42_TS_DBNCE_125: i32 = 1;
pub const CS42L42_TS_DBNCE_250: i32 = 2;
pub const CS42L42_TS_DBNCE_500: i32 = 3;
pub const CS42L42_TS_DBNCE_750: i32 = 4;
pub const CS42L42_TS_DBNCE_1000: i32 = 5;
pub const CS42L42_TS_DBNCE_1250: i32 = 6;
pub const CS42L42_TS_DBNCE_1500: i32 = 7;

/* Button Press Software Debounce Times */
pub const CS42L42_BTN_DET_INIT_DBNCE_MIN: i32 = 0;
pub const CS42L42_BTN_DET_INIT_DBNCE_DEFAULT: i32 = 100;
pub const CS42L42_BTN_DET_INIT_DBNCE_MAX: i32 = 200;

pub const CS42L42_BTN_DET_EVENT_DBNCE_MIN: i32 = 0;
pub const CS42L42_BTN_DET_EVENT_DBNCE_DEFAULT: i32 = 10;
pub const CS42L42_BTN_DET_EVENT_DBNCE_MAX: i32 = 20;

/* Button Detect Level Sensitivities */
pub const CS42L42_NUM_BIASES: i32 = 4;

pub const CS42L42_HS_DET_LEVEL_15: i32 = 0x0F;
pub const CS42L42_HS_DET_LEVEL_8: i32 = 0x08;
pub const CS42L42_HS_DET_LEVEL_4: i32 = 0x04;
pub const CS42L42_HS_DET_LEVEL_1: i32 = 0x01;

pub const CS42L42_HS_DET_LEVEL_MIN: i32 = 0;
pub const CS42L42_HS_DET_LEVEL_MAX: i32 = 0x3F;

/* HS Bias Ramp Rate */

pub const CS42L42_HSBIAS_RAMP_FAST_RISE_SLOW_FALL: i32 = 0;
pub const CS42L42_HSBIAS_RAMP_FAST: i32 = 1;
pub const CS42L42_HSBIAS_RAMP_SLOW: i32 = 2;
pub const CS42L42_HSBIAS_RAMP_SLOWEST: i32 = 3;

pub const CS42L42_HSBIAS_RAMP_TIME0: i32 = 10;
pub const CS42L42_HSBIAS_RAMP_TIME1: i32 = 40;
pub const CS42L42_HSBIAS_RAMP_TIME2: i32 = 90;
pub const CS42L42_HSBIAS_RAMP_TIME3: i32 = 170;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
