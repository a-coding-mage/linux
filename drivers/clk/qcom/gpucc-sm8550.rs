// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Translated from gpucc-sm8550.c. Kernel-provided types, constants, macros,
// operations, and declarations are supplied by the corresponding Rust bindings.

const DT_BI_TCXO: u32 = 0;
const DT_GPLL0_OUT_MAIN: u32 = 1;
const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;

const P_BI_TCXO: u32 = 0;
const P_GPLL0_OUT_MAIN: u32 = 1;
const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_GPU_CC_PLL0_OUT_MAIN: u32 = 3;
const P_GPU_CC_PLL1_OUT_MAIN: u32 = 4;

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1e, alpha: 0xbaaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, vco_table: lucid_ole_vco.as_ptr(), num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x16, alpha: 0xeaaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, vco_table: lucid_ole_vco.as_ptr(), num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    } } },
};

// Parent maps and frequency tables retain the exact source values and ordering.
static gpu_cc_parent_map_0: [parent_map; 3] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 }];
static gpu_cc_parent_data_0: [clk_parent_data; 3] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV }];
static gpu_cc_parent_map_1: [parent_map; 5] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 }, parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 }];
static gpu_cc_parent_map_2: [parent_map; 4] = [parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 }];
static gpu_cc_parent_map_3: [parent_map; 1] = [parent_map { src: P_BI_TCXO, cfg: 0 }];

static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 2] = [F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl::default()];
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [F(19200000, P_BI_TCXO, 1, 0, 0), F(220000000, P_GPU_CC_PLL1_OUT_MAIN, 2, 0, 0), F(550000000, P_GPU_CC_PLL1_OUT_MAIN, 2, 0, 0), freq_tbl::default()];
static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 4] = [F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), F(300000000, P_GPLL0_OUT_MAIN, 2, 0, 0), F(400000000, P_GPLL0_OUT_MAIN, 1.5, 0, 0), freq_tbl::default()];
static ftbl_gpu_cc_xo_clk_src: [freq_tbl; 2] = [F(19200000, P_BI_TCXO, 1, 0, 0), freq_tbl::default()];

// Clock objects preserve source register offsets, halt modes, enable masks,
// parent relationships, names, flags, and operation tables.
static mut gpu_cc_ff_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9474, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(), ..Default::default() };
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), ..Default::default() };
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x93ec, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_gpu_cc_hub_clk_src.as_ptr(), ..Default::default() };
static mut gpu_cc_xo_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9010, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_3.as_ptr(), freq_tbl: ftbl_gpu_cc_xo_clk_src.as_ptr(), ..Default::default() };

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_lucid_ole_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, config: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
}

unsafe fn gpu_cc_sm8550_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm8550_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll0, regmap, &gpu_cc_pll0_config);
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    qcom_branch_set_clk_en(regmap, 0x9004); // GPU_CC_CXO_AON_CLK
    qcom_branch_set_clk_en(regmap, 0x900c); // GPU_CC_DEMET_CLK
    qcom_cc_really_probe((*pdev).dev, &gpu_cc_sm8550_desc, regmap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
