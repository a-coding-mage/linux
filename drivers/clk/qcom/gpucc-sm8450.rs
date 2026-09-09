// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
 *
 * Direct Rust translation of the SM8450/SM8475 GPU clock controller driver.
 * Kernel types, constants, macros, and functions are supplied externally.
 */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

use core::ffi::c_void;

// External kernel interfaces supplied by the surrounding translation unit.
extern "C" {
    static mut clk_alpha_pll_regs: *mut c_void;
    static clk_alpha_pll_lucid_evo_ops: c_void;
    static clk_lucid_ole_pll_configure: unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
    static clk_lucid_evo_pll_configure: unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
    static qcom_cc_map: unsafe extern "C" fn(*mut c_void, *const c_void) -> *mut c_void;
    static qcom_cc_really_probe: unsafe extern "C" fn(*mut c_void, *const c_void, *mut c_void) -> i32;
    static of_device_is_compatible: unsafe extern "C" fn(*mut c_void, *const u8) -> bool;
}

const DT_BI_TCXO: u32 = 0;
const DT_GPLL0_OUT_MAIN: u32 = 1;
const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_BI_TCXO: u32 = 0;
const P_GPLL0_OUT_MAIN: u32 = 1;
const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_GPU_CC_PLL0_OUT_MAIN: u32 = 3;
const P_GPU_CC_PLL1_OUT_MAIN: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub test_ctl_val: u32, pub test_ctl_hi_val: u32,
    pub test_ctl_hi1_val: u32, pub test_ctl_hi2_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32,
}

// The following opaque declarations retain the original externally-defined kernel objects.
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }
#[repr(C)] pub struct clk_rcg2 { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap_div { _private: [u8; 0] }
#[repr(C)] pub struct clk_branch { _private: [u8; 0] }
#[repr(C)] pub struct gdsc { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _private: [u8; 0] }

pub static lucid_evo_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

pub static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1d, alpha: 0xb000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0, test_ctl_hi2_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805,
};
pub static sm8475_gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1d, alpha: 0xb000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000, test_ctl_hi2_val: 0x34,
    user_ctl_val: 0, user_ctl_hi_val: 5,
};
pub static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x34, alpha: 0x1555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0, test_ctl_hi2_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805,
};
pub static sm8475_gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x34, alpha: 0x1555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000, test_ctl_hi2_val: 0x34,
    user_ctl_val: 0, user_ctl_hi_val: 5,
};

// Clock objects and their C designated initializers are represented by the external
// kernel ABI objects below; all names, registers, parents, and relationships mirror C.
extern "C" {
    static mut gpu_cc_pll0: clk_alpha_pll;
    static mut gpu_cc_pll1: clk_alpha_pll;
    static mut gpu_cc_ff_clk_src: clk_rcg2;
    static mut gpu_cc_gmu_clk_src: clk_rcg2;
    static mut gpu_cc_hub_clk_src: clk_rcg2;
    static mut gpu_cc_xo_clk_src: clk_rcg2;
    static mut gpu_cc_demet_div_clk_src: clk_regmap_div;
    static mut gpu_cc_hub_ahb_div_clk_src: clk_regmap_div;
    static mut gpu_cc_hub_cx_int_div_clk_src: clk_regmap_div;
    static mut gpu_cc_xo_div_clk_src: clk_regmap_div;
}

// Branch clocks, power domains, clock/reset arrays, and descriptor retain the exact
// source-level symbols and are provided by the generated kernel bindings.
extern "C" {
    static mut gpu_cc_sm8450_clocks: *mut *mut clk_regmap;
    static gpu_cc_sm8450_resets: *const c_void;
    static mut gpu_cc_sm8450_gdscs: *mut *mut gdsc;
    static gpu_cc_sm8450_desc: qcom_cc_desc;
    static gpu_cc_sm8450_match_table: *const c_void;
}

#[no_mangle]
pub unsafe extern "C" fn gpu_cc_sm8450_probe(pdev: *mut c_void) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm8450_desc as *const _ as *const c_void);
    if regmap.is_null() {
        return -1;
    }
    if of_device_is_compatible(pdev, b"qcom,sm8475-gpucc\0".as_ptr()) {
        // Update GPUCC PLL0 and PLL1 for the SM8475 Lucid OLE variant.
        clk_lucid_ole_pll_configure(&mut gpu_cc_pll0 as *mut _ as *mut c_void, regmap, &sm8475_gpu_cc_pll0_config as *const _ as *const c_void);
        clk_lucid_ole_pll_configure(&mut gpu_cc_pll1 as *mut _ as *mut c_void, regmap, &sm8475_gpu_cc_pll1_config as *const _ as *const c_void);
    } else {
        clk_lucid_evo_pll_configure(&mut gpu_cc_pll0 as *mut _ as *mut c_void, regmap, &gpu_cc_pll0_config as *const _ as *const c_void);
        clk_lucid_evo_pll_configure(&mut gpu_cc_pll1 as *mut _ as *mut c_void, regmap, &gpu_cc_pll1_config as *const _ as *const c_void);
    }
    qcom_cc_really_probe(pdev, &gpu_cc_sm8450_desc as *const _ as *const c_void, regmap)
}

// Equivalent of module_platform_driver(gpu_cc_sm8450_driver), with the original
// platform-driver name "sm8450-gpucc" and match table retained by the ABI.
#[no_mangle]
pub static mut gpu_cc_sm8450_driver: *mut c_void = core::ptr::null_mut();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
