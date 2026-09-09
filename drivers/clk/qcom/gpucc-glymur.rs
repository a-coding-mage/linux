// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Linux clock-provider, module, platform-device, regmap, device-tree, and
// Qualcomm clock support declarations are supplied by the surrounding crate.

enum DtClock { DT_BI_TCXO, DT_GPLL0_OUT_MAIN, DT_GPLL0_OUT_MAIN_DIV }
enum Parent { P_BI_TCXO, P_GPLL0_OUT_MAIN, P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_OUT_EVEN, P_GPU_CC_PLL0_OUT_MAIN, P_GPU_CC_PLL0_OUT_ODD }

static TAYCAN_EKO_T_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

/* 1150.0 MHz Configuration */
static GPU_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x3b, alpha: 0xe555, config_ctl_val: 0x25c400e7,
    config_ctl_hi_val: 0x0a8060e0, config_ctl_hi1_val: 0xf51dea20,
    user_ctl_val: 0x00000408, user_ctl_hi_val: 0x00000002,
};

static mut GPU_CC_PLL0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &GPU_CC_PLL0_CONFIG, vco_table: &TAYCAN_EKO_T_VCO,
    num_vco: ARRAY_SIZE!(TAYCAN_EKO_T_VCO), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T],
    clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data { name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DtClock::DT_BI_TCXO as u32 }, num_parents: 1, ops: &clk_alpha_pll_taycan_eko_t_ops } } },
};

static POST_DIV_TABLE_GPU_CC_PLL0_OUT_EVEN: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 }, clk_div_table { val: 0, div: 0 }
];

static mut GPU_CC_PLL0_OUT_EVEN: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 10, post_div_table: &POST_DIV_TABLE_GPU_CC_PLL0_OUT_EVEN,
    num_post_div: ARRAY_SIZE!(POST_DIV_TABLE_GPU_CC_PLL0_OUT_EVEN), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_EKO_T],
    clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "gpu_cc_pll0_out_even", parent_hws: &[&raw mut GPU_CC_PLL0.clkr.hw as *mut _],
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_taycan_eko_t_ops,
    } } },
};

static GPU_CC_PARENT_MAP_0: [parent_map; 3] = [
    parent_map { parent: Parent::P_BI_TCXO as u32, val: 0 }, parent_map { parent: Parent::P_GPLL0_OUT_MAIN as u32, val: 5 }, parent_map { parent: Parent::P_GPLL0_OUT_MAIN_DIV as u32, val: 6 }
];
static GPU_CC_PARENT_DATA_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DtClock::DT_BI_TCXO as u32 }, clk_parent_data { index: DtClock::DT_GPLL0_OUT_MAIN as u32 }, clk_parent_data { index: DtClock::DT_GPLL0_OUT_MAIN_DIV as u32 }
];
static GPU_CC_PARENT_MAP_1: [parent_map; 6] = [
    parent_map { parent: Parent::P_BI_TCXO as u32, val: 0 }, parent_map { parent: Parent::P_GPU_CC_PLL0_OUT_MAIN as u32, val: 1 }, parent_map { parent: Parent::P_GPU_CC_PLL0_OUT_EVEN as u32, val: 2 }, parent_map { parent: Parent::P_GPU_CC_PLL0_OUT_ODD as u32, val: 3 }, parent_map { parent: Parent::P_GPLL0_OUT_MAIN as u32, val: 5 }, parent_map { parent: Parent::P_GPLL0_OUT_MAIN_DIV as u32, val: 6 }
];
static mut GPU_CC_PARENT_DATA_1: [clk_parent_data; 6] = [
    clk_parent_data { index: DtClock::DT_BI_TCXO as u32 }, clk_parent_data { hw: &raw mut GPU_CC_PLL0.clkr.hw as *mut _ }, clk_parent_data { hw: &raw mut GPU_CC_PLL0_OUT_EVEN.clkr.hw as *mut _ }, clk_parent_data { hw: &raw mut GPU_CC_PLL0.clkr.hw as *mut _ }, clk_parent_data { index: DtClock::DT_GPLL0_OUT_MAIN as u32 }, clk_parent_data { index: DtClock::DT_GPLL0_OUT_MAIN_DIV as u32 }
];

macro_rules! F { ($f:expr, $p:expr, $d:expr, $m:expr, $n:expr) => { freq_tbl { freq: $f, src: $p as u32, pre_div: $d, m: $m, n: $n } }; }
static FTBL_GPU_CC_FF_CLK_SRC: [freq_tbl; 2] = [F!(200000000, Parent::P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 }];
static FTBL_GPU_CC_GMU_CLK_SRC: [freq_tbl; 6] = [F!(19200000, Parent::P_BI_TCXO, 1, 0, 0), F!(575000000, Parent::P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), F!(700000000, Parent::P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), F!(725000000, Parent::P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), F!(750000000, Parent::P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0), freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 }];
static FTBL_GPU_CC_HUB_CLK_SRC: [freq_tbl; 4] = [F!(200000000, Parent::P_GPLL0_OUT_MAIN, 3, 0, 0), F!(300000000, Parent::P_GPLL0_OUT_MAIN, 2, 0, 0), F!(400000000, Parent::P_GPLL0_OUT_MAIN, 1.5, 0, 0), freq_tbl { freq: 0, src: 0, pre_div: 0, m: 0, n: 0 }];

