/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rl6231.h - RL6231 class device shared support
 *
 * Copyright 2014 Realtek Semiconductor Corp.
 *
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

use core::ffi::{c_int, c_uint};

pub const RL6231_PLL_INP_MAX: c_int = 50000000;
pub const RL6231_PLL_INP_MIN: c_int = 256000;
pub const RL6231_PLL_N_MAX: c_int = 0x1ff;
pub const RL6231_PLL_K_MAX: c_int = 0x1f;
pub const RL6231_PLL_M_MAX: c_int = 0xf;

#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: bool, /* Indicates bypass m code or not. */
    pub k_bp: bool, /* Indicates bypass k code or not. */
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

/* struct regmap is supplied by an external dependency. */
pub enum regmap {}

unsafe extern "C" {
    pub fn rl6231_calc_dmic_clk(rate: c_int) -> c_int;
    pub fn rl6231_pll_calc(
        freq_in: c_uint,
        freq_out: c_uint,
        pll_code: *mut rl6231_pll_code,
    ) -> c_int;
    pub fn rl6231_get_clk_info(sclk: c_int, rate: c_int) -> c_int;
    pub fn rl6231_get_pre_div(map: *mut regmap, reg: c_uint, sft: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
