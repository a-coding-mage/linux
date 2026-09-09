/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * max1586.h  --  Voltage regulation for the Maxim 1586
 *
 * Copyright (C) 2008 Robert Jarzmik
 */

use core::ffi::c_char;

// Dependency supplied by linux/regulator/machine.h.
#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

pub const MAX1586_V3: i32 = 0;
pub const MAX1586_V6: i32 = 1;

/* precalculated values for v3_gain */
pub const MAX1586_GAIN_NO_R24: i32 = 1_000_000; /* 700000 .. 1475000 mV */
pub const MAX1586_GAIN_R24_3K32: i32 = 1_051_098; /* 735768 .. 1550369 mV */
pub const MAX1586_GAIN_R24_5K11: i32 = 1_078_648; /* 755053 .. 1591005 mV */
pub const MAX1586_GAIN_R24_7K5: i32 = 1_115_432; /* 780802 .. 1645262 mV */

/**
 * max1586_subdev_data - regulator data
 * @id: regulator Id (either MAX1586_V3 or MAX1586_V6)
 * @name: regulator cute name (example for V3: "vcc_core")
 * @platform_data: regulator init data (constraints, supplies, ...)
 */
#[repr(C)]
pub struct max1586_subdev_data {
    pub id: i32,
    pub name: *const c_char,
    pub platform_data: *mut regulator_init_data,
}

/**
 * max1586_platform_data - platform data for max1586
 * @num_subdevs: number of regulators used (may be 1 or 2)
 * @subdevs: regulator used
 *           At most, there will be a regulator for V3 and one for V6 voltages.
 * @v3_gain: gain on the V3 voltage output multiplied by 1e6.
 *           This can be calculated as ((1 + R24/R25 + R24/185.5kOhm) * 1e6)
 *           for an external resistor configuration as described in the
 *           data sheet (R25=100kOhm).
 */
#[repr(C)]
pub struct max1586_platform_data {
    pub num_subdevs: i32,
    pub subdevs: *mut max1586_subdev_data,
    pub v3_gain: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
