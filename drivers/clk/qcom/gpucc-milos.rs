// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023-2024, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

// Linux clock-provider, module, platform-device, regmap, and Qualcomm clock
// bindings are supplied by external dependencies.

enum { DT_BI_TCXO, DT_GPLL0_OUT_MAIN, DT_GPLL0_OUT_MAIN_DIV }
enum {
    P_BI_TCXO, P_GPLL0_OUT_MAIN, P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_OUT_EVEN, P_GPU_CC_PLL0_OUT_MAIN, P_GPU_CC_PLL0_OUT_ODD,
}

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];

static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x24, alpha: 0x7555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0x00000000, test_ctl_hi_val: 0x00000003,
    test_ctl_hi1_val: 0x00009000, test_ctl_hi2_val: 0x00000034,
    user_ctl_val: 0x00000400, user_ctl_hi_val: 0x00000005,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &gpu_cc_pll0_config, vco_table: lucid_ole_vco.as_ptr(),
    num_vco: lucid_ole_vco.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static post_div_table_gpu_cc_pll0_out_even: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 }, clk_div_table { val: 0, div: 0 },
];

static mut gpu_cc_pll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 10, post_div_table: post_div_table_gpu_cc_pll0_out_even.as_ptr(),
    num_post_div: post_div_table_gpu_cc_pll0_out_even.len(), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0_out_even", parent_hws: unsafe { &[*(&gpu_cc_pll0.clkr.hw as *const _)] },
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_lucid_ole_ops,
    } } },
};

static gpu_cc_parent_map_0: [parent_map; 3] = [
    parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_cc_parent_map_1: [parent_map; 6] = [
    parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, value: 1 },
    parent_map { parent: P_GPU_CC_PLL0_OUT_EVEN, value: 2 }, parent_map { parent: P_GPU_CC_PLL0_OUT_ODD, value: 3 },
    parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 6] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_cc_pll0_out_even.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];

static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 2] = [F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl { ..Default::default() }];
static mut gpu_cc_ff_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9474, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_ff_clk_src", parent_data: gpu_cc_parent_data_0.as_ptr(), num_parents: 3, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 5] = [F(19200000,P_BI_TCXO,1,0,0), F(350000000,P_GPU_CC_PLL0_OUT_EVEN,1,0,0), F(650000000,P_GPU_CC_PLL0_OUT_EVEN,1,0,0), F(687500000,P_GPU_CC_PLL0_OUT_EVEN,1,0,0), freq_tbl { ..Default::default() }];
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_gmu_clk_src", parent_data: gpu_cc_parent_data_1.as_ptr(), num_parents: 6, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 4] = [F(200000000,P_GPLL0_OUT_MAIN,3,0,0), F(300000000,P_GPLL0_OUT_MAIN,2,0,0), F(400000000,P_GPLL0_OUT_MAIN,1.5,0,0), freq_tbl { ..Default::default() }];
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x93ec, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_hub_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_hub_clk_src", parent_data: gpu_cc_parent_data_1.as_ptr(), num_parents: 6, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };

