// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2020, AngeloGioacchino Del Regno
 *                     <angelogioacchino.delregno@somainline.org>
 */

// Translated from the Linux kernel implementation.  The referenced clock,
// regmap, platform, reset, and device-tree types are supplied by dependencies.

const P_GPU_XO: u32 = 0;
const P_GPLL0_OUT_MAIN: u32 = 1;
const P_GPLL0_OUT_MAIN_DIV: u32 = 2;
const P_GPU_PLL0_PLL_OUT_MAIN: u32 = 3;
const P_GPU_PLL1_PLL_OUT_MAIN: u32 = 4;

static mut gpucc_cxo_clk: clk_branch = clk_branch {
    halt_reg: 0x1020,
    clkr: clk_regmap { enable_reg: 0x1020, enable_mask: BIT(0), hw: clk_init_data {
        name: "gpucc_cxo_clk", parent_data: &[clk_parent_data { fw_name: Some("xo") }],
        num_parents: 1, ops: &clk_branch2_ops, flags: CLK_IS_CRITICAL,
    } },
};

static gpu_vco: [pll_vco; 3] = [
    pll_vco { min: 1000000000, max: 2000000000, val: 0 },
    pll_vco { min: 500000000, max: 1000000000, val: 2 },
    pll_vco { min: 250000000, max: 500000000, val: 3 },
];

static mut gpu_pll0_pll_out_main: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    vco_table: &gpu_vco, num_vco: gpu_vco.len(),
    clkr: clk_regmap { hw: clk_init_data { name: "gpu_pll0_pll_out_main",
        parent_hws: &[unsafe { &gpucc_cxo_clk.clkr.hw }], num_parents: 1,
        ops: &clk_alpha_pll_ops, } },
};

static mut gpu_pll1_pll_out_main: clk_alpha_pll = clk_alpha_pll {
    offset: 0x40, regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_DEFAULT],
    vco_table: &gpu_vco, num_vco: gpu_vco.len(),
    clkr: clk_regmap { hw: clk_init_data { name: "gpu_pll1_pll_out_main",
        parent_hws: &[unsafe { &gpucc_cxo_clk.clkr.hw }], num_parents: 1,
        ops: &clk_alpha_pll_ops, } },
};

static gpucc_parent_map_1: [parent_map; 4] = [
    parent_map { src: P_GPU_XO, cfg: 0 }, parent_map { src: P_GPU_PLL0_PLL_OUT_MAIN, cfg: 1 },
    parent_map { src: P_GPU_PLL1_PLL_OUT_MAIN, cfg: 3 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
];
static mut gpucc_parent_data_1: [clk_parent_data; 4] = [
    clk_parent_data { hw: Some(unsafe { &gpucc_cxo_clk.clkr.hw }) },
    clk_parent_data { hw: Some(unsafe { &gpu_pll0_pll_out_main.clkr.hw }) },
    clk_parent_data { hw: Some(unsafe { &gpu_pll1_pll_out_main.clkr.hw }) },
    clk_parent_data { fw_name: Some("gcc_gpu_gpll0_clk") },
];

static mut gfx3d_clk_src: clk_rcg2_gfx3d = clk_rcg2_gfx3d {
    div: 2, rcg: clk_rcg2 { cmd_rcgr: 0x1070, mnd_width: 0, hid_width: 5,
        parent_map: &gpucc_parent_map_1, clkr: clk_regmap { hw: clk_init_data {
            name: "gfx3d_clk_src", parent_data: unsafe { &gpucc_parent_data_1 },
            num_parents: 4, ops: &clk_gfx3d_ops, flags: CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE,
        } } },
    hws: &[unsafe { &gpucc_cxo_clk.clkr.hw }, unsafe { &gpu_pll0_pll_out_main.clkr.hw },
          unsafe { &gpu_pll1_pll_out_main.clkr.hw }],
};

static mut gpucc_gfx3d_clk: clk_branch = clk_branch {
    halt_reg: 0x1098, halt_check: BRANCH_HALT, hwcg_reg: 0x1098, hwcg_bit: 1,
    clkr: clk_regmap { enable_reg: 0x1098, enable_mask: BIT(0), hw: clk_init_data {
        name: "gpucc_gfx3d_clk", parent_hws: &[unsafe { &gfx3d_clk_src.rcg.clkr.hw }],
        num_parents: 1, ops: &clk_branch2_ops, flags: CLK_SET_RATE_PARENT,
    } },
};

static gpucc_parent_map_0: [parent_map; 3] = [
    parent_map { src: P_GPU_XO, cfg: 0 }, parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];
static gpucc_parent_data_0: [clk_parent_data; 3] = [
    clk_parent_data { hw: Some(unsafe { &gpucc_cxo_clk.clkr.hw }) },
    clk_parent_data { fw_name: Some("gcc_gpu_gpll0_clk") },
    clk_parent_data { fw_name: Some("gcc_gpu_gpll0_div_clk") },
];
static ftbl_rbbmtimer_clk_src: [freq_tbl; 2] = [ F(19200000, P_GPU_XO, 1, 0, 0), freq_tbl::EMPTY ];
static mut rbbmtimer_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x10b0, mnd_width: 0, hid_width: 5,
    parent_map: &gpucc_parent_map_0, freq_tbl: &ftbl_rbbmtimer_clk_src, clkr: clk_regmap { hw: clk_init_data {
        name: "rbbmtimer_clk_src", parent_data: &gpucc_parent_data_0, num_parents: 3, ops: &clk_rcg2_ops,
    } } };
