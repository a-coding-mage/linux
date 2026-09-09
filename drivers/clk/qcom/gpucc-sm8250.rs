// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux clock, Qualcomm clock, reset, and GDSC
// subsystems are intentionally referenced but not implemented here.

const CX_GMU_CBCR_SLEEP_MASK: u32 = 0xf;
const CX_GMU_CBCR_SLEEP_SHIFT: u32 = 4;
const CX_GMU_CBCR_WAKE_MASK: u32 = 0xf;
const CX_GMU_CBCR_WAKE_SHIFT: u32 = 8;

enum {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL0_OUT_MAIN,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static lucid_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x1a,
    alpha: 0xaaa,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00002261,
    config_ctl_hi1_val: 0x029a699c,
    user_ctl_val: 0x00000000,
    user_ctl_hi_val: 0x00000805,
    user_ctl_hi1_val: 0x00000000,
};

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100,
    vco_table: lucid_vco.as_ptr(),
    num_vco: lucid_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "gpu_cc_pll1",
                parent_data: &clk_parent_data { fw_name: "bi_tcxo" },
                num_parents: 1,
                ops: &clk_alpha_pll_lucid_ops,
            },
        },
    },
};

static gpu_cc_parent_map_0: [parent_map; 4] = [
    parent_map { src: P_BI_TCXO, cfg: 0 },
    parent_map { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    parent_map { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    parent_map { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];

static gpu_cc_parent_data_0: [clk_parent_data; 4] = [
    clk_parent_data { fw_name: "bi_tcxo" },
    clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { fw_name: "gcc_gpu_gpll0_clk_src" },
    clk_parent_data { fw_name: "gcc_gpu_gpll0_div_clk_src" },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 4] = [
    FREQ(19200000, P_BI_TCXO, 1, 0, 0),
    FREQ(200000000, P_GPLL0_OUT_MAIN_DIV, 1.5, 0, 0),
    FREQ(500000000, P_GPU_CC_PLL1_OUT_MAIN, 1, 0, 0),
    freq_tbl {},
];

static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x1120,
    mnd_width: 0,
    hid_width: 5,
    parent_map: gpu_cc_parent_map_0.as_ptr(),
    freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(),
    clkr: clk_regmap_init!("gpu_cc_gmu_clk_src", gpu_cc_parent_data_0.as_ptr(), gpu_cc_parent_data_0.len(), CLK_SET_RATE_PARENT, &clk_rcg2_ops),
};

macro_rules! branch { ($name:ident, $reg:expr, $halt:expr) => {
    static mut $name: clk_branch = clk_branch_init!($reg, $halt, stringify!($name), &clk_branch2_ops);
}; }

branch!(gpu_cc_ahb_clk, 0x1078, BRANCH_HALT_DELAY);
branch!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_VOTED);
branch!(gpu_cc_cx_apb_clk, 0x1088, BRANCH_HALT_VOTED);
branch!(gpu_cc_cx_gmu_clk, 0x1098, BRANCH_HALT);
branch!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_VOTED);
branch!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_VOTED);
branch!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT);
branch!(gpu_cc_gx_gmu_clk, 0x1064, BRANCH_HALT);
branch!(gpu_cc_hlos1_vote_gpu_smmu_clk, 0x5000, BRANCH_VOTED);

static mut gpu_cx_gdsc: gdsc = gdsc {
    gdscr: 0x106c, gds_hw_ctrl: 0x1540,
    pd: generic_pm_domain { name: "gpu_cx_gdsc" },
    pwrsts: PWRSTS_OFF_ON, flags: VOTABLE,
};

static mut gpu_gx_gdsc: gdsc = gdsc {
    gdscr: 0x100c, clamp_io_ctrl: 0x1508,
    pd: generic_pm_domain { name: "gpu_gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable) },
    pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO | AON_RESET | POLL_CFG_GDSCR,
};

static mut gpu_cc_sm8250_clocks: [*mut clk_regmap; 11] = [
    &mut gpu_cc_ahb_clk.clkr, &mut gpu_cc_crc_ahb_clk.clkr,
    &mut gpu_cc_cx_apb_clk.clkr, &mut gpu_cc_cx_gmu_clk.clkr,
    &mut gpu_cc_cx_snoc_dvm_clk.clkr, &mut gpu_cc_cxo_aon_clk.clkr,
    &mut gpu_cc_cxo_clk.clkr, &mut gpu_cc_gmu_clk_src.clkr,
    &mut gpu_cc_gx_gmu_clk.clkr, &mut gpu_cc_pll1.clkr,
    &mut gpu_cc_hlos1_vote_gpu_smmu_clk.clkr,
];

static gpu_cc_sm8250_resets: [qcom_reset_map; 6] = [
    qcom_reset_map { reg: 0x1160 }, qcom_reset_map { reg: 0x1068 },
    qcom_reset_map { reg: 0x10a0 }, qcom_reset_map { reg: 0x111c },
    qcom_reset_map { reg: 0x1008 }, qcom_reset_map { reg: 0x1000 },
];

static mut gpu_cc_sm8250_gdscs: [*mut gdsc; 2] = [&mut gpu_cx_gdsc, &mut gpu_gx_gdsc];

static gpu_cc_sm8250_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8008, fast_io: true,
};

static gpu_cc_sm8250_desc: qcom_cc_desc = qcom_cc_desc {
    config: &gpu_cc_sm8250_regmap_config,
    clks: gpu_cc_sm8250_clocks.as_ptr(), num_clks: gpu_cc_sm8250_clocks.len(),
    resets: gpu_cc_sm8250_resets.as_ptr(), num_resets: gpu_cc_sm8250_resets.len(),
    gdscs: gpu_cc_sm8250_gdscs.as_ptr(), num_gdscs: gpu_cc_sm8250_gdscs.len(),
};

static gpu_cc_sm8250_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sm8250-gpucc" }, of_device_id {},
];

unsafe fn gpu_cc_sm8250_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sm8250_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }

    clk_lucid_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);

    // Configure gpu_cc_cx_gmu_clk with recommended wakeup/sleep settings.
    let mut mask = CX_GMU_CBCR_WAKE_MASK << CX_GMU_CBCR_WAKE_SHIFT;
    mask |= CX_GMU_CBCR_SLEEP_MASK << CX_GMU_CBCR_SLEEP_SHIFT;
    let value = 0xf << CX_GMU_CBCR_WAKE_SHIFT | 0xf << CX_GMU_CBCR_SLEEP_SHIFT;
    regmap_update_bits(regmap, 0x1098, mask, value);

    qcom_cc_really_probe(&(*pdev).dev, &gpu_cc_sm8250_desc, regmap)
}

static mut gpu_cc_sm8250_driver: platform_driver = platform_driver {
    probe: Some(gpu_cc_sm8250_probe),
    driver: device_driver { name: "sm8250-gpucc", of_match_table: gpu_cc_sm8250_match_table.as_ptr() },
};

module_platform_driver!(gpu_cc_sm8250_driver);
module_description!("QTI GPU_CC SM8250 Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
