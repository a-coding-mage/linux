// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Linux/QCOM dependencies supplied by the surrounding kernel translation.

enum { DT_BI_TCXO, DT_GCC_GPU_GPLL0_CLK_SRC, DT_GCC_GPU_GPLL0_DIV_CLK_SRC, DT_GCC_GPU_SNOC_DVM_GFX_CLK }
enum { P_BI_TCXO, P_GCC_GPU_GPLL0_CLK_SRC, P_GCC_GPU_GPLL0_DIV_CLK_SRC, P_GPU_CC_PLL0_OUT_EVEN, P_GPU_CC_PLL0_OUT_MAIN, P_GPU_CC_PLL0_OUT_ODD, P_GPU_CC_PLL1_OUT_EVEN, P_GPU_CC_PLL1_OUT_MAIN, P_GPU_CC_PLL1_OUT_ODD }

static lucid_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

/* 532MHz Configuration */
static gpucc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x1b, alpha: 0xb555, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329a299c, user_ctl_val: 0x00000001, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0x00000000 };

static mut gpucc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: lucid_vco.as_ptr(), num_vco: lucid_vco.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpucc_pll0", parent_data: &clk_parent_data { index: P_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_lucid_ops } } },
};

/* 514MHz Configuration */
static gpucc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x1a, alpha: 0xc555, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329a299c, user_ctl_val: 0x00000001, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0x00000000 };

static mut gpucc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, vco_table: lucid_vco.as_ptr(), num_vco: lucid_vco.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpucc_pll1", parent_data: &clk_parent_data { index: P_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_lucid_ops } } },
};

static gpucc_parent_map_0: [parent_map; 5] = [parent_map { parent: P_BI_TCXO, cfg: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 }, parent_map { parent: P_GCC_GPU_GPLL0_CLK_SRC, cfg: 5 }, parent_map { parent: P_GCC_GPU_GPLL0_DIV_CLK_SRC, cfg: 6 }];
static gpucc_parent_data_0: [clk_parent_data; 5] = [clk_parent_data { index: P_BI_TCXO }, clk_parent_data { hw: unsafe { &gpucc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpucc_pll1.clkr.hw } }, clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }, clk_parent_data { index: DT_GCC_GPU_GPLL0_DIV_CLK_SRC }];
static gpucc_parent_map_1: [parent_map; 6] = [parent_map { parent: P_BI_TCXO, cfg: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_EVEN, cfg: 1 }, parent_map { parent: P_GPU_CC_PLL0_OUT_ODD, cfg: 2 }, parent_map { parent: P_GPU_CC_PLL1_OUT_EVEN, cfg: 3 }, parent_map { parent: P_GPU_CC_PLL1_OUT_ODD, cfg: 4 }, parent_map { parent: P_GCC_GPU_GPLL0_CLK_SRC, cfg: 5 }];
static gpucc_parent_data_1: [clk_parent_data; 6] = [clk_parent_data { index: P_BI_TCXO }, clk_parent_data { hw: unsafe { &gpucc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpucc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpucc_pll1.clkr.hw } }, clk_parent_data { hw: unsafe { &gpucc_pll1.clkr.hw } }, clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }];

