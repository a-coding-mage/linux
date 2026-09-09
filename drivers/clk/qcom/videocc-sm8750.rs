// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// External Linux/QCOM clock-provider, platform, regmap, device-tree, and
// module symbols are supplied by the surrounding translation unit.

extern "C" {
    static clk_alpha_pll_regs: [*const clk_regmap; 16];
    static clk_alpha_pll_taycan_elu_ops: clk_ops;
    static clk_rcg2_ops: clk_ops;
    static clk_rcg2_shared_ops: clk_ops;
    static clk_regmap_div_ro_ops: clk_ops;
    static clk_branch2_ops: clk_ops;
    static clk_branch2_mem_ops: clk_ops;
    fn regmap_update_bits(regmap: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

const DT_BI_TCXO: usize = 0;
const DT_BI_TCXO_AO: usize = 1;
const DT_SLEEP_CLK: usize = 2;

const P_BI_TCXO: usize = 0;
const P_SLEEP_CLK: usize = 1;
const P_VIDEO_CC_PLL0_OUT_MAIN: usize = 2;

static taycan_elu_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

static video_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x25, alpha: 0x8000, config_ctl_val: 0x19660387,
    config_ctl_hi_val: 0x098060a0, config_ctl_hi1_val: 0xb416cb20,
    user_ctl_val: 0x00000000, user_ctl_hi_val: 0x00000002,
};

static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &video_cc_pll0_config, vco_table: &taycan_elu_vco,
    num_vco: 1, regs: core::ptr::null(), clkr: clk_regmap { hw: clk_hw_init { init: core::ptr::null() } },
};

static video_cc_parent_map_0: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];
static video_cc_parent_data_0_ao: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO_AO, hw: core::ptr::null() }];
static video_cc_parent_map_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];
static video_cc_parent_data_1: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() }, clk_parent_data { index: 0, hw: unsafe { &video_cc_pll0.clkr.hw } }];
static video_cc_parent_map_2: [parent_map; 1] = [parent_map { src: P_SLEEP_CLK, cfg: 0 }];
static video_cc_parent_data_2_ao: [clk_parent_data; 1] = [clk_parent_data { index: DT_SLEEP_CLK, hw: core::ptr::null() }];

static ftbl_video_cc_ahb_clk_src: [freq_tbl; 2] = [F(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl::ZERO];
static ftbl_video_cc_mvs0_clk_src: [freq_tbl; 8] = [
    F(720000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1014000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1260000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1332000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1600000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), F(1710000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0),
    F(1890000000, P_VIDEO_CC_PLL0_OUT_MAIN, 1, 0, 0), freq_tbl::ZERO,
];
static ftbl_video_cc_sleep_clk_src: [freq_tbl; 2] = [F(32000, P_SLEEP_CLK, 1, 0, 0), freq_tbl::ZERO];

// Clock, divider, branch, memory-branch, and GDSC objects retain the exact
// register values and parent relationships from the C implementation.
static mut video_cc_ahb_clk_src: clk_rcg2 = clk_rcg2::new(0x8018, 0, 5, &video_cc_parent_map_0, &ftbl_video_cc_ahb_clk_src, "video_cc_ahb_clk_src", &video_cc_parent_data_0_ao, &clk_rcg2_ops);
static mut video_cc_mvs0_clk_src: clk_rcg2 = clk_rcg2::new(0x8000, 0, 5, &video_cc_parent_map_1, &ftbl_video_cc_mvs0_clk_src, "video_cc_mvs0_clk_src", &video_cc_parent_data_1, &clk_rcg2_shared_ops);
static mut video_cc_sleep_clk_src: clk_rcg2 = clk_rcg2::new(0x80e0, 0, 5, &video_cc_parent_map_2, &ftbl_video_cc_sleep_clk_src, "video_cc_sleep_clk_src", &video_cc_parent_data_2_ao, &clk_rcg2_ops);
static mut video_cc_xo_clk_src: clk_rcg2 = clk_rcg2::new(0x80bc, 0, 5, &video_cc_parent_map_0, &ftbl_video_cc_ahb_clk_src, "video_cc_xo_clk_src", &video_cc_parent_data_0_ao, &clk_rcg2_ops);

static mut video_cc_mvs0_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x809c, 0, 4, "video_cc_mvs0_div_clk_src", &clk_regmap_div_ro_ops);
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x8060, 0, 4, "video_cc_mvs0c_div2_div_clk_src", &clk_regmap_div_ro_ops);

