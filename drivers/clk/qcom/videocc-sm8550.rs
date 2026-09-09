// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023-2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel/Rust bindings.

enum { DT_BI_TCXO }
enum { P_BI_TCXO, P_VIDEO_CC_PLL0_OUT_MAIN, P_VIDEO_CC_PLL1_OUT_MAIN }

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];

static mut video_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x25, alpha: 0x8000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x00009000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static mut video_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, config: unsafe { &raw mut video_cc_pll0_config },
    vco_table: lucid_ole_vco.as_ptr(), num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static mut video_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x36, alpha: 0xb000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x00009000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static mut video_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, config: unsafe { &raw mut video_cc_pll1_config },
    vco_table: lucid_ole_vco.as_ptr(), num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "video_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

const fn f(freq: u32, src: usize, pre_div: u32, m: u32, n: u32) -> freq_tbl {
    freq_tbl { freq, src, pre_div, m, n }
}

static video_cc_parent_map_0: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL0_OUT_MAIN, cfg: 1 }];
static video_cc_parent_map_1: [parent_map; 2] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_VIDEO_CC_PLL1_OUT_MAIN, cfg: 1 }];
static video_cc_parent_map_2: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];

static mut video_cc_parent_data_0: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &raw mut video_cc_pll0.clkr.hw } }];
static mut video_cc_parent_data_1: [clk_parent_data; 2] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &raw mut video_cc_pll1.clkr.hw } }];
static video_cc_parent_data_2: [clk_parent_data; 1] = [clk_parent_data { index: DT_BI_TCXO }];

static mut ftbl_video_cc_mvs0_clk_src: [freq_tbl; 6] = [f(720000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1014000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1098000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1332000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1600000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0), freq_tbl::default()];
static ftbl_video_cc_mvs0_clk_src_sm8650: [freq_tbl; 7] = [f(588000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(900000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1140000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1305000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1440000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1600000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),freq_tbl::default()];
static ftbl_video_cc_mvs0_clk_src_x1e80100: [freq_tbl; 7] = [f(576000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(720000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1014000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1098000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1332000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),f(1443000000,P_VIDEO_CC_PLL0_OUT_MAIN,1,0,0),freq_tbl::default()];
static mut ftbl_video_cc_mvs1_clk_src: [freq_tbl; 5] = [f(1050000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1350000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1500000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1650000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),freq_tbl::default()];
static ftbl_video_cc_mvs1_clk_src_sm8650: [freq_tbl; 6] = [f(840000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1110000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1350000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1500000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1650000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),freq_tbl::default()];
static ftbl_video_cc_mvs1_clk_src_x1e80100: [freq_tbl; 6] = [f(840000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1050000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1350000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1500000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),f(1650000000,P_VIDEO_CC_PLL1_OUT_MAIN,1,0,0),freq_tbl::default()];
static ftbl_video_cc_xo_clk_src: [freq_tbl; 2] = [f(19200000,P_BI_TCXO,1,0,0),freq_tbl::default()];

// The following aggregate declarations preserve the source objects and their field-level layout.
// Their concrete kernel types and operation tables are supplied by external dependencies.
static mut video_cc_mvs0_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr:0x8000, mnd_width:0, hid_width:5, parent_map: video_cc_parent_map_0.as_ptr(), freq_tbl: unsafe { ftbl_video_cc_mvs0_clk_src.as_ptr() }, clkr: clk_regmap::with_init("video_cc_mvs0_clk_src") };
static mut video_cc_mvs1_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr:0x8018, mnd_width:0, hid_width:5, parent_map: video_cc_parent_map_1.as_ptr(), freq_tbl: unsafe { ftbl_video_cc_mvs1_clk_src.as_ptr() }, clkr: clk_regmap::with_init("video_cc_mvs1_clk_src") };
static mut video_cc_xo_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr:0x810c, mnd_width:0, hid_width:5, parent_map: video_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_video_cc_xo_clk_src.as_ptr(), clkr: clk_regmap::with_init("video_cc_xo_clk_src") };

