/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file is part of the ROHM BH1770GLC / OSRAM SFH7770 sensor driver.
 * Chip is combined proximity and ambient light sensor.
 *
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
 *
 * Contact: Samu Onkalo <samu.p.onkalo@nokia.com>
 */

// C header dependency: __u8, __u32, and int are represented by Rust integer types.

/**
 * struct bh1770_platform_data - platform data for bh1770glc driver
 * @led_def_curr: IR led driving current.
 * @glass_attenuation: Attenuation factor for covering window.
 * @setup_resources: Call back for interrupt line setup function
 * @release_resources: Call back for interrupte line release function
 *
 * Example of glass attenuation: 16384 * 385 / 100 means attenuation factor
 * of 3.85. i.e. light_above_sensor = light_above_cover_window / 3.85
 */

pub const BH1770_LED_5mA: i32 = 0;
pub const BH1770_LED_10mA: i32 = 1;
pub const BH1770_LED_20mA: i32 = 2;
pub const BH1770_LED_50mA: i32 = 3;
pub const BH1770_LED_100mA: i32 = 4;
pub const BH1770_LED_150mA: i32 = 5;
pub const BH1770_LED_200mA: i32 = 6;

pub const BH1770_NEUTRAL_GA: u32 = 16384; /* 16384 / 16384 = 1 */

#[repr(C)]
pub struct bh1770_platform_data {
    pub led_def_curr: u8,
    pub glass_attenuation: u32,
    pub setup_resources: Option<unsafe extern "C" fn() -> i32>,
    pub release_resources: Option<unsafe extern "C" fn() -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
