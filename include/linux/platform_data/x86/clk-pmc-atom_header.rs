/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Atom platform clocks for BayTrail and CherryTrail SoC.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Irina Tirdea <irina.tirdea@intel.com>
 */

/**
 * struct pmc_clk - PMC platform clock configuration
 *
 * @name: identified, typically pmc_plt_clk_<x>, x=[0..5]
 * @freq: in Hz, 19.2MHz and 25MHz (Baytrail only) supported
 * @parent_name: one of 'xtal' or 'osc'
 */
#[repr(C)]
pub struct pmc_clk {
    pub name: *const core::ffi::c_char,
    pub freq: core::ffi::c_ulong,
    pub parent_name: *const core::ffi::c_char,
}

/**
 * struct pmc_clk_data - common PMC clock configuration
 *
 * @base: PMC clock register base offset
 * @clks: pointer to set of registered clocks, typically 0..5
 * @critical: flag to indicate if firmware enabled pmc_plt_clks
 *            should be marked as critial or not
 */
#[repr(C)]
pub struct pmc_clk_data {
    pub base: *mut core::ffi::c_void,
    pub clks: *const pmc_clk,
    pub critical: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
