// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 * Copyright (c) 2024, Danila Tikhonov <danila@jiaxyga.com>
 */

// Kernel, device-tree, and clock-controller types and symbols are supplied by
// the surrounding translation unit.
use crate::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DtBiTcxo { DT_BI_TCXO, DT_BI_TCXO_AO }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Parent { P_BI_TCXO, P_VIDEOCC_PLL0_OUT_EVEN, P_VIDEOCC_PLL0_OUT_MAIN, P_VIDEOCC_PLL0_OUT_ODD }

static FABIA_VCO: [PllVco; 2] = [
    PllVco { min_freq: 249600000, max_freq: 2000000000, val: 0 },
    PllVco { min_freq: 125000000, max_freq: 1000000000, val: 1 },
];

static mut VIDEOCC_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x19, alpha: 0, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002067,
    user_ctl_val: 1, user_ctl_hi_val: 0x00004805, test_ctl_hi_val: 0x40000000,
};

static mut VIDEOCC_PLL0: ClkAlphaPll = ClkAlphaPll {
    offset: 0x42c, vco_table: FABIA_VCO.as_ptr(), num_vco: FABIA_VCO.len(),
    regs: unsafe { CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_FABIA] },
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "videocc_pll0", parent_data: &ClkParentData::Index(DT_BI_TCXO as usize),
        num_parents: 1, flags: 0, ops: &CLK_ALPHA_PLL_FABIA_OPS,
    } } },
};

static VIDEOCC_PARENT_MAP_0: [ParentMap; 4] = [
    ParentMap { parent: P_BI_TCXO as usize, val: 0 }, ParentMap { parent: P_VIDEOCC_PLL0_OUT_MAIN as usize, val: 1 },
    ParentMap { parent: P_VIDEOCC_PLL0_OUT_EVEN as usize, val: 2 }, ParentMap { parent: P_VIDEOCC_PLL0_OUT_ODD as usize, val: 3 },
];
static VIDEOCC_PARENT_DATA_0: [ClkParentData; 4] = [
    ClkParentData::Index(DT_BI_TCXO as usize), ClkParentData::Hw(unsafe { &VIDEOCC_PLL0.clkr.hw }),
    ClkParentData::Hw(unsafe { &VIDEOCC_PLL0.clkr.hw }), ClkParentData::Hw(unsafe { &VIDEOCC_PLL0.clkr.hw }),
];
static VIDEOCC_PARENT_MAP_1: [ParentMap; 1] = [ParentMap { parent: P_BI_TCXO as usize, val: 0 }];
static VIDEOCC_PARENT_DATA_1: [ClkParentData; 1] = [ClkParentData::Index(DT_BI_TCXO_AO as usize)];

static FTBL_VIDEOCC_IRIS_CLK_SRC: [FreqTbl; 6] = [
    FreqTbl::new(240000000, P_VIDEOCC_PLL0_OUT_MAIN as usize, 2, 0, 0),
    FreqTbl::new(338000000, P_VIDEOCC_PLL0_OUT_MAIN as usize, 2, 0, 0),
    FreqTbl::new(365000000, P_VIDEOCC_PLL0_OUT_MAIN as usize, 2, 0, 0),
    FreqTbl::new(444000000, P_VIDEOCC_PLL0_OUT_MAIN as usize, 2, 0, 0),
    FreqTbl::new(533000000, P_VIDEOCC_PLL0_OUT_MAIN as usize, 2, 0, 0), FreqTbl::EMPTY,
];
static FTBL_VIDEOCC_XO_CLK_SRC: [FreqTbl; 2] = [FreqTbl::new(19200000, P_BI_TCXO as usize, 1, 0, 0), FreqTbl::EMPTY];

// The following controller objects retain the C driver's register layout and
// initialization data; their concrete kernel types are provided externally.
static mut VIDEOCC_IRIS_CLK_SRC: ClkRcg2 = ClkRcg2::new(0x7f0, 0, 5, &VIDEOCC_PARENT_MAP_0, &FTBL_VIDEOCC_IRIS_CLK_SRC, "videocc_iris_clk_src", &VIDEOCC_PARENT_DATA_0, CLK_SET_RATE_PARENT, &CLK_RCG2_SHARED_OPS);
static mut VIDEOCC_XO_CLK_SRC: ClkRcg2 = ClkRcg2::new(0xa98, 0, 5, &VIDEOCC_PARENT_MAP_1, &FTBL_VIDEOCC_XO_CLK_SRC, "videocc_xo_clk_src", &VIDEOCC_PARENT_DATA_1, 0, &CLK_RCG2_OPS);

