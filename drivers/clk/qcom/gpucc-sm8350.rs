// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Linaro Limited
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Linux clock, platform, regmap, Qualcomm clock, reset, and device-tree
// declarations are supplied by the surrounding kernel Rust bindings.

#[repr(usize)]
enum Parent { PBiTcxo, PGpll0OutMain, PGpll0OutMainDiv, PGpuCcPll0OutMain, PGpuCcPll1OutMain }

static LUCID_5LPE_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 1750000000, val: 0 }];

static GPU_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x18, alpha: 0x6000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261,
    config_ctl_hi1_val: 0x2a9a699c, test_ctl_val: 0, test_ctl_hi_val: 0,
    test_ctl_hi1_val: 0x01800000, user_ctl_val: 0, user_ctl_hi_val: 0x805, user_ctl_hi1_val: 0,
};
static GPU_CC_PARENT: clk_parent_data = clk_parent_data { fw_name: "bi_tcxo", hw: core::ptr::null() };

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, vco_table: LUCID_5LPE_VCO.as_ptr(), num_vco: 1,
    regs: unsafe { clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID] },
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll0", parent_data: &GPU_CC_PARENT, num_parents: 1, ops: &clk_alpha_pll_lucid_5lpe_ops } } },
};
static GPU_CC_PLL1_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x1a, alpha: 0xaaa, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261,
    config_ctl_hi1_val: 0x2a9a699c, test_ctl_val: 0, test_ctl_hi_val: 0,
    test_ctl_hi1_val: 0x01800000, user_ctl_val: 0, user_ctl_hi_val: 0x805, user_ctl_hi1_val: 0,
};
static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, vco_table: LUCID_5LPE_VCO.as_ptr(), num_vco: 1,
    regs: unsafe { clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID] },
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll1", parent_data: &GPU_CC_PARENT, num_parents: 1, ops: &clk_alpha_pll_lucid_5lpe_ops } } },
};

static GPU_CC_PARENT_MAP_0: [parent_map; 5] = [
    parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 },
    parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static GPU_CC_PARENT_MAP_1: [parent_map; 4] = [
    parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];

static FTBL_GPU_CC_GMU_CLK_SRC: [freq_tbl; 4] = [
    F(19200000, P_BI_TCXO, 1, 0, 0), F(200000000, P_GPLL0_OUT_MAIN_DIV, 1.5, 0, 0),
    F(500000000, P_GPU_CC_PLL1_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: GPU_CC_PARENT_MAP_0.as_ptr(), freq_tbl: FTBL_GPU_CC_GMU_CLK_SRC.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_gmu_clk_src", parent_data: core::ptr::null(), num_parents: 5, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
static FTBL_GPU_CC_HUB_CLK_SRC: [freq_tbl; 4] = [
    F(150000000, P_GPLL0_OUT_MAIN_DIV, 2, 0, 0), F(240000000, P_GPLL0_OUT_MAIN, 2.5, 0, 0),
    F(300000000, P_GPLL0_OUT_MAIN, 2, 0, 0), freq_tbl::default(),
];
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x117c, mnd_width: 0, hid_width: 5, parent_map: GPU_CC_PARENT_MAP_1.as_ptr(), freq_tbl: FTBL_GPU_CC_HUB_CLK_SRC.as_ptr(), clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_hub_clk_src", parent_data: core::ptr::null(), num_parents: 4, flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_shared_ops } } } };
static mut gpu_cc_hub_ahb_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0x11c0, shift: 0, width: 4, clkr: clk_regmap::default() };
static mut gpu_cc_hub_cx_int_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0x11bc, shift: 0, width: 4, clkr: clk_regmap::default() };

// The following declarations preserve the C driver's clock topology and
// register programming data.  The binding types and constants are external.
macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $ops:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt,
        clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data { name: stringify!($name), ops: &$ops } } } };
}; }
branch!(gpu_cc_ahb_clk, 0x1078, BRANCH_HALT_DELAY, clk_branch2_ops);
branch!(gpu_cc_cb_clk, 0x1170, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_apb_clk, 0x1088, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_gmu_clk, 0x1098, BRANCH_HALT, clk_branch2_aon_ops);
branch!(gpu_cc_cx_qdss_at_clk, 0x1080, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_qdss_trig_clk, 0x1094, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_qdss_tsctr_clk, 0x1084, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_freq_measure_clk, 0x120c, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_gx_gmu_clk, 0x1064, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_gx_qdss_tsctr_clk, 0x105c, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_gx_vsense_clk, 0x1058, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x5000, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_hub_aon_clk, 0x1178, BRANCH_HALT, clk_branch2_aon_ops);
branch!(gpu_cc_hub_cx_int_clk, 0x1204, BRANCH_HALT, clk_branch2_aon_ops);
branch!(gpu_cc_mnd1x_0_gfx3d_clk, 0x802c, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_mnd1x_1_gfx3d_clk, 0x8030, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_sleep_clk, 0x1090, BRANCH_HALT_VOTED, clk_branch2_ops);

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, pd: generic_pm_domain { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x100c, clamp_io_ctrl: 0x1508, pd: generic_pm_domain { name: "gpu_gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable) }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | AON_RESET | POLL_CFG_GDSCR };

static GPU_CC_SM8350_RESETS: [qcom_reset_map; 8] = [
    qcom_reset_map { reg: 0x1160 }, qcom_reset_map { reg: 0x116c }, qcom_reset_map { reg: 0x1068 },
    qcom_reset_map { reg: 0x1174 }, qcom_reset_map { reg: 0x10a0 }, qcom_reset_map { reg: 0x111c },
    qcom_reset_map { reg: 0x1008 }, qcom_reset_map { reg: 0x1000 },
];
static GPU_CC_SM8350_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8030, fast_io: true };
static mut GPU_CC_SM8350_CLOCKS: [*mut clk_regmap; 30] = [core::ptr::null_mut(); 30];
static mut GPU_CC_SM8350_GDSCS: [*mut gdsc; 2] = [&mut gpu_cx_gdsc, &mut gpu_gx_gdsc];
static GPU_CC_SM8350_DESC: qcom_cc_desc = qcom_cc_desc { config: &GPU_CC_SM8350_REGMAP_CONFIG, clks: GPU_CC_SM8350_CLOCKS.as_ptr(), num_clks: 30, resets: GPU_CC_SM8350_RESETS.as_ptr(), num_resets: 8, gdscs: GPU_CC_SM8350_GDSCS.as_ptr(), num_gdscs: 2 };

unsafe fn gpu_cc_sm8350_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &GPU_CC_SM8350_DESC);
    if IS_ERR(regmap) { dev_err(&(*pdev).dev, "Failed to map gpu cc registers\n"); return PTR_ERR(regmap); }
    clk_lucid_pll_configure(&mut gpu_cc_pll0, regmap, &GPU_CC_PLL0_CONFIG);
    clk_lucid_pll_configure(&mut gpu_cc_pll1, regmap, &GPU_CC_PLL1_CONFIG);
    qcom_cc_really_probe(&(*pdev).dev, &GPU_CC_SM8350_DESC, regmap)
}
static GPU_CC_SM8350_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: "qcom,sm8350-gpucc" }, of_device_id { compatible: "" }];
static mut gpu_cc_sm8350_driver: platform_driver = platform_driver { probe: Some(gpu_cc_sm8350_probe), driver: driver { name: "sm8350-gpucc", of_match_table: GPU_CC_SM8350_MATCH_TABLE.as_ptr() } };
module_platform_driver!(gpu_cc_sm8350_driver);
MODULE_DESCRIPTION!("QTI GPU_CC SM8350 Driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
