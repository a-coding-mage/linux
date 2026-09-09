// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

// Translated from the C implementation. Kernel-provided types, constants,
// operations, and helper functions are supplied by the surrounding crate.

const DT_BI_TCXO: usize = 0;
const DT_GCC_GPU_GPLL0_CLK_SRC: usize = 1;
const DT_GCC_GPU_GPLL0_DIV_CLK_SRC: usize = 2;

const P_BI_TCXO: usize = 0;
const P_GCC_GPU_GPLL0_CLK_SRC: usize = 1;
const P_GCC_GPU_GPLL0_DIV_CLK_SRC: usize = 2;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 3;
const P_GPU_CC_PLL1_OUT_MAIN: usize = 4;

static parent_data_tcxo: clk_parent_data = clk_parent_data { index: DT_BI_TCXO };

static lucid_5lpe_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 1800000000, val: 0 }];

static mut gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x1c, alpha: 0xa555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x2a9a699c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0x01800000,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0,
};

static mut gpu_cc_pll0: clk_alpha_pll = clk_alpha_pll {
    offset: 0, vco_table: lucid_5lpe_vco.as_ptr(), num_vco: lucid_5lpe_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll0", parent_data: &parent_data_tcxo, num_parents: 1,
        ops: &clk_alpha_pll_lucid_5lpe_ops,
    } } },
};

static mut gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x1A, alpha: 0xaaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x2a9a699c,
    test_ctl_val: 0, test_ctl_hi_val: 0, test_ctl_hi1_val: 0x01800000,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805, user_ctl_hi1_val: 0,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100, vco_table: lucid_5lpe_vco.as_ptr(), num_vco: lucid_5lpe_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_pll1", parent_data: &parent_data_tcxo, num_parents: 1,
        ops: &clk_alpha_pll_lucid_5lpe_ops,
    } } },
};

static gpu_cc_parent_map_0: [parent_map; 5] = [
    parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL0_OUT_MAIN, cfg: 1 },
    parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 }, parent_map { src: P_GCC_GPU_GPLL0_CLK_SRC, cfg: 5 },
    parent_map { src: P_GCC_GPU_GPLL0_DIV_CLK_SRC, cfg: 6 },
];
static gpu_cc_parent_data_0: [clk_parent_data; 5] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll0.clkr.hw } },
    clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } }, clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC },
    clk_parent_data { index: DT_GCC_GPU_GPLL0_DIV_CLK_SRC },
];
static gpu_cc_parent_map_1: [parent_map; 4] = [
    parent_map { src: P_BI_TCXO, cfg: 0 }, parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GCC_GPU_GPLL0_CLK_SRC, cfg: 5 }, parent_map { src: P_GCC_GPU_GPLL0_DIV_CLK_SRC, cfg: 6 },
];
static gpu_cc_parent_data_1: [clk_parent_data; 4] = [
    clk_parent_data { index: DT_BI_TCXO }, clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { index: DT_GCC_GPU_GPLL0_CLK_SRC }, clk_parent_data { index: DT_GCC_GPU_GPLL0_DIV_CLK_SRC },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [
    F!(19200000, P_BI_TCXO, 1, 0, 0), F!(200000000, P_GCC_GPU_GPLL0_DIV_CLK_SRC, 1.5, 0, 0),
    F!(500000000, P_GPU_CC_PLL1_OUT_MAIN, 1, 0, 0), freq_tbl::default(),
];
static ftbl_gpu_cc_hub_clk_src: [freq_tbl; 4] = [
    F!(200000000, P_GCC_GPU_GPLL0_CLK_SRC, 3, 0, 0), F!(300000000, P_GCC_GPU_GPLL0_CLK_SRC, 2, 0, 0),
    F!(400000000, P_GCC_GPU_GPLL0_CLK_SRC, 1.5, 0, 0), freq_tbl::default(),
];

// The remaining clock, power-domain, descriptor, probe, driver, and module
// definitions retain the C aggregate layout through the corresponding kernel
// Rust bindings.
static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_0.as_ptr(), freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(), clkr: clk_regmap::with_init("gpu_cc_gmu_clk_src", &gpu_cc_parent_data_0, &clk_rcg2_shared_ops) };
static mut gpu_cc_hub_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x117c, mnd_width: 0, hid_width: 5, parent_map: gpu_cc_parent_map_1.as_ptr(), freq_tbl: ftbl_gpu_cc_hub_clk_src.as_ptr(), clkr: clk_regmap::with_init("gpu_cc_hub_clk_src", &gpu_cc_parent_data_1, &clk_rcg2_shared_ops) };

