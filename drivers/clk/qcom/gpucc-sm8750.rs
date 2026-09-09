// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */
// External Linux clock-controller headers and device-tree bindings are supplied by dependencies.

const DT_BI_TCXO: usize = 0;
const DT_GPLL0_OUT_MAIN: usize = 1;
const DT_GPLL0_OUT_MAIN_DIV: usize = 2;

const P_BI_TCXO: usize = 0;
const P_GPLL0_OUT_MAIN: usize = 1;
const P_GPLL0_OUT_MAIN_DIV: usize = 2;
const P_GPU_CC_PLL0_OUT_EVEN: usize = 3;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 4;
const P_GPU_CC_PLL0_OUT_ODD: usize = 5;

static TAYCAN_ELU_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2500000000, val: 0 }];

static GPU_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x34,
    alpha: 0x1555,
    config_ctl_val: 0x19660387,
    config_ctl_hi_val: 0x098060a0,
    config_ctl_hi1_val: 0xb416cb20,
    user_ctl_val: 0x00000400,
    user_ctl_hi_val: 0x00000002,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    config: &GPU_CC_PLL0_CONFIG,
    vco_table: &TAYCAN_ELU_VCO,
    num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_ELU],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0",
        parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1,
        ops: &clk_alpha_pll_taycan_elu_ops,
    } } },
};

static POST_DIV_TABLE_GPU_CC_PLL0_OUT_EVEN: [clk_div_table; 2] = [
    clk_div_table { val: 0x1, div: 2 },
    clk_div_table { val: 0, div: 0 },
];

static mut gpu_cc_pll0_out_even: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0,
    post_div_shift: 10,
    post_div_table: &POST_DIV_TABLE_GPU_CC_PLL0_OUT_EVEN,
    num_post_div: 2,
    width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TAYCAN_ELU],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0_out_even",
        parent_hws: &[&raw mut gpu_cc_pll0.clkr.hw],
        num_parents: 1,
        flags: CLK_SET_RATE_PARENT,
        ops: &clk_alpha_pll_postdiv_taycan_elu_ops,
    } } },
};

static GPU_CC_PARENT_MAP_1: [parent_map; 6] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 },
    parent_map { src: P_GPU_CC_PLL0_OUT_EVEN, cfg: 2 },
    parent_map { src: P_GPU_CC_PLL0_OUT_ODD, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];

static mut GPU_CC_PARENT_DATA_1: [clk_parent_data; 6] = [
    clk_parent_data { index: DT_BI_TCXO },
    clk_parent_data { hw: &raw mut gpu_cc_pll0.clkr.hw },
    clk_parent_data { hw: &raw mut gpu_cc_pll0_out_even.clkr.hw },
    clk_parent_data { hw: &raw mut gpu_cc_pll0.clkr.hw },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];

static FTBL_GPU_CC_GMU_CLK_SRC: [freq_tbl; 5] = [
    F(19200000, P_BI_TCXO, 1, 0, 0),
    F(500000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    F(650000000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    F(687500000, P_GPU_CC_PLL0_OUT_EVEN, 1, 0, 0),
    freq_tbl { ..Default::default() },
];

static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5,
    parent_map: &GPU_CC_PARENT_MAP_1, freq_tbl: &FTBL_GPU_CC_GMU_CLK_SRC,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_gmu_clk_src", parent_data: &GPU_CC_PARENT_DATA_1,
        num_parents: 6, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops,
    } } },
};

static FTBL_GPU_CC_HUB_CLK_SRC: [freq_tbl; 4] = [
    F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0),
    F(300000000, P_GPLL0_OUT_MAIN, 2, 0, 0),
    F(400000000, P_GPLL0_OUT_MAIN, 1.5, 0, 0),
    freq_tbl { ..Default::default() },
];

static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x93ec, mnd_width: 0, hid_width: 5,
    parent_map: &GPU_CC_PARENT_MAP_1, freq_tbl: &FTBL_GPU_CC_HUB_CLK_SRC,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_hub_clk_src", parent_data: &GPU_CC_PARENT_DATA_1,
        num_parents: 6, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops,
    } } },
};

static mut gpu_cc_hub_div_clk_src: clk_regmap_div = clk_regmap_div {
    reg: 0x942c, shift: 0, width: 4,
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_hub_div_clk_src", parent_hws: &[&raw mut gpu_cc_hub_clk_src.clkr.hw],
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_regmap_div_ro_ops,
    } } },
};

// Branch clocks retain the source names, registers, halt modes, parents, and operations.
macro_rules! branch_clock {
    ($name:ident, $reg:expr, $halt:expr, $clk_name:expr, $ops:expr) => {
        static mut $name: clk_branch = clk_branch {
            halt_reg: $reg, halt_check: $halt,
            clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
                name: $clk_name, ops: $ops,
            } } },
        };
    };
}

branch_clock!(gpu_cc_cx_accu_shift_clk, 0x910c, BRANCH_HALT_VOTED, "gpu_cc_cx_accu_shift_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_cxo_clk, 0x90e4, BRANCH_HALT, "gpu_cc_cxo_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_demet_clk, 0x9010, BRANCH_HALT_VOTED, "gpu_cc_demet_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_dpm_clk, 0x9110, BRANCH_HALT, "gpu_cc_dpm_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_freq_measure_clk, 0x900c, BRANCH_HALT, "gpu_cc_freq_measure_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_gx_accu_shift_clk, 0x9070, BRANCH_HALT_VOTED, "gpu_cc_gx_accu_shift_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x7000, BRANCH_HALT_VOTED, "gpu_cc_hlos1_vote_gpu_smmu_clk", &clk_branch2_ops);
branch_clock!(gpu_cc_memnoc_gfx_clk, 0x90f4, BRANCH_HALT_VOTED, "gpu_cc_memnoc_gfx_clk", &clk_branch2_ops);

