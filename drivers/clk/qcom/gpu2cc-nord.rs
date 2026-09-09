// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const DT_BI_TCXO: usize = 0;
const DT_GPLL0_OUT_MAIN: usize = 1;
const DT_GPLL0_OUT_MAIN_DIV: usize = 2;

const P_BI_TCXO: usize = 0;
const P_GPLL0_OUT_MAIN: usize = 1;
const P_GPLL0_OUT_MAIN_DIV: usize = 2;
const P_GPU_2_CC_PLL0_OUT_MAIN: usize = 3;
const P_GPU_2_CC_PLL1_OUT_MAIN: usize = 4;

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];

/* 934.0 MHz Configuration */
static gpu_2_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x30, alpha: 0xa555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 0x00400005,
};

static mut gpu_2_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &gpu_2_cc_pll0_config, vco_table: lucid_ole_vco.as_ptr(),
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_2_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

/* 1100.0 MHz Configuration */
static gpu_2_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x39, alpha: 0x4aaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 0x00400005,
};

static mut gpu_2_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, config: &gpu_2_cc_pll1_config, vco_table: lucid_ole_vco.as_ptr(),
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_2_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static gpu_2_cc_parent_map_0: [parent_map; 3] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_2_cc_parent_data_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_2_cc_parent_map_1: [parent_map; 5] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPU_2_CC_PLL0_OUT_MAIN, val: 1 },
    parent_map { parent: P_GPU_2_CC_PLL1_OUT_MAIN, val: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_2_cc_parent_data_1: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_2_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_2_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_2_cc_parent_map_2: [parent_map; 4] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPU_2_CC_PLL1_OUT_MAIN, val: 3 },
    parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_2_cc_parent_data_2: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_2_cc_pll1.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];

static ftbl_gpu_2_cc_ff_clk_src: [freq_tbl; 2] = [ F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl::default() ];
static ftbl_gpu_2_cc_gmu_clk_src: [freq_tbl; 2] = [ F(550000000, P_GPU_2_CC_PLL1_OUT_MAIN, 2, 0, 0), freq_tbl::default() ];

// The following clock, power-domain, reset, descriptor, and driver objects preserve
// the source layout and values; their kernel types and helper macros are external.
static gpu_2_cc_ff_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x91c4, mnd_width: 0, hid_width: 5, parent_map: gpu_2_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_2_cc_ff_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::with_init("gpu_2_cc_ff_clk_src", gpu_2_cc_parent_data_0.as_ptr(), 3, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };
static gpu_2_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9174, mnd_width: 0, hid_width: 5, parent_map: gpu_2_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_2_cc_gmu_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::with_init("gpu_2_cc_gmu_clk_src", gpu_2_cc_parent_data_1.as_ptr(), 5, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };
static gpu_2_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x91a8, mnd_width: 0, hid_width: 5, parent_map: gpu_2_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_gpu_2_cc_ff_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::with_init("gpu_2_cc_hub_clk_src", gpu_2_cc_parent_data_2.as_ptr(), 4, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };

// Branch declarations retain each original register, halt mode, name, parent, and ops.
macro_rules! branch { ($n:ident, $r:expr, $h:expr, $ops:expr) => { static $n: clk_branch = clk_branch::new($r, $h, stringify!($n), $ops); }; }
branch!(gpu_2_cc_ahb_clk, 0x90cc, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_2_cc_crc_ahb_clk, 0x90d0, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_2_cc_cx_accu_shift_clk, 0x9114, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_2_cc_cx_ff_clk, 0x9100, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_2_cc_cx_gmu_clk, 0x90e8, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_2_cc_cxo_clk, 0x90f8, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_2_cc_freq_measure_clk, 0x9008, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_2_cc_gpu_smmu_vote_clk, 0x7000, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_2_cc_hub_aon_clk, 0x91a4, BRANCH_HALT, &clk_branch2_aon_ops);
branch!(gpu_2_cc_hub_cx_int_clk, 0x90fc, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_2_cc_memnoc_gfx_clk, 0x9104, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_2_cc_mnd1x_0_gfx3d_clk, 0x9164, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_2_cc_mnd1x_1_gfx3d_clk, 0x9168, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_2_cc_sleep_clk, 0x90e0, BRANCH_HALT_SKIP, &clk_branch2_ops);

