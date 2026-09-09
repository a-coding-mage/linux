// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018, The Linux Foundation. All rights reserved. */
// C dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub enum VideoParent {
    PBiTcxo,
    PVideoPll0OutMain,
}

static VIDEO_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig { l: 0x10, alpha: 0xaaab };

static mut VIDEO_PLL0: ClkAlphaPll = ClkAlphaPll {
    offset: 0x42c,
    regs: unsafe { clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA] },
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "video_pll0", parent_data: &ClkParentData { fw_name: "bi_tcxo", name: "bi_tcxo", hw: core::ptr::null() },
        num_parents: 1, parent_hws: core::ptr::null(), flags: 0, ops: &clk_alpha_pll_fabia_ops,
    } } },
};

static VIDEO_CC_PARENT_MAP_0: [ParentMap; 2] = [
    ParentMap { src: PVideoParent::PBiTcxo as u32, cfg: 0 },
    ParentMap { src: PVideoParent::PVideoPll0OutMain as u32, cfg: 1 },
];

static mut VIDEO_CC_PARENT_DATA_0: [ClkParentData; 2] = [
    ClkParentData { fw_name: "bi_tcxo", name: "bi_tcxo", hw: core::ptr::null() },
    ClkParentData { fw_name: core::ptr::null(), name: core::ptr::null(), hw: unsafe { &VIDEO_PLL0.clkr.hw } },
];

static FTBL_VIDEO_CC_VENUS_CLK_SRC: [FreqTbl; 7] = [
    FreqTbl { freq: 100000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 4, m: 0, n: 0 },
    FreqTbl { freq: 200000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 330000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 404000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 444000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 533000000, src: PVideoParent::PVideoPll0OutMain as u32, pre_div: 1, m: 0, n: 0 },
    FreqTbl::default(),
];

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $label:expr) => {
    static mut $name: ClkBranch = ClkBranch { halt_reg: $reg, halt_check: $halt, clkr: ClkRegmap { enable_reg: $reg, enable_mask: 1, hw: ClkHw { init: &ClkInitData { name: $label, parent_data: core::ptr::null(), parent_hws: core::ptr::null(), num_parents: 0, flags: 0, ops: &clk_branch2_ops } } } };
} }

