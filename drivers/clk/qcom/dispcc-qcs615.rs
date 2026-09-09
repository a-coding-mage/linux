// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
//
// Direct Rust translation of the QCS615 display clock controller.
// Kernel-provided types, constants, operations, and helper macros are external
// dependencies supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::mem::MaybeUninit;

extern "C" {
    // Definitions supplied by the Linux clock-controller dependencies.
    static mut clk_alpha_pll_regs: [*const core::ffi::c_void; 16];
    static clk_alpha_pll_slew_ops: core::ffi::c_void;
    static clk_rcg2_shared_ops: core::ffi::c_void;
    static clk_byte2_ops: core::ffi::c_void;
    static clk_dp_ops: core::ffi::c_void;
    static clk_rcg2_ops: core::ffi::c_void;
    static clk_pixel_ops: core::ffi::c_void;
    static clk_regmap_div_ro_ops: core::ffi::c_void;
    static clk_branch2_ops: core::ffi::c_void;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub vco_val: u32, pub vco_mask: u32, pub main_output_mask: u32, pub config_ctl_val: u32, pub test_ctl_hi_val: u32, pub test_ctl_hi_mask: u32 }
#[repr(C)] pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub config: *mut alpha_pll_config, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const core::ffi::c_void, pub clkr: clk_regmap_wrapper }
#[repr(C)] pub struct clk_regmap_wrapper { pub hw: clk_hw, pub enable_reg: u32, pub enable_mask: u32 }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl, pub clkr: clk_regmap_wrapper }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap_wrapper }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub clkr: clk_regmap_wrapper }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32, pub pwrsts: u32, pub flags: u32 }
#[repr(C)] pub struct parent_map { pub src: u32, pub cfg: u32 }
#[repr(C)] pub struct freq_tbl { pub freq: u32, pub src: u32, pub div: u32, pub m: u32, pub n: u32 }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_cc_driver_data { pub alpha_plls: *const *mut clk_alpha_pll, pub num_alpha_plls: usize, pub clk_cbcrs: *const u32, pub num_clk_cbcrs: usize }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *const *mut clk_regmap, pub num_clks: usize, pub gdscs: *const *mut gdsc, pub num_gdscs: usize, pub driver_data: *const qcom_cc_driver_data }

const DT_BI_TCXO: u32 = 0;
const DT_GPLL0: u32 = 1;
const DT_DSI0_PHY_PLL_OUT_BYTECLK: u32 = 2;
const DT_DSI0_PHY_PLL_OUT_DSICLK: u32 = 3;
const DT_DSI1_PHY_PLL_OUT_DSICLK: u32 = 4;
const DT_DP_PHY_PLL_LINK_CLK: u32 = 5;
const DT_DP_PHY_PLL_VCO_DIV_CLK: u32 = 6;
const P_BI_TCXO: u32 = 0;
const P_DISP_CC_PLL0_OUT_MAIN: u32 = 1;
const P_DP_PHY_PLL_LINK_CLK: u32 = 2;
const P_DP_PHY_PLL_VCO_DIV_CLK: u32 = 3;
const P_DSI0_PHY_PLL_OUT_BYTECLK: u32 = 4;
const P_DSI0_PHY_PLL_OUT_DSICLK: u32 = 5;
const P_DSI1_PHY_PLL_OUT_DSICLK: u32 = 6;
const P_GPLL0_OUT_MAIN: u32 = 7;

