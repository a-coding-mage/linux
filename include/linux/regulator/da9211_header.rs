/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da9211.h - Regulator device driver for DA9211/DA9212
 * /DA9213/DA9223/DA9214/DA9224/DA9215/DA9225
 * Copyright (C) 2015  Dialog Semiconductor Ltd.
 */

// Dependency supplied by the Linux regulator machine interface.

pub const DA9211_MAX_REGULATORS: usize = 2;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da9211_chip_id {
    DA9211,
    DA9212,
    DA9213,
    DA9223,
    DA9214,
    DA9224,
    DA9215,
    DA9225,
}

#[repr(C)]
pub struct da9211_pdata {
    /*
     * Number of buck
     * 1 : 4 phase 1 buck
     * 2 : 2 phase 2 buck
     */
    pub num_buck: i32,
    pub gpiod_ren: [*mut gpio_desc; DA9211_MAX_REGULATORS],
    pub reg_node: [*mut device_node; DA9211_MAX_REGULATORS],
    pub init_data: [*mut regulator_init_data; DA9211_MAX_REGULATORS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
