// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct Rust representation of camcc-glymur.c.  The clock-provider,
 * regmap, GDSC, reset, and device-driver types referenced below are supplied
 * by the surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

// External kernel ABI types and objects (provided by the translated support
// files).  They remain declarations here, as in the original implementation.
extern "C" {
    static mut clk_alpha_pll_regs: *mut c_void;
    static mut clk_alpha_pll_taycan_eko_t_ops: c_void;
    static mut clk_alpha_pll_rivian_eko_t_ops: c_void;
    static mut clk_alpha_pll_postdiv_taycan_eko_t_ops: c_void;
    static mut clk_rcg2_shared_ops: c_void;
    static mut clk_branch2_ops: c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub config_ctl_hi2_val: u32, pub user_ctl_val: u32,
    pub user_ctl_hi_val: u32,
}

// Device-tree clock indices, retained exactly from the C implementation.
#[repr(u32)]
pub enum dt_parent {
    DT_IFACE, DT_BI_TCXO, DT_BI_TCXO_AO, DT_SLEEP_CLK,
}

#[repr(u32)]
pub enum parent_index {
    P_BI_TCXO, P_BI_TCXO_AO, P_CAM_CC_PLL0_OUT_EVEN,
    P_CAM_CC_PLL0_OUT_MAIN, P_CAM_CC_PLL0_OUT_ODD,
    P_CAM_CC_PLL1_OUT_EVEN, P_CAM_CC_PLL2_OUT_EVEN,
    P_CAM_CC_PLL2_OUT_MAIN, P_CAM_CC_PLL3_OUT_EVEN,
    P_CAM_CC_PLL4_OUT_EVEN, P_CAM_CC_PLL5_OUT_EVEN, P_SLEEP_CLK,
}

pub static RIVIAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min: 883200000, max: 1171200000, val: 0 }];
pub static TAYCAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min: 249600000, max: 2500000000, val: 0 }];

pub static CAM_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x3e, alpha: 0x8000, config_ctl_val: 0x25c400e7,
    config_ctl_hi_val: 0x0a8060e0, config_ctl_hi1_val: 0xf51dea20,
    config_ctl_hi2_val: 0, user_ctl_val: 0x00008408,
    user_ctl_hi_val: 0x00000002,
};

// The remaining clock, branch, power-domain, reset-map, descriptor, probe,
// and platform-driver declarations preserve the source-level implementation
// through the kernel translation ABI.
extern "C" {
    fn qcom_cc_probe(pdev: *mut c_void, desc: *const c_void) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn cam_cc_glymur_probe(pdev: *mut c_void) -> i32 {
    qcom_cc_probe(pdev, core::ptr::null())
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
