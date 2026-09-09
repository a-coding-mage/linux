// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Linux kernel dependencies are supplied by the surrounding Rust bindings.

const DT_BI_TCXO: usize = 0;
const DT_GPLL0_OUT_MAIN: usize = 1;
const DT_GPLL0_OUT_MAIN_DIV: usize = 2;

const P_BI_TCXO: usize = 0;
const P_GPLL0_OUT_MAIN: usize = 1;
const P_GPLL0_OUT_MAIN_DIV: usize = 2;
const P_GPU_CC_PLL0_OUT_EVEN: usize = 3;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 4;
const P_GPU_CC_PLL0_OUT_ODD: usize = 5;

static taycan_eko_t_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

/* 950.0 MHz Configuration */
static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x31, cal_l: 0x48, alpha: 0x7aaa,
    config_ctl_val: 0x25c400e7, config_ctl_hi_val: 0x0a8062e0,
    config_ctl_hi1_val: 0xf51dea20, user_ctl_val: 0x00000408,
    user_ctl_hi_val: 0x00000002,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &gpu_cc_pll0_config, vco_table: taycan_eko_t_vco.as_ptr(),
    num_vco: 1, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init_data { } },
};

static post_div_table_gpu_cc_pll0_out_even: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 }, clk_div_table { val: 0, div: 0 },
];

static mut gpu_cc_pll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 10, post_div_table: post_div_table_gpu_cc_pll0_out_even.as_ptr(),
    num_post_div: 2, width: 4, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T], clkr: clk_regmap { hw: clk_hw_init_data { } },
};

static gpu_cc_parent_map_0: [parent_map; 6] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, val: 1 },
    parent_map { parent: P_GPU_CC_PLL0_OUT_EVEN, val: 2 }, parent_map { parent: P_GPU_CC_PLL0_OUT_ODD, val: 3 },
    parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];

static mut gpu_cc_parent_data_0: [clk_parent_data; 6] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_cc_pll0_out_even.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 7] = [
    F(19200000, P_BI_TCXO, 1, 0, 0), F(475000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    F(575000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), F(700000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    F(725000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), F(750000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];
static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 5] = [
    F(150000000, P_GPLL0_OUT_MAIN_DIV, 2, 0, 0), F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0),
    F(300000000, P_GPLL0_OUT_MAIN, 2, 0, 0), F(400000000, P_GPLL0_OUT_MAIN, 1.5, 0, 0),
    freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 },
];

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $ops:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt,
        clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw_init_data { ops: $ops } } };
}; }

static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), hw_clk_ctrl: true, freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw_init_data { } } };
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x93f0, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), hw_clk_ctrl: true, freq_tbl: ftbl_gpu_cc_hub_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw_init_data { } } };
static mut gpu_cc_hub_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0x9430, shift: 0, width: 4, clkr: clk_regmap { hw: clk_hw_init_data { } } };

branch!(gpu_cc_ahb_clk, 0x90bc, BRANCH_HALT_DELAY, &clk_branch2_ops);
branch!(gpu_cc_cx_accu_shift_clk, 0x9104, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_cx_gmu_clk, 0x90d4, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_cc_cxo_clk, 0x90e4, BRANCH_HALT, &clk_branch2_aon_ops);
branch!(gpu_cc_demet_clk, 0x9010, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_dpm_clk, 0x9108, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_freq_measure_clk, 0x900c, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_gpu_smmu_vote_clk, 0x7000, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_gx_accu_shift_clk, 0x9070, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_gx_gmu_clk, 0x9060, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_hub_aon_clk, 0x93ec, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_cc_hub_cx_int_clk, 0x90e8, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_cc_memnoc_gfx_clk, 0x90ec, BRANCH_HALT_VOTED, &clk_branch2_ops);

static mut gpu_cc_cx_gdsc: gdsc = gdsc { gdscr: 0x9080, gds_hw_ctrl: 0x9094, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x8, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE };

static mut gpu_cc_kaanapali_clocks: [*mut clk_regmap; 18] = [
    &mut gpu_cc_ahb_clk.clkr, &mut gpu_cc_cx_accu_shift_clk.clkr, &mut gpu_cc_cx_gmu_clk.clkr, &mut gpu_cc_cxo_clk.clkr,
    &mut gpu_cc_demet_clk.clkr, &mut gpu_cc_dpm_clk.clkr, &mut gpu_cc_freq_measure_clk.clkr, &mut gpu_cc_gmu_clk_src.clkr,
    &mut gpu_cc_gpu_smmu_vote_clk.clkr, &mut gpu_cc_gx_accu_shift_clk.clkr, &mut gpu_cc_gx_gmu_clk.clkr, &mut gpu_cc_hub_aon_clk.clkr,
    &mut gpu_cc_hub_clk_src.clkr, &mut gpu_cc_hub_cx_int_clk.clkr, &mut gpu_cc_hub_div_clk_src.clkr, &mut gpu_cc_memnoc_gfx_clk.clkr,
    &mut gpu_cc_pll0.clkr, &mut gpu_cc_pll0_out_even.clkr,
];
static mut gpu_cc_kaanapali_gdscs: [*mut gdsc; 1] = [&mut gpu_cc_cx_gdsc];
static gpu_cc_kaanapali_resets: [qcom_reset_map; 7] = [qcom_reset_map { reg: 0x93a0 }, qcom_reset_map { reg: 0x907c }, qcom_reset_map { reg: 0x93e4 }, qcom_reset_map { reg: 0x9470 }, qcom_reset_map { reg: 0x9314 }, qcom_reset_map { reg: 0x905c }, qcom_reset_map { reg: 0x9000 }];
static mut gpu_cc_kaanapali_plls: [*mut clk_alpha_pll; 1] = [&mut gpu_cc_pll0];
static gpu_cc_kaanapali_critical_cbcrs: [u32; 3] = [0x9008, 0x93e8, 0x9004];
static gpu_cc_kaanapali_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x95e8, fast_io: true };
static gpu_cc_kaanapali_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: gpu_cc_kaanapali_plls.as_ptr(), num_alpha_plls: 1, clk_cbcrs: gpu_cc_kaanapali_critical_cbcrs.as_ptr(), num_clk_cbcrs: 3 };
static gpu_cc_kaanapali_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_kaanapali_regmap_config, clks: gpu_cc_kaanapali_clocks.as_ptr(), num_clks: 18, resets: gpu_cc_kaanapali_resets.as_ptr(), num_resets: 7, gdscs: gpu_cc_kaanapali_gdscs.as_ptr(), num_gdscs: 1, use_rpm: true, driver_data: &gpu_cc_kaanapali_driver_data };

static gpu_cc_kaanapali_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,kaanapali-gpucc" }, of_device_id { }];
unsafe extern "C" fn gpu_cc_kaanapali_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &gpu_cc_kaanapali_desc) }
static mut gpu_cc_kaanapali_driver: platform_driver = platform_driver { probe: Some(gpu_cc_kaanapali_probe), driver: driver { name: "gpucc-kaanapali", of_match_table: gpu_cc_kaanapali_match_table.as_ptr() } };
module_platform_driver!(gpu_cc_kaanapali_driver);
MODULE_DESCRIPTION!("QTI GPUCC Kaanapali Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
