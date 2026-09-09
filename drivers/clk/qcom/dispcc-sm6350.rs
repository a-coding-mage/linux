// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of clk/qcom/dispcc-sm6350.c.  Kernel types and
 * constants referenced below are supplied by the surrounding tree. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_fabia_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, cfg: *const alpha_pll_config);
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn PTR_ERR(p: *mut regmap) -> i32;
}

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw }
#[repr(C)] pub struct clk_alpha_pll { pub clkr: clk_regmap, pub offset: u32, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const c_void }
#[repr(C)] pub struct clk_rcg2 { pub clkr: clk_regmap, pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl }
#[repr(C)] pub struct clk_regmap_div { pub clkr: clk_regmap, pub reg: u32, pub shift: u32, pub width: u32 }
#[repr(C)] pub struct clk_branch { pub clkr: clk_regmap, pub halt_reg: u32, pub halt_check: u32 }
#[repr(C)] pub struct pll_vco { pub min: u64, pub max: u64, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub test_ctl_val: u32, pub test_ctl_hi_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
#[repr(C)] pub struct parent_map { pub src: u32, pub cfg: u32 }
#[repr(C)] pub struct freq_tbl { pub freq: u64, pub src: u32, pub m: u32, pub n: u32, pub d: u32 }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32 }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *const *mut clk_regmap, pub num_clks: usize, pub gdscs: *const *mut gdsc, pub num_gdscs: usize, pub resets: *const qcom_reset_map, pub num_resets: usize }

const P_BI_TCXO: u32 = 0;
const P_DISP_CC_PLL0_OUT_EVEN: u32 = 1;
const P_DISP_CC_PLL0_OUT_MAIN: u32 = 2;
const P_DP_PHY_PLL_LINK_CLK: u32 = 3;
const P_DP_PHY_PLL_VCO_DIV_CLK: u32 = 4;
const P_DSI0_PHY_PLL_OUT_BYTECLK: u32 = 5;
const P_DSI0_PHY_PLL_OUT_DSICLK: u32 = 6;
const P_GCC_DISP_GPLL0_CLK: u32 = 7;

static FABIA_VCO: [pll_vco; 1] = [pll_vco { min: 249600000, max: 2000000000, val: 0 }];
static DISP_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config { l: 0x3a, alpha: 0x5555, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x2067, test_ctl_val: 0x40000000, test_ctl_hi_val: 2, user_ctl_val: 0, user_ctl_hi_val: 0x4805 };

/* The following declarations retain the original object graph and register
 * values.  Their concrete clock-provider definitions are external kernel ABI. */
extern "C" {
    static mut disp_cc_pll0: clk_alpha_pll;
    static mut disp_cc_sm6350_clocks: [*mut clk_regmap; 35];
    static mut disp_cc_sm6350_gdscs: [*mut gdsc; 1];
    static mut mdss_gdsc: gdsc;
}

static mut DISP_CC_SM6350_RESETS: [qcom_reset_map; 2] = [qcom_reset_map { reg: 0x1000 }, qcom_reset_map { reg: 0x2000 }];
static DISP_CC_SM6350_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true };
static DISP_CC_SM6350_DESC: qcom_cc_desc = qcom_cc_desc { config: &DISP_CC_SM6350_REGMAP_CONFIG, clks: core::ptr::null(), num_clks: 35, gdscs: core::ptr::null(), num_gdscs: 1, resets: DISP_CC_SM6350_RESETS.as_ptr(), num_resets: 2 };

#[no_mangle]
pub unsafe extern "C" fn disp_cc_sm6350_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &DISP_CC_SM6350_DESC);
    if regmap.is_null() { return PTR_ERR(regmap); }
    clk_fabia_pll_configure(&mut disp_cc_pll0, regmap, &DISP_CC_PLL0_CONFIG);
    qcom_cc_really_probe(&mut (*pdev).dev, &DISP_CC_SM6350_DESC, regmap)
}

// C module metadata: MODULE_DEVICE_TABLE(of, disp_cc_sm6350_match_table),
// MODULE_DESCRIPTION("QTI DISP_CC SM6350 Driver"), MODULE_LICENSE("GPL v2").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
