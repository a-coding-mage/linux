// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2-specific DPLL control functions
 *
 * Copyright (C) 2011 Nokia Corporation
 * Paul Walmsley
 */

// Dependencies supplied by the surrounding clock implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct clk_hw_omap {
    pub dpll_data: *mut c_void,
}

#[repr(C)]
pub struct clk_hw_omap_ops {
    pub allow_idle: Option<unsafe extern "C" fn(*mut clk_hw_omap)>,
    pub deny_idle: Option<unsafe extern "C" fn(*mut clk_hw_omap)>,
}

unsafe extern "C" {
    fn omap2xxx_cm_set_dpll_auto_low_power_stop();
    fn omap2xxx_cm_set_dpll_disable_autoidle();
}

/* Private functions */

/**
 * _allow_idle - enable DPLL autoidle bits
 * @clk: struct clk * of the DPLL to operate on
 *
 * Enable DPLL automatic idle control.  The DPLL will enter low-power
 * stop when its downstream clocks are gated.  No return value.
 * REVISIT: DPLL can optionally enter low-power bypass by writing 0x1
 * instead.  Add some mechanism to optionally enter this mode.
 */
unsafe extern "C" fn _allow_idle(clk: *mut clk_hw_omap) {
    if clk.is_null() || (*clk).dpll_data.is_null() {
        return;
    }

    omap2xxx_cm_set_dpll_auto_low_power_stop();
}

/**
 * _deny_idle - prevent DPLL from automatically idling
 * @clk: struct clk * of the DPLL to operate on
 *
 * Disable DPLL automatic idle control.  No return value.
 */
unsafe extern "C" fn _deny_idle(clk: *mut clk_hw_omap) {
    if clk.is_null() || (*clk).dpll_data.is_null() {
        return;
    }

    omap2xxx_cm_set_dpll_disable_autoidle();
}

/* Public data */
pub static clkhwops_omap2xxx_dpll: clk_hw_omap_ops = clk_hw_omap_ops {
    allow_idle: Some(_allow_idle),
    deny_idle: Some(_deny_idle),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
