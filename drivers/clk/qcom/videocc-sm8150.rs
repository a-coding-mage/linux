// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017-2020, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel clock-controller sources.

#[repr(C)]
enum ParentId {
    PBiTcxo,
    PVideoPll0OutMain,
}

static TRION_VCO: [PllVco; 1] = [PllVco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static mut VIDEO_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x14,
    alpha: 0xD555,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002267,
    config_ctl_hi1_val: 0x00000024,
    test_ctl_hi1_val: 0x00000020,
    user_ctl_val: 0x00000000,
    user_ctl_hi_val: 0x00000805,
    user_ctl_hi1_val: 0x000000D0,
};

static mut VIDEO_PLL0: ClkAlphaPll = ClkAlphaPll {
    offset: 0x42c,
    vco_table: TRION_VCO.as_ptr(),
    num_vco: TRION_VCO.len(),
    regs: CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_TRION],
    clkr: ClkRegmap {
        hw: ClkHw {
            init: &ClkInitData {
                name: "video_pll0",
                parent_data: &ClkParentData { fw_name: Some("bi_tcxo"), hw: None },
                num_parents: 1,
                ops: &CLK_ALPHA_PLL_TRION_OPS,
                ..ClkHw::default()
            },
            ..ClkHw::default()
        },
        ..ClkRegmap::default()
    },
};

static VIDEO_CC_PARENT_MAP_0: [ParentMap; 2] = [
    ParentMap { src: ParentId::PBiTcxo as u32, cfg: 0 },
    ParentMap { src: ParentId::PVideoPll0OutMain as u32, cfg: 1 },
];

static VIDEO_CC_PARENT_DATA_0: [ClkParentData; 2] = [
    ClkParentData { fw_name: Some("bi_tcxo"), hw: None },
    ClkParentData { fw_name: None, hw: Some(unsafe { &VIDEO_PLL0.clkr.hw }) },
];

static FTBL_VIDEO_CC_IRIS_CLK_SRC: [FreqTbl; 8] = [
    FreqTbl { freq: 19200000, src: ParentId::PBiTcxo as u32, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 200000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 240000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 338000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 365000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 444000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 533000000, src: ParentId::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl::default(),
];

static mut VIDEO_CC_IRIS_CLK_SRC: ClkRcg2 = ClkRcg2 {
    cmd_rcgr: 0x7f0,
    mnd_width: 0,
    hid_width: 5,
    parent_map: VIDEO_CC_PARENT_MAP_0.as_ptr(),
    freq_tbl: FTBL_VIDEO_CC_IRIS_CLK_SRC.as_ptr(),
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "video_cc_iris_clk_src", parent_data: VIDEO_CC_PARENT_DATA_0.as_ptr(),
        num_parents: VIDEO_CC_PARENT_DATA_0.len(), flags: CLK_SET_RATE_PARENT,
        ops: &CLK_RCG2_SHARED_OPS, ..ClkHw::default()
    }, ..ClkHw::default() }, ..ClkRegmap::default() },
};

macro_rules! branch {
    ($name:literal, $reg:expr, $halt:expr) => {
        ClkBranch { halt_reg: $reg, halt_check: $halt, clkr: ClkRegmap { enable_reg: $reg, enable_mask: BIT(0), hw: ClkHw { init: &ClkInitData { name: $name, parent_hws: unsafe { &VIDEO_CC_IRIS_CLK_SRC.clkr.hw }, num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &CLK_BRANCH2_OPS, ..ClkHw::default() }, ..ClkHw::default() }, ..ClkRegmap::default() } }
    };
}

static VIDEO_CC_IRIS_AHB_CLK: ClkBranch = branch!("video_cc_iris_ahb_clk", 0x8f4, BRANCH_VOTED);
static VIDEO_CC_MVS0_CORE_CLK: ClkBranch = branch!("video_cc_mvs0_core_clk", 0x890, BRANCH_VOTED);
static VIDEO_CC_MVS1_CORE_CLK: ClkBranch = branch!("video_cc_mvs1_core_clk", 0x8d0, BRANCH_VOTED);
static VIDEO_CC_MVSC_CORE_CLK: ClkBranch = branch!("video_cc_mvsc_core_clk", 0x850, BRANCH_HALT);

