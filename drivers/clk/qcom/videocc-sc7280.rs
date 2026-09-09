// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// External Linux kernel, Qualcomm clock, reset, GDSC, and device-tree bindings
// are supplied by the surrounding translation environment.

#[repr(usize)]
enum Parent {
    PBiTcxo,
    PSleepClk,
    PVideoPll0OutEven,
}

static LUCID_VCO: [PllVco; 1] = [PllVco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

/* 400MHz Configuration */
static VIDEO_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x14,
    alpha: 0xD555,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261,
    config_ctl_hi1_val: 0x329A299C,
    user_ctl_val: 0x00000001,
    user_ctl_hi_val: 0x00000805,
    user_ctl_hi1_val: 0x00000000,
};

static mut VIDEO_PLL0: ClkAlphaPll = ClkAlphaPll {
    offset: 0x0,
    vco_table: LUCID_VCO.as_ptr(),
    num_vco: LUCID_VCO.len(),
    regs: unsafe { CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_LUCID] },
    clkr: ClkRegmap {
        hw: ClkHw { init: &ClkInitData {
            name: "video_pll0",
            parent_data: &[ClkParentData { fw_name: Some("bi_tcxo"), hw: None }],
            num_parents: 1,
            ops: &CLK_ALPHA_PLL_LUCID_OPS,
            ..ClkInitData::default()
        }, ..ClkHw::default() },
        ..ClkRegmap::default()
    },
};

static VIDEO_CC_PARENT_MAP_0: [ParentMap; 2] = [
    ParentMap { src: Parent::PBiTcxo as i32, cfg: 0 },
    ParentMap { src: Parent::PVideoPll0OutEven as i32, cfg: 3 },
];
static VIDEO_CC_PARENT_DATA_0: [ClkParentData; 2] = [
    ClkParentData { fw_name: Some("bi_tcxo"), hw: None },
    ClkParentData { fw_name: None, hw: Some(unsafe { &VIDEO_PLL0.clkr.hw }) },
];
static VIDEO_CC_PARENT_MAP_1: [ParentMap; 1] = [ParentMap { src: Parent::PSleepClk as i32, cfg: 0 }];
static VIDEO_CC_PARENT_DATA_1: [ClkParentData; 1] = [ClkParentData { fw_name: Some("sleep_clk"), hw: None }];

static FTBL_VIDEO_CC_IRIS_CLK_SRC: [FreqTbl; 6] = [
    FreqTbl { freq: 133333333, src: Parent::PVideoPll0OutEven as i32, pre_div: 3, m: 0, n: 0 },
    FreqTbl { freq: 240000000, src: Parent::PVideoPll0OutEven as i32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 335000000, src: Parent::PVideoPll0OutEven as i32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 424000000, src: Parent::PVideoPll0OutEven as i32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 460000000, src: Parent::PVideoPll0OutEven as i32, pre_div: 2, m: 0, n: 0 },
    FreqTbl::default(),
];
static mut VIDEO_CC_IRIS_CLK_SRC: ClkRcg2 = rcg2!(0x1000, 0, 5, &VIDEO_CC_PARENT_MAP_0, &FTBL_VIDEO_CC_IRIS_CLK_SRC,
    "video_cc_iris_clk_src", &VIDEO_CC_PARENT_DATA_0, CLK_SET_RATE_PARENT, &CLK_RCG2_SHARED_OPS);

static FTBL_VIDEO_CC_SLEEP_CLK_SRC: [FreqTbl; 2] = [
    FreqTbl { freq: 32000, src: Parent::PSleepClk as i32, pre_div: 1, m: 0, n: 0 },
    FreqTbl::default(),
];
static mut VIDEO_CC_SLEEP_CLK_SRC: ClkRcg2 = rcg2!(0x701c, 0, 5, &VIDEO_CC_PARENT_MAP_1, &FTBL_VIDEO_CC_SLEEP_CLK_SRC,
    "video_cc_sleep_clk_src", &VIDEO_CC_PARENT_DATA_1, 0, &CLK_RCG2_OPS);

