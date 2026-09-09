// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019, The Linux Foundation. All rights reserved.
 */

// External Linux clock, module, platform, regmap, device-tree, and local clock
// definitions are supplied by the surrounding kernel/Rust bindings.

const CX_GMU_CBCR_SLEEP_MASK: u32 = 0xF;
const CX_GMU_CBCR_SLEEP_SHIFT: u32 = 4;
const CX_GMU_CBCR_WAKE_MASK: u32 = 0xF;
const CX_GMU_CBCR_WAKE_SHIFT: u32 = 8;

enum Parent {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static FABIA_VCO: [pll_vco; 1] = [
    pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 },
];

static mut gpu_cc_pll1: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100,
    vco_table: FABIA_VCO.as_ptr(),
    num_vco: FABIA_VCO.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "gpu_cc_pll1",
                parent_data: &clk_parent_data { fw_name: "bi_tcxo" },
                num_parents: 1,
                ops: &clk_alpha_pll_fabia_ops,
            },
        },
    },
};

static gpu_cc_parent_map_0: [parent_map; 4] = [
    parent_map { src: Parent::P_BI_TCXO as u32, cfg: 0 },
    parent_map { src: Parent::P_GPU_CC_PLL1_OUT_MAIN as u32, cfg: 3 },
    parent_map { src: Parent::P_GPLL0_OUT_MAIN as u32, cfg: 5 },
    parent_map { src: Parent::P_GPLL0_OUT_MAIN_DIV as u32, cfg: 6 },
];

static mut gpu_cc_parent_data_0: [clk_parent_data; 4] = [
    clk_parent_data { fw_name: "bi_tcxo" },
    clk_parent_data { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    clk_parent_data { fw_name: "gcc_gpu_gpll0_clk_src" },
    clk_parent_data { fw_name: "gcc_gpu_gpll0_div_clk_src" },
];

static ftbl_gpu_cc_gmu_clk_src: [freq_tbl; 3] = [
    FREQ_TBL { freq: 19200000, src: Parent::P_BI_TCXO as u32, pre_div: 1, m: 0, n: 0 },
    FREQ_TBL { freq: 200000000, src: Parent::P_GPLL0_OUT_MAIN_DIV as u32, pre_div: 1.5, m: 0, n: 0 },
    FREQ_TBL { ..Default::default() },
];

static mut gpu_cc_gmu_clk_src: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x1120,
    mnd_width: 0,
    hid_width: 5,
    parent_map: gpu_cc_parent_map_0.as_ptr(),
    freq_tbl: ftbl_gpu_cc_gmu_clk_src.as_ptr(),
    clkr: clk_regmap { hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_gmu_clk_src",
        parent_data: unsafe { gpu_cc_parent_data_0.as_ptr() },
        num_parents: gpu_cc_parent_data_0.len(),
        flags: CLK_SET_RATE_PARENT,
        ops: &clk_rcg2_shared_ops,
    } } },
};

macro_rules! branch_clock {
    ($name:ident, $reg:expr, $halt:expr, $clk_name:expr) => {
        static mut $name: clk_branch = clk_branch {
            halt_reg: $reg,
            halt_check: $halt,
            clkr: clk_regmap { enable_reg: $reg, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
                name: $clk_name, ops: &clk_branch2_ops,
            } } },
        };
    };
}

branch_clock!(gpu_cc_crc_ahb_clk, 0x107c, BRANCH_HALT_DELAY, "gpu_cc_crc_ahb_clk");
branch_clock!(gpu_cc_cx_snoc_dvm_clk, 0x108c, BRANCH_HALT_DELAY, "gpu_cc_cx_snoc_dvm_clk");
branch_clock!(gpu_cc_cxo_aon_clk, 0x1004, BRANCH_HALT_DELAY, "gpu_cc_cxo_aon_clk");
branch_clock!(gpu_cc_cxo_clk, 0x109c, BRANCH_HALT, "gpu_cc_cxo_clk");

