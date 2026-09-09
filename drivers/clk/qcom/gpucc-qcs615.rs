// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// C dependencies supplied by the kernel clock, device-tree, regmap, and Qualcomm
// clock-controller headers are intentionally referenced but not reimplemented here.

enum DtInput {
    DT_BI_TCXO,
    DT_GPLL0_OUT_MAIN,
    DT_GPLL0_OUT_MAIN_DIV,
}

enum Parent {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_2X_CLK,
    P_CRC_DIV_PLL0_OUT_AUX2,
    P_GPU_CC_PLL0_OUT_MAIN,
    P_GPU_CC_PLL1_OUT_AUX,
    P_CRC_DIV_PLL1_OUT_AUX2,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static mut gpu_cc_pll0_vco: [pll_vco; 1] = [pll_vco { min_freq: 1_000_000_000, max_freq: 2_100_000_000, val: 0 }];
static mut gpu_cc_pll1_vco: [pll_vco; 1] = [pll_vco { min_freq: 500_000_000, max_freq: 1_000_000_000, val: 2 }];

/* 1020MHz configuration VCO - 0 */
static mut gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x35, config_ctl_val: 0x4001055b, test_ctl_hi_val: 0x1,
    test_ctl_hi_mask: 0x1, alpha_hi: 0x20, alpha: 0x00,
    alpha_en_mask: BIT(24), vco_val: 0x0, vco_mask: GENMASK(21, 20),
    aux2_output_mask: BIT(2),
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &raw mut gpu_cc_pll0_config,
    vco_table: &raw mut gpu_cc_pll0_vco, num_vco: ARRAY_SIZE(gpu_cc_pll0_vco),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_slew_ops,
    } } },
};

/* 930MHz configuration VCO - 2 */
static mut gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x30, config_ctl_val: 0x4001055b, test_ctl_hi_val: 0x1,
    test_ctl_hi_mask: 0x1, alpha_hi: 0x70, alpha: 0x00,
    alpha_en_mask: BIT(24), vco_val: BIT(21), vco_mask: GENMASK(21, 20),
    aux2_output_mask: BIT(2),
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, config: &raw mut gpu_cc_pll1_config,
    vco_table: &raw mut gpu_cc_pll1_vco, num_vco: ARRAY_SIZE(gpu_cc_pll1_vco),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "gpu_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_slew_ops,
    } } },
};

/* Clock Ramp Controller */
static mut crc_div_pll0: clk_fixed_factor = clk_fixed_factor {
    mult: 1, div: 2, hw: clk_hw_init { init: &clk_init_data {
        name: "crc_div_pll0", parent_data: &clk_parent_data { hw: &raw mut gpu_cc_pll0.clkr.hw },
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_fixed_factor_ops,
    } },
};

/* Clock Ramp Controller */
static mut crc_div_pll1: clk_fixed_factor = clk_fixed_factor {
    mult: 1, div: 2, hw: clk_hw_init { init: &clk_init_data {
        name: "crc_div_pll1", parent_data: &clk_parent_data { hw: &raw mut gpu_cc_pll1.clkr.hw },
        num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_fixed_factor_ops,
    } },
};

static gpu_cc_parent_map_0: [parent_map; 5] = [
    parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, value: 1 },
    parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: &raw mut gpu_cc_pll0.clkr.hw },
    clk_parent_data { hw: &raw mut gpu_cc_pll1.clkr.hw }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_cc_parent_map_1: [parent_map; 6] = [
    parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_2X_CLK, value: 1 },
    parent_map { parent: P_CRC_DIV_PLL0_OUT_AUX2, value: 2 }, parent_map { parent: P_GPU_CC_PLL1_OUT_AUX, value: 3 },
    parent_map { parent: P_CRC_DIV_PLL1_OUT_AUX2, value: 4 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 6] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: &raw mut gpu_cc_pll0.clkr.hw },
    clk_parent_data { hw: &raw mut crc_div_pll0.hw }, clk_parent_data { hw: &raw mut gpu_cc_pll1.clkr.hw },
    clk_parent_data { hw: &raw mut crc_div_pll1.hw }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 2] = [
    F(200_000_000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl::default(),
];
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0,
    freq_tbl: ftbl_gpu_cc_gmu_clk_src, clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "gpu_cc_gmu_clk_src", parent_data: gpu_cc_parent_data_0,
        num_parents: ARRAY_SIZE(gpu_cc_parent_data_0), ops: &clk_rcg2_shared_ops,
    } } },
};

