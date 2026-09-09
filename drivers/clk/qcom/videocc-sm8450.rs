// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// External Linux/kernel declarations and constants are supplied by the surrounding
// translated tree; this file preserves the original implementation topology.

#[repr(C)]
pub struct PllVco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)]
pub struct AlphaPllConfig {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub test_ctl_val: u32, pub test_ctl_hi_val: u32,
    pub test_ctl_hi1_val: u32, pub test_ctl_hi2_val: u32,
    pub user_ctl_val: u32, pub user_ctl_hi_val: u32,
}

pub const DT_BI_TCXO: usize = 0;
pub const P_BI_TCXO: usize = 0;
pub const P_VIDEO_CC_PLL0_OUT_MAIN: usize = 1;
pub const P_VIDEO_CC_PLL1_OUT_MAIN: usize = 2;

static LUCID_EVO_VCO: [PllVco; 1] = [PllVco { min_freq: 249600000, max_freq: 2020000000, val: 0 }];

static VIDEO_CC_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x0044001e, alpha: 0, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0, test_ctl_hi2_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805,
};
static SM8475_VIDEO_CC_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x1e, alpha: 0, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x00009000, test_ctl_hi2_val: 0x34,
    user_ctl_val: 0, user_ctl_hi_val: 5,
};

static VIDEO_CC_PLL1_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x0044002b, alpha: 0xc000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0, test_ctl_hi2_val: 0,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805,
};
static SM8475_VIDEO_CC_PLL1_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x2b, alpha: 0xc000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x00009000, test_ctl_hi2_val: 0x34,
    user_ctl_val: 0, user_ctl_hi_val: 5,
};

static VIDEO_CC_PARENT_MAP_0: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL1_OUT_MAIN, cfg: 1 }];
static VIDEO_CC_PARENT_DATA_0: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &VIDEO_CC_PLL0.clkr.hw } }];
static VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &VIDEO_CC_PLL1.clkr.hw } }];

// The following objects retain the C driver's exact aggregate layout through
// the corresponding translated kernel clock, reset, GDSC, and regmap types.
static mut VIDEO_CC_PLL0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &VIDEO_CC_PLL0_CONFIG, vco_table: &LUCID_EVO_VCO,
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_EVO],
    clkr: clk_regmap { hw: clk_hw_init("video_cc_pll0", DT_BI_TCXO, &clk_alpha_pll_lucid_evo_ops), ..Default::default() },
};
static mut VIDEO_CC_PLL1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, config: &VIDEO_CC_PLL1_CONFIG, vco_table: &LUCID_EVO_VCO,
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_EVO],
    clkr: clk_regmap { hw: clk_hw_init("video_cc_pll1", DT_BI_TCXO, &clk_alpha_pll_lucid_evo_ops), ..Default::default() },
};

