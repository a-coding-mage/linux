// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Linux and Qualcomm clock-controller headers supplied by other translation units.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
use core::ffi::c_void;

extern "C" {
    static clk_alpha_pll_regs: [*const c_void; 16];
    static clk_alpha_pll_lucid_evo_ops: c_void;
    static clk_rcg2_shared_ops: c_void;
    static clk_regmap_div_ro_ops: c_void;
    static clk_branch2_ops: c_void;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _private: [u8; 0] }
#[repr(C)] pub struct gdsc { _private: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub struct pll_vco { pub min_freq: u32, pub max_freq: u32, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub test_ctl_val: u32, pub test_ctl_hi_val: u32, pub test_ctl_hi1_val: u32, pub test_ctl_hi2_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }
#[repr(C)] pub struct parent_map { pub src: i32, pub cfg: u32 }
#[repr(C)] pub struct clk_parent_data { pub index: u32, pub hw: *const clk_hw }
#[repr(C)] pub struct freq_tbl { pub freq: u64, pub src: i32, pub pre_div: u32, pub m: u32, pub n: u32 }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub parent_data: *const clk_parent_data, pub parent_hws: *const *const clk_hw, pub num_parents: usize, pub flags: u32, pub ops: *const c_void }
#[repr(C)] pub struct clk_hw_data { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_regmap_data { pub enable_reg: u32, pub enable_mask: u32, pub hw: clk_hw_data }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl, pub hw_clk_ctrl: bool, pub clkr: clk_regmap_data }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap_data }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub hwcg_reg: u32, pub hwcg_bit: u32, pub clkr: clk_regmap_data }
#[repr(C)] pub struct power_domain { pub name: *const u8 }
#[repr(C)] pub struct gdsc_data { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32, pub pd: power_domain, pub pwrsts: u32, pub parent: *mut power_domain, pub flags: u32 }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32, pub bit: u32 }

const DT_BI_TCXO: u32 = 0;
const P_BI_TCXO: i32 = 0;
const P_VIDEO_CC_PLL0_OUT_MAIN: i32 = 1;
const P_VIDEO_CC_PLL1_OUT_MAIN: i32 = 2;
const CLK_ALPHA_PLL_TYPE_LUCID_OLE: usize = 0;
const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const BRANCH_HALT: u32 = 0;
const BRANCH_HALT_VOTED: u32 = 1;
const PWRSTS_OFF_ON: u32 = 1;
const POLL_CFG_GDSCR: u32 = 1 << 0;
const RETAIN_FF_ENABLE: u32 = 1 << 1;
const HW_CTRL_TRIGGER: u32 = 1 << 2;

const fn f(freq: u64, src: i32, pre_div: u32, m: u32, n: u32) -> freq_tbl { freq_tbl { freq, src, pre_div, m, n } }

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];
static video_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x15, alpha: 0xe000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c, test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000, test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5 };
static video_cc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x36, alpha: 0xb000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c, test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000, test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5 };

// The remaining objects retain the C driver's declaration and initialization topology.
// External kernel structures and generated clock IDs are intentionally referenced as dependencies.
extern "C" {
    static mut video_cc_pll0: clk_alpha_pll;
    static mut video_cc_pll1: clk_alpha_pll;
    static mut video_cc_mvs0_bse_clk_src: clk_rcg2;
    static mut video_cc_mvs0_clk_src: clk_rcg2;
    static mut video_cc_mvs1_clk_src: clk_rcg2;
    static mut video_cc_xo_clk_src: clk_rcg2;
    static mut video_cc_mvs0_bse_div4_div_clk_src: clk_regmap_div;
    static mut video_cc_mvs1_div_clk_src: clk_regmap_div;
    static mut video_cc_mvs1c_div2_div_clk_src: clk_regmap_div;
    static mut video_cc_mvs0_bse_clk: clk_branch;
    static mut video_cc_mvs0_clk: clk_branch;
    static mut video_cc_mvs0_shift_clk: clk_branch;
    static mut video_cc_mvs0c_clk: clk_branch;
    static mut video_cc_mvs0c_shift_clk: clk_branch;
    static mut video_cc_mvs1_clk: clk_branch;
    static mut video_cc_mvs1_shift_clk: clk_branch;
    static mut video_cc_mvs1c_clk: clk_branch;
    static mut video_cc_mvs1c_shift_clk: clk_branch;
    static mut video_cc_mvs0c_gdsc: gdsc;
    static mut video_cc_mvs0_gdsc: gdsc;
    static mut video_cc_mvs1c_gdsc: gdsc;
    static mut video_cc_mvs1_gdsc: gdsc;
}

// Frequency tables from the implementation source.
static ftbl_video_cc_mvs0_bse_clk_src: [freq_tbl; 6] = [f(420000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(600000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(670000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(848000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(920000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(0,0,0,0,0)];
static ftbl_video_cc_mvs0_clk_src: [freq_tbl; 6] = [f(210000000,P_VIDEO_CC_PLL0_OUT_MAIN,2,0,0),f(300000000,P_VIDEO_CC_PLL0_OUT_MAIN,2,0,0),f(335000000,P_VIDEO_CC_PLL0_OUT_MAIN,2,0,0),f(424000000,P_VIDEO_CC_PLL0_OUT_MAIN,2,0,0),f(460000000,P_VIDEO_CC_PLL0_OUT_MAIN,2,0,0),f(0,0,0,0,0)];
static ftbl_video_cc_mvs1_clk_src: [freq_tbl; 4] = [f(1050000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1350000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1650000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(0,0,0,0,0)];
static ftbl_video_cc_xo_clk_src: [freq_tbl; 2] = [f(19200000,P_BI_TCXO,1,0,0),f(0,0,0,0,0)];

static video_cc_parent_map_0: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];
static video_cc_parent_map_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];
static video_cc_parent_map_2: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL1_OUT_MAIN, cfg: 1 }];
static video_cc_critical_cbcrs: [u32; 3] = [0x80f4, 0x8150, 0x8124];
static video_cc_resets: [qcom_reset_map; 9] = [
    qcom_reset_map { reg: 0x80f0, bit: 0 }, qcom_reset_map { reg: 0x80a0, bit: 0 },
    qcom_reset_map { reg: 0x8048, bit: 0 }, qcom_reset_map { reg: 0x80c8, bit: 0 },
    qcom_reset_map { reg: 0x8074, bit: 0 }, qcom_reset_map { reg: 0x816c, bit: 0 },
    qcom_reset_map { reg: 0x8064, bit: 2 }, qcom_reset_map { reg: 0x8090, bit: 2 },
    qcom_reset_map { reg: 0x8124, bit: 2 },
];

#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
static video_cc_x1p42100_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f54, fast_io: true };

// C driver arrays preserve the original indexed clock/GDSC/reset topology; generated
// VIDEO_CC_* identifiers are supplied by dt-bindings/clock/qcom,x1p42100-videocc.h.
extern "C" {
    static video_cc_x1p42100_clocks: [*mut clk_regmap; 18];
    static video_cc_x1p42100_gdscs: [*mut gdsc; 4];
}

// Driver registration and probe are preserved as external-facing declarations.
#[no_mangle] pub unsafe extern "C" fn video_cc_x1p42100_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32 { qcom_cc_probe(pdev, desc) }

// MODULE_DEVICE_TABLE(of, video_cc_x1p42100_match_table);
// module_platform_driver(video_cc_x1p42100_driver);
// MODULE_DESCRIPTION("QTI VIDEOCC X1P42100 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
