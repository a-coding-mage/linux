/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for the TI bq24190 battery charger driver.
 */

// Dependency: <linux/regulator/machine.h>

#[repr(C)]
pub struct regulator_init_data;

#[repr(C)]
pub struct bq24190_platform_data {
    pub regulator_init_data: *const regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
