// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Translated from the Linux kernel C implementation. External kernel types,
// constants, macros, and functions are supplied by the surrounding tree.

enum DtBi { Tcx0, Tcx0Ao, SleepClk }
enum Parent { BiTcx0, BiTcx0Ao, SleepClk, VideoPll0OutMain, VideoPll1OutMain }

static LUCID_5LPE_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 1750000000, val: 0 }];
static LUCID_5LPE_VCO_8280XP: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 1800000000, val: 0 }];

static VIDEO_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x25, alpha: 0x8000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x2a9a699c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0x01800000,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0,
};

static mut video_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x42c, vco_table: &LUCID_5LPE_VCO, num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_pll0", parent_data: &[clk_parent_data { index: DT_BI_TCXO }],
        num_parents: 1, ops: &clk_alpha_pll_lucid_5lpe_ops,
    } } },
};

static VIDEO_PLL1_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x2b, alpha: 0xc000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x2a9a699c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0x01800000,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0,
};

static mut video_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x7d0, vco_table: &LUCID_5LPE_VCO, num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_pll1", parent_data: &[clk_parent_data { index: DT_BI_TCXO }],
        num_parents: 1, ops: &clk_alpha_pll_lucid_5lpe_ops,
    } } },
};

static VIDEO_CC_PARENT_MAP_0: [parent_map; 1] = [parent_map { parent: P_BI_TCXO_AO, val: 0 }];
static VIDEO_CC_PARENT_DATA_0: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO_AO }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_VIDEO_PLL0_OUT_MAIN, val: 1 }];
static VIDEO_CC_PARENT_MAP_2: [parent_map; 2] = [parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_VIDEO_PLL1_OUT_MAIN, val: 1 }];

static VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [
    clk_parent_data { index: DT_BI_TCXO },
    clk_parent_data { hw: unsafe { &video_pll0.clkr.hw } },
];
static VIDEO_CC_PARENT_DATA_2: [clk_parent_data; 2] = [
    clk_parent_data { index: DT_BI_TCXO },
    clk_parent_data { hw: unsafe { &video_pll1.clkr.hw } },
];

static FTBL_VIDEO_CC_AHB_CLK_SRC: [freq_tbl; 2] = [F(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl::EMPTY];
static FTBL_VIDEO_CC_MVS0_CLK_SRC: [freq_tbl; 5] = [F(720000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1014000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1098000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1332000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),freq_tbl::EMPTY];
static FTBL_VIDEO_CC_MVS0_CLK_SRC_8280XP: [freq_tbl; 7] = [F(720000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1014000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1098000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1332000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1599000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),F(1680000000,P_VIDEO_PLL0_OUT_MAIN,1,0,0),freq_tbl::EMPTY];
static FTBL_VIDEO_CC_MVS1_CLK_SRC: [freq_tbl; 4] = [F(840000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1098000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1332000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),freq_tbl::EMPTY];
static FTBL_VIDEO_CC_MVS1_CLK_SRC_8280XP: [freq_tbl; 6] = [F(840000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1098000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1332000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1600000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),F(1800000000,P_VIDEO_PLL1_OUT_MAIN,1,0,0),freq_tbl::EMPTY];
static FTBL_VIDEO_CC_SLEEP_CLK_SRC: [freq_tbl; 2] = [F(32000,P_SLEEP_CLK,1,0,0),freq_tbl::EMPTY];

// The following declarations retain the C driver's register layout and initialization.
static mut video_cc_ahb_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0xbd4, mnd_width: 0, hid_width: 5, parent_map: &VIDEO_CC_PARENT_MAP_0, freq_tbl: &FTBL_VIDEO_CC_AHB_CLK_SRC, ..clk_rcg2::DEFAULT };
static mut video_cc_mvs0_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0xb94, mnd_width: 0, hid_width: 5, parent_map: &VIDEO_CC_PARENT_MAP_1, freq_tbl: &FTBL_VIDEO_CC_MVS0_CLK_SRC, ..clk_rcg2::DEFAULT };
static mut video_cc_mvs1_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0xbb4, mnd_width: 0, hid_width: 5, parent_map: &VIDEO_CC_PARENT_MAP_2, freq_tbl: &FTBL_VIDEO_CC_MVS1_CLK_SRC, ..clk_rcg2::DEFAULT };
static mut video_cc_sleep_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0xef0, mnd_width: 0, hid_width: 5, freq_tbl: &FTBL_VIDEO_CC_SLEEP_CLK_SRC, ..clk_rcg2::DEFAULT };
static mut video_cc_xo_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0xecc, mnd_width: 0, hid_width: 5, parent_map: &VIDEO_CC_PARENT_MAP_0, freq_tbl: &FTBL_VIDEO_CC_AHB_CLK_SRC, ..clk_rcg2::DEFAULT };

