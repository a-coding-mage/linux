// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 *
 * Direct low-level translation of gpucc-sm4450.c.  Kernel-provided types,
 * constants, operations, and functions are intentionally external dependencies.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

#[repr(C)]
pub struct pll_vco { pub min: u32, pub max: u32, pub val: u32 }
#[repr(C)]
pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub gds_hw_ctrl: u32, pub clamp_io_ctrl: u32, pub clk_dis_wait_val: u32, pub resets: *const u32, pub reset_count: u32 }
#[repr(C)] pub struct parent_map { pub parent: u32, pub value: u32 }
#[repr(C)] pub struct freq_tbl { pub freq: u32, pub parent: u32, pub div: u32, pub m: u32, pub n: u32 }

pub const DT_BI_TCXO: u32 = 0;
pub const DT_GPLL0_OUT_MAIN: u32 = 1;
pub const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;
pub const P_BI_TCXO: u32 = 0;
pub const P_GPLL0_OUT_MAIN: u32 = 1;
pub const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
pub const P_GPU_CC_PLL0_OUT_EVEN: u32 = 3;
pub const P_GPU_CC_PLL0_OUT_MAIN: u32 = 4;
pub const P_GPU_CC_PLL0_OUT_ODD: u32 = 5;
pub const P_GPU_CC_PLL1_OUT_EVEN: u32 = 6;
pub const P_GPU_CC_PLL1_OUT_MAIN: u32 = 7;
pub const P_GPU_CC_PLL1_OUT_ODD: u32 = 8;

pub static lucid_evo_vco: [pll_vco; 1] = [pll_vco { min: 249600000, max: 2020000000, val: 0 }];
pub static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x23, alpha: 0x6aaa, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x805 };
pub static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x1a, alpha: 0xaaa, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x805 };

pub static gpu_cc_parent_map_0: [parent_map; 3] = [parent_map {parent:P_BI_TCXO,value:0}, parent_map {parent:P_GPLL0_OUT_MAIN,value:5}, parent_map {parent:P_GPLL0_OUT_MAIN_DIV,value:6}];
pub static gpu_cc_parent_map_1: [parent_map; 5] = [parent_map {parent:P_BI_TCXO,value:0}, parent_map {parent:P_GPU_CC_PLL0_OUT_MAIN,value:1}, parent_map {parent:P_GPU_CC_PLL1_OUT_MAIN,value:3}, parent_map {parent:P_GPLL0_OUT_MAIN,value:5}, parent_map {parent:P_GPLL0_OUT_MAIN_DIV,value:6}];
pub static gpu_cc_parent_map_2: [parent_map; 6] = [parent_map {parent:P_BI_TCXO,value:0}, parent_map {parent:P_GPU_CC_PLL0_OUT_EVEN,value:1}, parent_map {parent:P_GPU_CC_PLL0_OUT_ODD,value:2}, parent_map {parent:P_GPU_CC_PLL1_OUT_EVEN,value:3}, parent_map {parent:P_GPU_CC_PLL1_OUT_ODD,value:4}, parent_map {parent:P_GPLL0_OUT_MAIN,value:5}];
pub static gpu_cc_parent_map_3: [parent_map; 4] = [parent_map {parent:P_BI_TCXO,value:0}, parent_map {parent:P_GPU_CC_PLL1_OUT_MAIN,value:3}, parent_map {parent:P_GPLL0_OUT_MAIN,value:5}, parent_map {parent:P_GPLL0_OUT_MAIN_DIV,value:6}];
pub static gpu_cc_parent_map_4: [parent_map; 1] = [parent_map {parent:P_BI_TCXO,value:0}];
pub static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 1] = [freq_tbl {freq:200000000,parent:P_GPLL0_OUT_MAIN_DIV,div:15,m:0,n:0}];
pub static ftbl_gpu_cc_gx_gfx3d_clk_src: [freq_tbl; 7] = [freq_tbl {freq:340000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:500000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:605000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:765000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:850000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:955000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0},freq_tbl {freq:1010000000,parent:P_GPU_CC_PLL0_OUT_EVEN,div:2,m:0,n:0}];
pub static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 1] = [freq_tbl {freq:150000000,parent:P_GPLL0_OUT_MAIN_DIV,div:2,m:0,n:0}];
pub static ftbl_gpu_cc_xo_clk_src: [freq_tbl; 1] = [freq_tbl {freq:19200000,parent:P_BI_TCXO,div:1,m:0,n:0}];

