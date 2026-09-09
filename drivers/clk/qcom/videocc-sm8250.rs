// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
 */

// Dependencies are supplied by the surrounding kernel clock-controller bindings.

enum Parent {
    PBiTcxo,
    PVideoPll0OutMain,
    PVideoPll1OutMain,
}

static LUCID_VCO: [PllVco; 1] = [PllVco { min: 249600000, max: 2000000000, reserved: 0 }];

static VIDEO_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x25, alpha: 0x8000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329A699C,
    user_ctl_val: 0x00000000, user_ctl_hi_val: 0x00000805,
    user_ctl_hi1_val: 0x00000000,
};

static mut VIDEO_PLL0: ClkAlphaPll = ClkAlphaPll {
    offset: 0x42c, vco_table: &LUCID_VCO, num_vco: 1,
    regs: CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "video_pll0", parent_data: Some(ClkParentData::FwName("bi_tcxo")),
        num_parents: 1, flags: 0, ops: &CLK_ALPHA_PLL_LUCID_OPS,
    } } },
};

static VIDEO_PLL1_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x2B, alpha: 0xC000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329A699C,
    user_ctl_val: 0x00000000, user_ctl_hi_val: 0x00000805,
    user_ctl_hi1_val: 0x00000000,
};

static mut VIDEO_PLL1: ClkAlphaPll = ClkAlphaPll {
    offset: 0x7d0, vco_table: &LUCID_VCO, num_vco: 1,
    regs: CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "video_pll1", parent_data: Some(ClkParentData::FwName("bi_tcxo")),
        num_parents: 1, flags: 0, ops: &CLK_ALPHA_PLL_LUCID_OPS,
    } } },
};

static VIDEO_CC_PARENT_MAP_1: [ParentMap; 2] = [
    ParentMap { parent: Parent::PBiTcxo, val: 0 },
    ParentMap { parent: Parent::PVideoPll0OutMain, val: 1 },
];
static VIDEO_CC_PARENT_MAP_2: [ParentMap; 2] = [
    ParentMap { parent: Parent::PBiTcxo, val: 0 },
    ParentMap { parent: Parent::PVideoPll1OutMain, val: 1 },
];

static VIDEO_CC_PARENT_DATA_1: [ClkParentData; 2] = [
    ClkParentData::FwName("bi_tcxo"), ClkParentData::Hw(unsafe { &VIDEO_PLL0.clkr.hw }),
];
static VIDEO_CC_PARENT_DATA_2: [ClkParentData; 2] = [
    ClkParentData::FwName("bi_tcxo"), ClkParentData::Hw(unsafe { &VIDEO_PLL1.clkr.hw }),
];

static FTBL_VIDEO_CC_MVS0_CLK_SRC: [FreqTbl; 6] = [
    FreqTbl::new(19200000, Parent::PBiTcxo, 1, 0, 0),
    FreqTbl::new(720000000, Parent::PVideoPll0OutMain, 1, 0, 0),
    FreqTbl::new(1014000000, Parent::PVideoPll0OutMain, 1, 0, 0),
    FreqTbl::new(1098000000, Parent::PVideoPll0OutMain, 1, 0, 0),
    FreqTbl::new(1332000000, Parent::PVideoPll0OutMain, 1, 0, 0), FreqTbl::empty(),
];
static FTBL_VIDEO_CC_MVS1_CLK_SRC: [FreqTbl; 5] = [
    FreqTbl::new(19200000, Parent::PBiTcxo, 1, 0, 0),
    FreqTbl::new(840000000, Parent::PVideoPll1OutMain, 1, 0, 0),
    FreqTbl::new(1098000000, Parent::PVideoPll1OutMain, 1, 0, 0),
    FreqTbl::new(1332000000, Parent::PVideoPll1OutMain, 1, 0, 0), FreqTbl::empty(),
];

static mut VIDEO_CC_MVS0_CLK_SRC: ClkRcg2 = ClkRcg2::new(0xb94, 0, 5, &VIDEO_CC_PARENT_MAP_1, &FTBL_VIDEO_CC_MVS0_CLK_SRC, "video_cc_mvs0_clk_src", &VIDEO_CC_PARENT_DATA_1);
static mut VIDEO_CC_MVS1_CLK_SRC: ClkRcg2 = ClkRcg2::new(0xbb4, 0, 5, &VIDEO_CC_PARENT_MAP_2, &FTBL_VIDEO_CC_MVS1_CLK_SRC, "video_cc_mvs1_clk_src", &VIDEO_CC_PARENT_DATA_2);

static mut VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: ClkRegmapDiv = ClkRegmapDiv::new(0xc54, "video_cc_mvs0c_div2_div_clk_src", unsafe { &VIDEO_CC_MVS0_CLK_SRC.clkr.hw });
static mut VIDEO_CC_MVS0_DIV_CLK_SRC: ClkRegmapDiv = ClkRegmapDiv::new(0xd54, "video_cc_mvs0_div_clk_src", unsafe { &VIDEO_CC_MVS0_CLK_SRC.clkr.hw });
static mut VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC: ClkRegmapDiv = ClkRegmapDiv::new(0xcf4, "video_cc_mvs1c_div2_div_clk_src", unsafe { &VIDEO_CC_MVS1_CLK_SRC.clkr.hw });

