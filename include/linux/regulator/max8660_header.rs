/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * max8660.h  --  Voltage regulation for the Maxim 8660/8661
 *
 * Copyright (C) 2009 Wolfram Sang, Pengutronix e.K.
 */

// Dependency intent: this declaration is supplied by linux/regulator/machine.h.
pub enum regulator_init_data {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum max8660_regulator_id {
    MAX8660_V3,
    MAX8660_V4,
    MAX8660_V5,
    MAX8660_V6,
    MAX8660_V7,
    MAX8660_V_END,
}

/**
 * max8660_subdev_data - regulator subdev data
 * @id: regulator id
 * @name: regulator name
 * @platform_data: regulator init data
 */
#[repr(C)]
pub struct max8660_subdev_data {
    pub id: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub platform_data: *mut regulator_init_data,
}

/**
 * max8660_platform_data - platform data for max8660
 * @num_subdevs: number of regulators used
 * @subdevs: pointer to regulators used
 * @en34_is_high: if EN34 is driven high, regulators cannot be en-/disabled.
 */
#[repr(C)]
pub struct max8660_platform_data {
    pub num_subdevs: ::core::ffi::c_int,
    pub subdevs: *mut max8660_subdev_data,
    // C bit-field: unsigned en34_is_high:1;
    pub en34_is_high: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