static mut video_cc_mvs0_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0xd54, shift: 0, width: 4, ..clk_regmap_div::DEFAULT };
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0xc54, shift: 0, width: 4, ..clk_regmap_div::DEFAULT };
static mut video_cc_mvs1_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0xdd4, shift: 0, width: 4, ..clk_regmap_div::DEFAULT };
static mut video_cc_mvs1c_div2_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0xcf4, shift: 0, width: 4, ..clk_regmap_div::DEFAULT };

static mut video_cc_mvs0_clk: clk_branch = clk_branch { halt_reg: 0xd34, halt_check: BRANCH_HALT_VOTED, hwcg_reg: 0xd34, hwcg_bit: 1, ..clk_branch::DEFAULT };
static mut video_cc_mvs0c_clk: clk_branch = clk_branch { halt_reg: 0xc34, halt_check: BRANCH_HALT, ..clk_branch::DEFAULT };
static mut video_cc_mvs1_clk: clk_branch = clk_branch { halt_reg: 0xdb4, halt_check: BRANCH_HALT_VOTED, hwcg_reg: 0xdb4, hwcg_bit: 1, ..clk_branch::DEFAULT };
static mut video_cc_mvs1_div2_clk: clk_branch = clk_branch { halt_reg: 0xdf4, halt_check: BRANCH_HALT_VOTED, hwcg_reg: 0xdf4, hwcg_bit: 1, ..clk_branch::DEFAULT };
static mut video_cc_mvs1c_clk: clk_branch = clk_branch { halt_reg: 0xcd4, halt_check: BRANCH_HALT, ..clk_branch::DEFAULT };
static mut video_cc_sleep_clk: clk_branch = clk_branch { halt_reg: 0xf10, halt_check: BRANCH_HALT, ..clk_branch::DEFAULT };

static mut mvs0c_gdsc: gdsc = gdsc { gdscr: 0xbf8, pd: generic_pm_domain { name: "mvs0c_gdsc" }, flags: RETAIN_FF_ENABLE, pwrsts: PWRSTS_OFF_ON, ..gdsc::DEFAULT };
static mut mvs1c_gdsc: gdsc = gdsc { gdscr: 0xc98, pd: generic_pm_domain { name: "mvs1c_gdsc" }, flags: RETAIN_FF_ENABLE, pwrsts: PWRSTS_OFF_ON, ..gdsc::DEFAULT };
static mut mvs0_gdsc: gdsc = gdsc { gdscr: 0xd18, pd: generic_pm_domain { name: "mvs0_gdsc" }, flags: HW_CTRL_TRIGGER | RETAIN_FF_ENABLE, pwrsts: PWRSTS_OFF_ON, ..gdsc::DEFAULT };
static mut mvs1_gdsc: gdsc = gdsc { gdscr: 0xd98, pd: generic_pm_domain { name: "mvs1_gdsc" }, flags: HW_CTRL_TRIGGER | RETAIN_FF_ENABLE, pwrsts: PWRSTS_OFF_ON, ..gdsc::DEFAULT };