static mut gpu_cc_ahb_clk: clk_branch = clk_branch { halt_reg: 0x90bc, halt_check: BRANCH_HALT_DELAY, clkr: clk_regmap { enable_reg: 0x90bc, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "gpu_cc_ahb_clk", parent_hws: &[&raw mut gpu_cc_hub_div_clk_src.clkr.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops } } } };
static mut gpu_cc_cx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x90d4, halt_check: BRANCH_HALT_VOTED, clkr: clk_regmap { enable_reg: 0x90d4, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "gpu_cc_cx_gmu_clk", parent_hws: &[&raw mut gpu_cc_gmu_clk_src.clkr.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_aon_ops } } } };
static mut gpu_cc_gx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x9060, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x9060, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "gpu_cc_gx_gmu_clk", parent_hws: &[&raw mut gpu_cc_gmu_clk_src.clkr.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_aon_ops } } } };
static mut gpu_cc_hub_aon_clk: clk_branch = clk_branch { halt_reg: 0x93e8, halt_check: BRANCH_HALT_VOTED, clkr: clk_regmap { enable_reg: 0x93e8, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "gpu_cc_hub_aon_clk", parent_hws: &[&raw mut gpu_cc_hub_clk_src.clkr.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_aon_ops } } } };
static mut gpu_cc_hub_cx_int_clk: clk_branch = clk_branch { halt_reg: 0x90e8, halt_check: BRANCH_HALT_VOTED, clkr: clk_regmap { enable_reg: 0x90e8, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: "gpu_cc_hub_cx_int_clk", parent_hws: &[&raw mut gpu_cc_hub_clk_src.clkr.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_aon_ops } } } };

static mut gpu_cc_cx_gdsc: gdsc = gdsc { gdscr: 0x9080, gds_hw_ctrl: 0x9094, en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x8, pd: genpd { name: "gpu_cc_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE | VOTABLE };

static mut gpu_cc_sm8750_clocks: [*mut clk_regmap; 18] = [
    &raw mut gpu_cc_ahb_clk.clkr, &raw mut gpu_cc_cx_accu_shift_clk.clkr, &raw mut gpu_cc_cx_gmu_clk.clkr,
    &raw mut gpu_cc_cxo_clk.clkr, &raw mut gpu_cc_demet_clk.clkr, &raw mut gpu_cc_dpm_clk.clkr,
    &raw mut gpu_cc_freq_measure_clk.clkr, &raw mut gpu_cc_gmu_clk_src.clkr, &raw mut gpu_cc_gx_accu_shift_clk.clkr,
    &raw mut gpu_cc_gx_gmu_clk.clkr, &raw mut gpu_cc_hlos1_vote_gpu_smmu_clk.clkr, &raw mut gpu_cc_hub_aon_clk.clkr,
    &raw mut gpu_cc_hub_clk_src.clkr, &raw mut gpu_cc_hub_cx_int_clk.clkr, &raw mut gpu_cc_hub_div_clk_src.clkr,
    &raw mut gpu_cc_memnoc_gfx_clk.clkr, &raw mut gpu_cc_pll0.clkr, &raw mut gpu_cc_pll0_out_even.clkr,
];
static mut gpu_cc_sm8750_gdscs: [*mut gdsc; 1] = [&raw mut gpu_cc_cx_gdsc];
static GPU_CC_SM8750_RESETS: [qcom_reset_map; 6] = [
    qcom_reset_map { reg: 0x9000 }, qcom_reset_map { reg: 0x905c }, qcom_reset_map { reg: 0x907c },
    qcom_reset_map { reg: 0x9314 }, qcom_reset_map { reg: 0x93a0 }, qcom_reset_map { reg: 0x93e4 },
];
static GPU_CC_SM8750_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9800, fast_io: true };
static mut GPU_CC_ALPHA_PLLS: [*mut clk_alpha_pll; 1] = [&raw mut gpu_cc_pll0];
static GPU_CC_SM8750_CRITICAL_CBCRS: [u32; 6] = [0x9004, 0x9008, 0x9064, 0x90cc, 0x93a4, 0x93a8];
static GPU_CC_SM8750_DRIVER_DATA: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &GPU_CC_ALPHA_PLLS, num_alpha_plls: 1, clk_cbcrs: &GPU_CC_SM8750_CRITICAL_CBCRS, num_clk_cbcrs: 6 };
static GPU_CC_SM8750_DESC: qcom_cc_desc = qcom_cc_desc { config: &GPU_CC_SM8750_REGMAP_CONFIG, clks: &gpu_cc_sm8750_clocks, num_clks: 18, resets: &GPU_CC_SM8750_RESETS, num_resets: 6, gdscs: &gpu_cc_sm8750_gdscs, num_gdscs: 1, use_rpm: true, driver_data: &GPU_CC_SM8750_DRIVER_DATA };

static GPU_CC_SM8750_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,sm8750-gpucc" }, of_device_id::default()];

unsafe extern "C" {
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

unsafe fn gpu_cc_sm8750_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &GPU_CC_SM8750_DESC) }

static mut gpu_cc_sm8750_driver: platform_driver = platform_driver {
    probe: Some(gpu_cc_sm8750_probe),
    driver: device_driver { name: "sm8750-gpucc", of_match_table: &GPU_CC_SM8750_MATCH_TABLE },
};

// Equivalent of module_platform_driver(gpu_cc_sm8750_driver).
// MODULE_DEVICE_TABLE(of, gpu_cc_sm8750_match_table);
// MODULE_DESCRIPTION("QTI GPU_CC SM8750 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