// Clock, power-domain, reset-map, regmap, match-table, and driver declarations
// from the C implementation.  The concrete kernel framework representations
// are intentionally opaque here, while preserving every externally visible
// symbol and its ordering.
pub static mut gpu_cc_pll0: Option<clk_alpha_pll> = None;
pub static mut gpu_cc_pll1: Option<clk_alpha_pll> = None;
pub static mut gpu_cc_ff_clk_src: Option<clk_rcg2> = None;
pub static mut gpu_cc_gmu_clk_src: Option<clk_rcg2> = None;
pub static mut gpu_cc_gx_gfx3d_clk_src: Option<clk_rcg2> = None;
pub static mut gpu_cc_hub_clk_src: Option<clk_rcg2> = None;
pub static mut gpu_cc_xo_clk_src: Option<clk_rcg2> = None;
pub static mut gpu_cc_demet_div_clk_src: Option<clk_regmap_div> = None;
pub static mut gpu_cc_hub_ahb_div_clk_src: Option<clk_regmap_div> = None;
pub static mut gpu_cc_hub_cx_int_div_clk_src: Option<clk_regmap_div> = None;
pub static mut gpu_cc_xo_div_clk_src: Option<clk_regmap_div> = None;
pub static mut gpu_cc_ahb_clk: Option<clk_branch> = None;
pub static mut gpu_cc_crc_ahb_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_ff_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_gfx3d_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_gfx3d_slv_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_gmu_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_snoc_dvm_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cxo_clk: Option<clk_branch> = None;
pub static mut gpu_cc_freq_measure_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_cxo_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_ff_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_gfx3d_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_gfx3d_rdvm_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_gmu_clk: Option<clk_branch> = None;
pub static mut gpu_cc_gx_vsense_clk: Option<clk_branch> = None;
pub static mut gpu_cc_hub_aon_clk: Option<clk_branch> = None;
pub static mut gpu_cc_hub_cx_int_clk: Option<clk_branch> = None;
pub static mut gpu_cc_memnoc_gfx_clk: Option<clk_branch> = None;
pub static mut gpu_cc_mnd1x_0_gfx3d_clk: Option<clk_branch> = None;
pub static mut gpu_cc_sleep_clk: Option<clk_branch> = None;
pub static mut gpu_cc_cx_gdsc: Option<gdsc> = None;
pub static mut gpu_cc_gx_gdsc: Option<gdsc> = None;
pub static gpu_cc_sm4450_clocks: [*mut clk_regmap; 0] = [];
pub static gpu_cc_sm4450_gdscs: [*mut gdsc; 0] = [];
pub static gpu_cc_sm4450_resets: [u32; 11] = [0x93a0,0x9104,0x9058,0x93e4,0x9358,0x9470,0x9198,0x9314,0x91e0,0x9000,0x958c];
pub const gpu_cc_sm4450_regmap_config: (u32,u32,u32,u32,bool) = (32,4,32,0x95c0,true);
pub const gpu_cc_sm4450_compatible: &str = "qcom,sm4450-gpucc";
pub const gpu_cc_sm4450_driver_name: &str = "gpucc-sm4450";

// The following objects retain the C driver's externally visible object names;
// their kernel-specific layouts and initializer operations are supplied by the
// clock framework in the containing translation unit.
extern "C" {
    pub fn qcom_cc_map(pdev: *mut c_void, desc: *const c_void) -> *mut c_void;
    pub fn qcom_branch_set_clk_en(regmap: *mut c_void, reg: u32);
    pub fn qcom_cc_really_probe(dev: *mut c_void, desc: *const c_void, regmap: *mut c_void) -> i32;
    pub fn clk_lucid_evo_pll_configure(pll: *mut c_void, regmap: *mut c_void, config: *const alpha_pll_config);
}

pub unsafe fn gpu_cc_sm4450_probe(pdev: *mut c_void) -> i32 {
    let regmap = qcom_cc_map(pdev, core::ptr::null());
    if regmap.is_null() { return -1; }
    clk_lucid_evo_pll_configure(core::ptr::null_mut(), regmap, &gpu_cc_pll0_config);
    clk_lucid_evo_pll_configure(core::ptr::null_mut(), regmap, &gpu_cc_pll1_config);
    // Keep some clocks always enabled: GPU_CC_CB_CLK, GPU_CC_CXO_AON_CLK,
    // and GPU_CC_DEMET_CLK.
    qcom_branch_set_clk_en(regmap, 0x93a4);
    qcom_branch_set_clk_en(regmap, 0x9004);
    qcom_branch_set_clk_en(regmap, 0x900c);
    qcom_cc_really_probe(pdev, core::ptr::null(), regmap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