// The following clock objects retain the C driver's static object layout.
static mut GPU_CC_FF_CLK_SRC: clk_rcg2 = clk_rcg2::new(0x9474, 0, 5, &GPU_CC_PARENT_MAP_0, &FTBL_GPU_CC_FF_CLK_SRC, "gpu_cc_ff_clk_src", &GPU_CC_PARENT_DATA_0, &clk_rcg2_shared_ops);
static mut GPU_CC_GMU_CLK_SRC: clk_rcg2 = clk_rcg2::new(0x9318, 0, 5, &GPU_CC_PARENT_MAP_1, &FTBL_GPU_CC_GMU_CLK_SRC, "gpu_cc_gmu_clk_src", &GPU_CC_PARENT_DATA_1, &clk_rcg2_shared_ops);
static mut GPU_CC_HUB_CLK_SRC: clk_rcg2 = clk_rcg2::new(0x93f0, 0, 5, &GPU_CC_PARENT_MAP_1, &FTBL_GPU_CC_HUB_CLK_SRC, "gpu_cc_hub_clk_src", &GPU_CC_PARENT_DATA_1, &clk_rcg2_shared_ops);
static mut GPU_CC_HUB_DIV_CLK_SRC: clk_regmap_div = clk_regmap_div::new(0x9430, 0, 4, "gpu_cc_hub_div_clk_src", &GPU_CC_HUB_CLK_SRC, &clk_regmap_div_ro_ops);

macro_rules! branch { ($name:ident, $halt:expr, $check:expr, $reg:expr, $ops:expr) => { static mut $name: clk_branch = clk_branch::new($halt, $check, $reg, $ops); }; }
branch!(GPU_CC_AHB_CLK, 0x90bc, BRANCH_HALT_DELAY, 0x90bc, &clk_branch2_ops);
branch!(GPU_CC_CX_ACCU_SHIFT_CLK, 0x9108, BRANCH_HALT_VOTED, 0x9108, &clk_branch2_ops);
branch!(GPU_CC_CX_FF_CLK, 0x90ec, BRANCH_HALT, 0x90ec, &clk_branch2_ops);
branch!(GPU_CC_CX_GMU_CLK, 0x90d4, BRANCH_HALT_VOTED, 0x90d4, &clk_branch2_aon_ops);
branch!(GPU_CC_CXO_CLK, 0x90e4, BRANCH_HALT, 0x90e4, &clk_branch2_ops);
branch!(GPU_CC_DEMET_CLK, 0x9010, BRANCH_HALT_VOTED, 0x9010, &clk_branch2_ops);
branch!(GPU_CC_DPM_CLK, 0x910c, BRANCH_HALT, 0x910c, &clk_branch2_ops);
branch!(GPU_CC_FREQ_MEASURE_CLK, 0x900c, BRANCH_HALT, 0x900c, &clk_branch2_ops);
branch!(GPU_CC_GPU_SMMU_VOTE_CLK, 0x7000, BRANCH_HALT_VOTED, 0x7000, &clk_branch2_ops);
branch!(GPU_CC_GX_ACCU_SHIFT_CLK, 0x9070, BRANCH_HALT_VOTED, 0x9070, &clk_branch2_ops);
branch!(GPU_CC_GX_ACD_AHB_FF_CLK, 0x9068, BRANCH_HALT, 0x9068, &clk_branch2_ops);
branch!(GPU_CC_GX_AHB_FF_CLK, 0x9064, BRANCH_HALT, 0x9064, &clk_branch2_ops);
branch!(GPU_CC_GX_GMU_CLK, 0x9060, BRANCH_HALT, 0x9060, &clk_branch2_ops);
branch!(GPU_CC_GX_RCG_AHB_FF_CLK, 0x906c, BRANCH_HALT_VOTED, 0x906c, &clk_branch2_ops);
branch!(GPU_CC_HUB_AON_CLK, 0x93ec, BRANCH_HALT_VOTED, 0x93ec, &clk_branch2_aon_ops);
branch!(GPU_CC_HUB_CX_INT_CLK, 0x90e8, BRANCH_HALT_VOTED, 0x90e8, &clk_branch2_aon_ops);
branch!(GPU_CC_MEMNOC_GFX_CLK, 0x90f0, BRANCH_HALT_VOTED, 0x90f0, &clk_branch2_ops);
branch!(GPU_CC_RSCC_HUB_AON_CLK, 0x93e8, BRANCH_HALT, 0x93e8, &clk_branch2_ops);
branch!(GPU_CC_SLEEP_CLK, 0x90cc, BRANCH_HALT, 0x90cc, &clk_branch2_ops);

