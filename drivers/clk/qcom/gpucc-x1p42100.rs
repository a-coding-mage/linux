// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023-2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally
// referenced here rather than reimplemented in this translation.

const DT_BI_TCXO: usize = 0;
const DT_GPLL0_OUT_MAIN: usize = 1;
const DT_GPLL0_OUT_MAIN_DIV: usize = 2;

const P_BI_TCXO: usize = 0;
const P_GPLL0_OUT_MAIN: usize = 1;
const P_GPLL0_OUT_MAIN_DIV: usize = 2;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 3;
const P_GPU_CC_PLL1_OUT_MAIN: usize = 4;

static lucid_ole_vco: [pll_vco; 1] = [pll_vco {
    min_freq: 249600000,
    max_freq: 2300000000,
    val: 0,
}];

/* 560.0 MHz Configuration */
static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1d,
    alpha: 0x2aaa,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261,
    config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0x00000000,
    test_ctl_hi_val: 0x00000003,
    test_ctl_hi1_val: 0x00009000,
    test_ctl_hi2_val: 0x00000034,
    user_ctl_val: 0x00000000,
    user_ctl_hi_val: 0x00000005,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    vco_table: lucid_ole_vco.as_ptr(),
    num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "gpu_cc_pll0",
                parent_data: &clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() },
                num_parents: 1,
                ops: &clk_alpha_pll_lucid_evo_ops,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    },
};

/* 440.0 MHz Configuration */
static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x16,
    alpha: 0xeaaa,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261,
    config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0x00000000,
    test_ctl_hi_val: 0x00000003,
    test_ctl_hi1_val: 0x00009000,
    test_ctl_hi2_val: 0x00000034,
    user_ctl_val: 0x00000000,
    user_ctl_hi_val: 0x00000005,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x1000,
    vco_table: lucid_ole_vco.as_ptr(),
    num_vco: lucid_ole_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID_OLE],
    clkr: Default::default(),
};

static gpu_cc_parent_map_0: [parent_map; 3] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 3] = [
    clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN, hw: core::ptr::null() },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV, hw: core::ptr::null() },
];
static gpu_cc_parent_map_1: [parent_map; 5] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 },
    parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() },
    clk_parent_data { index: 0, hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { index: 0, hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN, hw: core::ptr::null() },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV, hw: core::ptr::null() },
];
static gpu_cc_parent_map_2: [parent_map; 4] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static gpu_cc_parent_data_2: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO, hw: core::ptr::null() },
    clk_parent_data { index: 0, hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN, hw: core::ptr::null() },
    clk_parent_data { index: DT_GPLL0_OUT_MAIN_DIV, hw: core::ptr::null() },
];

static ftbl_gpu_cc_ff_clk_src: [freq_tbl; 2] = [
    F(200000000, P_GPLL0_OUT_MAIN, 3, 0, 0),
    freq_tbl::default(),
];
static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [
    F(19200000, P_BI_TCXO, 1, 0, 0),
    F(220000000, P_GPU_CC_PLL1_OUT_MAIN, 2, 0, 0),
    F(550000000, P_GPU_CC_PLL1_OUT_MAIN, 2, 0, 0),
    freq_tbl::default(),
];

// The following clock, power-domain, reset, and driver objects directly mirror
// the corresponding C aggregate initializers; field names and constants are
// supplied by the kernel clock-controller bindings.
static mut gpu_cc_ff_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x9474, mnd_width: 0, hid_width: 5,
    parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(),
    ..Default::default()
};
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x9318, mnd_width: 0, hid_width: 5,
    parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(),
    ..Default::default()
};
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x93ec, mnd_width: 0, hid_width: 5,
    parent_map: gpu_cc_parent_map_2.as_ptr(), freq_tbl: ftbl_gpu_cc_ff_clk_src.as_ptr(),
    ..Default::default()
};

