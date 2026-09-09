/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * clk-dfll.h - prototypes and macros for the Tegra DFLL clocksource driver
 * Copyright (C) 2013-2019 NVIDIA Corporation.  All rights reserved.
 *
 * Aleksandr Frid <afrid@nvidia.com>
 * Paul Walmsley <pwalmsley@nvidia.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cvb_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rail_alignment {
    _private: [u8; 0],
}

/**
 * struct tegra_dfll_soc_data - SoC-specific hooks/integration for the DFLL driver
 * @dev: struct device * that holds the OPP table for the DFLL
 * @max_freq: maximum frequency supported on this SoC
 * @cvb: CPU frequency table for this SoC
 * @alignment: parameters of the regulator step and offset
 * @init_clock_trimmers: callback to initialize clock trimmers
 * @set_clock_trimmers_high: callback to tune clock trimmers for high voltage
 * @set_clock_trimmers_low: callback to tune clock trimmers for low voltage
 */
#[repr(C)]
pub struct tegra_dfll_soc_data {
    pub dev: *mut device,
    pub max_freq: c_ulong,
    pub cvb: *const cvb_table,
    pub alignment: rail_alignment,

    pub init_clock_trimmers: Option<unsafe extern "C" fn()>,
    pub set_clock_trimmers_high: Option<unsafe extern "C" fn()>,
    pub set_clock_trimmers_low: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub fn tegra_dfll_register(
        pdev: *mut platform_device,
        soc: *mut tegra_dfll_soc_data,
    ) -> i32;
    pub fn tegra_dfll_unregister(
        pdev: *mut platform_device,
    ) -> *mut tegra_dfll_soc_data;
    pub fn tegra_dfll_runtime_suspend(dev: *mut device) -> i32;
    pub fn tegra_dfll_runtime_resume(dev: *mut device) -> i32;
    pub fn tegra_dfll_suspend(dev: *mut device) -> i32;
    pub fn tegra_dfll_resume(dev: *mut device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