static ftbl_gpucc_gmu_clk_src: [freq_tbl; 2] = [F(200000000, P_GCC_GPU_GPLL0_DIV_CLK_SRC, 1.5, 0, 0), freq_tbl {}];
static mut gpucc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: gpucc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpucc_gmu_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpucc_gmu_clk_src", parent_data: gpucc_parent_data_0.as_ptr(), num_parents: gpucc_parent_data_0.len(), ops: &clk_rcg2_shared_ops } } } };
static ftbl_gpucc_gx_gfx3d_clk_src: [freq_tbl; 8] = [F(266000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(390000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(490000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(650000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(770000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(840000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), F(900000000, P_GPU_CC_PLL0_OUT_EVEN, 2, 0, 0), freq_tbl {}];
static mut gpucc_gx_gfx3d_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x101c, mnd_width: 0, hid_width: 5, parent_map: gpucc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpucc_gx_gfx3d_clk_src.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpucc_gx_gfx3d_clk_src", parent_data: gpucc_parent_data_1.as_ptr(), num_parents: gpucc_parent_data_1.len(), flags: CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE, ops: &clk_rcg2_ops } } } };

// Branches retain the source register layout and initialization data.
static mut gpucc_ahb_clk: clk_branch = branch!("gpucc_ahb_clk", 0x1078, BRANCH_HALT_DELAY, 0x1078, CLK_IS_CRITICAL, clk_branch2_ops);
static mut gpucc_cx_gfx3d_clk: clk_branch = branch_parent!("gpucc_cx_gfx3d_clk", 0x10a4, BRANCH_HALT_DELAY, 0x10a4, gpucc_gx_gfx3d_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
static mut gpucc_cx_gfx3d_slv_clk: clk_branch = branch_parent!("gpucc_cx_gfx3d_slv_clk", 0x10a8, BRANCH_HALT_DELAY, 0x10a8, gpucc_gx_gfx3d_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
static mut gpucc_cx_gmu_clk: clk_branch = branch_parent!("gpucc_cx_gmu_clk", 0x1098, BRANCH_HALT, 0x1098, gpucc_gmu_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
static mut gpucc_cx_snoc_dvm_clk: clk_branch = branch_index!("gpucc_cx_snoc_dvm_clk", 0x108c, BRANCH_HALT_DELAY, 0x108c, DT_GCC_GPU_SNOC_DVM_GFX_CLK, clk_branch2_ops);
static mut gpucc_cxo_aon_clk: clk_branch = branch!("gpucc_cxo_aon_clk", 0x1004, BRANCH_HALT_DELAY, 0x1004, 0, clk_branch2_ops);
static mut gpucc_cxo_clk: clk_branch = branch!("gpucc_cxo_clk", 0x109c, BRANCH_HALT, 0x109c, 0, clk_branch2_ops);
static mut gpucc_gx_cxo_clk: clk_branch = branch!("gpucc_gx_cxo_clk", 0x1060, BRANCH_HALT_DELAY, 0x1060, CLK_IS_CRITICAL, clk_branch2_ops);
static mut gpucc_gx_gfx3d_clk: clk_branch = branch_parent!("gpucc_gx_gfx3d_clk", 0x1054, BRANCH_HALT_DELAY, 0x1054, gpucc_gx_gfx3d_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
static mut gpucc_gx_gmu_clk: clk_branch = branch_parent!("gpucc_gx_gmu_clk", 0x1064, BRANCH_HALT, 0x1064, gpucc_gmu_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
static mut gpucc_sleep_clk: clk_branch = branch!("gpucc_sleep_clk", 0x1090, BRANCH_HALT_VOTED, 0x1090, 0, clk_branch2_ops);

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, clk_dis_wait_val: 8, pd: generic_pd { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x100c, clamp_io_ctrl: 0x1508, resets: [GPU_GX_BCR, GPU_ACD_BCR, GPU_GX_ACD_MISC_BCR].as_ptr(), reset_count: 3, pd: generic_pd { name: "gpu_gx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | SW_RESET | AON_RESET };

static mut gpucc_sm6375_clocks: [*mut clk_regmap; 15] = [
    &mut gpucc_ahb_clk.clkr, &mut gpucc_cx_gfx3d_clk.clkr, &mut gpucc_cx_gfx3d_slv_clk.clkr, &mut gpucc_cx_gmu_clk.clkr, &mut gpucc_cx_snoc_dvm_clk.clkr, &mut gpucc_cxo_aon_clk.clkr, &mut gpucc_cxo_clk.clkr, &mut gpucc_gmu_clk_src.clkr, &mut gpucc_gx_cxo_clk.clkr, &mut gpucc_gx_gfx3d_clk.clkr, &mut gpucc_gx_gfx3d_clk_src.clkr, &mut gpucc_gx_gmu_clk.clkr, &mut gpucc_pll0.clkr, &mut gpucc_pll1.clkr, &mut gpucc_sleep_clk.clkr,
];
static gpucc_sm6375_resets: [qcom_reset_map; 3] = [qcom_reset_map { reg: 0x1008 }, qcom_reset_map { reg: 0x1160 }, qcom_reset_map { reg: 0x8004 }];
static mut gpucc_sm6375_gdscs: [*mut gdsc; 2] = [&mut gpu_cx_gdsc, &mut gpu_gx_gdsc];
static gpucc_sm6375_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9000, fast_io: true };
static gpucc_sm6375_desc: qcom_cc_desc = qcom_cc_desc { config: &gpucc_sm6375_regmap_config, clks: gpucc_sm6375_clocks.as_mut_ptr(), num_clks: gpucc_sm6375_clocks.len(), resets: gpucc_sm6375_resets.as_ptr(), num_resets: gpucc_sm6375_resets.len(), gdscs: gpucc_sm6375_gdscs.as_mut_ptr(), num_gdscs: gpucc_sm6375_gdscs.len() };

static gpucc_sm6375_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,sm6375-gpucc" }, of_device_id {}];
MODULE_DEVICE_TABLE!(of, gpucc_sm6375_match_table);

unsafe fn gpucc_sm6375_probe(pdev: *mut platform_device) -> c_int {
    let mut regmap: *mut regmap;
    let mut ret: c_int;
    ret = devm_pm_runtime_enable(&mut (*pdev).dev); if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev); if ret != 0 { return ret; }
    regmap = qcom_cc_map(pdev, &gpucc_sm6375_desc);
    if IS_ERR(regmap) { pm_runtime_put(&mut (*pdev).dev); return PTR_ERR(regmap); }
    clk_lucid_pll_configure(&mut gpucc_pll0, regmap, &gpucc_pll0_config);
    clk_lucid_pll_configure(&mut gpucc_pll1, regmap, &gpucc_pll1_config);
    ret = qcom_cc_really_probe(&mut (*pdev).dev, &gpucc_sm6375_desc, regmap);
    pm_runtime_put(&mut (*pdev).dev); ret
}

static mut gpucc_sm6375_driver: platform_driver = platform_driver { probe: Some(gpucc_sm6375_probe), driver: driver { name: "gpucc-sm6375", of_match_table: gpucc_sm6375_match_table.as_ptr() } };
module_platform_driver!(gpucc_sm6375_driver);
MODULE_DESCRIPTION!("QTI GPUCC SM6375 Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
