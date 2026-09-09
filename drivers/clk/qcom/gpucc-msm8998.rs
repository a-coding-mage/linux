// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019, Jeffrey Hugo
 */

// Linux and Qualcomm clock-driver dependencies are supplied externally.

enum {
    P_XO,
    P_GPLL0,
    P_GPUPLL0_OUT_EVEN,
}

/* Instead of going directly to the block, XO is routed through this branch */
static mut gpucc_cxo_clk: clk_branch = clk_branch {
    halt_reg: 0x1020,
    clkr: clk_regmap { enable_reg: 0x1020, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "gpucc_cxo_clk", parent_data: &[clk_parent_data { fw_name: Some("xo"), ..Default::default() }],
        num_parents: 1, ops: &clk_branch2_ops, flags: CLK_IS_CRITICAL,
    } } },
};

static fabia_vco: [pll_vco; 2] = [
    pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 },
    pll_vco { min_freq: 125000000, max_freq: 1000000000, val: 1 },
];

static post_div_table_fabia_even: [clk_div_table; 5] = [
    clk_div_table { val: 0x0, div: 1 }, clk_div_table { val: 0x1, div: 2 },
    clk_div_table { val: 0x3, div: 4 }, clk_div_table { val: 0x7, div: 8 },
    clk_div_table { val: 0, div: 0 },
];

static mut gpupll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA], vco_table: &fabia_vco,
    num_vco: fabia_vco.len(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpupll0", parent_hws: &[], num_parents: 1, ops: &clk_alpha_pll_fabia_ops,
    } } },
};

static mut gpupll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 8, post_div_table: &post_div_table_fabia_even,
    num_post_div: post_div_table_fabia_even.len(), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA], clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpupll0_out_even", parent_hws: &[], num_parents: 1,
        flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_fabia_ops,
    } } },
};

static gpu_xo_gpll0_map: [parent_map; 2] = [parent_map { parent: P_XO, val: 0 }, parent_map { parent: P_GPLL0, val: 5 }];
static gpu_xo_gpll0: [clk_parent_data; 2] = [
    clk_parent_data { hw: unsafe { Some(&mut gpucc_cxo_clk.clkr.hw) }, ..Default::default() },
    clk_parent_data { fw_name: Some("gpll0"), name: Some("gcc_gpu_gpll0_clk"), ..Default::default() },
];
static gpu_xo_gpupll0_map: [parent_map; 2] = [parent_map { parent: P_XO, val: 0 }, parent_map { parent: P_GPUPLL0_OUT_EVEN, val: 1 }];
static gpu_xo_gpupll0: [*const clk_hw; 2] = [
    unsafe { &gpucc_cxo_clk.clkr.hw }, unsafe { &gpupll0_out_even.clkr.hw },
];

static ftbl_rbcpr_clk_src: [freq_tbl; 3] = [F(19200000, P_XO, 1, 0, 0), F(50000000, P_GPLL0, 12, 0, 0), freq_tbl::EMPTY];
static mut rbcpr_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1030, hid_width: 5, parent_map: &gpu_xo_gpll0_map, freq_tbl: &ftbl_rbcpr_clk_src, clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "rbcpr_clk_src", parent_data: &gpu_xo_gpll0, num_parents: 2, ops: &clk_rcg2_ops } } } };

static ftbl_gfx3d_clk_src: [freq_tbl; 2] = [freq_tbl { src: P_GPUPLL0_OUT_EVEN, pre_div: 3, ..Default::default() }, freq_tbl::EMPTY];
static mut gfx3d_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1070, hid_width: 5, parent_map: &gpu_xo_gpupll0_map, freq_tbl: &ftbl_gfx3d_clk_src, clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gfx3d_clk_src", parent_hws: &gpu_xo_gpupll0, num_parents: 2, ops: &clk_rcg2_ops, flags: CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE } } } };

static ftbl_rbbmtimer_clk_src: [freq_tbl; 2] = [F(19200000, P_XO, 1, 0, 0), freq_tbl::EMPTY];
static mut rbbmtimer_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x10b0, hid_width: 5, parent_map: &gpu_xo_gpll0_map, freq_tbl: &ftbl_rbbmtimer_clk_src, clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "rbbmtimer_clk_src", parent_data: &gpu_xo_gpll0, num_parents: 2, ops: &clk_rcg2_ops } } } };

