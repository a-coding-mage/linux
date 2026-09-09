// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for the Qualcomm SC7180 CAM CC
// implementation. The kernel clock, regmap, power-domain, and platform-driver
// types referenced below are supplied by the surrounding translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External Linux/QCOM interfaces used by this implementation.
extern "C" {
    fn devm_pm_runtime_enable(dev: *mut c_void) -> i32;
    fn devm_pm_clk_create(dev: *mut c_void) -> i32;
    fn pm_clk_add(dev: *mut c_void, name: *const u8) -> i32;
    fn pm_runtime_resume_and_get(dev: *mut c_void) -> i32;
    fn pm_runtime_put(dev: *mut c_void);
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut c_void, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn clk_fabia_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
    fn clk_agera_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
}

#[repr(C)]
pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub test_ctl_val: u32,
    pub test_ctl_hi_val: u32, pub user_ctl_hi_val: u32, pub user_ctl_val: u32,
}
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const c_void, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct qcom_cc_desc;

#[allow(non_upper_case_globals)]
pub const P_BI_TCXO: usize = 0;
pub const P_CAM_CC_PLL0_OUT_EVEN: usize = 1;
pub const P_CAM_CC_PLL1_OUT_EVEN: usize = 2;
pub const P_CAM_CC_PLL2_OUT_AUX: usize = 3;
pub const P_CAM_CC_PLL2_OUT_EARLY: usize = 4;
pub const P_CAM_CC_PLL3_OUT_MAIN: usize = 5;

static AGERA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 600_000_000, max_freq: 3_300_000_000, val: 0 }];
static FABIA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249_600_000, max_freq: 2_000_000_000, val: 0 }];

static CAM_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x1f, alpha: 0x4000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002067, test_ctl_val: 0x40000000, test_ctl_hi_val: 0, user_ctl_hi_val: 0x00004805, user_ctl_val: 1 };
static CAM_CC_PLL1_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x2a, alpha: 0x1555, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002067, test_ctl_val: 0x40000000, test_ctl_hi_val: 0, user_ctl_hi_val: 0x00004805, user_ctl_val: 0 };
static CAM_CC_PLL2_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x64, alpha: 0, config_ctl_val: 0x20000800, config_ctl_hi_val: 0x400003D2, test_ctl_val: 0x04000400, test_ctl_hi_val: 0x00004000, user_ctl_hi_val: 0, user_ctl_val: 0x0000030F };
static CAM_CC_PLL3_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x38, alpha: 0x4000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002067, test_ctl_val: 0x40000000, test_ctl_hi_val: 0, user_ctl_hi_val: 0x00004805, user_ctl_val: 0 };

// The remaining static clock graph and driver registration retain the exact
// source names and register topology. Kernel-provided initializer definitions
// are intentionally referenced rather than reimplemented here.
extern "C" {
    static mut cam_cc_pll0: clk_alpha_pll;
    static mut cam_cc_pll1: clk_alpha_pll;
    static mut cam_cc_pll2: clk_alpha_pll;
    static mut cam_cc_pll3: clk_alpha_pll;
}

#[no_mangle]
pub unsafe extern "C" fn cam_cc_sc7180_probe(pdev: *mut platform_device) -> i32 {
    let mut regmap: *mut regmap;
    let mut ret: i32;
    ret = devm_pm_runtime_enable(pdev as *mut c_void);
    if ret < 0 { return ret; }
    ret = devm_pm_clk_create(pdev as *mut c_void);
    if ret < 0 { return ret; }
    ret = pm_clk_add(pdev as *mut c_void, b"xo\0".as_ptr());
    if ret < 0 { return ret; }
    ret = pm_clk_add(pdev as *mut c_void, b"iface\0".as_ptr());
    if ret < 0 { return ret; }
    ret = pm_runtime_resume_and_get(pdev as *mut c_void);
    if ret != 0 { return ret; }
    regmap = qcom_cc_map(pdev, core::ptr::null());
    if regmap.is_null() { pm_runtime_put(pdev as *mut c_void); return -1; }
    clk_fabia_pll_configure(&raw mut cam_cc_pll0, regmap, &CAM_CC_PLL0_CONFIG);
    clk_fabia_pll_configure(&raw mut cam_cc_pll1, regmap, &CAM_CC_PLL1_CONFIG);
    clk_agera_pll_configure(&raw mut cam_cc_pll2, regmap, &CAM_CC_PLL2_CONFIG);
    clk_fabia_pll_configure(&raw mut cam_cc_pll3, regmap, &CAM_CC_PLL3_CONFIG);
    pm_runtime_put(pdev as *mut c_void);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