static mut video_cc_sm8350_clocks: [*mut clk_regmap; VIDEO_CC_XO_CLK_SRC + 1] = [core::ptr::null_mut(); VIDEO_CC_XO_CLK_SRC + 1];
static video_cc_sm8350_resets: [qcom_reset_map; 7] = [
    qcom_reset_map { reg: 0xe54, ..qcom_reset_map::DEFAULT }, qcom_reset_map { reg: 0xd14, ..qcom_reset_map::DEFAULT },
    qcom_reset_map { reg: 0xc34, bit: 2, udelay: 400, ..qcom_reset_map::DEFAULT }, qcom_reset_map { reg: 0xbf4, ..qcom_reset_map::DEFAULT },
    qcom_reset_map { reg: 0xd94, ..qcom_reset_map::DEFAULT }, qcom_reset_map { reg: 0xcd4, bit: 2, udelay: 400, ..qcom_reset_map::DEFAULT },
    qcom_reset_map { reg: 0xc94, ..qcom_reset_map::DEFAULT },
];
static mut video_cc_sm8350_gdscs: [*mut gdsc; 4] = [&raw mut mvs0c_gdsc, &raw mut mvs1c_gdsc, &raw mut mvs0_gdsc, &raw mut mvs1_gdsc];
static video_cc_sm8350_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true, ..regmap_config::DEFAULT };
static video_cc_sm8350_desc: qcom_cc_desc = qcom_cc_desc { config: &video_cc_sm8350_regmap_config, clks: unsafe { &video_cc_sm8350_clocks }, num_clks: VIDEO_CC_XO_CLK_SRC + 1, resets: &video_cc_sm8350_resets, num_resets: 7, gdscs: unsafe { &video_cc_sm8350_gdscs }, num_gdscs: 4 };

static video_cc_sm8350_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,sc8280xp-videocc" },
    of_device_id { compatible: "qcom,sm8350-videocc" }, of_device_id::EMPTY,
];
static mut video_cc_sm8350_driver: platform_driver = platform_driver { probe: Some(video_cc_sm8350_probe_rs), name: "sm8350-videocc", of_match_table: &video_cc_sm8350_match_table };

#[no_mangle]
pub unsafe extern "C" fn video_cc_sm8350_probe_rs(pdev: *mut platform_device) -> i32 {
    let mut video_cc_xo_clk_cbcr: u32 = 0xeec;
    let ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    let ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 { return ret; }
    if of_device_is_compatible((*pdev).dev.of_node, c"qcom,sc8280xp-videocc".as_ptr()) {
        video_cc_sleep_clk_src.cmd_rcgr = 0xf38;
        video_cc_xo_clk_src.cmd_rcgr = 0xf14;
        video_cc_xo_clk_cbcr = 0xf34;
        video_pll0.vco_table = &LUCID_5LPE_VCO_8280XP;
        video_pll1.vco_table = &LUCID_5LPE_VCO_8280XP;
        video_cc_mvs0_clk_src.freq_tbl = &FTBL_VIDEO_CC_MVS0_CLK_SRC_8280XP;
        video_cc_mvs1_clk_src.freq_tbl = &FTBL_VIDEO_CC_MVS1_CLK_SRC_8280XP;
    }
    let regmap = qcom_cc_map(pdev, &video_cc_sm8350_desc);
    if IS_ERR(regmap) { pm_runtime_put(&mut (*pdev).dev); return PTR_ERR(regmap); }
    clk_lucid_pll_configure(&mut video_pll0, regmap, &VIDEO_PLL0_CONFIG);
    clk_lucid_pll_configure(&mut video_pll1, regmap, &VIDEO_PLL1_CONFIG);
    qcom_branch_set_clk_en(regmap, 0xe58);
    qcom_branch_set_clk_en(regmap, video_cc_xo_clk_cbcr);
    let ret = qcom_cc_really_probe(&mut (*pdev).dev, &video_cc_sm8350_desc, regmap);
    pm_runtime_put(&mut (*pdev).dev);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