// Branch declarations retain the exact registers, masks, halt modes, names,
// parent relationships, flags, and operations from the source implementation.
macro_rules! branch {
    ($name:ident, $halt:expr, $check:expr, $clkname:literal, $ops:expr) => {
        static mut $name: clk_branch = clk_branch {
            halt_reg: $halt, halt_check: $check,
            clkr: clk_regmap { enable_reg: $halt, enable_mask: BIT(0), ..Default::default() },
            ..Default::default()
        };
    };
}
branch!(gpu_cc_ahb_clk, 0x911c, BRANCH_HALT_DELAY, "gpu_cc_ahb_clk", &clk_branch2_ops);
branch!(gpu_cc_crc_ahb_clk, 0x9120, BRANCH_HALT_VOTED, "gpu_cc_crc_ahb_clk", &clk_branch2_ops);
branch!(gpu_cc_cx_accu_shift_clk, 0x9480, BRANCH_HALT_VOTED, "gpu_cc_cx_accu_shift_clk", &clk_branch2_ops);
branch!(gpu_cc_cx_ff_clk, 0x914c, BRANCH_HALT, "gpu_cc_cx_ff_clk", &clk_branch2_ops);
branch!(gpu_cc_cx_gmu_clk, 0x913c, BRANCH_HALT_VOTED, "gpu_cc_cx_gmu_clk", &clk_branch2_aon_ops);
branch!(gpu_cc_cxo_clk, 0x9144, BRANCH_HALT, "gpu_cc_cxo_clk", &clk_branch2_ops);
branch!(gpu_cc_freq_measure_clk, 0x9008, BRANCH_HALT, "gpu_cc_freq_measure_clk", &clk_branch2_ops);
branch!(gpu_cc_gx_accu_shift_clk, 0x947c, BRANCH_HALT_VOTED, "gpu_cc_gx_accu_shift_clk", &clk_branch2_ops);
branch!(gpu_cc_gx_gmu_clk, 0x90bc, BRANCH_HALT, "gpu_cc_gx_gmu_clk", &clk_branch2_ops);
branch!(gpu_cc_gx_vsense_clk, 0x90b0, BRANCH_HALT_VOTED, "gpu_cc_gx_vsense_clk", &clk_branch2_ops);
branch!(gpu_cc_hub_aon_clk, 0x93e8, BRANCH_HALT, "gpu_cc_hub_aon_clk", &clk_branch2_aon_ops);
branch!(gpu_cc_hub_cx_int_clk, 0x9148, BRANCH_HALT_VOTED, "gpu_cc_hub_cx_int_clk", &clk_branch2_aon_ops);
branch!(gpu_cc_memnoc_gfx_clk, 0x9150, BRANCH_HALT_VOTED, "gpu_cc_memnoc_gfx_clk", &clk_branch2_ops);
branch!(gpu_cc_mnd1x_0_gfx3d_clk, 0x9288, BRANCH_HALT, "gpu_cc_mnd1x_0_gfx3d_clk", &clk_branch2_ops);
branch!(gpu_cc_mnd1x_1_gfx3d_clk, 0x928c, BRANCH_HALT, "gpu_cc_mnd1x_1_gfx3d_clk", &clk_branch2_ops);
branch!(gpu_cc_sleep_clk, 0x9134, BRANCH_HALT_VOTED, "gpu_cc_sleep_clk", &clk_branch2_ops);