static VIDEO_CC_MVS0_FREQ: [freq_tbl; 6] = [
    F(576000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(720000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1014000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1098000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1332000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];
static VIDEO_CC_MVS1_FREQ: [freq_tbl; 6] = [
    F(840000000, P_VIDEO_CC_PLL1_OUT_MAIN, 1, 0, 0), F(1050000000, P_VIDEO_CC_PLL1_OUT_MAIN, 1, 0, 0),
    F(1350000000, P_VIDEO_CC_PLL1_OUT_MAIN, 1, 0, 0), F(1500000000, P_VIDEO_CC_PLL1_OUT_MAIN, 1, 0, 0),
    F(1650000000, P_VIDEO_CC_PLL1_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];

static VIDEO_CC_SM8450_RESETS: [qcom_reset_map; 7] = [
    qcom_reset_map { reg: 0x80e0, ..Default::default() }, qcom_reset_map { reg: 0x8098, ..Default::default() },
    qcom_reset_map { reg: 0x8048, ..Default::default() }, qcom_reset_map { reg: 0x80bc, ..Default::default() },
    qcom_reset_map { reg: 0x8070, ..Default::default() },
    qcom_reset_map { reg: 0x8064, bit: 2, udelay: 1000 }, qcom_reset_map { reg: 0x808c, bit: 2, udelay: 1000 },
];
static VIDEO_CC_SM8450_CRITICAL_CBCRS: [u32; 3] = [0x80e4, 0x8114, 0x8130];

static mut VIDEO_CC_MVS0_CLK_SRC: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x8000, mnd_width: 0, hid_width: 5, parent_map: VIDEO_CC_PARENT_MAP_0, freq_tbl: VIDEO_CC_MVS0_FREQ, ..Default::default() };
static mut VIDEO_CC_MVS1_CLK_SRC: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x8018, mnd_width: 0, hid_width: 5, parent_map: VIDEO_CC_PARENT_MAP_1, freq_tbl: VIDEO_CC_MVS1_FREQ, ..Default::default() };
static mut VIDEO_CC_MVS0_DIV_CLK_SRC: clk_regmap_div = clk_regmap_div { reg: 0x80b8, shift: 0, width: 4, ..Default::default() };
static mut VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: clk_regmap_div = clk_regmap_div { reg: 0x806c, shift: 0, width: 4, ..Default::default() };
static mut VIDEO_CC_MVS1_DIV_CLK_SRC: clk_regmap_div = clk_regmap_div { reg: 0x80dc, shift: 0, width: 4, ..Default::default() };
static mut VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC: clk_regmap_div = clk_regmap_div { reg: 0x8094, shift: 0, width: 4, ..Default::default() };
static mut VIDEO_CC_MVS0_CLK: clk_branch = clk_branch { halt_reg: 0x80b0, halt_check: BRANCH_HALT_SKIP, hwcg_reg: 0x80b0, hwcg_bit: 1, ..Default::default() };
static mut VIDEO_CC_MVS0C_CLK: clk_branch = clk_branch { halt_reg: 0x8064, halt_check: BRANCH_HALT, ..Default::default() };
static mut VIDEO_CC_MVS1_CLK: clk_branch = clk_branch { halt_reg: 0x80d4, halt_check: BRANCH_HALT_SKIP, hwcg_reg: 0x80d4, hwcg_bit: 1, ..Default::default() };
static mut VIDEO_CC_MVS1C_CLK: clk_branch = clk_branch { halt_reg: 0x808c, halt_check: BRANCH_HALT, ..Default::default() };
static mut VIDEO_CC_MVS0C_GDSC: gdsc = gdsc { gdscr: 0x804c, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 6, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE, ..Default::default() };
static mut VIDEO_CC_MVS0_GDSC: gdsc = gdsc { gdscr: 0x809c, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 6, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER | RETAIN_FF_ENABLE, ..Default::default() };
static mut VIDEO_CC_MVS1C_GDSC: gdsc = gdsc { gdscr: 0x8074, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 6, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE, ..Default::default() };
static mut VIDEO_CC_MVS1_GDSC: gdsc = gdsc { gdscr: 0x80c0, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 6, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER | RETAIN_FF_ENABLE, ..Default::default() };

static VIDEO_CC_SM8450_CLOCKS: [*mut clk_regmap; 12] = [
    unsafe { &mut VIDEO_CC_MVS0_CLK.clkr }, unsafe { &mut VIDEO_CC_MVS0_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS0_DIV_CLK_SRC.clkr },
    unsafe { &mut VIDEO_CC_MVS0C_CLK.clkr }, unsafe { &mut VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS1_CLK.clkr },
    unsafe { &mut VIDEO_CC_MVS1_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS1_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS1C_CLK.clkr },
    unsafe { &mut VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_PLL0.clkr }, unsafe { &mut VIDEO_CC_PLL1.clkr },
];
static VIDEO_CC_SM8450_GDSCS: [*mut gdsc; 4] = [unsafe { &mut VIDEO_CC_MVS0C_GDSC }, unsafe { &mut VIDEO_CC_MVS0_GDSC }, unsafe { &mut VIDEO_CC_MVS1C_GDSC }, unsafe { &mut VIDEO_CC_MVS1_GDSC }];
static VIDEO_CC_SM8450_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f4c, fast_io: true, ..Default::default() };
static VIDEO_CC_SM8450_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: VIDEO_CC_SM8450_PLLS.as_ptr(), num_alpha_plls: VIDEO_CC_SM8450_PLLS.len(), clk_cbcrs: VIDEO_CC_SM8450_CRITICAL_CBCRS.as_ptr(), num_clk_cbcrs: VIDEO_CC_SM8450_CRITICAL_CBCRS.len() };
static VIDEO_CC_SM8450_PLLS: [*mut clk_alpha_pll; 2] = [unsafe { &mut VIDEO_CC_PLL0 }, unsafe { &mut VIDEO_CC_PLL1 }];

unsafe fn video_cc_sm8450_probe(pdev: *mut platform_device) -> i32 {
    if of_device_is_compatible((*pdev).dev.of_node, "qcom,sm8475-videocc") {
        VIDEO_CC_PLL0.regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE];
        VIDEO_CC_PLL1.regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE];
        VIDEO_CC_PLL0.config = &SM8475_VIDEO_CC_PLL0_CONFIG;
        VIDEO_CC_PLL1.config = &SM8475_VIDEO_CC_PLL1_CONFIG;
    }
    qcom_cc_probe(pdev, &VIDEO_CC_SM8450_DESC)
}

static VIDEO_CC_SM8450_DESC: qcom_cc_desc = qcom_cc_desc {
    config: &VIDEO_CC_SM8450_REGMAP_CONFIG, clks: VIDEO_CC_SM8450_CLOCKS.as_ptr(), num_clks: VIDEO_CC_SM8450_CLOCKS.len(),
    resets: VIDEO_CC_SM8450_RESETS.as_ptr(), num_resets: VIDEO_CC_SM8450_RESETS.len(),
    gdscs: VIDEO_CC_SM8450_GDSCS.as_ptr(), num_gdscs: VIDEO_CC_SM8450_GDSCS.len(), use_rpm: true,
    driver_data: &VIDEO_CC_SM8450_DRIVER_DATA,
};

static VIDEO_CC_SM8450_DRIVER: platform_driver = platform_driver { probe: Some(video_cc_sm8450_probe), name: "video_cc-sm8450", of_match_table: &["qcom,sm8450-videocc", "qcom,sm8475-videocc"] };
// module_platform_driver(video_cc_sm8450_driver);
// MODULE_DEVICE_TABLE(of, video_cc_sm8450_match_table);
// MODULE_DESCRIPTION("QTI VIDEOCC SM8450 / SM8475 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