// Branch clocks and divider clocks, preserving the source register values and
// parent relationships.  `clk_branch!` and `clk_divider!` are binding-level
// equivalents of the C designated initializers.
clk_divider!(gpu_cc_hub_ahb_div_clk_src, 0x11c0, 0, 4, gpu_cc_hub_clk_src, CLK_SET_RATE_PARENT, clk_regmap_div_ro_ops);
clk_divider!(gpu_cc_hub_cx_int_div_clk_src, 0x11bc, 0, 4, gpu_cc_hub_clk_src, CLK_SET_RATE_PARENT, clk_regmap_div_ro_ops);
clk_branch!(gpu_cc_ahb_clk, 0x1078, BRANCH_HALT_DELAY, "gpu_cc_ahb_clk", gpu_cc_hub_ahb_div_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
clk_branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_VOTED, "gpu_cc_crc_ahb_clk", gpu_cc_hub_ahb_div_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
clk_branch!(gpu_cc_cx_gmu_clk, 0x1098, BRANCH_HALT, "gpu_cc_cx_gmu_clk", gpu_cc_gmu_clk_src, CLK_SET_RATE_PARENT, clk_branch2_aon_ops);
clk_branch!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_VOTED, "gpu_cc_cx_snoc_dvm_clk", None, 0, clk_branch2_ops);
clk_branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_VOTED, "gpu_cc_cxo_aon_clk", None, 0, clk_branch2_ops);
clk_branch!(gpu_cc_gx_gmu_clk, 0x1064, BRANCH_HALT, "gpu_cc_gx_gmu_clk", gpu_cc_gmu_clk_src, CLK_SET_RATE_PARENT, clk_branch2_ops);
clk_branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x5000, BRANCH_HALT_VOTED, "gpu_cc_hlos1_vote_gpu_smmu_clk", None, 0, clk_branch2_ops);
clk_branch!(gpu_cc_hub_aon_clk, 0x1178, BRANCH_HALT, "gpu_cc_hub_aon_clk", gpu_cc_hub_clk_src, CLK_SET_RATE_PARENT, clk_branch2_aon_ops);
clk_branch!(gpu_cc_hub_cx_int_clk, 0x1204, BRANCH_HALT, "gpu_cc_hub_cx_int_clk", gpu_cc_hub_cx_int_div_clk_src, CLK_SET_RATE_PARENT, clk_branch2_aon_ops);
clk_branch!(gpu_cc_sleep_clk, 0x1090, BRANCH_HALT_VOTED, "gpu_cc_sleep_clk", None, 0, clk_branch2_ops);

static gpu_cc_sc8280xp_clocks: [*mut clk_regmap; 16] = [
    &gpu_cc_ahb_clk.clkr, &gpu_cc_crc_ahb_clk.clkr, &gpu_cc_cx_gmu_clk.clkr,
    &gpu_cc_cx_snoc_dvm_clk.clkr, &gpu_cc_cxo_aon_clk.clkr, &gpu_cc_gmu_clk_src.clkr,
    &gpu_cc_gx_gmu_clk.clkr, &gpu_cc_hlos1_vote_gpu_smmu_clk.clkr,
    &gpu_cc_hub_ahb_div_clk_src.clkr, &gpu_cc_hub_aon_clk.clkr, &gpu_cc_hub_clk_src.clkr,
    &gpu_cc_hub_cx_int_clk.clkr, &gpu_cc_hub_cx_int_div_clk_src.clkr,
    &gpu_cc_pll0.clkr, &gpu_cc_pll1.clkr, &gpu_cc_sleep_clk.clkr,
];
static mut cx_gdsc: gdsc = gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, name: "cx_gdsc", pwrsts: PWRSTS_OFF_ON, flags: VOTABLE | RETAIN_FF_ENABLE };
static mut gx_gdsc: gdsc = gdsc { gdscr: 0x100c, clamp_io_ctrl: 0x1508, name: "gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable), pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | RETAIN_FF_ENABLE, supply: "vdd-gfx" };
static gpu_cc_sc8280xp_gdscs: [*mut gdsc; 2] = [&mut cx_gdsc, &mut gx_gdsc];
static gpu_cc_sc8280xp_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8030, fast_io: true };
static gpu_cc_sc8280xp_desc: qcom_cc_desc = qcom_cc_desc { config: &gpu_cc_sc8280xp_regmap_config, clks: gpu_cc_sc8280xp_clocks.as_ptr(), num_clks: gpu_cc_sc8280xp_clocks.len(), gdscs: gpu_cc_sc8280xp_gdscs.as_ptr(), num_gdscs: gpu_cc_sc8280xp_gdscs.len() };

unsafe fn gpu_cc_sc8280xp_probe(pdev: *mut platform_device) -> i32 {
    let mut regmap: *mut regmap;
    let mut ret: i32;
    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    regmap = qcom_cc_map(pdev, &gpu_cc_sc8280xp_desc);
    if IS_ERR(regmap) {
        pm_runtime_put(&mut (*pdev).dev);
        return PTR_ERR(regmap);
    }
    clk_lucid_pll_configure(&mut gpu_cc_pll0, regmap, &gpu_cc_pll0_config);
    clk_lucid_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    qcom_branch_set_clk_en(regmap, 0x1170); // GPU_CC_CB_CLK
    qcom_branch_set_clk_en(regmap, 0x109c); // GPU_CC_CXO_CLK
    ret = qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_sc8280xp_desc, regmap);
    pm_runtime_put(&mut (*pdev).dev);
    ret
}

// C-only registration and the numerous clk_branch/clk_regmap_div aggregates
// are represented by the kernel binding's declarative registration macro.
qcom_gpucc_register!(gpu_cc_sc8280xp, gpu_cc_sc8280xp_probe,
    "qcom,sc8280xp-gpucc", "gpu_cc-sc8280xp", "Qualcomm SC8280XP GPU clock controller");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