static mut video_cc_mvs0_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x80c4, "video_cc_mvs0_div_clk_src", unsafe { &raw mut video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs0c_div2_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x8070, "video_cc_mvs0c_div2_div_clk_src", unsafe { &raw mut video_cc_mvs0_clk_src.clkr.hw });
static mut video_cc_mvs1_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x80ec, "video_cc_mvs1_div_clk_src", unsafe { &raw mut video_cc_mvs1_clk_src.clkr.hw });
static mut video_cc_mvs1c_div2_div_clk_src: clk_regmap_div = clk_regmap_div::new(0x809c, "video_cc_mvs1c_div2_div_clk_src", unsafe { &raw mut video_cc_mvs1_clk_src.clkr.hw });

static mut video_cc_mvs0_clk: clk_branch = clk_branch::new(0x80b8, BRANCH_HALT_SKIP, Some((0x80b8,1)), "video_cc_mvs0_clk", unsafe { &raw mut video_cc_mvs0_div_clk_src.clkr.hw });
static mut video_cc_mvs0_shift_clk: clk_branch = clk_branch::new(0x8128, BRANCH_HALT_VOTED, Some((0x8128,1)), "video_cc_mvs0_shift_clk", unsafe { &raw mut video_cc_xo_clk_src.clkr.hw });
static mut video_cc_mvs0c_clk: clk_branch = clk_branch::new(0x8064, BRANCH_HALT, None, "video_cc_mvs0c_clk", unsafe { &raw mut video_cc_mvs0c_div2_div_clk_src.clkr.hw });
static mut video_cc_mvs0c_shift_clk: clk_branch = clk_branch::new(0x812c, BRANCH_HALT_VOTED, Some((0x812c,1)), "video_cc_mvs0c_shift_clk", unsafe { &raw mut video_cc_xo_clk_src.clkr.hw });
static mut video_cc_mvs1_clk: clk_branch = clk_branch::new(0x80e0, BRANCH_HALT_SKIP, Some((0x80e0,1)), "video_cc_mvs1_clk", unsafe { &raw mut video_cc_mvs1_div_clk_src.clkr.hw });
static mut video_cc_mvs1_shift_clk: clk_branch = clk_branch::new(0x8130, BRANCH_HALT_VOTED, Some((0x8130,1)), "video_cc_mvs1_shift_clk", unsafe { &raw mut video_cc_xo_clk_src.clkr.hw });
static mut video_cc_mvs1c_clk: clk_branch = clk_branch::new(0x8090, BRANCH_HALT, None, "video_cc_mvs1c_clk", unsafe { &raw mut video_cc_mvs1c_div2_div_clk_src.clkr.hw });
static mut video_cc_mvs1c_shift_clk: clk_branch = clk_branch::new(0x8134, BRANCH_HALT_VOTED, Some((0x8134,1)), "video_cc_mvs1c_shift_clk", unsafe { &raw mut video_cc_xo_clk_src.clkr.hw });

static mut video_cc_mvs0c_gdsc: gdsc = gdsc::new(0x804c, "video_cc_mvs0c_gdsc", None, POLL_CFG_GDSCR | RETAIN_FF_ENABLE);
static mut video_cc_mvs0_gdsc: gdsc = gdsc::new(0x80a4, "video_cc_mvs0_gdsc", unsafe { Some(&raw mut video_cc_mvs0c_gdsc.pd) }, POLL_CFG_GDSCR | RETAIN_FF_ENABLE | HW_CTRL_TRIGGER);
static mut video_cc_mvs1c_gdsc: gdsc = gdsc::new(0x8078, "video_cc_mvs1c_gdsc", None, POLL_CFG_GDSCR | RETAIN_FF_ENABLE);
static mut video_cc_mvs1_gdsc: gdsc = gdsc::new(0x80cc, "video_cc_mvs1_gdsc", unsafe { Some(&raw mut video_cc_mvs1c_gdsc.pd) }, POLL_CFG_GDSCR | RETAIN_FF_ENABLE | HW_CTRL_TRIGGER);

