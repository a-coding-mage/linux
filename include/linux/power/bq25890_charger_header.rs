/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for the TI bq25890 battery charger driver.
 */

// Forward declaration of the externally defined regulator initialization data.
pub enum regulator_init_data {}

#[repr(C)]
pub struct bq25890_platform_data {
    pub regulator_init_data: *const regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
