/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) William Breathitt Gray */

// Opaque declarations corresponding to the C forward declarations.
pub struct device;
pub struct regmap;

/**
 * Configuration for the register map of an i8254
 * @parent: parent device
 * @map: regmap for the i8254
 */
#[repr(C)]
pub struct i8254_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
}

unsafe extern "C" {
    pub fn devm_i8254_regmap_register(
        dev: *mut device,
        config: *const i8254_regmap_config,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