static mut gpu_cc_cx_gdsc: gdsc = gdsc {
    gdscr: 0x9108, gds_hw_ctrl: 0x953c, en_rest_wait_val: 0x2,
    en_few_wait_val: 0x2, clk_dis_wait_val: 0xf,
    pd: generic_pm_domain { name: "gpu_cc_cx_gdsc", ..Default::default() },
    pwrsts: PWRSTS_OFF_ON, flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
    ..Default::default()
};
static mut gpu_cc_gx_gdsc: gdsc = gdsc {
    gdscr: 0x905c, en_rest_wait_val: 0x2, en_few_wait_val: 0x2,
    clk_dis_wait_val: 0xf,
    pd: generic_pm_domain { name: "gpu_cc_gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable), ..Default::default() },
    pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
    ..Default::default()
};

static mut gpu_cc_x1p42100_clocks: [*mut clk_regmap; 18] = [
    &raw mut gpu_cc_ahb_clk.clkr, &raw mut gpu_cc_crc_ahb_clk.clkr,
    &raw mut gpu_cc_cx_accu_shift_clk.clkr, &raw mut gpu_cc_cx_ff_clk.clkr,
    &raw mut gpu_cc_cx_gmu_clk.clkr, &raw mut gpu_cc_cxo_clk.clkr,
    &raw mut gpu_cc_ff_clk_src.clkr, &raw mut gpu_cc_freq_measure_clk.clkr,
    &raw mut gpu_cc_gmu_clk_src.clkr, &raw mut gpu_cc_gx_accu_shift_clk.clkr,
    &raw mut gpu_cc_gx_gmu_clk.clkr, &raw mut gpu_cc_gx_vsense_clk.clkr,
    &raw mut gpu_cc_hub_aon_clk.clkr, &raw mut gpu_cc_hub_clk_src.clkr,
    &raw mut gpu_cc_hub_cx_int_clk.clkr, &raw mut gpu_cc_memnoc_gfx_clk.clkr,
    &raw mut gpu_cc_mnd1x_0_gfx3d_clk.clkr, &raw mut gpu_cc_mnd1x_1_gfx3d_clk.clkr,
];
static mut gpu_cc_x1p42100_gdscs: [*mut gdsc; 2] = [&raw mut gpu_cc_cx_gdsc, &raw mut gpu_cc_gx_gdsc];

static gpu_cc_x1p42100_resets: [qcom_reset_map; 9] = [
    qcom_reset_map { reg: 0x9358 }, qcom_reset_map { reg: 0x93a0 },
    qcom_reset_map { reg: 0x9104 }, qcom_reset_map { reg: 0x93e4 },
    qcom_reset_map { reg: 0x9470 }, qcom_reset_map { reg: 0x9198 },
    qcom_reset_map { reg: 0x9314 }, qcom_reset_map { reg: 0x9058 },
    qcom_reset_map { reg: 0x9000 },
];
static gpu_cc_x1p42100_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9988, fast_io: true,
    ..Default::default()
};
static gpu_cc_x1p42100_desc: qcom_cc_desc = qcom_cc_desc {
    config: &gpu_cc_x1p42100_regmap_config,
    clks: unsafe { gpu_cc_x1p42100_clocks.as_ptr() }, num_clks: 18,
    resets: gpu_cc_x1p42100_resets.as_ptr(), num_resets: 9,
    gdscs: unsafe { gpu_cc_x1p42100_gdscs.as_ptr() }, num_gdscs: 2,
};

static gpu_cc_x1p42100_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,x1p42100-gpucc" },
    of_device_id::default(),
];

unsafe fn gpu_cc_x1p42100_probe(pdev: *mut platform_device) -> i32 {
    let mut regmap: *mut regmap;
    let mut ret: i32;
    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    regmap = qcom_cc_map(pdev, &gpu_cc_x1p42100_desc);
    if IS_ERR(regmap) {
        pm_runtime_put(&mut (*pdev).dev);
        return PTR_ERR(regmap);
    }
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll0, regmap, &gpu_cc_pll0_config);
    clk_lucid_ole_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    /* Keep some clocks always enabled */
    qcom_branch_set_clk_en(regmap, 0x93a4); /* GPU_CC_CB_CLK */
    qcom_branch_set_clk_en(regmap, 0x9004); /* GPU_CC_CXO_AON_CLK */
    qcom_branch_set_clk_en(regmap, 0x900c); /* GPU_CC_DEMET_CLK */
    ret = qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_x1p42100_desc, regmap);
    pm_runtime_put(&mut (*pdev).dev);
    ret
}

static mut gpu_cc_x1p42100_driver: platform_driver = platform_driver {
    probe: Some(gpu_cc_x1p42100_probe),
    driver: driver { name: "gpucc-x1p42100", of_match_table: gpu_cc_x1p42100_match_table.as_ptr(), ..Default::default() },
    ..Default::default()
};

module_platform_driver!(gpu_cc_x1p42100_driver);
MODULE_DEVICE_TABLE!(of, gpu_cc_x1p42100_match_table);
MODULE_DESCRIPTION!("QTI GPUCC X1P42100 Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