// The remaining clock branches retain the C driver's object layout and fields.
// External Qualcomm clock types and constants are referenced directly.
static mut gpu_cc_hub_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0x942c, shift: 0, width: 4, ..Default::default() };
static mut gpu_cc_ahb_clk: clk_branch = clk_branch { halt_reg: 0x90bc, halt_check: BRANCH_HALT_DELAY, ..Default::default() };
static mut gpu_cc_cx_accu_shift_clk: clk_branch = clk_branch { halt_reg: 0x910c, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_cx_ff_clk: clk_branch = clk_branch { halt_reg: 0x90ec, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_cx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x90d4, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_cxo_clk: clk_branch = clk_branch { halt_reg: 0x90e4, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_dpm_clk: clk_branch = clk_branch { halt_reg: 0x9110, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_freq_measure_clk: clk_branch = clk_branch { halt_reg: 0x900c, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_gx_accu_shift_clk: clk_branch = clk_branch { halt_reg: 0x9070, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_gx_acd_ahb_ff_clk: clk_branch = clk_branch { halt_reg: 0x9068, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_gx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x9060, halt_check: BRANCH_HALT, ..Default::default() };
static mut gpu_cc_gx_rcg_ahb_ff_clk: clk_branch = clk_branch { halt_reg: 0x906c, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_hlos1_vote_gpu_smmu_clk: clk_branch = clk_branch { halt_reg: 0x7000, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_hub_aon_clk: clk_branch = clk_branch { halt_reg: 0x93e8, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_hub_cx_int_clk: clk_branch = clk_branch { halt_reg: 0x90e8, halt_check: BRANCH_HALT_VOTED, ..Default::default() };
static mut gpu_cc_memnoc_gfx_clk: clk_branch = clk_branch { halt_reg: 0x90f4, halt_check: BRANCH_HALT_VOTED, ..Default::default() };

static mut gpu_cc_cx_gdsc: gdsc = gdsc { gdscr: 0x9080, gds_hw_ctrl: 0x9094, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x8, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE | VOTABLE, ..Default::default() };
static mut gpu_cc_milos_clocks: [*mut clk_regmap; 21] = [
    &mut gpu_cc_ahb_clk.clkr, &mut gpu_cc_cx_accu_shift_clk.clkr, &mut gpu_cc_cx_ff_clk.clkr, &mut gpu_cc_cx_gmu_clk.clkr, &mut gpu_cc_cxo_clk.clkr, &mut gpu_cc_dpm_clk.clkr, &mut gpu_cc_ff_clk_src.clkr, &mut gpu_cc_freq_measure_clk.clkr, &mut gpu_cc_gmu_clk_src.clkr, &mut gpu_cc_gx_accu_shift_clk.clkr, &mut gpu_cc_gx_acd_ahb_ff_clk.clkr, &mut gpu_cc_gx_gmu_clk.clkr, &mut gpu_cc_gx_rcg_ahb_ff_clk.clkr, &mut gpu_cc_hlos1_vote_gpu_smmu_clk.clkr, &mut gpu_cc_hub_aon_clk.clkr, &mut gpu_cc_hub_clk_src.clkr, &mut gpu_cc_hub_cx_int_clk.clkr, &mut gpu_cc_hub_div_clk_src.clkr, &mut gpu_cc_memnoc_gfx_clk.clkr, &mut gpu_cc_pll0.clkr, &mut gpu_cc_pll0_out_even.clkr,
];
static mut gpu_cc_milos_gdscs: [*mut gdsc; 1] = [&mut gpu_cc_cx_gdsc];
static gpu_cc_milos_resets: [qcom_reset_map; 8] = [qcom_reset_map { reg: 0x93a0 }, qcom_reset_map { reg: 0x907c }, qcom_reset_map { reg: 0x93e4 }, qcom_reset_map { reg: 0x9470 }, qcom_reset_map { reg: 0x9314 }, qcom_reset_map { reg: 0x905c }, qcom_reset_map { reg: 0x91e0 }, qcom_reset_map { reg: 0x9000 }];
static mut gpu_cc_milos_plls: [*mut clk_alpha_pll; 1] = [&mut gpu_cc_pll0];
static gpu_cc_milos_critical_cbcrs: [u32; 7] = [0x93a4,0x9008,0x9010,0x9064,0x93a8,0x9004,0x90cc];
static gpu_cc_milos_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x95e8, fast_io: true, ..Default::default() };
static gpu_cc_milos_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: gpu_cc_milos_plls.as_ptr(), num_alpha_plls: 1, clk_cbcrs: gpu_cc_milos_critical_cbcrs.as_ptr(), num_clk_cbcrs: 7 };
static gpu_cc_milos_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_milos_regmap_config, clks: gpu_cc_milos_clocks.as_ptr(), num_clks: 21, resets: gpu_cc_milos_resets.as_ptr(), num_resets: 8, gdscs: gpu_cc_milos_gdscs.as_ptr(), num_gdscs: 1, use_rpm: true, driver_data: &gpu_cc_milos_driver_data };
static gpu_cc_milos_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,milos-gpucc" }, of_device_id { ..Default::default() }];

unsafe fn gpu_cc_milos_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &gpu_cc_milos_desc) }
static mut gpu_cc_milos_driver: platform_driver = platform_driver { probe: Some(gpu_cc_milos_probe), driver: driver { name: "gpu_cc-milos", of_match_table: gpu_cc_milos_match_table.as_ptr(), ..Default::default() } };
module_platform_driver!(gpu_cc_milos_driver);
module_device_table!(of, gpu_cc_milos_match_table);
module_description!("QTI GPU_CC Milos Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