static VENUS_GDSC: Gdsc = Gdsc { gdscr: 0x814, pd: PowerDomain { name: "venus_gdsc" }, flags: 0, pwrsts: PWRSTS_OFF_ON };
static VCODEC0_GDSC: Gdsc = Gdsc { gdscr: 0x874, pd: PowerDomain { name: "vcodec0_gdsc" }, flags: HW_CTRL_TRIGGER, pwrsts: PWRSTS_OFF_ON };
static VCODEC1_GDSC: Gdsc = Gdsc { gdscr: 0x8b4, pd: PowerDomain { name: "vcodec1_gdsc" }, flags: HW_CTRL_TRIGGER, pwrsts: PWRSTS_OFF_ON };

static mut VIDEO_CC_SM8150_CLOCKS: [*mut ClkRegmap; 6] = [
    &VIDEO_CC_IRIS_AHB_CLK.clkr, &VIDEO_CC_IRIS_CLK_SRC.clkr, &VIDEO_CC_MVS0_CORE_CLK.clkr,
    &VIDEO_CC_MVS1_CORE_CLK.clkr, &VIDEO_CC_MVSC_CORE_CLK.clkr, unsafe { &mut VIDEO_PLL0.clkr },
];
static mut VIDEO_CC_SM8150_GDSCS: [*mut Gdsc; 3] = [&VENUS_GDSC as *const _ as *mut _, &VCODEC0_GDSC as *const _ as *mut _, &VCODEC1_GDSC as *const _ as *mut _];

static VIDEO_CC_SM8150_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb94, fast_io: true };
static VIDEO_CC_SM8150_RESETS: [QcomResetMap; 5] = [
    QcomResetMap { reg: 0x850, bit: 2, udelay: 150 },
    QcomResetMap { reg: 0x8f0, ..QcomResetMap::default() }, QcomResetMap { reg: 0x870, ..QcomResetMap::default() },
    QcomResetMap { reg: 0x8b0, ..QcomResetMap::default() }, QcomResetMap { reg: 0x810, ..QcomResetMap::default() },
];
static VIDEO_CC_SM8150_DESC: QcomCcDesc = QcomCcDesc { config: &VIDEO_CC_SM8150_REGMAP_CONFIG, clks: unsafe { VIDEO_CC_SM8150_CLOCKS.as_ptr() }, num_clks: 6, resets: VIDEO_CC_SM8150_RESETS.as_ptr(), num_resets: 5, gdscs: unsafe { VIDEO_CC_SM8150_GDSCS.as_ptr() }, num_gdscs: 3 };

static VIDEO_CC_SM8150_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId { compatible: "qcom,sm8150-videocc" }, OfDeviceId::default()];

unsafe fn video_cc_sm8150_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut regmap: *mut Regmap;
    let mut ret = devm_pm_runtime_enable((*pdev).dev());
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get((*pdev).dev());
    if ret != 0 { return ret; }
    regmap = qcom_cc_map(pdev, &VIDEO_CC_SM8150_DESC);
    if IS_ERR(regmap) {
        pm_runtime_put_sync((*pdev).dev());
        return PTR_ERR(regmap);
    }
    clk_trion_pll_configure(&mut VIDEO_PLL0, regmap, &VIDEO_PLL0_CONFIG);
    // Keep VIDEO_CC_XO_CLK ALWAYS-ON
    regmap_update_bits(regmap, 0x984, 0x1, 0x1);
    ret = qcom_cc_really_probe((*pdev).dev(), &VIDEO_CC_SM8150_DESC, regmap);
    pm_runtime_put_sync((*pdev).dev());
    ret
}

static VIDEO_CC_SM8150_DRIVER: PlatformDriver = PlatformDriver {
    probe: video_cc_sm8150_probe,
    driver: Driver { name: "video_cc-sm8150", of_match_table: VIDEO_CC_SM8150_MATCH_TABLE.as_ptr() },
};

// Equivalent of module_platform_driver(video_cc_sm8150_driver).
module_platform_driver!(VIDEO_CC_SM8150_DRIVER);

// MODULE_DEVICE_TABLE(of, video_cc_sm8150_match_table);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("QTI VIDEOCC SM8150 Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