static mut GPU_CC_CX_GDSC: gdsc = gdsc { gdscr: 0x9080, gds_hw_ctrl: 0x9094, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0xf, pd: power_domain { name: "gpu_cc_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE };

static mut GPU_CC_GLYMUR_CLOCKS: [*mut clk_regmap; 25] = [
    &raw mut GPU_CC_AHB_CLK.clkr, &raw mut GPU_CC_CX_ACCU_SHIFT_CLK.clkr, &raw mut GPU_CC_CX_FF_CLK.clkr, &raw mut GPU_CC_CX_GMU_CLK.clkr, &raw mut GPU_CC_CXO_CLK.clkr, &raw mut GPU_CC_DEMET_CLK.clkr, &raw mut GPU_CC_DPM_CLK.clkr, &raw mut GPU_CC_FF_CLK_SRC.clkr, &raw mut GPU_CC_FREQ_MEASURE_CLK.clkr, &raw mut GPU_CC_GMU_CLK_SRC.clkr, &raw mut GPU_CC_GPU_SMMU_VOTE_CLK.clkr, &raw mut GPU_CC_GX_ACCU_SHIFT_CLK.clkr, &raw mut GPU_CC_GX_ACD_AHB_FF_CLK.clkr, &raw mut GPU_CC_GX_AHB_FF_CLK.clkr, &raw mut GPU_CC_GX_GMU_CLK.clkr, &raw mut GPU_CC_GX_RCG_AHB_FF_CLK.clkr, &raw mut GPU_CC_HUB_AON_CLK.clkr, &raw mut GPU_CC_HUB_CLK_SRC.clkr, &raw mut GPU_CC_HUB_CX_INT_CLK.clkr, &raw mut GPU_CC_HUB_DIV_CLK_SRC.clkr, &raw mut GPU_CC_MEMNOC_GFX_CLK.clkr, &raw mut GPU_CC_PLL0.clkr, &raw mut GPU_CC_PLL0_OUT_EVEN.clkr, &raw mut GPU_CC_RSCC_HUB_AON_CLK.clkr, &raw mut GPU_CC_SLEEP_CLK.clkr
];
static mut GPU_CC_GLYMUR_GDSCS: [*mut gdsc; 1] = [&raw mut GPU_CC_CX_GDSC];
static GPU_CC_GLYMUR_RESETS: [qcom_reset_map; 7] = [qcom_reset_map { reg: 0x93a0 }, qcom_reset_map { reg: 0x907c }, qcom_reset_map { reg: 0x93e4 }, qcom_reset_map { reg: 0x9470 }, qcom_reset_map { reg: 0x9314 }, qcom_reset_map { reg: 0x905c }, qcom_reset_map { reg: 0x9000 }];
static mut GPU_CC_GLYMUR_PLLS: [*mut clk_alpha_pll; 1] = [&raw mut GPU_CC_PLL0];
static GPU_CC_GLYMUR_CRITICAL_CBCRS: [u32; 3] = [0x93a4, 0x9008, 0x9004];
static GPU_CC_GLYMUR_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x95e8, fast_io: true };
static GPU_CC_GLYMUR_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &GPU_CC_GLYMUR_PLLS, num_alpha_plls: 1, clk_cbcrs: &GPU_CC_GLYMUR_CRITICAL_CBCRS, num_clk_cbcrs: 3 };
static GPU_CC_GLYMUR_DESC: qcom_cc_desc = qcom_cc_desc { config: &GPU_CC_GLYMUR_REGMAP_CONFIG, clks: &GPU_CC_GLYMUR_CLOCKS, num_clks: 25, resets: &GPU_CC_GLYMUR_RESETS, num_resets: 7, gdscs: &GPU_CC_GLYMUR_GDSCS, num_gdscs: 1, use_rpm: true, driver_data: &GPU_CC_GLYMUR_DRIVER_DATA };

static GPU_CC_GLYMUR_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,glymur-gpucc" }, of_device_id { compatible: "" }];
unsafe fn gpu_cc_glymur_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &GPU_CC_GLYMUR_DESC) }
static mut GPU_CC_GLYMUR_DRIVER: platform_driver = platform_driver::new(gpu_cc_glymur_probe, "gpucc-glymur", &GPU_CC_GLYMUR_MATCH_TABLE);

module_platform_driver!(GPU_CC_GLYMUR_DRIVER);
MODULE_DEVICE_TABLE!(of, GPU_CC_GLYMUR_MATCH_TABLE);
MODULE_DESCRIPTION!("QTI GPUCC Glymur Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