static ftbl_gfx3d_isense_clk_src: [freq_tbl; 5] = [F(19200000, P_XO, 1, 0, 0), F(40000000, P_GPLL0, 15, 0, 0), F(200000000, P_GPLL0, 3, 0, 0), F(300000000, P_GPLL0, 2, 0, 0), freq_tbl::EMPTY];
static mut gfx3d_isense_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1100, hid_width: 5, parent_map: &gpu_xo_gpll0_map, freq_tbl: &ftbl_gfx3d_isense_clk_src, clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gfx3d_isense_clk_src", parent_data: &gpu_xo_gpll0, num_parents: 2, ops: &clk_rcg2_ops } } } };

macro_rules! branch { ($name:ident, $reg:expr, $src:ident, $flags:expr) => { static mut $name: clk_branch = clk_branch { halt_reg: $reg, clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: stringify!($name), parent_hws: &[], num_parents: 1, ops: &clk_branch2_ops, flags: $flags } } } }; } }
branch!(rbcpr_clk, 0x1054, rbcpr_clk_src, CLK_SET_RATE_PARENT);
branch!(gfx3d_clk, 0x1098, gfx3d_clk_src, CLK_SET_RATE_PARENT);
branch!(rbbmtimer_clk, 0x10d0, rbbmtimer_clk_src, CLK_SET_RATE_PARENT);
branch!(gfx3d_isense_clk, 0x1124, gfx3d_isense_clk_src, 0);

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x1004, gds_hw_ctrl: 0x1008, pd: generic_pm_domain { name: "gpu_cx" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x1094, clamp_io_ctrl: 0x130, resets: &[GPU_GX_BCR], reset_count: 1, cxcs: &[0x1098], cxc_count: 1, pd: generic_pm_domain { name: "gpu_gx" }, parent: unsafe { Some(&mut gpu_cx_gdsc.pd) }, pwrsts: PWRSTS_OFF_ON | PWRSTS_RET, flags: CLAMP_IO | SW_RESET | AON_RESET | NO_RET_PERIPH };

static mut gpucc_msm8998_clocks: [*mut clk_regmap; 11] = [
    unsafe { &mut gpupll0.clkr }, unsafe { &mut gpupll0_out_even.clkr }, unsafe { &mut rbcpr_clk_src.clkr }, unsafe { &mut gfx3d_clk_src.clkr }, unsafe { &mut rbbmtimer_clk_src.clkr }, unsafe { &mut gfx3d_isense_clk_src.clkr }, unsafe { &mut rbcpr_clk.clkr }, unsafe { &mut gfx3d_clk.clkr }, unsafe { &mut rbbmtimer_clk.clkr }, unsafe { &mut gfx3d_isense_clk.clkr }, unsafe { &mut gpucc_cxo_clk.clkr },
];
static mut gpucc_msm8998_gdscs: [*mut gdsc; 2] = [unsafe { &mut gpu_cx_gdsc }, unsafe { &mut gpu_gx_gdsc }];
static gpucc_msm8998_resets: [qcom_reset_map; 4] = [qcom_reset_map { reg: 0x1000 }, qcom_reset_map { reg: 0x1050 }, qcom_reset_map { reg: 0x1090 }, qcom_reset_map { reg: 0x1120 }];
static gpucc_msm8998_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9000, fast_io: true };
static gpucc_msm8998_desc: qcom_cc_desc = qcom_cc_desc { config: &gpucc_msm8998_regmap_config, clks: &gpucc_msm8998_clocks, num_clks: 11, resets: &gpucc_msm8998_resets, num_resets: 4, gdscs: &gpucc_msm8998_gdscs, num_gdscs: 2 };

static gpucc_msm8998_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,msm8998-gpucc" }, of_device_id::EMPTY];

unsafe fn gpucc_msm8998_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpucc_msm8998_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    /* force periph logic on to avoid perf counter corruption */
    regmap_write_bits(regmap, gfx3d_clk.clkr.enable_reg, BIT(13), BIT(13));
    /* tweak droop detector (GPUCC_GPU_DD_WRAP_CTRL) to reduce leakage */
    regmap_write_bits(regmap, gfx3d_clk.clkr.enable_reg, BIT(0), BIT(0));
    qcom_cc_really_probe(&(*pdev).dev, &gpucc_msm8998_desc, regmap)
}

static mut gpucc_msm8998_driver: platform_driver = platform_driver { probe: Some(gpucc_msm8998_probe), driver: driver { name: "gpucc-msm8998", of_match_table: &gpucc_msm8998_match_table } };
module_platform_driver!(gpucc_msm8998_driver);
MODULE_DESCRIPTION!("QCOM GPUCC MSM8998 Driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