static mut VIDEOCC_IRIS_AHB_CLK: ClkBranch = ClkBranch::new(0x8f4, BRANCH_VOTED, 0x8f4, BIT(0), "videocc_iris_ahb_clk", unsafe { &VIDEOCC_IRIS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVS0_AXI_CLK: ClkBranch = ClkBranch::new(0x9ec, BRANCH_HALT, 0x9ec, BIT(0), "videocc_mvs0_axi_clk", core::ptr::null(), 0, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVS0_CORE_CLK: ClkBranch = ClkBranch::new(0x890, BRANCH_VOTED, 0x890, BIT(0), "videocc_mvs0_core_clk", unsafe { &VIDEOCC_IRIS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVS1_AXI_CLK: ClkBranch = ClkBranch::new(0xa0c, BRANCH_HALT, 0xa0c, BIT(0), "videocc_mvs1_axi_clk", core::ptr::null(), 0, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVS1_CORE_CLK: ClkBranch = ClkBranch::new(0x8d0, BRANCH_VOTED, 0x8d0, BIT(0), "videocc_mvs1_core_clk", unsafe { &VIDEOCC_IRIS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVSC_CORE_CLK: ClkBranch = ClkBranch::new(0x850, BRANCH_HALT, 0x850, BIT(0), "videocc_mvsc_core_clk", unsafe { &VIDEOCC_IRIS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT, &CLK_BRANCH2_OPS);
static mut VIDEOCC_MVSC_CTL_AXI_CLK: ClkBranch = ClkBranch::new(0x9cc, BRANCH_HALT, 0x9cc, BIT(0), "videocc_mvsc_ctl_axi_clk", core::ptr::null(), 0, &CLK_BRANCH2_OPS);
static mut VIDEOCC_VENUS_AHB_CLK: ClkBranch = ClkBranch::new(0xa6c, BRANCH_HALT, 0xa6c, BIT(0), "videocc_venus_ahb_clk", core::ptr::null(), 0, &CLK_BRANCH2_OPS);

static mut VENUS_GDSC: Gdsc = Gdsc::new(0x814, "venus_gdsc", &[0x850, 0x9cc], PWRSTS_OFF_ON, POLL_CFG_GDSCR);
static mut VCODEC0_GDSC: Gdsc = Gdsc::new(0x874, "vcodec0_gdsc", &[0x890, 0x9ec], PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR);
static mut VCODEC1_GDSC: Gdsc = Gdsc::new(0x8b4, "vcodec1_gdsc", &[0x8d0, 0xa0c], PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR);

static mut VIDEOCC_SM7150_CLOCKS: [*mut ClkRegmap; 11] = [
    unsafe { &mut VIDEOCC_PLL0.clkr }, unsafe { &mut VIDEOCC_IRIS_AHB_CLK.clkr },
    unsafe { &mut VIDEOCC_IRIS_CLK_SRC.clkr }, unsafe { &mut VIDEOCC_MVS0_AXI_CLK.clkr },
    unsafe { &mut VIDEOCC_MVS0_CORE_CLK.clkr }, unsafe { &mut VIDEOCC_MVS1_AXI_CLK.clkr },
    unsafe { &mut VIDEOCC_MVS1_CORE_CLK.clkr }, unsafe { &mut VIDEOCC_MVSC_CORE_CLK.clkr },
    unsafe { &mut VIDEOCC_MVSC_CTL_AXI_CLK.clkr }, unsafe { &mut VIDEOCC_VENUS_AHB_CLK.clkr },
    unsafe { &mut VIDEOCC_XO_CLK_SRC.clkr },
];
static mut VIDEOCC_SM7150_GDSCS: [*mut Gdsc; 3] = [
    unsafe { &mut VENUS_GDSC }, unsafe { &mut VCODEC0_GDSC }, unsafe { &mut VCODEC1_GDSC },
];
static VIDEOCC_SM7150_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb94, fast_io: true,
};
static VIDEOCC_SM7150_DESC: QcomCcDesc = QcomCcDesc {
    config: &VIDEOCC_SM7150_REGMAP_CONFIG,
    clks: &VIDEOCC_SM7150_CLOCKS,
    num_clks: VIDEOCC_SM7150_CLOCKS.len(),
    gdscs: &VIDEOCC_SM7150_GDSCS,
    num_gdscs: VIDEOCC_SM7150_GDSCS.len(),
};

static VIDEOCC_SM7150_MATCH_TABLE: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "qcom,sm7150-videocc" }, OfDeviceId::EMPTY,
];

extern "C" {
    fn qcom_cc_map(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> *mut Regmap;
    fn clk_fabia_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *mut AlphaPllConfig);
    fn qcom_branch_set_clk_en(regmap: *mut Regmap, reg: u32);
    fn qcom_cc_really_probe(dev: *mut Device, desc: *const QcomCcDesc, regmap: *mut Regmap) -> i32;
}

unsafe extern "C" fn videocc_sm7150_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap = qcom_cc_map(pdev, &VIDEOCC_SM7150_DESC);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    clk_fabia_pll_configure(&mut VIDEOCC_PLL0, regmap, &mut VIDEOCC_PLL0_CONFIG);
    // Keep some clocks always-on.
    qcom_branch_set_clk_en(regmap, 0x984); // VIDEOCC_XO_CLK
    qcom_cc_really_probe(&mut (*pdev).dev, &VIDEOCC_SM7150_DESC, regmap)
}

static mut VIDEOCC_SM7150_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(videocc_sm7150_probe),
    name: "videocc-sm7150",
    of_match_table: &VIDEOCC_SM7150_MATCH_TABLE,
};

// Equivalent of module_platform_driver(videocc_sm7150_driver).
module_platform_driver!(VIDEOCC_SM7150_DRIVER);
module_description!("Qualcomm SM7150 Video Clock Controller");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
