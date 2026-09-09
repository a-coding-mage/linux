// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(usize)]
enum DtInput { DT_IFACE, DT_BI_TCXO, DT_BI_TCXO_AO, DT_SLEEP_CLK }

#[repr(usize)]
enum Parent { P_BI_TCXO, P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN, P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN, P_SLEEP_CLK }

static RIVIAN_ELU_VCO: [pll_vco; 2] = [
    pll_vco { min_freq: 833000000, max_freq: 1125000000, val: 0 },
    pll_vco { min_freq: 777000000, max_freq: 1062000000, val: 1 },
];

/* 960.0 MHz Configuration */
static CAM_BIST_MCLK_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x32, alpha: 0x0, config_ctl_val: 0x12000000,
    config_ctl_hi_val: 0x00890263, config_ctl_hi1_val: 0x1af04237,
    config_ctl_hi2_val: 0x00000000,
};

static mut cam_bist_mclk_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &CAM_BIST_MCLK_CC_PLL0_CONFIG,
    vco_table: &RIVIAN_ELU_VCO, num_vco: RIVIAN_ELU_VCO.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_RIVIAN_ELU],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "cam_bist_mclk_cc_pll0", parent_data: &clk_parent_data { index: DtInput::DT_BI_TCXO as usize },
        num_parents: 1, ops: &clk_alpha_pll_rivian_elu_ops,
    } } },
};

static CAM_BIST_MCLK_CC_PARENT_MAP_0: [parent_map; 3] = [
    parent_map { parent: Parent::P_BI_TCXO as usize, val: 0 },
    parent_map { parent: Parent::P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN as usize, val: 3 },
    parent_map { parent: Parent::P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN as usize, val: 5 },
];
static CAM_BIST_MCLK_CC_PARENT_DATA_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DtInput::DT_BI_TCXO as usize },
    clk_parent_data { hw: unsafe { &(*(&raw mut cam_bist_mclk_cc_pll0).cast::<clk_alpha_pll>()).clkr.hw } },
    clk_parent_data { hw: unsafe { &(*(&raw mut cam_bist_mclk_cc_pll0).cast::<clk_alpha_pll>()).clkr.hw } },
];
static CAM_BIST_MCLK_CC_PARENT_MAP_1: [parent_map; 1] = [parent_map { parent: Parent::P_SLEEP_CLK as usize, val: 0 }];
static CAM_BIST_MCLK_CC_PARENT_DATA_1: [clk_parent_data; 1] = [clk_parent_data { index: DtInput::DT_SLEEP_CLK as usize }];

static FTBL_CAM_BIST_MCLK_CC_MCLK0_CLK_SRC: [freq_tbl; 5] = [
    F(12000000, Parent::P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN as usize, 10, 1, 8),
    F(19200000, Parent::P_BI_TCXO as usize, 1, 0, 0),
    F(24000000, Parent::P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN as usize, 10, 1, 4),
    F(68571429, Parent::P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN as usize, 14, 0, 0),
    freq_tbl { ..Default::default() },
];

macro_rules! rcg2 { ($name:ident, $addr:expr, $src:expr) => {
    static mut $name: clk_rcg2 = clk_rcg2 { cmd_rcgr: $addr, mnd_width: 8, hid_width: 5,
        parent_map: &CAM_BIST_MCLK_CC_PARENT_MAP_0, freq_tbl: &FTBL_CAM_BIST_MCLK_CC_MCLK0_CLK_SRC,
        clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: $src,
            parent_data: &CAM_BIST_MCLK_CC_PARENT_DATA_0, num_parents: 3,
            flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
} }
rcg2!(cam_bist_mclk_cc_mclk0_clk_src, 0x4000, "cam_bist_mclk_cc_mclk0_clk_src");
rcg2!(cam_bist_mclk_cc_mclk1_clk_src, 0x401c, "cam_bist_mclk_cc_mclk1_clk_src");
rcg2!(cam_bist_mclk_cc_mclk2_clk_src, 0x4038, "cam_bist_mclk_cc_mclk2_clk_src");
rcg2!(cam_bist_mclk_cc_mclk3_clk_src, 0x4054, "cam_bist_mclk_cc_mclk3_clk_src");
rcg2!(cam_bist_mclk_cc_mclk4_clk_src, 0x4070, "cam_bist_mclk_cc_mclk4_clk_src");
rcg2!(cam_bist_mclk_cc_mclk5_clk_src, 0x408c, "cam_bist_mclk_cc_mclk5_clk_src");
rcg2!(cam_bist_mclk_cc_mclk6_clk_src, 0x40a8, "cam_bist_mclk_cc_mclk6_clk_src");
rcg2!(cam_bist_mclk_cc_mclk7_clk_src, 0x40c4, "cam_bist_mclk_cc_mclk7_clk_src");

static FTBL_CAM_BIST_MCLK_CC_SLEEP_CLK_SRC: [freq_tbl; 2] = [
    F(32000, Parent::P_SLEEP_CLK as usize, 1, 0, 0), freq_tbl { ..Default::default() },
];
static mut cam_bist_mclk_cc_sleep_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x40e0, mnd_width: 0, hid_width: 5,
    parent_map: &CAM_BIST_MCLK_CC_PARENT_MAP_1, freq_tbl: &FTBL_CAM_BIST_MCLK_CC_SLEEP_CLK_SRC,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "cam_bist_mclk_cc_sleep_clk_src",
        parent_data: &CAM_BIST_MCLK_CC_PARENT_DATA_1, num_parents: 1,
        flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };

