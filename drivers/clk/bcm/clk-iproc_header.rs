/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2014 Broadcom Corporation */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original header.

pub const IPROC_CLK_NAME_LEN: usize = 25;
pub const IPROC_CLK_INVALID_OFFSET: u32 = 0xffff_ffff;
pub const fn bit_mask(width: u32) -> u32 {
    (1u32 << width).wrapping_sub(1)
}

/* clocks that should not be disabled at runtime */
pub const IPROC_CLK_AON: u32 = 1u32 << 0;

/* PLL that requires gating through ASIU */
pub const IPROC_CLK_PLL_ASIU: u32 = 1u32 << 1;

/* PLL that has fractional part of the NDIV */
pub const IPROC_CLK_PLL_HAS_NDIV_FRAC: u32 = 1u32 << 2;

/*
 * Some of the iProc PLL/clocks may have an ASIC bug that requires read back
 * of the same register following the write to flush the write transaction into
 * the intended register
 */
pub const IPROC_CLK_NEEDS_READ_BACK: u32 = 1u32 << 3;

/*
 * Some PLLs require the PLL SW override bit to be set before changes can be
 * applied to the PLL
 */
pub const IPROC_CLK_PLL_NEEDS_SW_CFG: u32 = 1u32 << 4;

/*
 * Some PLLs use a different way to control clock power, via the PWRDWN bit in
 * the PLL control register
 */
pub const IPROC_CLK_EMBED_PWRCTRL: u32 = 1u32 << 5;

/*
 * Some PLLs have separate registers for Status and Control.  Identify this to
 * let the driver know if additional registers need to be used
 */
pub const IPROC_CLK_PLL_SPLIT_STAT_CTRL: u32 = 1u32 << 6;

/*
 * Some PLLs have an additional divide by 2 in master clock calculation;
 * MCLK = VCO_freq / (Mdiv * 2). Identify this to let the driver know
 * of modified calculations
 */
pub const IPROC_CLK_MCLK_DIV_BY_2: u32 = 1u32 << 7;

/*
 * Some PLLs provide a look up table for the leaf clock frequencies and
 * auto calculates VCO frequency parameters based on the provided leaf
 * clock frequencies. They have a user mode that allows the divider
 * controls to be determined by the user
 */
pub const IPROC_CLK_PLL_USER_MODE_ON: u32 = 1u32 << 8;

/* Some PLLs have an active low reset */
pub const IPROC_CLK_PLL_RESET_ACTIVE_LOW: u32 = 1u32 << 9;

/* Calculate the PLL parameters are runtime, instead of using table */
pub const IPROC_CLK_PLL_CALC_PARAM: u32 = 1u32 << 10;

/*
 * Parameters for VCO frequency configuration
 *
 * VCO frequency =
 * ((ndiv_int + ndiv_frac / 2^20) * (ref frequency  / pdiv)
 */
#[repr(C)]
pub struct iproc_pll_vco_param {
    pub rate: core::ffi::c_ulong,
    pub ndiv_int: u32,
    pub ndiv_frac: u32,
    pub pdiv: u32,
}

#[repr(C)]
pub struct iproc_clk_reg_op {
    pub offset: u32,
    pub shift: u32,
    pub width: u32,
}

/* Clock gating control at the top ASIU level */
#[repr(C)]
pub struct iproc_asiu_gate {
    pub offset: u32,
    pub en_shift: u32,
}

/* Control of powering on/off of a PLL
 *
 * Before powering off a PLL, input isolation (ISO) needs to be enabled
 */
#[repr(C)]
pub struct iproc_pll_aon_pwr_ctrl {
    pub offset: u32,
    pub pwr_width: u32,
    pub pwr_shift: u32,
    pub iso_shift: u32,
}

/* Control of the PLL reset */
#[repr(C)]
pub struct iproc_pll_reset_ctrl {
    pub offset: u32,
    pub reset_shift: u32,
    pub p_reset_shift: u32,
}

/* Control of the Ki, Kp, and Ka parameters */
#[repr(C)]
pub struct iproc_pll_dig_filter_ctrl {
    pub offset: u32,
    pub ki_shift: u32,
    pub ki_width: u32,
    pub kp_shift: u32,
    pub kp_width: u32,
    pub ka_shift: u32,
    pub ka_width: u32,
}

/* To enable SW control of the PLL */
#[repr(C)]
pub struct iproc_pll_sw_ctrl {
    pub offset: u32,
    pub shift: u32,
}

#[repr(C)]
pub struct iproc_pll_vco_ctrl {
    pub u_offset: u32,
    pub l_offset: u32,
}

/* Main PLL control parameters */
#[repr(C)]
pub struct iproc_pll_ctrl {
    pub flags: core::ffi::c_ulong,
    pub aon: iproc_pll_aon_pwr_ctrl,
    pub asiu: iproc_asiu_gate,
    pub reset: iproc_pll_reset_ctrl,
    pub dig_filter: iproc_pll_dig_filter_ctrl,
    pub sw_ctrl: iproc_pll_sw_ctrl,
    pub ndiv_int: iproc_clk_reg_op,
    pub ndiv_frac: iproc_clk_reg_op,
    pub pdiv: iproc_clk_reg_op,
    pub vco_ctrl: iproc_pll_vco_ctrl,
    pub status: iproc_clk_reg_op,
    pub macro_mode: iproc_clk_reg_op,
}

/* Controls enabling/disabling a PLL derived clock */
#[repr(C)]
pub struct iproc_clk_enable_ctrl {
    pub offset: u32,
    pub enable_shift: u32,
    pub hold_shift: u32,
    pub bypass_shift: u32,
}

/* Main clock control parameters for clocks derived from the PLLs */
#[repr(C)]
pub struct iproc_clk_ctrl {
    pub channel: u32,
    pub flags: core::ffi::c_ulong,
    pub enable: iproc_clk_enable_ctrl,
    pub mdiv: iproc_clk_reg_op,
}

/* Divisor of the ASIU clocks */
#[repr(C)]
pub struct iproc_asiu_div {
    pub offset: u32,
    pub en_shift: u32,
    pub high_shift: u32,
    pub high_width: u32,
    pub low_shift: u32,
    pub low_width: u32,
}

extern "C" {
    pub fn iproc_armpll_setup(node: *mut device_node);
    pub fn iproc_pll_clk_setup(
        node: *mut device_node,
        pll_ctrl: *const iproc_pll_ctrl,
        vco: *const iproc_pll_vco_param,
        num_vco_entries: u32,
        clk_ctrl: *const iproc_clk_ctrl,
        num_clks: u32,
    );
    pub fn iproc_asiu_setup(
        node: *mut device_node,
        div: *const iproc_asiu_div,
        gate: *const iproc_asiu_gate,
        num_clks: u32,
    );
}

// Supplied by the Linux device-tree translation.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