static mut VIDEO_CC_VENUS_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x7f0, mnd_width: 0, hid_width: 5, parent_map: &VIDEO_CC_PARENT_MAP_0, freq_tbl: &FTBL_VIDEO_CC_VENUS_CLK_SRC, clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData { name: "video_cc_venus_clk_src", parent_data: unsafe { &VIDEO_CC_PARENT_DATA_0 }, parent_hws: core::ptr::null(), num_parents: 2, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
branch!(VIDEO_CC_APB_CLK, 0x990, BRANCH_HALT, "video_cc_apb_clk");
branch!(VIDEO_CC_AT_CLK, 0x9f0, BRANCH_HALT, "video_cc_at_clk");
branch!(VIDEO_CC_QDSS_TRIG_CLK, 0x970, BRANCH_HALT, "video_cc_qdss_trig_clk");
branch!(VIDEO_CC_QDSS_TSCTR_DIV8_CLK, 0x9d0, BRANCH_HALT, "video_cc_qdss_tsctr_div8_clk");
branch!(VIDEO_CC_VCODEC0_AXI_CLK, 0x930, BRANCH_HALT, "video_cc_vcodec0_axi_clk");
branch!(VIDEO_CC_VCODEC1_AXI_CLK, 0x950, BRANCH_HALT, "video_cc_vcodec1_axi_clk");
branch!(VIDEO_CC_VENUS_AHB_CLK, 0x9b0, BRANCH_HALT, "video_cc_venus_ahb_clk");
branch!(VIDEO_CC_VENUS_CTL_AXI_CLK, 0x910, BRANCH_HALT, "video_cc_venus_ctl_axi_clk");

static mut VIDEO_CC_VCODEC0_CORE_CLK: ClkBranch = core_clk!(0x890, BRANCH_VOTED, "video_cc_vcodec0_core_clk", VIDEO_CC_VENUS_CLK_SRC);
static mut VIDEO_CC_VCODEC1_CORE_CLK: ClkBranch = core_clk!(0x8d0, BRANCH_VOTED, "video_cc_vcodec1_core_clk", VIDEO_CC_VENUS_CLK_SRC);
static mut VIDEO_CC_VENUS_CTL_CORE_CLK: ClkBranch = core_clk!(0x850, BRANCH_HALT, "video_cc_venus_ctl_core_clk", VIDEO_CC_VENUS_CLK_SRC);

static mut VENUS_GDSC: Gdsc = Gdsc { gdscr: 0x814, pd: GenericPmDomain { name: "venus_gdsc" }, cxcs: &[0x850, 0x910], cxc_count: 2, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR };
static mut VCODEC0_GDSC: Gdsc = Gdsc { gdscr: 0x874, pd: GenericPmDomain { name: "vcodec0_gdsc" }, cxcs: &[0x890, 0x930], cxc_count: 2, flags: HW_CTRL_TRIGGER | POLL_CFG_GDSCR, pwrsts: PWRSTS_OFF_ON };
static mut VCODEC1_GDSC: Gdsc = Gdsc { gdscr: 0x8b4, pd: GenericPmDomain { name: "vcodec1_gdsc" }, cxcs: &[0x8d0, 0x950], cxc_count: 2, flags: HW_CTRL_TRIGGER | POLL_CFG_GDSCR, pwrsts: PWRSTS_OFF_ON };

static mut VIDEO_CC_SDM845_CLOCKS: [*mut ClkRegmap; 13] = [
    &mut VIDEO_CC_APB_CLK.clkr, &mut VIDEO_CC_AT_CLK.clkr, &mut VIDEO_CC_QDSS_TRIG_CLK.clkr,
    &mut VIDEO_CC_QDSS_TSCTR_DIV8_CLK.clkr, &mut VIDEO_CC_VCODEC0_AXI_CLK.clkr, &mut VIDEO_CC_VCODEC0_CORE_CLK.clkr,
    &mut VIDEO_CC_VCODEC1_AXI_CLK.clkr, &mut VIDEO_CC_VCODEC1_CORE_CLK.clkr, &mut VIDEO_CC_VENUS_AHB_CLK.clkr,
    &mut VIDEO_CC_VENUS_CLK_SRC.clkr, &mut VIDEO_CC_VENUS_CTL_AXI_CLK.clkr, &mut VIDEO_CC_VENUS_CTL_CORE_CLK.clkr,
    &mut VIDEO_PLL0.clkr,
];
static mut VIDEO_CC_SDM845_GDSCS: [*mut Gdsc; 3] = [&mut VENUS_GDSC, &mut VCODEC0_GDSC, &mut VCODEC1_GDSC];

static VIDEO_CC_SDM845_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb90, fast_io: true };
static VIDEO_CC_SDM845_DESC: QcomCcDesc = QcomCcDesc { config: &VIDEO_CC_SDM845_REGMAP_CONFIG, clks: &VIDEO_CC_SDM845_CLOCKS, num_clks: 13, gdscs: &VIDEO_CC_SDM845_GDSCS, num_gdscs: 3 };
static VIDEO_CC_SDM845_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId { compatible: "qcom,sdm845-videocc" }, OfDeviceId::default()];

unsafe extern "C" fn video_cc_sdm845_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SDM845_DESC);
    if is_err(regmap) { return ptr_err(regmap); }
    clk_fabia_pll_configure(&mut VIDEO_PLL0, regmap, &VIDEO_PLL0_CONFIG);
    qcom_cc_really_probe(&mut (*pdev).dev, &VIDEO_CC_SDM845_DESC, regmap)
}

static mut VIDEO_CC_SDM845_DRIVER: PlatformDriver = PlatformDriver { probe: Some(video_cc_sdm845_probe), driver: Driver { name: "sdm845-videocc", of_match_table: &VIDEO_CC_SDM845_MATCH_TABLE } };

// module_platform_driver(video_cc_sdm845_driver);
// MODULE_DEVICE_TABLE(of, video_cc_sdm845_match_table);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("QTI SDM845 VIDEOCC Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
