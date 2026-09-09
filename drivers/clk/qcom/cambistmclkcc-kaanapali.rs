// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Direct Rust translation of the Kaanapali CAM BIST MCLK clock controller.
 * Linux clock-provider declarations and constants are supplied externally.
 */

#[repr(usize)]
enum DtClock { AhbClk, BiTcxo, BiTcxoAo, SleepClk }

#[repr(usize)]
enum Parent { BiTcxo, CamBistMclkCcPll0OutEven, CamBistMclkCcPll0OutMain }

static RIVIAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min_freq: 883200000, max_freq: 1171200000, val: 0 }];

static CAM_BIST_MCLK_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x32, cal_l: 0x32, alpha: 0, config_ctl_val: 0x12000000,
    config_ctl_hi_val: 0x00890263, config_ctl_hi1_val: 0x1af04237,
    config_ctl_hi2_val: 0,
};

static mut cam_bist_mclk_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, config: &CAM_BIST_MCLK_CC_PLL0_CONFIG, vco_table: &RIVIAN_EKO_T_VCO,
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_RIVIAN_EKO_T], clkr: clk_regmap { ..Default::default() },
};

static CAM_BIST_MCLK_CC_PARENT_MAP_0: [parent_map; 3] = [
    parent_map { src: Parent::BiTcxo as u32, cfg: 0 },
    parent_map { src: Parent::CamBistMclkCcPll0OutEven as u32, cfg: 3 },
    parent_map { src: Parent::CamBistMclkCcPll0OutMain as u32, cfg: 5 },
];

static CAM_BIST_MCLK_CC_PARENT_DATA_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DtClock::BiTcxo as u32 },
    clk_parent_data { hw: unsafe { &cam_bist_mclk_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &cam_bist_mclk_cc_pll0.clkr.hw } },
];

static FTBL_CAM_BIST_MCLK_CC_MCLK0_CLK_SRC: [freq_tbl; 4] = [
    freq_tbl { freq: 19200000, src: Parent::BiTcxo as u32, pre_div: 1, m: 0, n: 0 },
    freq_tbl { freq: 24000000, src: Parent::CamBistMclkCcPll0OutEven as u32, pre_div: 10, m: 1, n: 4 },
    freq_tbl { freq: 68571429, src: Parent::CamBistMclkCcPll0OutMain as u32, pre_div: 14, m: 0, n: 0 },
    freq_tbl::EMPTY,
];

macro_rules! rcg { ($n:ident, $off:expr) => {
    static mut $n: clk_rcg2 = clk_rcg2 {
        cmd_rcgr: $off, mnd_width: 8, hid_width: 5,
        parent_map: &CAM_BIST_MCLK_CC_PARENT_MAP_0, hw_clk_ctrl: true,
        freq_tbl: &FTBL_CAM_BIST_MCLK_CC_MCLK0_CLK_SRC,
        clkr: clk_regmap { ..Default::default() },
    };
}; }
rcg!(cam_bist_mclk_cc_mclk0_clk_src, 0x4000);
rcg!(cam_bist_mclk_cc_mclk1_clk_src, 0x401c);
rcg!(cam_bist_mclk_cc_mclk2_clk_src, 0x4038);
rcg!(cam_bist_mclk_cc_mclk3_clk_src, 0x4054);
rcg!(cam_bist_mclk_cc_mclk4_clk_src, 0x4070);
rcg!(cam_bist_mclk_cc_mclk5_clk_src, 0x408c);
rcg!(cam_bist_mclk_cc_mclk6_clk_src, 0x40a8);
rcg!(cam_bist_mclk_cc_mclk7_clk_src, 0x40c4);

macro_rules! branch { ($n:ident, $off:expr, $src:ident) => {
    static mut $n: clk_branch = clk_branch {
        halt_reg: $off, halt_check: BRANCH_HALT,
        clkr: clk_regmap { enable_reg: $off, enable_mask: BIT(0), ..Default::default() },
    };
}; }
branch!(cam_bist_mclk_cc_mclk0_clk, 0x4018, cam_bist_mclk_cc_mclk0_clk_src);
branch!(cam_bist_mclk_cc_mclk1_clk, 0x4034, cam_bist_mclk_cc_mclk1_clk_src);
branch!(cam_bist_mclk_cc_mclk2_clk, 0x4050, cam_bist_mclk_cc_mclk2_clk_src);
branch!(cam_bist_mclk_cc_mclk3_clk, 0x406c, cam_bist_mclk_cc_mclk3_clk_src);
branch!(cam_bist_mclk_cc_mclk4_clk, 0x4088, cam_bist_mclk_cc_mclk4_clk_src);
branch!(cam_bist_mclk_cc_mclk5_clk, 0x40a4, cam_bist_mclk_cc_mclk5_clk_src);
branch!(cam_bist_mclk_cc_mclk6_clk, 0x40c0, cam_bist_mclk_cc_mclk6_clk_src);
branch!(cam_bist_mclk_cc_mclk7_clk, 0x40dc, cam_bist_mclk_cc_mclk7_clk_src);

static CAM_BIST_MCLK_CC_KAANAPALI_CRITICAL_CBCRS: [u32; 1] = [0x40e0];
static mut CAM_BIST_MCLK_CC_KAANAPALI_CLOCKS: [*mut clk_regmap; 17] = [
    unsafe { &mut cam_bist_mclk_cc_mclk0_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk0_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk1_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk1_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk2_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk2_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk3_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk3_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk4_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk4_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk5_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk5_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk6_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk6_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_mclk7_clk.clkr }, unsafe { &mut cam_bist_mclk_cc_mclk7_clk_src.clkr },
    unsafe { &mut cam_bist_mclk_cc_pll0.clkr },
];
static CAM_BIST_MCLK_CC_KAANAPALI_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x5010, fast_io: true,
};

static CAM_BIST_MCLK_CC_KAANAPALI_DESC: qcom_cc_desc = qcom_cc_desc {
    config: &CAM_BIST_MCLK_CC_KAANAPALI_REGMAP_CONFIG,
    clks: &CAM_BIST_MCLK_CC_KAANAPALI_CLOCKS, num_clks: 17, use_rpm: true, driver_data: core::ptr::null(),
};

unsafe extern "C" { fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32; }

unsafe extern "C" fn cam_bist_mclk_cc_kaanapali_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &CAM_BIST_MCLK_CC_KAANAPALI_DESC)
}

static CAM_BIST_MCLK_CC_KAANAPALI_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,kaanapali-cambistmclkcc\0" }, of_device_id::EMPTY,
];
static mut CAM_BIST_MCLK_CC_KAANAPALI_DRIVER: platform_driver = platform_driver {
    probe: Some(cam_bist_mclk_cc_kaanapali_probe),
    driver: driver { name: "cambistmclkcc-kaanapali\0", of_match_table: &CAM_BIST_MCLK_CC_KAANAPALI_MATCH_TABLE },
};

// Equivalent of module_platform_driver(cam_bist_mclk_cc_kaanapali_driver).
// MODULE_DEVICE_TABLE(of, cam_bist_mclk_cc_kaanapali_match_table);
// MODULE_DESCRIPTION("QTI CAMBISTMCLKCC Kaanapali Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
