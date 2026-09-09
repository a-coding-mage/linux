// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies supplied by the surrounding kernel clock framework.

enum {
    DT_BI_TCXO,
    DT_GPLL0_OUT_MAIN,
    DT_GPLL0_OUT_MAIN_DIV,
}

enum {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_OUT_MAIN,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2300000000, val: 0 }];

/* 936.0 MHz Configuration */
static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x30, alpha: 0xc000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 0x00400005,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, config: &gpu_cc_pll0_config, vco_table: lucid_ole_vco.as_ptr(),
    num_vco: lucid_ole_vco.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    }}},
};

/* 1250.0 MHz Configuration */
static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x41, alpha: 0x1aaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 0x00400005,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000, config: &gpu_cc_pll1_config, vco_table: lucid_ole_vco.as_ptr(),
    num_vco: lucid_ole_vco.len(), regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll1", parent_data: &clk_parent_data { index: DT_BI_TCXO },
        num_parents: 1, ops: &clk_alpha_pll_lucid_evo_ops,
    }}},
};

static gpu_cc_parent_map_0: [parent_map; 3] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_cc_parent_map_1: [parent_map; 5] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPU_CC_PLL0_OUT_MAIN, val: 1 },
    parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, val: 3 }, parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 },
    parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GPLL0_OUT_MAIN },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];
static gpu_cc_parent_map_2: [parent_map; 4] = [
    parent_map { parent: P_BI_TCXO, val: 0 }, parent_map { parent: P_GPU_CC_PLL1_OUT_MAIN, val: 3 },
    parent_map { parent: P_GPLL0_OUT_MAIN, val: 5 }, parent_map { parent: P_GPLL0_OUT_MAIN_DIV, val: 6 },
];
static gpu_cc_parent_data_2: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN }, clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV },
];

// F(...) and trailing empty frequency entries are direct translations of the kernel tables.
static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 2] = [F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0), freq_tbl {}];
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 3] = [F(416666667, P_GPU_CC_PLL1_OUT_MAIN, 3, 0, 0), F(625000000, P_GPU_CC_PLL1_OUT_MAIN, 2, 0, 0), freq_tbl {}];
static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 2] = [F(300000000, P_GPLL0_OUT_MAIN, 2, 0, 0), freq_tbl {}];

macro_rules! branch { ($n:ident, $r:expr, $h:ident, $ops:ident $(, $p:expr)?) => {
    static mut $n: clk_branch = clk_branch { halt_reg: $r, halt_check: $h,
        clkr: clk_regmap { enable_reg: $r, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
            name: stringify!($n), $(parent_hws: &[$p], num_parents: 1, flags: CLK_SET_RATE_PARENT, )? ops: &$ops,
        }}}};
}; }

// The following clock objects retain the original register offsets, halt policies, parents, and ops.
branch!(gpu_cc_acd_gfx3d_clk, 0x92a8, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_acmu_clk, 0x9294, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_ahb_clk, 0x9150, BRANCH_HALT_DELAY, clk_branch2_ops, unsafe { &gpu_cc_hub_div_clk_src.clkr.hw });
branch!(gpu_cc_crc_ahb_clk, 0x9154, BRANCH_HALT_VOTED, clk_branch2_ops, unsafe { &gpu_cc_hub_div_clk_src.clkr.hw });
branch!(gpu_cc_cx_accu_shift_clk, 0x91a4, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_cx_ff_clk, 0x9184, BRANCH_HALT, clk_branch2_ops, unsafe { &gpu_cc_ff_clk_src.clkr.hw });
branch!(gpu_cc_cx_gmu_clk, 0x916c, BRANCH_HALT_VOTED, clk_branch2_aon_ops, unsafe { &gpu_cc_gmu_clk_src.clkr.hw });
branch!(gpu_cc_cxo_clk, 0x917c, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_dpm_clk, 0x91a8, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_freq_measure_clk, 0x9008, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_gpu_smmu_vote_clk, 0x7000, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_hub_aon_clk, 0x9388, BRANCH_HALT_VOTED, clk_branch2_aon_ops, unsafe { &gpu_cc_hub_clk_src.clkr.hw });
branch!(gpu_cc_hub_cx_int_clk, 0x9180, BRANCH_HALT_VOTED, clk_branch2_aon_ops, unsafe { &gpu_cc_hub_clk_src.clkr.hw });
branch!(gpu_cc_memnoc_gfx_clk, 0x9188, BRANCH_HALT_VOTED, clk_branch2_ops);
branch!(gpu_cc_mnd1x_gfx3d_clk, 0x92ac, BRANCH_HALT, clk_branch2_ops);
branch!(gpu_cc_sleep_clk, 0x9164, BRANCH_HALT_VOTED, clk_branch2_ops);

