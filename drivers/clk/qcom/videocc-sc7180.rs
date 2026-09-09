// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019, The Linux Foundation. All rights reserved.
 */

// Translated from the Linux Qualcomm SC7180 video clock controller.

#[repr(C)]
enum Parent {
    PBiTcxo,
    PVideoPll0OutMain,
}

static FABIA_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static mut VIDEO_PLL0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x42c,
    vco_table: FABIA_VCO.as_ptr(),
    num_vco: FABIA_VCO.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "video_pll0",
                parent_data: &clk_parent_data { fw_name: "bi_tcxo", hw: core::ptr::null() },
                num_parents: 1,
                ops: &clk_alpha_pll_fabia_ops,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    },
};

static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [
    parent_map { src: PBiTcxo as u32, cfg: 0 },
    parent_map { src: PVideoPll0OutMain as u32, cfg: 1 },
];

static VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: "bi_tcxo", hw: core::ptr::null() },
    clk_parent_data { fw_name: core::ptr::null(), hw: unsafe { &VIDEO_PLL0.clkr.hw } },
];

static FTBL_VIDEO_CC_VENUS_CLK_SRC: [freq_tbl; 7] = [
    F(19200000, PBiTcxo, 1, 0, 0),
    F(150000000, PVideoPll0OutMain, 4, 0, 0),
    F(270000000, PVideoPll0OutMain, 2.5, 0, 0),
    F(340000000, PVideoPll0OutMain, 2, 0, 0),
    F(434000000, PVideoPll0OutMain, 2, 0, 0),
    F(500000000, PVideoPll0OutMain, 2, 0, 0),
    freq_tbl::default(),
];

static mut VIDEO_CC_VENUS_CLK_SRC: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x7f0, mnd_width: 0, hid_width: 5,
    parent_map: VIDEO_CC_PARENT_MAP_1.as_ptr(), freq_tbl: FTBL_VIDEO_CC_VENUS_CLK_SRC.as_ptr(),
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_cc_venus_clk_src", parent_data: VIDEO_CC_PARENT_DATA_1.as_ptr(),
        num_parents: VIDEO_CC_PARENT_DATA_1.len(), flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops,
        ..Default::default()
    }, ..Default::default() }, ..Default::default() },
};

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $parent:expr, $flags:expr) => {
    static mut $name: clk_branch = clk_branch {
        halt_reg: $reg, halt_check: $halt,
        clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
            name: stringify!($name), parent_hws: $parent, num_parents: if $parent.is_null() { 0 } else { 1 },
            flags: $flags, ops: &clk_branch2_ops, ..Default::default()
        }, ..Default::default() }, ..Default::default() },
    };
}; }

branch!(VIDEO_CC_VCODEC0_AXI_CLK, 0x9ec, BRANCH_HALT, core::ptr::null(), 0);
branch!(VIDEO_CC_VCODEC0_CORE_CLK, 0x890, BRANCH_HALT_VOTED, unsafe { &VIDEO_CC_VENUS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT);
branch!(VIDEO_CC_VENUS_AHB_CLK, 0xa4c, BRANCH_HALT, core::ptr::null(), 0);
branch!(VIDEO_CC_VENUS_CTL_AXI_CLK, 0x9cc, BRANCH_HALT, core::ptr::null(), 0);
branch!(VIDEO_CC_VENUS_CTL_CORE_CLK, 0x850, BRANCH_HALT, unsafe { &VIDEO_CC_VENUS_CLK_SRC.clkr.hw }, CLK_SET_RATE_PARENT);

static mut VENUS_GDSC: gdsc = gdsc { gdscr: 0x814, pd: generic_pm_domain { name: "venus_gdsc", ..Default::default() }, pwrsts: PWRSTS_OFF_ON, ..Default::default() };
static mut VCODEC0_GDSC: gdsc = gdsc { gdscr: 0x874, pd: generic_pm_domain { name: "vcodec0_gdsc", ..Default::default() }, flags: HW_CTRL_TRIGGER, pwrsts: PWRSTS_OFF_ON, ..Default::default() };

static mut VIDEO_CC_SC7180_CLOCKS: [*mut clk_regmap; 7] = [
    unsafe { &mut VIDEO_CC_VCODEC0_AXI_CLK.clkr }, unsafe { &mut VIDEO_CC_VCODEC0_CORE_CLK.clkr },
    unsafe { &mut VIDEO_CC_VENUS_AHB_CLK.clkr }, unsafe { &mut VIDEO_CC_VENUS_CLK_SRC.clkr },
    unsafe { &mut VIDEO_CC_VENUS_CTL_AXI_CLK.clkr }, unsafe { &mut VIDEO_CC_VENUS_CTL_CORE_CLK.clkr },
    unsafe { &mut VIDEO_PLL0.clkr },
];
static mut VIDEO_CC_SC7180_GDSCS: [*mut gdsc; 2] = [unsafe { &mut VENUS_GDSC }, unsafe { &mut VCODEC0_GDSC }];

static VIDEO_CC_SC7180_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb94, fast_io: true, ..Default::default() };
static VIDEO_CC_SC7180_DESC: qcom_cc_desc = qcom_cc_desc {
    config: &VIDEO_CC_SC7180_REGMAP_CONFIG, clks: VIDEO_CC_SC7180_CLOCKS.as_ptr(), num_clks: VIDEO_CC_SC7180_CLOCKS.len(),
    gdscs: VIDEO_CC_SC7180_GDSCS.as_ptr(), num_gdscs: VIDEO_CC_SC7180_GDSCS.len(),
};

static VIDEO_CC_SC7180_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sc7180-videocc", ..Default::default() }, of_device_id::default(),
];

unsafe fn video_cc_sc7180_probe(pdev: *mut platform_device) -> i32 {
    let mut video_pll0_config: alpha_pll_config = core::mem::zeroed();
    let regmap = qcom_cc_map(pdev, &VIDEO_CC_SC7180_DESC);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    video_pll0_config.l = 0x1f;
    video_pll0_config.alpha = 0x4000;
    video_pll0_config.user_ctl_val = 0x00000001;
    video_pll0_config.user_ctl_hi_val = 0x00004805;
    clk_fabia_pll_configure(&mut VIDEO_PLL0, regmap, &video_pll0_config);
    // Keep VIDEO_CC_XO_CLK ALWAYS-ON
    regmap_update_bits(regmap, 0x984, 0x1, 0x1);
    qcom_cc_really_probe(&mut (*pdev).dev, &VIDEO_CC_SC7180_DESC, regmap)
}

static mut VIDEO_CC_SC7180_DRIVER: platform_driver = platform_driver {
    probe: Some(video_cc_sc7180_probe),
    driver: driver { name: "sc7180-videocc", of_match_table: VIDEO_CC_SC7180_MATCH_TABLE.as_ptr(), ..Default::default() },
};

module_platform_driver!(VIDEO_CC_SC7180_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