static mut gpu_cc_cx_gmu_clk: clk_branch = clk_branch {
    halt_reg: 0x1098, halt_check: BRANCH_HALT,
    clkr: clk_regmap { enable_reg: 0x1098, enable_mask: BIT(0), hw: clk_hw { init: &clk_init_data {
        name: "gpu_cc_cx_gmu_clk",
        parent_hws: unsafe { [&gpu_cc_gmu_clk_src.clkr.hw] }, num_parents: 1,
        flags: CLK_SET_RATE_PARENT, ops: &clk_branch2_ops,
    } } },
};

static mut cx_gdsc: gdsc = gdsc {
    gdscr: 0x106c, gds_hw_ctrl: 0x1540, clk_dis_wait_val: 8,
    pd: generic_pm_domain { name: "cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE,
};

static mut gx_gdsc: gdsc = gdsc {
    gdscr: 0x100c, clamp_io_ctrl: 0x1508,
    pd: generic_pm_domain { name: "gx_gdsc", power_on: Some(gdsc_gx_do_nothing_enable) },
    pwrsts: PWRSTS_OFF_ON, flags: CLAMP_IO,
};

static mut gpu_cc_sc7180_gdscs: [*mut gdsc; 2] = [unsafe { &mut cx_gdsc }, unsafe { &mut gx_gdsc }];

static mut gpu_cc_sc7180_clocks: [*mut clk_regmap; 7] = [
    unsafe { &mut gpu_cc_cxo_clk.clkr }, unsafe { &mut gpu_cc_crc_ahb_clk.clkr },
    unsafe { &mut gpu_cc_cx_gmu_clk.clkr }, unsafe { &mut gpu_cc_cx_snoc_dvm_clk.clkr },
    unsafe { &mut gpu_cc_cxo_aon_clk.clkr }, unsafe { &mut gpu_cc_gmu_clk_src.clkr },
    unsafe { &mut gpu_cc_pll1.clkr },
];

static gpu_cc_sc7180_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8008, fast_io: true,
};

static gpu_cc_sc7180_desc: qcom_cc_desc = qcom_cc_desc {
    config: &gpu_cc_sc7180_regmap_config,
    clks: gpu_cc_sc7180_clocks.as_ptr(), num_clks: gpu_cc_sc7180_clocks.len(),
    gdscs: gpu_cc_sc7180_gdscs.as_ptr(), num_gdscs: gpu_cc_sc7180_gdscs.len(),
};

static gpu_cc_sc7180_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,sc7180-gpucc" }, of_device_id { compatible: "" },
];

unsafe extern "C" fn gpu_cc_sc7180_probe(pdev: *mut platform_device) -> i32 {
    let regmap: *mut regmap;
    let mut gpu_cc_pll_config: alpha_pll_config = core::mem::zeroed();
    let (mut value, mut mask): (u32, u32);

    regmap = qcom_cc_map(pdev, &gpu_cc_sc7180_desc);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }

    // 360MHz Configuration
    gpu_cc_pll_config.l = 0x12;
    gpu_cc_pll_config.alpha = 0xc000;
    gpu_cc_pll_config.config_ctl_val = 0x20485699;
    gpu_cc_pll_config.config_ctl_hi_val = 0x00002067;
    gpu_cc_pll_config.user_ctl_val = 0x00000001;
    gpu_cc_pll_config.user_ctl_hi_val = 0x00004805;
    gpu_cc_pll_config.test_ctl_hi_val = 0x40000000;

    clk_fabia_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll_config);

    // Recommended WAKEUP/SLEEP settings for the gpu_cc_cx_gmu_clk
    mask = (CX_GMU_CBCR_WAKE_MASK << CX_GMU_CBCR_WAKE_SHIFT)
        | (CX_GMU_CBCR_SLEEP_MASK << CX_GMU_CBCR_SLEEP_SHIFT);
    value = (0xF << CX_GMU_CBCR_WAKE_SHIFT) | (0xF << CX_GMU_CBCR_SLEEP_SHIFT);
    regmap_update_bits(regmap, 0x1098, mask, value);

    qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_sc7180_desc, regmap)
}

static mut gpu_cc_sc7180_driver: platform_driver = platform_driver {
    probe: Some(gpu_cc_sc7180_probe),
    driver: device_driver { name: "sc7180-gpucc", of_match_table: gpu_cc_sc7180_match_table.as_ptr() },
};

module_platform_driver!(gpu_cc_sc7180_driver);
module_description!("QTI GPU_CC SC7180 Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