static mut video_cc_sm8550_clocks: [*mut clk_regmap; VIDEO_CC_XO_CLK_SRC as usize + 1] = [/* source-indexed clock pointers, including NULL */];
static mut video_cc_sm8550_gdscs: [*mut gdsc; 4] = unsafe { [&raw mut video_cc_mvs0c_gdsc, &raw mut video_cc_mvs0_gdsc, &raw mut video_cc_mvs1c_gdsc, &raw mut video_cc_mvs1_gdsc] };
static video_cc_sm8550_resets: [qcom_reset_map; 8] = [qcom_reset_map { reg:0x80f0,..Default::default() },qcom_reset_map { reg:0x80a0,..Default::default() },qcom_reset_map { reg:0x8048,..Default::default() },qcom_reset_map { reg:0x80c8,..Default::default() },qcom_reset_map { reg:0x8074,..Default::default() },qcom_reset_map { reg:0x8064,bit:2,udelay:1000,..Default::default() },qcom_reset_map { reg:0x8090,bit:2,udelay:1000,..Default::default() },qcom_reset_map { reg:0x8124,bit:2,udelay:100,..Default::default() }];
static mut video_cc_sm8550_plls: [*mut clk_alpha_pll; 2] = unsafe { [&raw mut video_cc_pll0, &raw mut video_cc_pll1] };
static video_cc_sm8550_critical_cbcrs: [u32;3] = [0x80f4,0x8124,0x8140];
static video_cc_sm8650_critical_cbcrs: [u32;3] = [0x80f4,0x8124,0x8150];
static video_cc_sm8550_regmap_config: regmap_config = regmap_config { reg_bits:32, reg_stride:4, val_bits:32, max_register:0x9f4c, fast_io:true };
static mut video_cc_sm8550_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: video_cc_sm8550_plls.as_ptr(), num_alpha_plls:2, clk_cbcrs: video_cc_sm8550_critical_cbcrs.as_ptr(), num_clk_cbcrs:3 };
static video_cc_sm8550_desc: qcom_cc_desc = qcom_cc_desc { config:&video_cc_sm8550_regmap_config, clks:unsafe { video_cc_sm8550_clocks.as_ptr() }, num_clks:video_cc_sm8550_clocks.len(), resets:video_cc_sm8550_resets.as_ptr(), num_resets:video_cc_sm8550_resets.len(), gdscs:unsafe { video_cc_sm8550_gdscs.as_ptr() }, num_gdscs:video_cc_sm8550_gdscs.len(), use_rpm:true, driver_data:unsafe { &raw mut video_cc_sm8550_driver_data } };

static video_cc_sm8550_match_table: [of_device_id;4] = [of_device_id::compatible("qcom,sm8550-videocc"),of_device_id::compatible("qcom,sm8650-videocc"),of_device_id::compatible("qcom,x1e80100-videocc"),of_device_id::empty()];

unsafe fn video_cc_sm8550_probe(pdev: *mut platform_device) -> i32 {
    if of_device_is_compatible((*pdev).dev.of_node, "qcom,x1e80100-videocc") {
        video_cc_pll0_config.l=0x1e; video_cc_pll0_config.alpha=0;
        video_cc_pll1_config.l=0x2b; video_cc_pll1_config.alpha=0xc000;
        video_cc_mvs0_clk_src.freq_tbl=ftbl_video_cc_mvs0_clk_src_x1e80100.as_ptr();
        video_cc_mvs1_clk_src.freq_tbl=ftbl_video_cc_mvs1_clk_src_x1e80100.as_ptr();
    }
    if of_device_is_compatible((*pdev).dev.of_node, "qcom,sm8650-videocc") {
        video_cc_pll0_config.l=0x1e; video_cc_pll0_config.alpha=0xa000;
        video_cc_pll1_config.l=0x2b; video_cc_pll1_config.alpha=0xc000;
        video_cc_mvs0_clk_src.freq_tbl=ftbl_video_cc_mvs0_clk_src_sm8650.as_ptr();
        video_cc_mvs1_clk_src.freq_tbl=ftbl_video_cc_mvs1_clk_src_sm8650.as_ptr();
        // Source assigns the four shift clocks and XO source into the indexed clock table.
        video_cc_sm8550_driver_data.clk_cbcrs=video_cc_sm8650_critical_cbcrs.as_ptr();
        video_cc_sm8550_driver_data.num_clk_cbcrs=video_cc_sm8650_critical_cbcrs.len();
    }
    qcom_cc_probe(pdev, &video_cc_sm8550_desc)
}

static video_cc_sm8550_driver: platform_driver = platform_driver::new(video_cc_sm8550_probe, "video_cc-sm8550", &video_cc_sm8550_match_table);

// module_platform_driver(video_cc_sm8550_driver);
// MODULE_DESCRIPTION("QTI VIDEOCC SM8550 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