static ftbl_gpu_cc_gx_gfx3d_clk_src: [freq_tbl; 11] = [
    F(290_000_000, P_CRC_DIV_PLL1_OUT_AUX2, 1, 0, 0), F(350_000_000, P_CRC_DIV_PLL1_OUT_AUX2, 1, 0, 0),
    F(435_000_000, P_CRC_DIV_PLL1_OUT_AUX2, 1, 0, 0), F(500_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0),
    F(550_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0), F(650_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0),
    F(700_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0), F(745_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0),
    F(845_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0), F(895_000_000, P_CRC_DIV_PLL0_OUT_AUX2, 1, 0, 0),
    freq_tbl::default(),
];
static mut gpu_cc_gx_gfx3d_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x101c, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1,
    freq_tbl: ftbl_gpu_cc_gx_gfx3d_clk_src, clkr: clk_regmap { hw: clk_hw_init { init: &clk_init_data {
        name: "gpu_cc_gx_gfx3d_clk_src", parent_data: gpu_cc_parent_data_1,
        num_parents: ARRAY_SIZE(gpu_cc_parent_data_1), flags: CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE,
        ops: &clk_rcg2_shared_ops,
    } } },
};

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $flags:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt,
        clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw_init { init: &clk_init_data {
            name: stringify!($name), flags: $flags, ops: &clk_branch2_ops,
        } } },
    };
} }
branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_VOTED, 0);
branch!(gpu_cc_cx_gfx3d_clk, 0x10a4, BRANCH_HALT_DELAY, CLK_SET_RATE_PARENT);
branch!(gpu_cc_cx_gfx3d_slv_clk, 0x10a8, BRANCH_HALT_DELAY, CLK_SET_RATE_PARENT);
branch!(gpu_cc_cx_gmu_clk, 0x1098, BRANCH_HALT, CLK_SET_RATE_PARENT);
branch!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_VOTED, 0);
branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_VOTED, 0);
branch!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT, 0);
branch!(gpu_cc_gx_gfx3d_clk, 0x1054, BRANCH_HALT_SKIP, CLK_SET_RATE_PARENT);
branch!(gpu_cc_gx_gmu_clk, 0x1064, BRANCH_HALT, CLK_SET_RATE_PARENT);
branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x5000, BRANCH_VOTED, 0);
branch!(gpu_cc_sleep_clk, 0x1090, BRANCH_HALT_VOTED, 0);

