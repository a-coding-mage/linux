// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2024, Linaro Limited
 */

// Linux clock, platform, regmap, device-tree, PLL, branch, RCG, GDSC, and
// reset declarations are supplied by the surrounding translation unit.

#[repr(C)]
pub struct GpuCcSar2130p;

pub const DT_BI_TCXO: u32 = 0;
pub const DT_GPLL0_OUT_MAIN: u32 = 1;
pub const DT_GPLL0_OUT_MAIN_DIV: u32 = 2;

pub const P_BI_TCXO: u32 = 0;
pub const P_GPLL0_OUT_MAIN: u32 = 1;
pub const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
pub const P_GPU_CC_PLL0_OUT_MAIN: u32 = 3;
pub const P_GPU_CC_PLL1_OUT_MAIN: u32 = 4;

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x18, alpha: 0x7aaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, vco_table: lucid_ole_vco.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops } } },
};

static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x16, alpha: 0xeaaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

static gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, vco_table: lucid_ole_vco.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data { name: "gpu_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO }, num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops } } },
};

static gpu_cc_parent_map_0: [parent_map; 3] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_data_0: [clk_parent_data; 3] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV }];
static gpu_cc_parent_map_1: [parent_map; 5] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, value: 1 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_data_1: [clk_parent_data; 5] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV }];
static gpu_cc_parent_map_2: [parent_map; 4] = [parent_map { parent: P_BI_TCXO, value: 0 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, value: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, value: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, value: 6 }];
static gpu_cc_parent_data_2: [clk_parent_data; 4] = [clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV }];

static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 2] = [freq_tbl { freq: 200000000, src: P_GPLL0_OUT_MAIN, pre_div: 3, m: 0, n: 0 }, freq_tbl::EMPTY];
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [freq_tbl { freq: 19200000, src: P_BI_TCXO, pre_div: 1, m: 0, n: 0 }, freq_tbl { freq: 220000000, src: P_GPU_CC_PLL1_OUT_MAIN, pre_div: 2, m: 0, n: 0 }, freq_tbl { freq: 550000000, src: P_GPU_CC_PLL1_OUT_MAIN, pre_div: 2, m: 0, n: 0 }, freq_tbl::EMPTY];

// The following clock, power-domain, reset, descriptor, match-table, and
// driver objects preserve the source declarations and their register values.
static gpu_cc_ff_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9474, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(), clkr: clk_regmap::with_init("gpu_cc_ff_clk_src", gpu_cc_parent_data_0.as_ptr(), 3, &clk_rcg2_shared_ops) };
static gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), clkr: clk_regmap::with_flags("gpu_cc_gmu_clk_src", gpu_cc_parent_data_1.as_ptr(), 5, CLK_SET_RATE_PARENT, &clk_rcg2_shared_ops) };
static gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x93ec, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(), clkr: clk_regmap::with_init("gpu_cc_hub_clk_src", gpu_cc_parent_data_2.as_ptr(), 4, &clk_rcg2_shared_ops) };

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr, $ops:expr) => { static $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $halt, clkr: clk_regmap::branch($reg, $name##_name, $ops) }; }; }
branch!(gpu_cc_ahb_clk, 0x911c, BRANCH_HALT_DELAY, &clk_branch2_ops);
branch!(gpu_cc_crc_ahb_clk, 0x9120, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_cx_ff_clk, 0x914c, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_cx_gmu_clk, 0x913c, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_cc_cxo_aon_clk, 0x9004, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_cxo_clk, 0x9144, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_gx_gmu_clk, 0x90bc, BRANCH_HALT, &clk_branch2_ops);
branch!(gpu_cc_hub_aon_clk, 0x93e8, BRANCH_HALT, &clk_branch2_aon_ops);
branch!(gpu_cc_hub_cx_int_clk, 0x9148, BRANCH_HALT_VOTED, &clk_branch2_aon_ops);
branch!(gpu_cc_memnoc_gfx_clk, 0x9150, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x7000, BRANCH_HALT_VOTED, &clk_branch2_ops);
branch!(gpu_cc_sleep_clk, 0x9134, BRANCH_HALT_VOTED, &clk_branch2_ops);

static gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x9108, gds_hw_ctrl: 0x953c, clk_dis_wait_val: 8, pd: generic_pm_domain { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE | RETAIN_FF_ENABLE };
static gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x905c, clamp_io_ctrl: 0x9504, resets: [GPUCC_GPU_CC_GX_BCR, GPUCC_GPU_CC_ACD_BCR, GPUCC_GPU_CC_GX_ACD_IROOT_BCR].as_ptr(), reset_count: 3, pd: generic_pm_domain { name: "gpu_gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable) }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | AON_RESET | SW_RESET };

static gpu_cc_sar2130p_resets: [qcom_reset_map; 3] = [qcom_reset_map { reg: 0x9358 }, qcom_reset_map { reg: 0x958c }, qcom_reset_map { reg: 0x9058 }];
static gpu_cc_sar2130p_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0xa000, fast_io: true };
static gpu_cc_sar2130p_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,sar2130p-gpucc" }, of_device_id::EMPTY];

unsafe fn gpu_cc_sar2130p_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let regmap = qcom_cc_map(pdev, &gpu_cc_sar2130p_desc);
    if IS_ERR(regmap) { return dev_err_probe(dev, PTR_ERR(regmap), "Couldn't map GPU_CC\n"); }
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll0, regmap, &gpu_cc_pll0_config);
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    qcom_branch_set_clk_en(regmap, 0x900c); // GPU_CC_DEMET_CLK
    qcom_cc_really_probe(dev, &gpu_cc_sar2130p_desc, regmap)
}

static gpu_cc_sar2130p_driver: platform_driver = platform_driver { probe: Some(gpu_cc_sar2130p_probe), name: "gpu_cc-sar2130p", of_match_table: gpu_cc_sar2130p_match_table.as_ptr() };

// module_platform_driver(gpu_cc_sar2130p_driver)
// MODULE_DEVICE_TABLE(of, gpu_cc_sar2130p_match_table)
// MODULE_DESCRIPTION("QTI GPU_CC SAR2130P Driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