static mut VIDEO_CC_MVS0C_CLK: ClkBranch = ClkBranch::new(0xc34, BRANCH_HALT, "video_cc_mvs0c_clk", unsafe { &VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC.clkr.hw });
static mut VIDEO_CC_MVS0_CLK: ClkBranch = ClkBranch::new(0xd34, BRANCH_HALT_VOTED, "video_cc_mvs0_clk", unsafe { &VIDEO_CC_MVS0_DIV_CLK_SRC.clkr.hw });
static mut VIDEO_CC_MVS1_DIV2_CLK: ClkBranch = ClkBranch::new(0xdf4, BRANCH_HALT_VOTED, "video_cc_mvs1_div2_clk", unsafe { &VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC.clkr.hw });
static mut VIDEO_CC_MVS1C_CLK: ClkBranch = ClkBranch::new(0xcd4, BRANCH_HALT_VOTED, "video_cc_mvs1c_clk", unsafe { &VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC.clkr.hw });

static mut MVS0C_GDSC: Gdsc = Gdsc::new(0xbf8, "mvs0c_gdsc", 0, PWRSTS_OFF_ON);
static mut MVS1C_GDSC: Gdsc = Gdsc::new(0xc98, "mvs1c_gdsc", 0, PWRSTS_OFF_ON);
static mut MVS0_GDSC: Gdsc = Gdsc::new(0xd18, "mvs0_gdsc", HW_CTRL_TRIGGER, PWRSTS_OFF_ON);
static mut MVS1_GDSC: Gdsc = Gdsc::new(0xd98, "mvs1_gdsc", HW_CTRL_TRIGGER, PWRSTS_OFF_ON);

static mut VIDEO_CC_SM8250_CLOCKS: [*mut ClkRegmap; 11] = [
    unsafe { &mut VIDEO_CC_MVS0_CLK.clkr }, unsafe { &mut VIDEO_CC_MVS0_CLK_SRC.clkr },
    unsafe { &mut VIDEO_CC_MVS0_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS0C_CLK.clkr },
    unsafe { &mut VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_CC_MVS1_CLK_SRC.clkr },
    unsafe { &mut VIDEO_CC_MVS1_DIV2_CLK.clkr }, unsafe { &mut VIDEO_CC_MVS1C_CLK.clkr },
    unsafe { &mut VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC.clkr }, unsafe { &mut VIDEO_PLL0.clkr }, unsafe { &mut VIDEO_PLL1.clkr },
];
static VIDEO_CC_SM8250_RESETS: [QcomResetMap; 7] = [
    QcomResetMap::new(0xe54), QcomResetMap::new(0xd14), QcomResetMap::with(0xc34, 2, 150),
    QcomResetMap::new(0xbf4), QcomResetMap::new(0xd94), QcomResetMap::with(0xcd4, 2, 150), QcomResetMap::new(0xc94),
];
static mut VIDEO_CC_SM8250_GDSCS: [*mut Gdsc; 4] = [unsafe { &mut MVS0C_GDSC }, unsafe { &mut MVS1C_GDSC }, unsafe { &mut MVS0_GDSC }, unsafe { &mut MVS1_GDSC }];

static VIDEO_CC_SM8250_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xf4c, fast_io: true };
static VIDEO_CC_SM8250_DESC: QcomCcDesc = QcomCcDesc { config: &VIDEO_CC_SM8250_REGMAP_CONFIG, clks: &VIDEO_CC_SM8250_CLOCKS, num_clks: 11, resets: &VIDEO_CC_SM8250_RESETS, num_resets: 7, gdscs: &VIDEO_CC_SM8250_GDSCS, num_gdscs: 4 };

static VIDEO_CC_SM8250_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId::compatible("qcom,sm8250-videocc"), OfDeviceId::empty()];

unsafe extern "C" fn video_cc_sm8250_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut ret = devm_pm_runtime_enable((*pdev).dev());
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get((*pdev).dev());
    if ret != 0 { return ret; }
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SM8250_DESC);
    if regmap.is_err() { pm_runtime_put((*pdev).dev()); return regmap.ptr_error(); }
    clk_lucid_pll_configure(&mut VIDEO_PLL0, regmap, &VIDEO_PLL0_CONFIG);
    clk_lucid_pll_configure(&mut VIDEO_PLL1, regmap, &VIDEO_PLL1_CONFIG);
    qcom_branch_set_clk_en(regmap, 0xe58); // VIDEO_CC_AHB_CLK
    qcom_branch_set_clk_en(regmap, 0xeec); // VIDEO_CC_XO_CLK
    ret = qcom_cc_really_probe((*pdev).dev(), &VIDEO_CC_SM8250_DESC, regmap);
    pm_runtime_put((*pdev).dev());
    ret
}

static VIDEO_CC_SM8250_DRIVER: PlatformDriver = PlatformDriver { probe: video_cc_sm8250_probe, name: "sm8250-videocc", of_match_table: &VIDEO_CC_SM8250_MATCH_TABLE };
module_platform_driver!(VIDEO_CC_SM8250_DRIVER);
module_license!("GPL v2");
module_description!("QTI VIDEOCC SM8250 Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