// Clock IDs are supplied by dt-bindings/clock/qcom,qcs615-dispcc.h.
extern "C" {
    static DISP_CC_MDSS_AHB_CLK: u32; static DISP_CC_MDSS_AHB_CLK_SRC: u32;
    static DISP_CC_MDSS_BYTE0_CLK: u32; static DISP_CC_MDSS_BYTE0_CLK_SRC: u32;
    static DISP_CC_MDSS_BYTE0_DIV_CLK_SRC: u32; static DISP_CC_MDSS_BYTE0_INTF_CLK: u32;
    static DISP_CC_MDSS_DP_AUX_CLK: u32; static DISP_CC_MDSS_DP_AUX_CLK_SRC: u32;
    static DISP_CC_MDSS_DP_CRYPTO_CLK: u32; static DISP_CC_MDSS_DP_CRYPTO_CLK_SRC: u32;
    static DISP_CC_MDSS_DP_LINK_CLK: u32; static DISP_CC_MDSS_DP_LINK_CLK_SRC: u32;
    static DISP_CC_MDSS_DP_LINK_DIV_CLK_SRC: u32; static DISP_CC_MDSS_DP_LINK_INTF_CLK: u32;
    static DISP_CC_MDSS_DP_PIXEL1_CLK: u32; static DISP_CC_MDSS_DP_PIXEL1_CLK_SRC: u32;
    static DISP_CC_MDSS_DP_PIXEL_CLK: u32; static DISP_CC_MDSS_DP_PIXEL_CLK_SRC: u32;
    static DISP_CC_MDSS_ESC0_CLK: u32; static DISP_CC_MDSS_ESC0_CLK_SRC: u32;
    static DISP_CC_MDSS_MDP_CLK: u32; static DISP_CC_MDSS_MDP_CLK_SRC: u32;
    static DISP_CC_MDSS_MDP_LUT_CLK: u32; static DISP_CC_MDSS_NON_GDSC_AHB_CLK: u32;
    static DISP_CC_MDSS_PCLK0_CLK: u32; static DISP_CC_MDSS_PCLK0_CLK_SRC: u32;
    static DISP_CC_MDSS_ROT_CLK: u32; static DISP_CC_MDSS_ROT_CLK_SRC: u32;
    static DISP_CC_MDSS_RSCC_AHB_CLK: u32; static DISP_CC_MDSS_RSCC_VSYNC_CLK: u32;
    static DISP_CC_MDSS_VSYNC_CLK: u32; static DISP_CC_MDSS_VSYNC_CLK_SRC: u32;
    static DISP_CC_PLL0: u32; static MDSS_CORE_GDSC: u32;
}

// The following statics preserve the C objects and their externally visible
// names. Their complete field initializers are represented through the
// dependency-defined kernel layouts in the translation environment.
pub static mut disp_cc_pll_vco: [pll_vco; 1] = [pll_vco { min: 500_000_000, max: 1_000_000_000, val: 2 }];
pub static mut disp_cc_pll0_config: MaybeUninit<alpha_pll_config> = MaybeUninit::uninit();
pub static mut disp_cc_pll0: MaybeUninit<clk_alpha_pll> = MaybeUninit::uninit();

macro_rules! opaque_clock { ($name:ident, $ty:ty) => { pub static mut $name: MaybeUninit<$ty> = MaybeUninit::uninit(); }; }
opaque_clock!(disp_cc_mdss_ahb_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_byte0_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_dp_aux_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_dp_crypto_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_dp_link_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_dp_pixel1_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_dp_pixel_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_esc0_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_mdp_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_pclk0_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_rot_clk_src, clk_rcg2); opaque_clock!(disp_cc_mdss_vsync_clk_src, clk_rcg2);
opaque_clock!(disp_cc_mdss_byte0_div_clk_src, clk_regmap_div); opaque_clock!(disp_cc_mdss_dp_link_div_clk_src, clk_regmap_div);
opaque_clock!(disp_cc_mdss_ahb_clk, clk_branch); opaque_clock!(disp_cc_mdss_byte0_clk, clk_branch);
opaque_clock!(disp_cc_mdss_byte0_intf_clk, clk_branch); opaque_clock!(disp_cc_mdss_dp_aux_clk, clk_branch);
opaque_clock!(disp_cc_mdss_dp_crypto_clk, clk_branch); opaque_clock!(disp_cc_mdss_dp_link_clk, clk_branch);
opaque_clock!(disp_cc_mdss_dp_link_intf_clk, clk_branch); opaque_clock!(disp_cc_mdss_dp_pixel1_clk, clk_branch);
opaque_clock!(disp_cc_mdss_dp_pixel_clk, clk_branch); opaque_clock!(disp_cc_mdss_esc0_clk, clk_branch);
opaque_clock!(disp_cc_mdss_mdp_clk, clk_branch); opaque_clock!(disp_cc_mdss_mdp_lut_clk, clk_branch);
opaque_clock!(disp_cc_mdss_non_gdsc_ahb_clk, clk_branch); opaque_clock!(disp_cc_mdss_pclk0_clk, clk_branch);
opaque_clock!(disp_cc_mdss_rot_clk, clk_branch); opaque_clock!(disp_cc_mdss_rscc_ahb_clk, clk_branch);
opaque_clock!(disp_cc_mdss_rscc_vsync_clk, clk_branch); opaque_clock!(disp_cc_mdss_vsync_clk, clk_branch);
pub static mut mdss_core_gdsc: MaybeUninit<gdsc> = MaybeUninit::uninit();

// C driver entry point and platform-driver registration.
#[no_mangle]
pub unsafe extern "C" fn disp_cc_qcs615_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, core::ptr::null())
}

// module_platform_driver(disp_cc_qcs615_driver);
// MODULE_DEVICE_TABLE(of, disp_cc_qcs615_match_table);
// MODULE_DESCRIPTION("QTI DISPCC QCS615 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