static mut video_cc_mvs0_clk: clk_branch = clk_branch::new(0x807c, BRANCH_HALT_VOTED, 0x807c, 1, 0, "video_cc_mvs0_clk", &clk_branch2_ops);
static mut video_cc_mvs0_freerun_clk: clk_mem_branch = clk_mem_branch::new(0x8090, 0x8090, BIT(3), GENMASK(11, 10), true, 0x808c, BRANCH_HALT, "video_cc_mvs0_freerun_clk", &clk_branch2_mem_ops);
static mut video_cc_mvs0_shift_clk: clk_branch = clk_branch::new(0x80d8, BRANCH_HALT_VOTED, 0x80d8, 1, 0, "video_cc_mvs0_shift_clk", &clk_branch2_ops);
static mut video_cc_mvs0c_clk: clk_branch = clk_branch::new(0x804c, BRANCH_HALT, 0, 0, 0, "video_cc_mvs0c_clk", &clk_branch2_ops);
static mut video_cc_mvs0c_freerun_clk: clk_branch = clk_branch::new(0x805c, BRANCH_HALT, 0, 0, 0, "video_cc_mvs0c_freerun_clk", &clk_branch2_ops);
static mut video_cc_mvs0c_shift_clk: clk_branch = clk_branch::new(0x80dc, BRANCH_HALT_VOTED, 0x80dc, 1, 0, "video_cc_mvs0c_shift_clk", &clk_branch2_ops);

static mut video_cc_mvs0c_gdsc: gdsc = gdsc::new(0x8034, 0x2, 0x2, 0x6, "video_cc_mvs0c_gdsc", PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE);
static mut video_cc_mvs0_gdsc: gdsc = gdsc::new_with_parent(0x8068, 0x2, 0x2, 0x6, "video_cc_mvs0_gdsc", unsafe { &video_cc_mvs0c_gdsc.pd }, PWRSTS_OFF_ON, POLL_CFG_GDSCR | RETAIN_FF_ENABLE | HW_CTRL_TRIGGER);

static mut video_cc_sm8750_clocks: [*mut clk_regmap; 13] = [
    &mut video_cc_ahb_clk_src.clkr, &mut video_cc_mvs0_clk.clkr, &mut video_cc_mvs0_clk_src.clkr,
    &mut video_cc_mvs0_div_clk_src.clkr, &mut video_cc_mvs0_freerun_clk.branch.clkr,
    &mut video_cc_mvs0_shift_clk.clkr, &mut video_cc_mvs0c_clk.clkr, &mut video_cc_mvs0c_div2_div_clk_src.clkr,
    &mut video_cc_mvs0c_freerun_clk.clkr, &mut video_cc_mvs0c_shift_clk.clkr, &mut video_cc_pll0.clkr,
    &mut video_cc_sleep_clk_src.clkr, &mut video_cc_xo_clk_src.clkr,
];
static mut video_cc_sm8750_gdscs: [*mut gdsc; 2] = [&mut video_cc_mvs0_gdsc, &mut video_cc_mvs0c_gdsc];
static video_cc_sm8750_resets: [qcom_reset_map; 7] = [qcom_reset_map { reg: 0x80a0, bit: 0 }, qcom_reset_map { reg: 0x8064, bit: 0 }, qcom_reset_map { reg: 0x804c, bit: 2 }, qcom_reset_map { reg: 0x8030, bit: 0 }, qcom_reset_map { reg: 0x808c, bit: 2 }, qcom_reset_map { reg: 0x805c, bit: 2 }, qcom_reset_map { reg: 0x80d4, bit: 2 }];
static video_cc_sm8750_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9f4c, fast_io: true };
static mut video_cc_sm8750_plls: [*mut clk_alpha_pll; 1] = [&mut video_cc_pll0];
static video_cc_sm8750_critical_cbcrs: [u32; 3] = [0x80a4, 0x80f8, 0x80d4];

unsafe fn clk_sm8750_regs_configure(_dev: *mut device, regmap: *mut regmap) {
    // Update DLY_ACCU_RED_SHIFTER_DONE to 0xF for mvs0, mvs0c
    regmap_update_bits(regmap, 0x8074, GENMASK(25, 21), GENMASK(25, 21));
    regmap_update_bits(regmap, 0x8040, GENMASK(25, 21), GENMASK(25, 21));
    regmap_update_bits(regmap, 0x9f24, BIT(0), BIT(0));
}

static video_cc_sm8750_driver_data: qcom_cc_driver_data = qcom_cc_driver_data::new(&video_cc_sm8750_plls, &video_cc_sm8750_critical_cbcrs, clk_sm8750_regs_configure);
static video_cc_sm8750_desc: qcom_cc_desc = qcom_cc_desc::new(&video_cc_sm8750_regmap_config, &video_cc_sm8750_clocks, &video_cc_sm8750_resets, &video_cc_sm8750_gdscs, true, &video_cc_sm8750_driver_data);

static video_cc_sm8750_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,sm8750-videocc" }, of_device_id::ZERO];

unsafe extern "C" fn video_cc_sm8750_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &video_cc_sm8750_desc) }
static mut video_cc_sm8750_driver: platform_driver = platform_driver::new(video_cc_sm8750_probe, "video_cc-sm8750", &video_cc_sm8750_match_table);

unsafe extern "C" fn video_cc_sm8750_init() -> i32 { platform_driver_register(&mut video_cc_sm8750_driver) }
unsafe extern "C" fn video_cc_sm8750_exit() { platform_driver_unregister(&mut video_cc_sm8750_driver); }

// subsys_initcall(video_cc_sm8750_init);
// module_exit(video_cc_sm8750_exit);
// MODULE_DESCRIPTION("QTI VIDEO_CC SM8750 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
