/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * DA9121 Single-channel dual-phase 10A buck converter
 * DA9130 Single-channel dual-phase 10A buck converter (Automotive)
 * DA9217 Single-channel dual-phase  6A buck converter
 * DA9122 Dual-channel single-phase  5A buck converter
 * DA9131 Dual-channel single-phase  5A buck converter (Automotive)
 * DA9220 Dual-channel single-phase  3A buck converter
 * DA9132 Dual-channel single-phase  3A buck converter (Automotive)
 *
 * Copyright (C) 2020  Dialog Semiconductor
 *
 * Authors: Adam Ward, Dialog Semiconductor
 */

// Dependency supplied by <linux/regulator/machine.h>.

pub struct gpio_desc;
pub struct device_node;
pub struct regulator_init_data;

pub const DA9121_IDX_BUCK1: usize = 0;
pub const DA9121_IDX_BUCK2: usize = 1;
pub const DA9121_IDX_MAX: usize = 2;

#[repr(C)]
pub struct da9121_pdata {
    pub num_buck: i32,
    pub gpiod_ren: [*mut gpio_desc; DA9121_IDX_MAX],
    pub reg_node: [*mut device_node; DA9121_IDX_MAX],
    pub init_data: [*mut regulator_init_data; DA9121_IDX_MAX],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
