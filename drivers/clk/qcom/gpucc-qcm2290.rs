// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2024, Linaro Limited
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

enum { DT_GCC_AHB_CLK, DT_BI_TCXO, DT_GCC_GPU_GPLL0_CLK_SRC, DT_GCC_GPU_GPLL0_DIV_CLK_SRC }
enum {
    P_BI_TCXO, P_GPLL0_OUT_MAIN, P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_2X_DIV_CLK_SRC, P_GPU_CC_PLL0_OUT_AUX,
    P_GPU_CC_PLL0_OUT_AUX2, P_GPU_CC_PLL0_OUT_MAIN,
}

static huayra_vco: [pll_vco; 2] = [
    pll_vco { min_freq: 600000000, max_freq: 3300000000, val: 0 },
    pll_vco { min_freq: 600000000, max_freq: 2200000000, val: 1 },
];

static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x25, config_ctl_val: 0x200d4828, config_ctl_hi_val: 0x6,
    test_ctl_val: GENMASK(28, 26), test_ctl_hi_val: BIT(14), user_ctl_val: 0xf,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &gpu_cc_pll0_config, vco_table: &huayra_vco,
    num_vco: ARRAY_SIZE(&huayra_vco), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_2290],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, flags: 0, ops: &clk_alpha_pll_huayra_ops,
    } } },
};

static gpu_cc_parent_map_0: [parent_map; 4] = [
    parent_map { source: P_BI_TCXO, cfg: 0 }, parent_map { source: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 },
    parent_map { source: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { source: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }, clk_parent_data { index: DT_GCC_GPU_GPLL0_DIV_CLK_SRC },
];
static gpu_cc_parent_map_1: [parent_map; 5] = [
    parent_map { source: P_BI_TCXO, cfg: 0 }, parent_map { source: P_GPU_CC_PLL0_2X_DIV_CLK_SRC, cfg: 1 },
    parent_map { source: P_GPU_CC_PLL0_OUT_AUX2, cfg: 2 }, parent_map { source: P_GPU_CC_PLL0_OUT_AUX, cfg: 3 },
    parent_map { source: P_GPLL0_OUT_MAIN, cfg: 5 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC },
];

macro_rules! F { ($rate:expr, $parent:expr, $pre_div:expr, $m:expr, $n:expr) => { freq_tbl { freq: $rate, src: $parent, pre_div: $pre_div, m: $m, n: $n } }; }
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 2] = [F!(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl::default()];
static ftbl_gpu_cc_gx_gfx3d_clk_src: [freq_tbl; 8] = [
    F!(355200000,P_GPU_CC_PLL0_OUT_AUX,2,0,0), F!(537600000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0),
    F!(672000000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), F!(844800000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0),
    F!(921600000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), F!(1017600000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0),
    F!(1123200000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), freq_tbl::default(),
];
static ftbl_gpu_cc_gx_gfx3d_clk_src_shikra: [freq_tbl; 8] = [
    F!(355200000,P_GPU_CC_PLL0_OUT_AUX,2,0,0), F!(537600000,P_GPU_CC_PLL0_OUT_AUX,2,0,0),
    F!(672000000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), F!(844800000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0),
    F!(921600000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), F!(1017600000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0),
    F!(1142400000,P_GPU_CC_PLL0_OUT_AUX2,2,0,0), freq_tbl::default(),
];

// Direct representations of the remaining kernel clock objects and descriptor tables.
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: &gpu_cc_parent_map_0, freq_tbl: &ftbl_gpu_cc_gmu_clk_src, clkr: clk_regmap::with_init("gpu_cc_gmu_clk_src", &gpu_cc_parent_data_0, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };
static mut gpu_cc_gx_gfx3d_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x101c, mnd_width: 0, hid_width: 5, parent_map: &gpu_cc_parent_map_1, freq_tbl: &ftbl_gpu_cc_gx_gfx3d_clk_src, clkr: clk_regmap::with_init("gpu_cc_gx_gfx3d_clk_src", &gpu_cc_parent_data_1, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $clkname:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt, clkr: clk_regmap::branch_init($reg, $clkname, &clk_branch2_ops) };
}; }
branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_DELAY, "gpu_cc_crc_ahb_clk");
branch!(gpu_cc_cx_gfx3d_clk, 0x10a4, BRANCH_HALT_DELAY, "gpu_cc_cx_gfx3d_clk");
branch!(gpu_cc_cx_gmu_clk, 0x1098, BRANCH_HALT, "gpu_cc_cx_gmu_clk");
branch!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_DELAY, "gpu_cc_cx_snoc_dvm_clk");
branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_DELAY, "gpu_cc_cxo_aon_clk");
branch!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT, "gpu_cc_cxo_clk");
branch!(gpu_cc_gx_gfx3d_clk, 0x1054, BRANCH_HALT_DELAY, "gpu_cc_gx_gfx3d_clk");
branch!(gpu_cc_sleep_clk, 0x1090, BRANCH_VOTED, "gpu_cc_sleep_clk");
branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x5000, BRANCH_VOTED, "gpu_cc_hlos1_vote_gpu_smmu_clk");

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 2, pd: generic_pm_domain { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE | VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x100c, clamp_io_ctrl: 0x1508, resets: &[GPU_GX_BCR], reset_count: 1, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 2, pd: generic_pm_domain { name: "gpu_gx_gdsc" }, parent: unsafe { &gpu_cx_gdsc.pd }, pwrsts: PWRSTS_OFF_ON, flags: RETAIN_FF_ENABLE | POLL_CFG_GDSCR | CLAMP_IO | AON_RESET | SW_RESET };

