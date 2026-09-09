/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for Arizona micsupp regulator
 *
 * Copyright 2017 Cirrus Logic
 */

// C forward declaration: struct regulator_init_data;
#[repr(C)]
pub struct regulator_init_data;

#[repr(C)]
pub struct arizona_micsupp_pdata {
	/** Regulator configuration for micsupp */
	pub init_data: *const regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
