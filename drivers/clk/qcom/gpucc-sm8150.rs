// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017-2020, The Linux Foundation. All rights reserved.
 */

// Linux and Qualcomm headers provide the types, constants, macros, and
// external functions referenced below.

enum {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static TRION_VCO: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static mut gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x1a, alpha: 0xaaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002267, config_ctl_hi1_val: 0x00000024,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0x20,
    user_ctl_val: 0, user_ctl_hi_val: 0x805, user_ctl_hi1_val: 0xd0,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, vco_table: TRION_VCO.as_ptr(), num_vco: 1,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_TRION],
    clkr: clk_regmap { hw: clk_hw_init_data { init: &clk_init_data {
        name: "gpu_cc_pll1", parent_data: &clk_parent_data { fw_name: "bi_tcxo" },
        num_parents: 1, ops: &clk_alpha_pll_trion_ops,
    }}},
};

static gpu_cc_parent_map_0: [parent_map; 4] = [
    parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 }, parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];

static gpu_cc_parent_data_0: [clk_parent_data; 4] = [
    clk_parent_data { fw_name: "bi_tcxo" }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { fw_name: "gcc_gpu_gpll0_clk_src" }, clk_parent_data { fw_name: "gcc_gpu_gpll0_div_clk_src" },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [
    F!(19200000, P_BI_TCXO, 1, 0, 0), F!(200000000, P_GPLL0_OUT_MAIN_DIV, 1.5, 0, 0),
    F!(500000000, P_GPU_CC_PLL1_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];
static ftbl_gpu_cc_gmu_clk_src_sc8180x: [freq_tbl; 5] = [
    F!(19200000, P_BI_TCXO, 1, 0, 0), F!(200000000, P_GPLL0_OUT_MAIN_DIV, 1.5, 0, 0),
    F!(400000000, P_GPLL0_OUT_MAIN, 1.5, 0, 0), F!(500000000, P_GPU_CC_PLL1_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];

static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(),
    freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(),
    clkr: clk_regmap { hw: clk_hw_init_data { init: &clk_init_data {
        name: "gpu_cc_gmu_clk_src", parent_data: gpu_cc_parent_data_0.as_ptr(), num_parents: 4,
        flags: CLK_SET_RATE_PARENT, ops: &clk_rcg2_ops,
    }}}},
};

macro_rules! branch { ($name:ident, $reg:expr, $check:expr) => {
    static mut $name: clk_branch = clk_branch { halt_reg: $reg, halt_check: $check,
        clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw_init_data { init: &clk_init_data {
            name: stringify!($name), ops: &clk_branch2_ops,
        }}}};
} }
branch!(gpu_cc_ahb_clk, 0x1078, BRANCH_HALT_DELAY);
branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT);
branch!(gpu_cc_cx_apb_clk, 0x1088, BRANCH_HALT);
branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT);
branch!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT);

// Parent-linked branches retain their explicit source hardware relationships.
static mut gpu_cc_cx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x1098, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x1098, enable_mask: BIT(0), hw: clk_hw_init_data { init: &clk_init_data { name: "gpu_cc_cx_gmu_clk", parent_hws: &gpu_cc_gmu_clk_src.clkr.hw, num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops }}}};
static mut gpu_cc_gx_gmu_clk: clk_branch = clk_branch { halt_reg: 0x1064, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x1064, enable_mask: BIT(0), hw: clk_hw_init_data { init: &clk_init_data { name: "gpu_cc_gx_gmu_clk", parent_hws: &gpu_cc_gmu_clk_src.clkr.hw, num_parents: 1, flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops }}}};
static mut gpu_cc_cx_snoc_dvm_clk: clk_branch = clk_branch { halt_reg: 0x108c, halt_check: BRANCH_HALT, clkr: clk_regmap { enable_reg: 0x108c, enable_mask: BIT(0), hw: clk_hw_init_data { init: &clk_init_data { name: "gpu_cc_cx_snoc_dvm_clk", ops: &clk_branch2_ops }}}};

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, pd: generic_pd { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x100c, clamp_io_ctrl: 0x1508, pd: generic_pd { name: "gpu_gx_gdsc", power_on: gdsc_gx_do_nothing_enable }, pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | AON_RESET | POLL_CFG_GDSCR };

static gpu_cc_sm8150_clocks: [*mut clk_regmap; 10] = [
    &gpu_cc_ahb_clk.clkr, &gpu_cc_crc_ahb_clk.clkr, &gpu_cc_cx_apb_clk.clkr, &gpu_cc_cx_gmu_clk.clkr,
    &gpu_cc_cx_snoc_dvm_clk.clkr, &gpu_cc_cxo_aon_clk.clkr, &gpu_cc_cxo_clk.clkr, &gpu_cc_gmu_clk_src.clkr,
    &gpu_cc_gx_gmu_clk.clkr, &gpu_cc_pll1.clkr,
];
static gpu_cc_sm8150_resets: [qcom_reset_map; 5] = [qcom_reset_map { reg: 0x1068 }, qcom_reset_map { reg: 0x111c }, qcom_reset_map { reg: 0x1008 }, qcom_reset_map { reg: 0x1110 }, qcom_reset_map { reg: 0x1000 }];
static gpu_cc_sm8150_gdscs: [*mut gdsc; 2] = [&gpu_cx_gdsc, &gpu_gx_gdsc];
static gpu_cc_sm8150_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8008, fast_io: true };
static gpu_cc_sm8150_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_sm8150_regmap_config, clks: &gpu_cc_sm8150_clocks, num_clks: 10, resets: &gpu_cc_sm8150_resets, num_resets: 5, gdscs: &gpu_cc_sm8150_gdscs, num_gdscs: 2 };

static gpu_cc_sm8150_match_table: [of_device_id; 3] = [of_device_id { compatible: "qcom,sc8180x-gpucc" }, of_device_id { compatible: "qcom,sm8150-gpucc" }, of_device_id::default()];

unsafe extern "C" fn gpu_cc_sm8150_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm8150_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    if of_device_is_compatible((*pdev).dev.of_node, "qcom,sc8180x-gpucc") { gpu_cc_gmu_clk_src.freq_tbl = ftbl_gpu_cc_gmu_clk_src_sc8180x.as_ptr(); }
    clk_trion_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_sm8150_desc, regmap)
}

static mut gpu_cc_sm8150_driver: platform_driver = platform_driver { probe: Some(gpu_cc_sm8150_probe), driver: driver { name: "sm8150-gpucc", of_match_table: &gpu_cc_sm8150_match_table } };
// module_platform_driver(gpu_cc_sm8150_driver);
// MODULE_DESCRIPTION("QTI GPUCC SM8150 Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
