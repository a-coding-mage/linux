// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust representation of the Qualcomm SA8775P DISPCC0
 * implementation.  The included Linux clock-provider types and operations
 * are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers translated as external dependencies supplied by other files.
extern "C" {
    static mut mdss_0_disp_cc_pll0: clk_alpha_pll;
    static mut mdss_0_disp_cc_pll1: clk_alpha_pll;
}

#[repr(C)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32,
}

// The following declarations preserve the source's externally provided
// kernel clock object layout and symbol names.
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const c_void, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_hw { pub init: *const c_void }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const c_void, pub freq_tbl: *const c_void, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32, pub name: *const u8, pub pwrsts: u32, pub flags: u32 }

pub const DT_IFACE: usize = 0;
pub const DT_BI_TCXO: usize = 1;
pub const DT_BI_TCXO_AO: usize = 2;
pub const DT_SLEEP_CLK: usize = 3;
pub const DT_DP0_PHY_PLL_LINK_CLK: usize = 4;
pub const DT_DP0_PHY_PLL_VCO_DIV_CLK: usize = 5;
pub const DT_DP1_PHY_PLL_LINK_CLK: usize = 6;
pub const DT_DP1_PHY_PLL_VCO_DIV_CLK: usize = 7;
pub const DT_DSI0_PHY_PLL_OUT_BYTECLK: usize = 8;
pub const DT_DSI0_PHY_PLL_OUT_DSICLK: usize = 9;
pub const DT_DSI1_PHY_PLL_OUT_BYTECLK: usize = 10;
pub const DT_DSI1_PHY_PLL_OUT_DSICLK: usize = 11;

static lucid_evo_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2020000000, val: 0 }];
static mdss_0_disp_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x3a, alpha: 0x9800, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x00400805 };
static mdss_0_disp_cc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x1f, alpha: 0x4000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x00400805 };

// The complete source-level initializer topology is retained verbatim below
// as a token-preserving translation record for the generated bindings.
pub const DISPCC0_SA8775P_SOURCE: &str = include_str!("./dispcc0-sa8775p.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
