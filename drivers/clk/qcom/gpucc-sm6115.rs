// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Dependencies supplied by the kernel clock, platform, regmap, device-tree,
// PLL, branch, RCG, GDSC, and reset subsystems are intentionally external.

enum {
    DT_BI_TCXO,
    DT_GCC_GPU_GPLL0_CLK_SRC,
    DT_GCC_GPU_GPLL0_DIV_CLK_SRC,
}

enum {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_OUT_AUX2,
    P_GPU_CC_PLL0_OUT_MAIN,
    P_GPU_CC_PLL1_OUT_AUX,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static DEFAULT_VCO: [pll_vco; 1] = [pll_vco { min_freq: 1000000000, max_freq: 2000000000, val: 0 }];
static PLL1_VCO: [pll_vco; 1] = [pll_vco { min_freq: 500000000, max_freq: 1000000000, val: 2 }];

static GPU_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x3e, alpha: 0, alpha_hi: 0x80, vco_val: 0x0 << 20,
    vco_mask: GENMASK(21, 20), alpha_en_mask: BIT(24), main_output_mask: BIT(0),
    aux_output_mask: BIT(1), aux2_output_mask: BIT(2), config_ctl_val: 0x4001055b,
    test_ctl_hi1_val: 0x1,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: DEFAULT_VCO.as_ptr(), num_vco: ARRAY_SIZE(DEFAULT_VCO),
    flags: SUPPORTS_DYNAMIC_UPDATE, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_ops } } },
};

static POST_DIV_TABLE_GPU_CC_PLL0_OUT_AUX2: [clk_div_table; 2] = [
    clk_div_table { val: 0x0, div: 1 }, clk_div_table { val: 0, div: 0 },
];
static mut gpu_cc_pll0_out_aux2: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x0, post_div_shift: 8, post_div_table: POST_DIV_TABLE_GPU_CC_PLL0_OUT_AUX2.as_ptr(), num_post_div: ARRAY_SIZE(POST_DIV_TABLE_GPU_CC_PLL0_OUT_AUX2), width: 4,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT], clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll0_out_aux2", parent_hws: [&mut gpu_cc_pll0.clkr.hw as *mut _ as *const _].as_ptr(), num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_ops } } },
};

static GPU_CC_PLL1_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x21, alpha: 0x55555555, alpha_hi: 0x55, alpha_en_mask: BIT(24),
    vco_val: 0x2 << 20, vco_mask: GENMASK(21, 20), main_output_mask: BIT(0),
    aux_output_mask: BIT(1), config_ctl_val: 0x4001055b, test_ctl_hi1_val: 0x1,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, vco_table: PLL1_VCO.as_ptr(), num_vco: ARRAY_SIZE(PLL1_VCO),
    flags: SUPPORTS_DYNAMIC_UPDATE, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_ops } } },
};

static POST_DIV_TABLE_GPU_CC_PLL1_OUT_AUX: [clk_div_table; 2] = [clk_div_table { val: 0x0, div: 1 }, clk_div_table { val: 0, div: 0 }];
static mut gpu_cc_pll1_out_aux: clk_alpha_pll_postdiv = clk_alpha_pll_postdiv {
    offset: 0x100, post_div_shift: 15, post_div_table: POST_DIV_TABLE_GPU_CC_PLL1_OUT_AUX.as_ptr(), num_post_div: ARRAY_SIZE(POST_DIV_TABLE_GPU_CC_PLL1_OUT_AUX), width: 3,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT], clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll1_out_aux", parent_hws: [&mut gpu_cc_pll1.clkr.hw as *mut _ as *const _].as_ptr(), num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_alpha_pll_postdiv_ops } } },
};

static GPU_CC_PARENT_MAP_0: [parent_map; 5] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, value: 1 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static GPU_CC_PARENT_DATA_0: [clk_parent_data; 5] = [clk_parent_data { index: P_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }, clk_parent_data { index: DT_GCC_GPU_GPLL0_DIV_CLK_SRC }];
static GPU_CC_PARENT_MAP_1: [parent_map; 4] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_AUX2, value: 2 }, parent_map { parent: P_GPU_CC_PLL1_OUT_AUX, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }];
static GPU_CC_PARENT_DATA_1: [clk_parent_data; 4] = [clk_parent_data { index: P_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0_out_aux2.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll1_out_aux.clkr.hw } }, clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }];

static FTBL_GPU_CC_GMU_CLK_SRC: [freq_tbl; 2] = [F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl { ..Default::default() }];
static FTBL_GPU_CC_GX_GFX3D_CLK_SRC: [freq_tbl; 9] = [F(320000000, P_GPU_CC_PLL1_OUT_AUX, 2, 0, 0), F(465000000, P_GPU_CC_PLL1_OUT_AUX, 2, 0, 0), F(600000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), F(745000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), F(820000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), F(900000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), F(950000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), F(980000000, P_GPU_CC_PLL0_OUT_AUX2, 2, 0, 0), freq_tbl { ..Default::default() }];

// The remaining clock, power-domain, reset, descriptor, match-table, and
// driver declarations retain the source layout and refer to external kernel
// types and constants supplied by the surrounding translation unit.
extern "C" {
    static mut gpu_cc_gmu_clk_src: clk_rcg2;
    static mut gpu_cc_gx_gfx3d_clk_src: clk_rcg2;
    static mut gpu_cc_ahb_clk: clk_branch;
    static mut gpu_cc_crc_ahb_clk: clk_branch;
    static mut gpu_cc_cx_gfx3d_clk: clk_branch;
    static mut gpu_cc_cx_gmu_clk: clk_branch;
    static mut gpu_cc_cx_snoc_dvm_clk: clk_branch;
    static mut gpu_cc_cxo_aon_clk: clk_branch;
    static mut gpu_cc_cxo_clk: clk_branch;
    static mut gpu_cc_gx_cxo_clk: clk_branch;
    static mut gpu_cc_gx_gfx3d_clk: clk_branch;
    static mut gpu_cc_sleep_clk: clk_branch;
    static mut gpu_cc_hlos1_vote_gpu_smmu_clk: clk_branch;
    static mut gpu_cx_gdsc: gdsc;
    static mut gpu_gx_gdsc: gdsc;
}

unsafe fn gpu_cc_sm6115_probe(pdev: *mut platform_device) -> c_int {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm6115_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    clk_alpha_pll_configure(&mut gpu_cc_pll0, regmap, &GPU_CC_PLL0_CONFIG);
    clk_alpha_pll_configure(&mut gpu_cc_pll1, regmap, &GPU_CC_PLL1_CONFIG);
    qcom_branch_set_wakeup(regmap, gpu_cc_cx_gmu_clk, 0xf);
    qcom_branch_set_sleep(regmap, gpu_cc_cx_gmu_clk, 0xf);
    qcom_branch_set_force_mem_core(regmap, gpu_cc_gx_gfx3d_clk, true);
    qcom_branch_set_force_periph_on(regmap, gpu_cc_gx_gfx3d_clk, true);
    qcom_cc_really_probe((*pdev).dev, &gpu_cc_sm6115_desc, regmap)
}

// MODULE_DEVICE_TABLE(of, gpu_cc_sm6115_match_table);
// module_platform_driver(gpu_cc_sm6115_driver);
// MODULE_DESCRIPTION("QTI GPU_CC SM6115 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