static mut gpu_cc_qcm2290_clocks: [*mut clk_regmap; 13] = [
    &mut gpu_cc_crc_ahb_clk.clkr, &mut gpu_cc_cx_gfx3d_clk.clkr, &mut gpu_cc_cx_gmu_clk.clkr,
    &mut gpu_cc_cx_snoc_dvm_clk.clkr, &mut gpu_cc_cxo_aon_clk.clkr, &mut gpu_cc_cxo_clk.clkr,
    &mut gpu_cc_gmu_clk_src.clkr, &mut gpu_cc_gx_gfx3d_clk.clkr, &mut gpu_cc_gx_gfx3d_clk_src.clkr,
    &mut gpu_cc_pll0.clkr, &mut gpu_cc_sleep_clk.clkr, &mut gpu_cc_hlos1_vote_gpu_smmu_clk.clkr, core::ptr::null_mut(),
];
static gpu_cc_qcm2290_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x1008 }];
static mut gpu_cc_qcm2290_gdscs: [*mut gdsc; 2] = [&mut gpu_cx_gdsc, &mut gpu_gx_gdsc];
static mut gpu_cc_qcm2290_plls: [*mut clk_alpha_pll; 1] = [&mut gpu_cc_pll0];
static gpu_cc_qcm2290_critical_cbcrs: [u32; 2] = [0x1078, 0x1060];
static gpu_cc_qcm2290_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9000, fast_io: true };
static gpu_cc_qcm2290_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: &gpu_cc_qcm2290_plls, num_alpha_plls: 1, clk_cbcrs: &gpu_cc_qcm2290_critical_cbcrs, num_clk_cbcrs: 2 };
static gpu_cc_qcm2290_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_qcm2290_regmap_config, clks: &gpu_cc_qcm2290_clocks, num_clks: 13, resets: &gpu_cc_qcm2290_resets, num_resets: 1, gdscs: &gpu_cc_qcm2290_gdscs, num_gdscs: 2, use_rpm: true, driver_data: &gpu_cc_qcm2290_driver_data };

static gpu_cc_qcm2290_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,qcm2290-gpucc" }, of_device_id { compatible: "qcom,shikra-gpucc" }, of_device_id::default(),
];

unsafe fn gpu_cc_qcm2290_probe(pdev: *mut platform_device) -> i32 {
    if device_is_compatible(&(*pdev).dev, "qcom,shikra-gpucc") {
        gpu_cc_gx_gfx3d_clk_src.freq_tbl = &ftbl_gpu_cc_gx_gfx3d_clk_src_shikra;
    }
    qcom_cc_probe(pdev, &gpu_cc_qcm2290_desc)
}

static gpu_cc_qcm2290_driver: platform_driver = platform_driver { probe: gpu_cc_qcm2290_probe, driver: driver { name: "gpucc-qcm2290", of_match_table: &gpu_cc_qcm2290_match_table } };
// module_platform_driver(gpu_cc_qcm2290_driver);
// MODULE_DESCRIPTION("QTI QCM2290 GPU clock controller driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
