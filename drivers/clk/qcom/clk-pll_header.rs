/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/clk-provider.h and clk-regmap.h

/**
 * struct pll_freq_tbl - PLL frequency table
 * @l: L value
 * @m: M value
 * @n: N value
 * @ibits: internal values
 */
#[repr(C)]
pub struct pll_freq_tbl {
    pub freq: ::core::ffi::c_ulong,
    pub l: u16,
    pub m: u16,
    pub n: u16,
    pub ibits: u32,
}

/**
 * struct clk_pll - phase locked loop (PLL)
 * @l_reg: L register
 * @m_reg: M register
 * @n_reg: N register
 * @config_reg: config register
 * @mode_reg: mode register
 * @status_reg: status register
 * @status_bit: ANDed with @status_reg to determine if PLL is enabled
 * @freq_tbl: PLL frequency table
 * @hw: handle between common and hardware-specific interfaces
 */
#[repr(C)]
pub struct clk_pll {
    pub l_reg: u32,
    pub m_reg: u32,
    pub n_reg: u32,
    pub config_reg: u32,
    pub mode_reg: u32,
    pub status_reg: u32,
    pub status_bit: u8,
    pub post_div_width: u8,
    pub post_div_shift: u8,
    pub freq_tbl: *const pll_freq_tbl,
    pub clkr: clk_regmap,
}

extern "C" {
    pub static clk_pll_ops: clk_ops;
    pub static clk_pll_vote_ops: clk_ops;
    pub static clk_pll_sr2_ops: clk_ops;

    pub fn clk_pll_configure_sr(
        pll: *mut clk_pll,
        regmap: *mut regmap,
        config: *const pll_config,
        fsm_mode: bool,
    );
    pub fn clk_pll_configure_sr_hpm_lp(
        pll: *mut clk_pll,
        regmap: *mut regmap,
        config: *const pll_config,
        fsm_mode: bool,
    );
}

// Equivalent to: container_of(to_clk_regmap(_hw), struct clk_pll, clkr)
macro_rules! to_clk_pll {
    ($hw:expr) => {{
        container_of!(to_clk_regmap!($hw), clk_pll, clkr)
    }};
}

#[repr(C)]
pub struct pll_config {
    pub l: u16,
    pub m: u32,
    pub n: u32,
    pub vco_val: u32,
    pub vco_mask: u32,
    pub pre_div_val: u32,
    pub pre_div_mask: u32,
    pub post_div_val: u32,
    pub post_div_mask: u32,
    pub mn_ena_mask: u32,
    pub main_output_mask: u32,
    pub aux_output_mask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
