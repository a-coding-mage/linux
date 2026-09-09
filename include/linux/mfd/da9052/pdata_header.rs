/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data declarations for DA9052 PMICs.
 *
 * Copyright(c) 2011 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */

// Original C header guard: __MFD_DA9052_PDATA_H__

pub const DA9052_MAX_REGULATORS: usize = 14;

pub struct da9052;
pub struct led_platform_data;
pub struct regulator_init_data;

#[repr(C)]
pub struct da9052_pdata {
    pub pled: *mut led_platform_data,
    pub init: Option<unsafe extern "C" fn(da9052: *mut da9052) -> i32>,
    pub irq_base: i32,
    pub gpio_base: i32,
    pub use_for_apm: i32,
    pub regulators: [*mut regulator_init_data; DA9052_MAX_REGULATORS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
