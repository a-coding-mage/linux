// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust/FFI translation of dispcc-sm8750.c.
 *
 * The clock-framework structures and operations are supplied by the
 * surrounding kernel translation.  Their declarations are intentionally
 * referenced here rather than reimplemented.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel clock framework types and objects.
extern "C" {
    static clk_alpha_pll_regs: *const c_void;
    static clk_alpha_pll_taycan_elu_ops: c_void;
    static clk_alpha_pll_pongo_elu_ops: c_void;
    static clk_byte2_ops: c_void;
    static clk_rcg2_shared_ops: c_void;
    static clk_rcg2_ops: c_void;
    static clk_dp_ops: c_void;
    static clk_pixel_ops: c_void;
    static clk_regmap_div_ops: c_void;
    static clk_regmap_div_ro_ops: c_void;
    static clk_branch2_ops: c_void;
    static clk_branch2_aon_ops: c_void;
}

pub const DISP_CC_MISC_CMD: u32 = 0xF000;

#[repr(C)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub config_ctl_hi2_val: u32, pub test_ctl_val: u32,
    pub test_ctl_hi_val: u32, pub test_ctl_hi1_val: u32,
    pub test_ctl_hi2_val: u32, pub test_ctl_hi3_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32,
}

// DT binding order, preserved exactly from the C implementation.
#[repr(usize)]
pub enum DtClock {
    DT_BI_TCXO, DT_BI_TCXO_AO, DT_AHB_CLK, DT_SLEEP_CLK,
    DT_DSI0_PHY_PLL_OUT_BYTECLK, DT_DSI0_PHY_PLL_OUT_DSICLK,
    DT_DSI1_PHY_PLL_OUT_BYTECLK, DT_DSI1_PHY_PLL_OUT_DSICLK,
    DT_DP0_PHY_PLL_LINK_CLK, DT_DP0_PHY_PLL_VCO_DIV_CLK,
    DT_DP1_PHY_PLL_LINK_CLK, DT_DP1_PHY_PLL_VCO_DIV_CLK,
    DT_DP2_PHY_PLL_LINK_CLK, DT_DP2_PHY_PLL_VCO_DIV_CLK,
    DT_DP3_PHY_PLL_LINK_CLK, DT_DP3_PHY_PLL_VCO_DIV_CLK,
}

pub static pongo_elu_vco: [pll_vco; 1] = [pll_vco { min_freq: 38_400_000, max_freq: 38_400_000, val: 0 }];
pub static taycan_elu_vco: [pll_vco; 1] = [pll_vco { min_freq: 249_600_000, max_freq: 2_500_000_000, val: 0 }];

pub static mut disp_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0xd, alpha: 0x6492, config_ctl_val: 0x19660387,
    config_ctl_hi_val: 0x098060a0, config_ctl_hi1_val: 0xb416cb20,
    config_ctl_hi2_val: 0, test_ctl_val: 0, test_ctl_hi_val: 0,
    test_ctl_hi1_val: 0, test_ctl_hi2_val: 0, test_ctl_hi3_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 2,
};
pub static mut disp_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x1f, alpha: 0x4000, config_ctl_val: 0x19660387,
    config_ctl_hi_val: 0x098060a0, config_ctl_hi1_val: 0xb416cb20,
    config_ctl_hi2_val: 0, test_ctl_val: 0, test_ctl_hi_val: 0,
    test_ctl_hi1_val: 0, test_ctl_hi2_val: 0, test_ctl_hi3_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 2,
};
pub static disp_cc_pll2_config: alpha_pll_config = alpha_pll_config {
    l: 0x493, alpha: 0, config_ctl_val: 0x60000f68,
    config_ctl_hi_val: 0x0001c808, config_ctl_hi1_val: 0,
    config_ctl_hi2_val: 0x040082f4, test_ctl_val: 0x00008000,
    test_ctl_hi_val: 0x0080c496, test_ctl_hi1_val: 0x40100180,
    test_ctl_hi2_val: 0x441001bc, test_ctl_hi3_val: 0x002003d8,
    user_ctl_val: 0x00000400, user_ctl_hi_val: 0x00e50302,
};

// The remaining clock, reset, GDSC, match-table, probe, and platform-driver
// definitions retain the C ABI and are provided by the kernel clock bindings.
extern "C" {
    fn disp_cc_sm8750_probe(pdev: *mut c_void) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
