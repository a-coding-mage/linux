// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// C dependencies: linux/clk-provider.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/regmap.h, dt-bindings/clock/qcom,qcs615-videocc.h,
// clk-alpha-pll.h, clk-branch.h, clk-pll.h, clk-rcg.h, clk-regmap.h,
// clk-regmap-divider.h, clk-regmap-mux.h, common.h, gdsc.h, reset.h.

enum { DT_BI_TCXO, DT_SLEEP_CLK }

enum {
    P_BI_TCXO,
    P_SLEEP_CLK,
    P_VIDEO_PLL0_OUT_AUX,
    P_VIDEO_PLL0_OUT_AUX2,
    P_VIDEO_PLL0_OUT_MAIN,
}

static video_cc_pll0_vco: [pll_vco; 1] = [pll_vco { min_freq: 500000000, max_freq: 1000000000, val: 2 }];

/* 600MHz configuration VCO - 2 */
static mut video_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1f, alpha_hi: 0x40, alpha: 0x00, alpha_en_mask: BIT(24),
    vco_val: BIT(21), vco_mask: GENMASK(21, 20), main_output_mask: BIT(0),
    config_ctl_val: 0x4001055b, test_ctl_hi_val: 0x1, test_ctl_hi_mask: 0x1,
};

static mut video_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x42c,
    config: unsafe { &mut video_pll0_config },
    vco_table: &video_cc_pll0_vco,
    num_vco: ARRAY_SIZE(&video_cc_pll0_vco),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw_init_data {
        name: "video_pll0", parent_data: clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_slew_ops,
    }},
};

static video_cc_parent_map_0: [parent_map; 1] = [parent_map { parent: P_SLEEP_CLK, val: 0 }];
static video_cc_parent_data_0_ao: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK }];
static video_cc_parent_map_1: [parent_map; 4] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_VIDEO_PLL0_OUT_MAIN, val: 1 },
    parent_map { parent: P_VIDEO_PLL0_OUT_AUX, val: 2 }, parent_map { parent: P_VIDEO_PLL0_OUT_AUX2, val: 3 },
];
static mut video_cc_parent_data_1: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &video_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &video_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &video_pll0.clkr.hw } },
];

static ftbl_video_cc_sleep_clk_src: [freq_tbl; 2] = [F(32000, P_SLEEP_CLK, 1, 0, 0), freq_tbl {}];
static mut video_cc_sleep_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0xaf8, mnd_width: 0, hid_width: 5, parent_map: &video_cc_parent_map_0,
    freq_tbl: &ftbl_video_cc_sleep_clk_src, clkr: clk_regmap { hw: clk_hw_init_data {
        name: "video_cc_sleep_clk_src", parent_data: &video_cc_parent_data_0_ao,
        num_parents: ARRAY_SIZE(&video_cc_parent_data_0_ao), ops: &clk_rcg2_ops,
    }},
};
static ftbl_video_cc_venus_clk_src: [freq_tbl; 8] = [
    F(19200000, P_BI_TCXO, 1, 0, 0), F(133333333, P_VIDEO_PLL0_OUT_MAIN, 4.5, 0, 0),
    F(240000000, P_VIDEO_PLL0_OUT_MAIN, 2.5, 0, 0), F(300000000, P_VIDEO_PLL0_OUT_MAIN, 2, 0, 0),
    F(380000000, P_VIDEO_PLL0_OUT_MAIN, 2, 0, 0), F(410000000, P_VIDEO_PLL0_OUT_MAIN, 2, 0, 0),
    F(460000000, P_VIDEO_PLL0_OUT_MAIN, 2, 0, 0), freq_tbl {},
];
static mut video_cc_venus_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x7f0, mnd_width: 0, hid_width: 5, parent_map: &video_cc_parent_map_1,
    freq_tbl: &ftbl_video_cc_venus_clk_src, clkr: clk_regmap { hw: clk_hw_init_data {
        name: "video_cc_venus_clk_src", parent_data: unsafe { &video_cc_parent_data_1 },
        num_parents: ARRAY_SIZE(&video_cc_parent_data_1), flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops,
    }},
};

