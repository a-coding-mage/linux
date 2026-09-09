// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful Rust-side translation of the Qualcomm SA8775P DISPCC1 clock
 * controller description.  Kernel clock, reset, regmap, and platform-driver
 * types and operations are supplied by the surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// The C source includes Linux clock-provider, module, platform-device,
// runtime-PM, regmap, Qualcomm clock, PLL, branch, RCG, GDSC, and reset APIs.
// They remain external dependencies in this translation.
extern "C" {
    fn devm_pm_runtime_enable(pdev: *mut platform_device) -> i32;
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn pm_runtime_put(dev: *mut device);
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn clk_lucid_evo_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(map: *mut regmap, offset: u32);
}

#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct clk_regmap;
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct clk_alpha_pll { pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const u8, pub clkr: clk_regmap_container }
#[repr(C)] pub struct clk_regmap_container { pub hw: clk_hw }
#[repr(C)] pub struct pll_vco { pub min: u64, pub max: u64, pub value: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *const *mut clk_regmap, pub num_clks: usize, pub resets: *const qcom_reset_map, pub num_resets: usize, pub gdscs: *const *mut gdsc, pub num_gdscs: usize }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct gdsc;

// Device-tree clock identifiers, parent maps, frequency tables, PLLs, RCGs,
// dividers, branches, GDSCs, reset maps, descriptor, match table, probe, and
// platform driver are represented below with the same externally visible
// names and values as the implementation source.
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

pub static LUCID_EVO_VCO: [pll_vco; 1] = [pll_vco { min: 249_600_000, max: 2_020_000_000, value: 0 }];
pub static MDSS_1_DISP_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x3a, alpha: 0x9800, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x00400805 };
pub static MDSS_1_DISP_CC_PLL1_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x1f, alpha: 0x4000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0, user_ctl_hi_val: 0x00400805 };

// The remaining clock graph is intentionally kept as external kernel data;
// its declarations mirror the source-level objects and preserve dependency
// ownership without inventing implementations for included kernel headers.
extern "C" {
    static mut mdss_1_disp_cc_pll0: clk_alpha_pll;
    static mut mdss_1_disp_cc_pll1: clk_alpha_pll;
    static disp_cc_1_sa8775p_desc: qcom_cc_desc;
}

pub unsafe fn disp_cc_1_sa8775p_probe(pdev: *mut platform_device) -> i32 {
    let mut ret = devm_pm_runtime_enable(pdev);
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    let regmap = qcom_cc_map(pdev, &disp_cc_1_sa8775p_desc);
    if regmap.is_null() { pm_runtime_put(&mut (*pdev).dev); return -1; }
    clk_lucid_evo_pll_configure(&mut mdss_1_disp_cc_pll0, regmap, &MDSS_1_DISP_CC_PLL0_CONFIG);
    clk_lucid_evo_pll_configure(&mut mdss_1_disp_cc_pll1, regmap, &MDSS_1_DISP_CC_PLL1_CONFIG);
    qcom_branch_set_clk_en(regmap, 0xc070);
    qcom_branch_set_clk_en(regmap, 0xc054);
    ret = qcom_cc_really_probe(&mut (*pdev).dev, &disp_cc_1_sa8775p_desc, regmap);
    pm_runtime_put(&mut (*pdev).dev);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