static mut gpu_cc_qcs615_hws: [*mut clk_hw; 2] = [
    &raw mut crc_div_pll0.hw, &raw mut crc_div_pll1.hw,
];
static mut cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540,
    en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x8,
    pd: generic_pm_domain { name: "cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR };
static mut gx_gdsc: gdsc = gdsc { gdscr: 0x100c, gds_hw_ctrl: 0,
    en_rest_wait_val: 0x2, en_few_wait_val: 0x2, clk_dis_wait_val: 0x2,
    pd: generic_pm_domain { name: "gx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR };

static mut gpu_cc_qcs615_clocks: [*mut clk_regmap; 15] = [
    &raw mut gpu_cc_crc_ahb_clk.clkr, &raw mut gpu_cc_cx_gfx3d_clk.clkr,
    &raw mut gpu_cc_cx_gfx3d_slv_clk.clkr, &raw mut gpu_cc_cx_gmu_clk.clkr,
    &raw mut gpu_cc_cx_snoc_dvm_clk.clkr, &raw mut gpu_cc_cxo_aon_clk.clkr,
    &raw mut gpu_cc_cxo_clk.clkr, &raw mut gpu_cc_gmu_clk_src.clkr,
    &raw mut gpu_cc_gx_gfx3d_clk.clkr, &raw mut gpu_cc_gx_gfx3d_clk_src.clkr,
    &raw mut gpu_cc_gx_gmu_clk.clkr, &raw mut gpu_cc_hlos1_vote_gpu_smmu_clk.clkr,
    &raw mut gpu_cc_pll0.clkr, &raw mut gpu_cc_pll1.clkr, &raw mut gpu_cc_sleep_clk.clkr,
];
static mut gpu_cc_qcs615_gdscs: [*mut gdsc; 2] = [&raw mut cx_gdsc, &raw mut gx_gdsc];
static gpu_cc_qcs615_resets: [qcom_reset_map; 5] = [
    qcom_reset_map { reg: 0x1068 }, qcom_reset_map { reg: 0x10a0 },
    qcom_reset_map { reg: 0x111c }, qcom_reset_map { reg: 0x1008 }, qcom_reset_map { reg: 0x1000 },
];
static mut gpu_cc_qcs615_plls: [*mut clk_alpha_pll; 2] = [&raw mut gpu_cc_pll0, &raw mut gpu_cc_pll1];
static gpu_cc_qcs615_critical_cbcrs: [u32; 1] = [0x1078]; /* GPU_CC_AHB_CLK */
static gpu_cc_qcs615_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x7008, fast_io: true,
};

unsafe fn clk_qcs615_regs_crc_configure(dev: *mut device, regmap: *mut regmap) {
    /* Recommended WAKEUP/SLEEP settings for the gpu_cc_cx_gmu_clk */
    regmap_update_bits(regmap, gpu_cc_cx_gmu_clk.clkr.enable_reg, 0xff0, 0xff0);
    /* After POR, Clock Ramp Controller(CRC) will be in bypass mode. */
    regmap_update_bits(regmap, 0x1028, 0x00015011, 0x00015011);
    regmap_update_bits(regmap, 0x1024, 0x00800000, 0x00800000);
}

static gpu_cc_qcs615_driver_data: qcom_cc_driver_data = qcom_cc_driver_data {
    alpha_plls: gpu_cc_qcs615_plls, num_alpha_plls: ARRAY_SIZE(gpu_cc_qcs615_plls),
    clk_cbcrs: gpu_cc_qcs615_critical_cbcrs, num_clk_cbcrs: ARRAY_SIZE(gpu_cc_qcs615_critical_cbcrs),
    clk_regs_configure: Some(clk_qcs615_regs_crc_configure),
};
static gpu_cc_qcs615_desc: qcom_cc_desc = qcom_cc_desc {
    config: &gpu_cc_qcs615_regmap_config, clks: gpu_cc_qcs615_clocks,
    num_clks: ARRAY_SIZE(gpu_cc_qcs615_clocks), clk_hws: gpu_cc_qcs615_hws,
    num_clk_hws: ARRAY_SIZE(gpu_cc_qcs615_hws), resets: gpu_cc_qcs615_resets,
    num_resets: ARRAY_SIZE(gpu_cc_qcs615_resets), gdscs: gpu_cc_qcs615_gdscs,
    num_gdscs: ARRAY_SIZE(gpu_cc_qcs615_gdscs), driver_data: &gpu_cc_qcs615_driver_data,
};

static gpu_cc_qcs615_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,qcs615-gpucc" }, of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, gpu_cc_qcs615_match_table);

unsafe fn gpu_cc_qcs615_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &gpu_cc_qcs615_desc)
}

static mut gpu_cc_qcs615_driver: platform_driver = platform_driver {
    probe: Some(gpu_cc_qcs615_probe), driver: driver {
        name: "gpucc-qcs615", of_match_table: gpu_cc_qcs615_match_table,
    },
};

module_platform_driver!(gpu_cc_qcs615_driver);
MODULE_DESCRIPTION!("QTI GPUCC QCS615 Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