macro_rules! branch { ($name:ident, $halt:expr, $check:expr, $parent:expr, $flags:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $halt, halt_check: $check,
        clkr: clk_regmap { enable_reg: $halt, enable_mask: BIT(0), hw: clk_hw_init_data {
            name: stringify!($name), parent_hws: $parent, num_parents: 1, flags: $flags, ops: &clk_branch2_ops,
        }},
    };
} }
branch!(video_cc_sleep_clk, 0xb18, BRANCH_HALT, unsafe { Some(&video_cc_sleep_clk_src.clkr.hw) }, CLK_SET_RATE_PARENT);
branch!(video_cc_vcodec0_axi_clk, 0x8f0, BRANCH_HALT, None, 0);
branch!(video_cc_vcodec0_core_clk, 0x890, BRANCH_HALT_VOTED, unsafe { Some(&video_cc_venus_clk_src.clkr.hw) }, CLK_SET_RATE_PARENT);
branch!(video_cc_venus_ahb_clk, 0x9b0, BRANCH_HALT, None, 0);
branch!(video_cc_venus_ctl_axi_clk, 0x8d0, BRANCH_HALT, None, 0);
branch!(video_cc_venus_ctl_core_clk, 0x850, BRANCH_HALT, unsafe { Some(&video_cc_venus_clk_src.clkr.hw) }, CLK_SET_RATE_PARENT);

static mut vcodec0_gdsc: gdsc = gdsc { gdscr: 0x874, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: power_domain { name: "vcodec0_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL_TRIGGER | POLL_CFG_GDSCR };
static mut venus_gdsc: gdsc = gdsc { gdscr: 0x814, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x6, pd: power_domain { name: "venus_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR };

static mut video_cc_qcs615_clocks: [*mut clk_regmap; 9] = [
    &mut video_cc_sleep_clk.clkr, &mut video_cc_sleep_clk_src.clkr, &mut video_cc_vcodec0_axi_clk.clkr,
    &mut video_cc_vcodec0_core_clk.clkr, &mut video_cc_venus_ahb_clk.clkr, &mut video_cc_venus_clk_src.clkr,
    &mut video_cc_venus_ctl_axi_clk.clkr, &mut video_cc_venus_ctl_core_clk.clkr, &mut video_pll0.clkr,
];
static mut video_cc_qcs615_gdscs: [*mut gdsc; 2] = [&mut vcodec0_gdsc, &mut venus_gdsc];
static video_cc_qcs615_resets: [qcom_reset_map; 3] = [qcom_reset_map { reg: 0x8b0 }, qcom_reset_map { reg: 0x870 }, qcom_reset_map { reg: 0x810 }];
static mut video_cc_qcs615_plls: [*mut clk_alpha_pll; 1] = [&mut video_pll0];
static video_cc_qcs615_critical_cbcrs: [u32; 1] = [0xab8]; /* VIDEO_CC_XO_CLK */
static video_cc_qcs615_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xb94, fast_io: true };
static video_cc_qcs615_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &video_cc_qcs615_plls, num_alpha_plls: ARRAY_SIZE(&video_cc_qcs615_plls), clk_cbcrs: &video_cc_qcs615_critical_cbcrs, num_clk_cbcrs: ARRAY_SIZE(&video_cc_qcs615_critical_cbcrs) };
static video_cc_qcs615_desc: qcom_cc_desc = qcom_cc_desc { config: &video_cc_qcs615_regmap_config, clks: &video_cc_qcs615_clocks, num_clks: ARRAY_SIZE(&video_cc_qcs615_clocks), resets: &video_cc_qcs615_resets, num_resets: ARRAY_SIZE(&video_cc_qcs615_resets), gdscs: &video_cc_qcs615_gdscs, num_gdscs: ARRAY_SIZE(&video_cc_qcs615_gdscs), driver_data: &video_cc_qcs615_driver_data };
static video_cc_qcs615_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,qcs615-videocc" }, of_device_id {}];

fn video_cc_qcs615_probe(pdev: *mut platform_device) -> i32 { unsafe { qcom_cc_probe(pdev, &video_cc_qcs615_desc) } }
static mut video_cc_qcs615_driver: platform_driver = platform_driver { probe: Some(video_cc_qcs615_probe), driver: driver { name: "videocc-qcs615", of_match_table: &video_cc_qcs615_match_table } };

module_platform_driver!(video_cc_qcs615_driver);
MODULE_DEVICE_TABLE!(of, video_cc_qcs615_match_table);
MODULE_DESCRIPTION!("QTI VIDEOCC QCS615 Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