static gpu_2_cc_cx_gdsc: gdsc = gdsc { gdscr: 0x9090, gds_hw_ctrl: 0x90a4, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd: genpd { name: "gpu_2_cc_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE | RETAIN_FF_ENABLE };
static gpu_2_cc_gx_gdsc: gdsc = gdsc { gdscr: 0x9034, clamp_io_ctrl: 0x9504, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd: genpd { name: "gpu_2_cc_gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable) }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | POLL_CFG_GDSCR | RETAIN_FF_ENABLE };

static gpu_2_cc_nord_clocks: [*mut clk_regmap; 19] = [
    &gpu_2_cc_ahb_clk.clkr, &gpu_2_cc_crc_ahb_clk.clkr, &gpu_2_cc_cx_accu_shift_clk.clkr, &gpu_2_cc_cx_ff_clk.clkr, &gpu_2_cc_cx_gmu_clk.clkr, &gpu_2_cc_cxo_clk.clkr, &gpu_2_cc_ff_clk_src.clkr, &gpu_2_cc_freq_measure_clk.clkr, &gpu_2_cc_gmu_clk_src.clkr, &gpu_2_cc_gpu_smmu_vote_clk.clkr, &gpu_2_cc_hub_aon_clk.clkr, &gpu_2_cc_hub_clk_src.clkr, &gpu_2_cc_hub_cx_int_clk.clkr, &gpu_2_cc_memnoc_gfx_clk.clkr, &gpu_2_cc_mnd1x_0_gfx3d_clk.clkr, &gpu_2_cc_mnd1x_1_gfx3d_clk.clkr, &gpu_2_cc_pll0.clkr, &gpu_2_cc_pll1.clkr, &gpu_2_cc_sleep_clk.clkr,
];
static gpu_2_cc_nord_gdscs: [*mut gdsc; 2] = [&gpu_2_cc_cx_gdsc, &gpu_2_cc_gx_gdsc];
static gpu_2_cc_nord_resets: [qcom_reset_map; 9] = [qcom_reset_map { reg: 0x918c }, qcom_reset_map { reg: 0x9198 }, qcom_reset_map { reg: 0x908c }, qcom_reset_map { reg: 0x91a0 }, qcom_reset_map { reg: 0x91c0 }, qcom_reset_map { reg: 0x9118 }, qcom_reset_map { reg: 0x9170 }, qcom_reset_map { reg: 0x9030 }, qcom_reset_map { reg: 0x9000 }];
static gpu_2_cc_nord_plls: [*mut clk_alpha_pll; 2] = [&gpu_2_cc_pll0, &gpu_2_cc_pll1];
static gpu_2_cc_nord_critical_cbcrs: [u32; 2] = [0x9004, 0x900c];
static gpu_2_cc_nord_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9ff0, fast_io: true };
static gpu_2_cc_nord_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: gpu_2_cc_nord_plls.as_ptr(), num_alpha_plls: 2, clk_cbcrs: gpu_2_cc_nord_critical_cbcrs.as_ptr(), num_clk_cbcrs: 2 };
static gpu_2_cc_nord_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_2_cc_nord_regmap_config, clks: gpu_2_cc_nord_clocks.as_ptr(), num_clks: 19, resets: gpu_2_cc_nord_resets.as_ptr(), num_resets: 9, gdscs: gpu_2_cc_nord_gdscs.as_ptr(), num_gdscs: 2, use_rpm: true, driver_data: &gpu_2_cc_nord_driver_data };
static gpu_2_cc_nord_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,nord-gpu2cc" }, of_device_id::empty()];

unsafe fn gpu_2_cc_nord_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &gpu_2_cc_nord_desc) }
static gpu_2_cc_nord_driver: platform_driver = platform_driver::new("gpu2cc-nord", gpu_2_cc_nord_probe, &gpu_2_cc_nord_match_table);
module_platform_driver!(gpu_2_cc_nord_driver);
module_description!("QTI GPU2CC Nord Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
