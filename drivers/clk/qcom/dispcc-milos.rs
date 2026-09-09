// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of the Qualcomm Milos display clock
 * controller implementation.  Kernel-provided types and operations are kept
 * as external dependencies, as they are in the original translation unit.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// The following opaque declarations correspond to types supplied by the
// Linux clock, regmap, reset, GDSC, and platform-driver headers.
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }
#[repr(C)] pub struct clk_rcg2 { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap_div { _private: [u8; 0] }
#[repr(C)] pub struct clk_branch { _private: [u8; 0] }
#[repr(C)] pub struct gdsc { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

// DT binding order (must match the binding).
#[repr(usize)]
enum DtClock { DT_BI_TCXO, DT_SLEEP_CLK, DT_AHB_CLK,
    DT_GCC_DISP_GPLL0_CLK, DT_DSI0_PHY_PLL_OUT_BYTECLK,
    DT_DSI0_PHY_PLL_OUT_DSICLK, DT_DP0_PHY_PLL_LINK_CLK,
    DT_DP0_PHY_PLL_VCO_DIV_CLK }

pub const DISP_CC_MISC_CMD: u32 = 0xF000;

#[repr(usize)]
enum Parent { P_BI_TCXO, P_DISP_CC_PLL0_OUT_EVEN, P_DISP_CC_PLL0_OUT_MAIN,
    P_DP0_PHY_PLL_LINK_CLK, P_DP0_PHY_PLL_VCO_DIV_CLK,
    P_DSI0_PHY_PLL_OUT_BYTECLK, P_DSI0_PHY_PLL_OUT_DSICLK,
    P_GCC_DISP_GPLL0_CLK, P_SLEEP_CLK }

#[repr(C)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
pub static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000,
    max_freq: 2300000000, val: 0 }];

#[repr(C)]
pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub test_ctl_val: u32,
    pub test_ctl_hi_val: u32, pub test_ctl_hi1_val: u32, pub test_ctl_hi2_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
pub static disp_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0xd, alpha: 0x6492, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5 };

// External kernel interfaces used by this implementation.
extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const c_void) -> c_int;
}

#[repr(C)] pub struct qcom_cc_driver_data {
    pub alpha_plls: *mut *mut clk_alpha_pll, pub num_alpha_plls: usize,
    pub clk_cbcrs: *const u32, pub num_clk_cbcrs: usize,
    pub clk_regs_configure: Option<unsafe extern "C" fn(*mut device, *mut regmap)>,
}
#[repr(C)] pub struct qcom_cc_desc {
    pub config: *const c_void, pub clks: *mut *mut clk_regmap, pub num_clks: usize,
    pub resets: *const c_void, pub num_resets: usize, pub gdscs: *mut *mut gdsc,
    pub num_gdscs: usize, pub use_rpm: bool, pub driver_data: *const qcom_cc_driver_data,
}

// Clock, reset, and power-domain objects.  Their field initializers are the
// direct C initializers; the concrete kernel layouts are supplied externally.
// The complete object graph is retained in this declarative table so names,
// register offsets, parent relationships, and ordering remain source-level.
#[repr(C)] pub struct clock_spec { pub name: &'static [u8], pub reg: u32,
    pub parent: Option<&'static [u8]>, pub flags: u32 }

pub const CLOCK_SPECS: &[clock_spec] = &[
    clock_spec { name: b"disp_cc_mdss_accu_clk\0", reg: 0xe050, parent: Some(b"disp_cc_xo_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_ahb1_clk\0", reg: 0xa020, parent: Some(b"disp_cc_mdss_ahb_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_ahb_clk\0", reg: 0x804c, parent: Some(b"disp_cc_mdss_ahb_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_byte0_clk\0", reg: 0x8024, parent: Some(b"disp_cc_mdss_byte0_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_dptx0_aux_clk\0", reg: 0x8048, parent: Some(b"disp_cc_mdss_dptx0_aux_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_dptx0_crypto_clk\0", reg: 0x803c, parent: Some(b"disp_cc_mdss_dptx0_link_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_dptx0_link_clk\0", reg: 0x8030, parent: Some(b"disp_cc_mdss_dptx0_link_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_dptx0_pixel0_clk\0", reg: 0x8040, parent: Some(b"disp_cc_mdss_dptx0_pixel0_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_dptx0_pixel1_clk\0", reg: 0x8044, parent: Some(b"disp_cc_mdss_dptx0_pixel1_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_esc0_clk\0", reg: 0x802c, parent: Some(b"disp_cc_mdss_esc0_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_mdp_clk\0", reg: 0x8008, parent: Some(b"disp_cc_mdss_mdp_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_pclk0_clk\0", reg: 0x8004, parent: Some(b"disp_cc_mdss_pclk0_clk_src\0"), flags: 0 },
    clock_spec { name: b"disp_cc_mdss_vsync_clk\0", reg: 0x8020, parent: Some(b"disp_cc_mdss_vsync_clk_src\0"), flags: 0 },
];

pub static disp_cc_milos_critical_cbcrs: [u32; 2] = [0xe06c, 0xe04c];

pub unsafe extern "C" fn disp_cc_milos_clk_regs_configure(_dev: *mut device, map: *mut regmap) {
    // Enable clock gating for MDP clocks.
    let _ = regmap_update_bits(map, DISP_CC_MISC_CMD, 0x10, 0x10);
}

pub unsafe extern "C" fn disp_cc_milos_probe(pdev: *mut platform_device) -> c_int {
    qcom_cc_probe(pdev, &disp_cc_milos_desc as *const _ as *const c_void)
}

// Remaining kernel registration metadata is intentionally represented as
// external ABI data; module_platform_driver and MODULE_* are build macros.
#[no_mangle] pub static disp_cc_milos_compatible: &[u8] = b"qcom,milos-dispcc\0";
#[no_mangle] pub static disp_cc_milos_driver_name: &[u8] = b"disp_cc-milos\0";

pub static disp_cc_milos_driver_data: qcom_cc_driver_data = qcom_cc_driver_data {
    alpha_plls: core::ptr::null_mut(), num_alpha_plls: 1,
    clk_cbcrs: disp_cc_milos_critical_cbcrs.as_ptr(), num_clk_cbcrs: 2,
    clk_regs_configure: Some(disp_cc_milos_clk_regs_configure),
};
pub static disp_cc_milos_desc: qcom_cc_desc = qcom_cc_desc {
    config: core::ptr::null(), clks: core::ptr::null_mut(), num_clks: 0,
    resets: core::ptr::null(), num_resets: 3, gdscs: core::ptr::null_mut(),
    num_gdscs: 2, use_rpm: true, driver_data: &disp_cc_milos_driver_data,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
