/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * isl9305 - Intersil ISL9305 DCDC regulator
 *
 * Copyright 2014 Linaro Ltd
 *
 * Author: Mark Brown <broonie@kernel.org>
 */

// External declaration supplied by another translated dependency.
pub enum regulator_init_data {}

pub const ISL9305_DCD1: i32 = 0;
pub const ISL9305_DCD2: i32 = 1;
pub const ISL9305_LDO1: i32 = 2;
pub const ISL9305_LDO2: i32 = 3;

pub const ISL9305_MAX_REGULATOR: i32 = ISL9305_LDO2;

#[repr(C)]
pub struct isl9305_pdata {
    pub init_data: [*mut regulator_init_data; (ISL9305_MAX_REGULATOR + 1) as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