static mut gpu_cc_ff_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x93d4, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::init("gpu_cc_ff_clk_src", &gpu_cc_parent_data_0, &clk_rcg2_shared_ops) };
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x92b8, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::init("gpu_cc_gmu_clk_src", &gpu_cc_parent_data_1, &clk_rcg2_shared_ops) };
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x938c, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_gpu_cc_hub_clk_src.as_ptr(), hw_clk_ctrl: true, clkr: clk_regmap::init("gpu_cc_hub_clk_src", &gpu_cc_parent_data_2, &clk_rcg2_shared_ops) };
static mut gpu_cc_hub_div_clk_src: clk_regmap_div = clk_regmap_div { reg: 0x93cc, shift: 0, width: 4, clkr: clk_regmap::init_hw("gpu_cc_hub_div_clk_src", unsafe { &[&gpu_cc_hub_clk_src.clkr.hw] }, CLK_SET_RATE_PARENT, &clk_regmap_div_ro_ops) };

static mut gpu_cc_cx_gdsc: gdsc = gdsc { gdscr: 0x90e8, gds_hw_ctrl: 0x9128, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd: generic_pm_domain { name: "gpu_cc_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE | RETAIN_FF_ENABLE };
static mut gpu_cc_gx_gdsc: gdsc = gdsc { gdscr: 0x905c, clamp_io_ctrl: 0x9504, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd: generic_pm_domain { name: "gpu_cc_gx_gdsc", power_on: gdsc_gx_do_nothing_enable }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | POLL_CFG_GDSCR | RETAIN_FF_ENABLE };

static mut gpu_cc_nord_clocks: [*mut clk_regmap; 22] = [
    &mut gpu_cc_acd_gfx3d_clk.clkr, &mut gpu_cc_acmu_clk.clkr, &mut gpu_cc_ahb_clk.clkr, &mut gpu_cc_crc_ahb_clk.clkr,
    &mut gpu_cc_cx_accu_shift_clk.clkr, &mut gpu_cc_cx_ff_clk.clkr, &mut gpu_cc_cx_gmu_clk.clkr, &mut gpu_cc_cxo_clk.clkr,
    &mut gpu_cc_dpm_clk.clkr, &mut gpu_cc_ff_clk_src.clkr, &mut gpu_cc_freq_measure_clk.clkr, &mut gpu_cc_gmu_clk_src.clkr,
    &mut gpu_cc_gpu_smmu_vote_clk.clkr, &mut gpu_cc_hub_aon_clk.clkr, &mut gpu_cc_hub_clk_src.clkr, &mut gpu_cc_hub_cx_int_clk.clkr,
    &mut gpu_cc_hub_div_clk_src.clkr, &mut gpu_cc_memnoc_gfx_clk.clkr, &mut gpu_cc_mnd1x_gfx3d_clk.clkr,
    &mut gpu_cc_pll0.clkr, &mut gpu_cc_pll1.clkr, &mut gpu_cc_sleep_clk.clkr,
];
static mut gpu_cc_nord_gdscs: [*mut gdsc; 2] = [&mut gpu_cc_cx_gdsc, &mut gpu_cc_gx_gdsc];
static gpu_cc_nord_resets: [qcom_reset_map; 7] = [qcom_reset_map { reg: 0x92f8 }, qcom_reset_map { reg: 0x9340 }, qcom_reset_map { reg: 0x90e4 }, qcom_reset_map { reg: 0x9384 }, qcom_reset_map { reg: 0x91ac }, qcom_reset_map { reg: 0x9058 }, qcom_reset_map { reg: 0x9000 }];
static mut gpu_cc_nord_plls: [*mut clk_alpha_pll; 2] = [&mut gpu_cc_pll0, &mut gpu_cc_pll1];
static gpu_cc_nord_critical_cbcrs: [u32; 2] = [0x9004, 0x900c];

static gpu_cc_nord_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9660, fast_io: true };
static gpu_cc_nord_driver_data: qcom_cc_driver_data = qcom_cc_driver_data { alpha_plls: gpu_cc_nord_plls.as_ptr(), num_alpha_plls: gpu_cc_nord_plls.len(), clk_cbcrs: gpu_cc_nord_critical_cbcrs.as_ptr(), num_clk_cbcrs: gpu_cc_nord_critical_cbcrs.len() };
static gpu_cc_nord_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_nord_regmap_config, clks: gpu_cc_nord_clocks.as_ptr(), num_clks: gpu_cc_nord_clocks.len(), resets: gpu_cc_nord_resets.as_ptr(), num_resets: gpu_cc_nord_resets.len(), gdscs: gpu_cc_nord_gdscs.as_ptr(), num_gdscs: gpu_cc_nord_gdscs.len(), use_rpm: true, driver_data: &gpu_cc_nord_driver_data };
static gpu_cc_nord_match_table: [of_device_id; 2] = [of_device_id { compatible: "qcom,nord-gpucc" }, of_device_id {}];

unsafe fn gpu_cc_nord_probe(pdev: *mut platform_device) -> i32 { qcom_cc_probe(pdev, &gpu_cc_nord_desc) }
static mut gpu_cc_nord_driver: platform_driver = platform_driver { probe: gpu_cc_nord_probe, driver: driver { name: "gpucc-nord", of_match_table: gpu_cc_nord_match_table.as_ptr() } };

// module_platform_driver(gpu_cc_nord_driver);
// MODULE_DESCRIPTION("QTI GPUCC Nord Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
