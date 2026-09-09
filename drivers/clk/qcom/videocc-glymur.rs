// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// External Linux/QCOM clock framework declarations are supplied by the surrounding build.

enum DtBi { Tcxo, TcxoAo, SleepClk }
const DT_BI_TCXO: usize = 0;
const DT_BI_TCXO_AO: usize = 1;
const DT_SLEEP_CLK: usize = 2;

const P_BI_TCXO: usize = 0;
const P_SLEEP_CLK: usize = 1;
const P_VIDEO_CC_PLL0_OUT_MAIN: usize = 2;

static TAYCAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

/* 720.0 MHz Configuration */
static VIDEO_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x25, alpha: 0x8000, config_ctl_val: 0x25c400e7,
    config_ctl_hi_val: 0x0a8060e0, config_ctl_hi1_val: 0xf51dea20,
    user_ctl_val: 0x00000008, user_ctl_hi_val: 0x00000002,
};

static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &VIDEO_CC_PLL0_CONFIG, vco_table: &TAYCAN_EKO_T_VCO,
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_taycan_eko_t_ops,
    } } },
};

static VIDEO_CC_PARENT_MAP_0: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];
static VIDEO_CC_PARENT_DATA_0: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO }];
static VIDEO_CC_PARENT_MAP_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];
static mut VIDEO_CC_PARENT_DATA_1: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &video_cc_pll0.clkr.hw } }];
static VIDEO_CC_PARENT_MAP_2: [parent_map; 1] = [parent_map { src: P_SLEEP_CLK, cfg: 0 }];
static VIDEO_CC_PARENT_DATA_2: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK }];