static ftbl_rbcpr_clk_src: [freq_tbl; 3] = [ F(19200000, P_GPU_XO, 1, 0, 0), F(50000000, P_GPLL0_OUT_MAIN_DIV, 6, 0, 0), freq_tbl::EMPTY ];
static mut rbcpr_clk_src: clk_rcg2 = clk_rcg2 { cmd_rcgr: 0x1030, mnd_width: 0, hid_width: 5,
    parent_map: &gpucc_parent_map_0, freq_tbl: &ftbl_rbcpr_clk_src, clkr: clk_regmap { hw: clk_init_data {
        name: "rbcpr_clk_src", parent_data: &gpucc_parent_data_0, num_parents: 3, ops: &clk_rcg2_ops,
    } } };

static mut gpucc_rbbmtimer_clk: clk_branch = branch(0x10d0, "gpucc_rbbmtimer_clk", rbbmtimer_clk_src, CLK_SET_RATE_PARENT);
static mut gpucc_rbcpr_clk: clk_branch = branch(0x1054, "gpucc_rbcpr_clk", rbcpr_clk_src, CLK_SET_RATE_PARENT);

static mut gpu_cx_gdsc: gdsc = gdsc { gdscr: 0x1004, gds_hw_ctrl: 0x1008,
    pd: power_domain { name: "gpu_cx" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut gpu_gx_gdsc: gdsc = gdsc { gdscr: 0x1094, clamp_io_ctrl: 0x130,
    resets: &[GPU_GX_BCR], reset_count: 1, cxcs: &[0x1098], cxc_count: 1,
    pd: power_domain { name: "gpu_gx" }, parent: unsafe { &gpu_cx_gdsc.pd },
    pwrsts: PWRSTS_OFF | PWRSTS_ON | PWRSTS_RET, flags: CLAMP_IO | SW_RESET | AON_RESET | NO_RET_PERIPH };

static mut gpucc_sdm660_gdscs: [*mut gdsc; 2] = [unsafe { &mut gpu_cx_gdsc }, unsafe { &mut gpu_gx_gdsc }];
static gpucc_sdm660_resets: [qcom_reset_map; 4] = [
    qcom_reset_map { reg: 0x1000 }, qcom_reset_map { reg: 0x1050 },
    qcom_reset_map { reg: 0x1090 }, qcom_reset_map { reg: 0x10E0 },
];
static mut gpucc_sdm660_clocks: [*mut clk_regmap; 9] = [
    unsafe { &mut gpucc_cxo_clk.clkr }, unsafe { &mut gpu_pll0_pll_out_main.clkr }, unsafe { &mut gpu_pll1_pll_out_main.clkr },
    unsafe { &mut gfx3d_clk_src.rcg.clkr }, unsafe { &mut rbcpr_clk_src.clkr }, unsafe { &mut rbbmtimer_clk_src.clkr },
    unsafe { &mut gpucc_rbcpr_clk.clkr }, unsafe { &mut gpucc_gfx3d_clk.clkr }, unsafe { &mut gpucc_rbbmtimer_clk.clkr },
];
static gpucc_660_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9034, fast_io: true };
static gpucc_sdm660_desc: qcom_cc_desc = qcom_cc_desc { config: &gpucc_660_regmap_config,
    clks: &gpucc_sdm660_clocks, num_clks: 9, resets: &gpucc_sdm660_resets, num_resets: 4,
    gdscs: &gpucc_sdm660_gdscs, num_gdscs: 2 };
static gpucc_sdm660_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,gpucc-sdm660" }, of_device_id { compatible: "qcom,gpucc-sdm630" }, of_device_id::EMPTY,
];

unsafe fn gpucc_sdm660_probe(pdev: *mut platform_device) -> i32 {
    let mut regmap: *mut regmap;
    let mut gpu_pll_config = alpha_pll_config { config_ctl_val: 0x4001055b, alpha: 0xaaaaab00,
        alpha_en_mask: BIT(24), vco_val: 0x2 << 20, vco_mask: 0x3 << 20, main_output_mask: 0x1, ..Default::default() };
    regmap = qcom_cc_map(pdev, &gpucc_sdm660_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    // 800MHz configuration for GPU PLL0
    gpu_pll_config.l = 0x29; gpu_pll_config.alpha_hi = 0xaa;
    clk_alpha_pll_configure(&mut gpu_pll0_pll_out_main, regmap, &gpu_pll_config);
    // 740MHz configuration for GPU PLL1
    gpu_pll_config.l = 0x26; gpu_pll_config.alpha_hi = 0x8a;
    clk_alpha_pll_configure(&mut gpu_pll1_pll_out_main, regmap, &gpu_pll_config);
    qcom_cc_really_probe(&(*pdev).dev, &gpucc_sdm660_desc, regmap)
}

static mut gpucc_sdm660_driver: platform_driver = platform_driver { probe: Some(gpucc_sdm660_probe), driver: driver {
    name: "gpucc-sdm660", of_match_table: &gpucc_sdm660_match_table,
} };

// module_platform_driver!(gpucc_sdm660_driver);
// MODULE_DEVICE_TABLE(of, gpucc_sdm660_match_table);
// MODULE_DESCRIPTION("Qualcomm SDM630/SDM660 GPUCC Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