static mut VIDEO_CC_IRIS_AHB_CLK: ClkBranch = branch!(0x5004, BRANCH_HALT_VOTED, 0x5004, BIT(0),
    "video_cc_iris_ahb_clk", &[unsafe { &VIDEO_CC_IRIS_CLK_SRC.clkr.hw }], CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_MVS0_AXI_CLK: ClkBranch = branch!(0x800c, BRANCH_HALT, 0x800c, BIT(0), "video_cc_mvs0_axi_clk", &[], 0, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_MVS0_CORE_CLK: ClkBranch = branch_hwcg!(0x3010, BRANCH_HALT_VOTED, 0x3010, 1, 0x3010, BIT(0),
    "video_cc_mvs0_core_clk", &[unsafe { &VIDEO_CC_IRIS_CLK_SRC.clkr.hw }], CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_MVSC_CORE_CLK: ClkBranch = branch!(0x2014, BRANCH_HALT, 0x2014, BIT(0), "video_cc_mvsc_core_clk",
    &[unsafe { &VIDEO_CC_IRIS_CLK_SRC.clkr.hw }], CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_MVSC_CTL_AXI_CLK: ClkBranch = branch!(0x8004, BRANCH_HALT, 0x8004, BIT(0), "video_cc_mvsc_ctl_axi_clk", &[], 0, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_SLEEP_CLK: ClkBranch = branch!(0x7034, BRANCH_HALT, 0x7034, BIT(0), "video_cc_sleep_clk",
    &[unsafe { &VIDEO_CC_SLEEP_CLK_SRC.clkr.hw }], CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEO_CC_VENUS_AHB_CLK: ClkBranch = branch!(0x801c, BRANCH_HALT, 0x801c, BIT(0), "video_cc_venus_ahb_clk", &[], 0, &CLK_BRANCH2_OPS);

static mut MVS0_GDSC: Gdsc = Gdsc { gdscr: 0x3004, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6,
    pd: PowerDomain { name: "mvs0_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER | RETAIN_FF_ENABLE };
static mut MVSC_GDSC: Gdsc = Gdsc { gdscr: 0x2004, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6,
    pd: PowerDomain { name: "mvsc_gdsc" }, flags: RETAIN_FF_ENABLE, pwrsts: PWRSTS_OFF_ON };

static mut VIDEO_CC_SC7280_CLOCKS: [*mut ClkRegmap; 10] = [
    &mut VIDEO_CC_IRIS_AHB_CLK.clkr, &mut VIDEO_CC_IRIS_CLK_SRC.clkr, &mut VIDEO_CC_MVS0_AXI_CLK.clkr,
    &mut VIDEO_CC_MVS0_CORE_CLK.clkr, &mut VIDEO_CC_MVSC_CORE_CLK.clkr, &mut VIDEO_CC_MVSC_CTL_AXI_CLK.clkr,
    &mut VIDEO_CC_SLEEP_CLK.clkr, &mut VIDEO_CC_SLEEP_CLK_SRC.clkr, &mut VIDEO_CC_VENUS_AHB_CLK.clkr, &mut VIDEO_PLL0.clkr,
];
static mut VIDEO_CC_SC7280_GDSCS: [*mut Gdsc; 2] = [&mut MVS0_GDSC, &mut MVSC_GDSC];
static VIDEO_CC_SC7280_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb000, fast_io: true };
static VIDEO_CC_SC7280_DESC: QcomCcDesc = QcomCcDesc { config: &VIDEO_CC_SC7280_REGMAP_CONFIG, clks: unsafe { &VIDEO_CC_SC7280_CLOCKS },
    num_clks: 10, gdscs: unsafe { &VIDEO_CC_SC7280_GDSCS }, num_gdscs: 2 };

static VIDEO_CC_SC7280_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId { compatible: Some("qcom,sc7280-videocc") }, OfDeviceId::default()];

unsafe extern "C" fn video_cc_sc7280_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SC7280_DESC);
    if is_err(regmap) { return ptr_err(regmap); }
    clk_lucid_pll_configure(&mut VIDEO_PLL0, regmap, &VIDEO_PLL0_CONFIG);
    qcom_cc_really_probe(&mut (*pdev).dev, &VIDEO_CC_SC7280_DESC, regmap)
}

static mut VIDEO_CC_SC7280_DRIVER: PlatformDriver = PlatformDriver { probe: Some(video_cc_sc7280_probe), driver: Driver {
    name: "video_cc-sc7280", of_match_table: &VIDEO_CC_SC7280_MATCH_TABLE, ..Driver::default() }, ..PlatformDriver::default() };

module_platform_driver!(VIDEO_CC_SC7280_DRIVER);
module_description!("QTI VIDEO_CC sc7280 Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