macro_rules! branch { ($name:ident, $addr:expr, $src:ident, $label:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $addr, halt_check: BRANCH_HALT,
        clkr: clk_regmap { enable_reg: $addr, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
            name: $label, parent_hws: &[unsafe { &mut $src.clkr.hw }], num_parents: 1,
            flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops } } } };
} }
branch!(cam_bist_mclk_cc_mclk0_clk, 0x4018, cam_bist_mclk_cc_mclk0_clk_src, "cam_bist_mclk_cc_mclk0_clk");
branch!(cam_bist_mclk_cc_mclk1_clk, 0x4034, cam_bist_mclk_cc_mclk1_clk_src, "cam_bist_mclk_cc_mclk1_clk");
branch!(cam_bist_mclk_cc_mclk2_clk, 0x4050, cam_bist_mclk_cc_mclk2_clk_src, "cam_bist_mclk_cc_mclk2_clk");
branch!(cam_bist_mclk_cc_mclk3_clk, 0x406c, cam_bist_mclk_cc_mclk3_clk_src, "cam_bist_mclk_cc_mclk3_clk");
branch!(cam_bist_mclk_cc_mclk4_clk, 0x4088, cam_bist_mclk_cc_mclk4_clk_src, "cam_bist_mclk_cc_mclk4_clk");
branch!(cam_bist_mclk_cc_mclk5_clk, 0x40a4, cam_bist_mclk_cc_mclk5_clk_src, "cam_bist_mclk_cc_mclk5_clk");
branch!(cam_bist_mclk_cc_mclk6_clk, 0x40c0, cam_bist_mclk_cc_mclk6_clk_src, "cam_bist_mclk_cc_mclk6_clk");
branch!(cam_bist_mclk_cc_mclk7_clk, 0x40dc, cam_bist_mclk_cc_mclk7_clk_src, "cam_bist_mclk_cc_mclk7_clk");

static mut cam_bist_mclk_cc_sm8750_clocks: [*mut clk_regmap; 19] = [
    &raw mut cam_bist_mclk_cc_mclk0_clk.clkr, &raw mut cam_bist_mclk_cc_mclk0_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk1_clk.clkr, &raw mut cam_bist_mclk_cc_mclk1_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk2_clk.clkr, &raw mut cam_bist_mclk_cc_mclk2_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk3_clk.clkr, &raw mut cam_bist_mclk_cc_mclk3_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk4_clk.clkr, &raw mut cam_bist_mclk_cc_mclk4_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk5_clk.clkr, &raw mut cam_bist_mclk_cc_mclk5_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk6_clk.clkr, &raw mut cam_bist_mclk_cc_mclk6_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_mclk7_clk.clkr, &raw mut cam_bist_mclk_cc_mclk7_clk_src.clkr,
    &raw mut cam_bist_mclk_cc_pll0.clkr, &raw mut cam_bist_mclk_cc_sleep_clk_src.clkr,
];
static mut cam_bist_mclk_cc_sm8750_plls: [*mut clk_alpha_pll; 1] = [&raw mut cam_bist_mclk_cc_pll0];
static CAM_BIST_MCLK_CC_SM8750_CRITICAL_CBCRS: [u32; 1] = [0x40f8]; // CAM_BIST_MCLK_CC_SLEEP_CLK

static CAM_BIST_MCLK_CC_SM8750_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x5010, fast_io: true,
};
static CAM_BIST_MCLK_CC_SM8750_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data {
    alpha_plls: &raw mut cam_bist_mclk_cc_sm8750_plls,
    num_alpha_plls: 1, clk_cbcrs: &CAM_BIST_MCLK_CC_SM8750_CRITICAL_CBCRS, num_clk_cbcrs: 1,
};
static CAM_BIST_MCLK_CC_SM8750_DESC: qcom_cc_desc = qcom_cc_desc {
    config: &CAM_BIST_MCLK_CC_SM8750_REGMAP_CONFIG, clks: &raw mut cam_bist_mclk_cc_sm8750_clocks,
    num_clks: 19, use_rpm: true, driver_data: &CAM_BIST_MCLK_CC_SM8750_DRIVER_DATA,
};
static CAM_BIST_MCLK_CC_SM8750_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sm8750-cambistmclkcc" }, of_device_id { ..Default::default() },
];

unsafe extern "C" fn cam_bist_mclk_cc_sm8750_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &CAM_BIST_MCLK_CC_SM8750_DESC)
}

static mut cam_bist_mclk_cc_sm8750_driver: platform_driver = platform_driver {
    probe: Some(cam_bist_mclk_cc_sm8750_probe),
    driver: device_driver { name: "cambistmclkcc-sm8750", of_match_table: &CAM_BIST_MCLK_CC_SM8750_MATCH_TABLE },
};

module_platform_driver!(cam_bist_mclk_cc_sm8750_driver);
module_description!("QTI CAMBISTMCLKCC SM8750 Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
