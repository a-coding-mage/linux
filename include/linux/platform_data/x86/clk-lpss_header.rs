/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Low Power Subsystem clocks.
 *
 * Copyright (C) 2013, Intel Corporation
 * Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
 *          Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// C header guard: __CLK_LPSS_H

// `clk` is supplied by the surrounding kernel translation unit.
#[repr(C)]
pub struct lpss_clk_data {
    pub name: *const core::ffi::c_char,
    pub clk: *mut clk,
}

unsafe extern "C" {
    pub fn lpss_atom_clk_init() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