static FTBL_VIDEO_CC_AHB_CLK_SRC: [freq_tbl; 2] = [F(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl::default()];
static FTBL_VIDEO_CC_MVS0_CLK_SRC: [freq_tbl; 7] = [
    F(720000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1014000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1098000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1332000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1600000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1965000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];
static FTBL_VIDEO_CC_SLEEP_CLK_SRC: [freq_tbl; 2] = [F(32000, P_SLEEP_CLK, 1, 0, 0), freq_tbl::default()];

// The following objects retain the C driver's register layout and framework initializers.
static mut video_cc_ahb_clk_src: clk_rcg2 = RCG2(0x8018, &VIDEO_CC_PARENT_MAP_0, &FTBL_VIDEO_CC_AHB_CLK_SRC, "video_cc_ahb_clk_src", &VIDEO_CC_PARENT_DATA_0);
static mut video_cc_mvs0_clk_src: clk_rcg2 = RCG2(0x8000, &VIDEO_CC_PARENT_MAP_1, &FTBL_VIDEO_CC_MVS0_CLK_SRC, "video_cc_mvs0_clk_src", unsafe { &VIDEO_CC_PARENT_DATA_1 });
static mut video_cc_sleep_clk_src: clk_rcg2 = RCG2(0x8120, &VIDEO_CC_PARENT_MAP_2, &FTBL_VIDEO_CC_SLEEP_CLK_SRC, "video_cc_sleep_clk_src", &VIDEO_CC_PARENT_DATA_2);
static mut video_cc_xo_clk_src: clk_rcg2 = RCG2(0x80f8, &VIDEO_CC_PARENT_MAP_0, &FTBL_VIDEO_CC_AHB_CLK_SRC, "video_cc_xo_clk_src", &VIDEO_CC_PARENT_DATA_0);

static mut video_cc_mvs0_div_clk_src: clk_regmap_div = DIV(0x809c, "video_cc_mvs0_div_clk_src", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = DIV(0x8060, "video_cc_mvs0c_div2_div_clk_src", unsafe { &video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs1_div_clk_src: clk_regmap_div = DIV(0x80d8, "video_cc_mvs1_div_clk_src", unsafe { &video_cc_mvs0_clk_src.clkr.hw });

static mut video_cc_mvs0_clk: clk_branch = BRANCH(0x807c, BRANCH_HALT_VOTED, "video_cc_mvs0_clk", unsafe { &video_cc_mvs0_div_clk_src.clkr.hw }, true);
static mut video_cc_mvs0_freerun_clk: clk_branch = BRANCH(0x808c, BRANCH_HALT, "video_cc_mvs0_freerun_clk", unsafe { &video_cc_mvs0_div_clk_src.clkr.hw }, false);
static mut video_cc_mvs0_shift_clk: clk_branch = BRANCH(0x8114, BRANCH_HALT_VOTED, "video_cc_mvs0_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw }, true);
static mut video_cc_mvs0c_clk: clk_branch = BRANCH(0x804c, BRANCH_HALT, "video_cc_mvs0c_clk", unsafe { &video_cc_mvs0c_div2_div_clk_src.clkr.hw }, false);
static mut video_cc_mvs0c_freerun_clk: clk_branch = BRANCH(0x805c, BRANCH_HALT, "video_cc_mvs0c_freerun_clk", unsafe { &video_cc_mvs0c_div2_div_clk_src.clkr.hw }, false);
static mut video_cc_mvs0c_shift_clk: clk_branch = BRANCH(0x811c, BRANCH_HALT_VOTED, "video_cc_mvs0c_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw }, true);
static mut video_cc_mvs1_clk: clk_branch = BRANCH(0x80b8, BRANCH_HALT_VOTED, "video_cc_mvs1_clk", unsafe { &video_cc_mvs1_div_clk_src.clkr.hw }, true);
static mut video_cc_mvs1_freerun_clk: clk_branch = BRANCH(0x80c8, BRANCH_HALT, "video_cc_mvs1_freerun_clk", unsafe { &video_cc_mvs1_div_clk_src.clkr.hw }, false);
static mut video_cc_mvs1_shift_clk: clk_branch = BRANCH(0x8118, BRANCH_HALT_VOTED, "video_cc_mvs1_shift_clk", unsafe { &video_cc_xo_clk_src.clkr.hw }, true);

static mut video_cc_mvs0c_gdsc: gdsc = GDSC(0x8034, "video_cc_mvs0c_gdsc", PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);
static mut video_cc_mvs0_gdsc: gdsc = GDSC(0x8068, "video_cc_mvs0_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, Some(unsafe { &video_cc_mvs0c_gdsc.pd }));
static mut video_cc_mvs1_gdsc: gdsc = GDSC(0x80a4, "video_cc_mvs1_gdsc", PWRSTS_OFF_ON, HW_CTRL_TRIGGER | POLL_CFG_GDSCR | RETAIN_FF_ENABLE, None);

unsafe fn clk_glymur_regs_configure(_dev: *mut device, regmap: *mut regmap) { regmap_update_bits(regmap, 0x9f24, BIT(0), BIT(0)); }

// External declarations and QCOM ID-indexed descriptor tables are intentionally retained as dependency-facing symbols.
extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

static mut video_cc_glymur_clocks: [*mut clk_regmap; 18] = [
    unsafe { &mut video_cc_ahb_clk_src.clkr }, unsafe { &mut video_cc_mvs0_clk.clkr }, unsafe { &mut video_cc_mvs0_clk_src.clkr },
    unsafe { &mut video_cc_mvs0_div_clk_src.clkr }, unsafe { &mut video_cc_mvs0_freerun_clk.clkr }, unsafe { &mut video_cc_mvs0_shift_clk.clkr },
    unsafe { &mut video_cc_mvs0c_clk.clkr }, unsafe { &mut video_cc_mvs0c_div2_div_clk_src.clkr }, unsafe { &mut video_cc_mvs0c_freerun_clk.clkr },
    unsafe { &mut video_cc_mvs0c_shift_clk.clkr }, unsafe { &mut video_cc_mvs1_clk.clkr }, unsafe { &mut video_cc_mvs1_div_clk_src.clkr },
    unsafe { &mut video_cc_mvs1_freerun_clk.clkr }, unsafe { &mut video_cc_mvs1_shift_clk.clkr }, unsafe { &mut video_cc_pll0.clkr },
    unsafe { &mut video_cc_sleep_clk_src.clkr }, unsafe { &mut video_cc_xo_clk_src.clkr }, core::ptr::null_mut(),
];
static mut video_cc_glymur_gdscs: [*mut gdsc; 3] = [unsafe { &mut video_cc_mvs0_gdsc }, unsafe { &mut video_cc_mvs0c_gdsc }, unsafe { &mut video_cc_mvs1_gdsc }];
static video_cc_glymur_resets: [qcom_reset_map; 7] = [
    qcom_reset_map { reg: 0x80dc, bit: 0 }, qcom_reset_map { reg: 0x8064, bit: 0 }, qcom_reset_map { reg: 0x805c, bit: 2 },
    qcom_reset_map { reg: 0x8030, bit: 0 }, qcom_reset_map { reg: 0x808c, bit: 2 }, qcom_reset_map { reg: 0x80c8, bit: 2 }, qcom_reset_map { reg: 0x80a0, bit: 0 },
];
static mut video_cc_glymur_plls: [*mut clk_alpha_pll; 1] = [unsafe { &mut video_cc_pll0 }];
static video_cc_glymur_critical_cbcrs: [u32; 3] = [0x80e0, 0x8138, 0x8110];

static video_cc_glymur_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f54, fast_io: true };
static video_cc_glymur_driver_data: qcom_cc_driver_data = qcom_cc_driver_data {
    alpha_plls: unsafe { &video_cc_glymur_plls }, num_alpha_plls: 1,
    clk_cbcrs: &video_cc_glymur_critical_cbcrs, num_clk_cbcrs: 3,
    clk_regs_configure: Some(clk_glymur_regs_configure),
};
static video_cc_glymur_desc: qcom_cc_desc = qcom_cc_desc {
    config: &video_cc_glymur_regmap_config, clks: unsafe { &video_cc_glymur_clocks }, num_clks: 18,
    resets: &video_cc_glymur_resets, num_resets: 7, gdscs: unsafe { &video_cc_glymur_gdscs }, num_gdscs: 3,
    use_rpm: true, driver_data: &video_cc_glymur_driver_data,
};
static video_cc_glymur_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,glymur-videocc" }, of_device_id::default()];
unsafe fn video_cc_glymur_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &video_cc_glymur_desc) }
static mut video_cc_glymur_driver: platform_driver = platform_driver {
    probe: Some(video_cc_glymur_probe), driver: driver { name: "videocc-glymur", of_match_table: &video_cc_glymur_match_table },
};

// Equivalent of module_platform_driver(video_cc_glymur_driver).
// MODULE_DEVICE_TABLE(of, video_cc_glymur_match_table);
// MODULE_DESCRIPTION("QTI VIDEOCC Glymur Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
