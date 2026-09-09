/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for Arizona LDO1 regulator
 *
 * Copyright 2017 Cirrus Logic
 */

// Forward declaration of the externally defined regulator initialization data.
#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arizona_ldo1_pdata {
    /** Regulator configuration for LDO1 */
    pub init_data: *const regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
