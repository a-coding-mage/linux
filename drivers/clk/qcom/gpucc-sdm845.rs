// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// Translated from the corresponding Linux kernel C implementation.

const CX_GMU_CBCR_SLEEP_MASK: u32 = 0xf;
const CX_GMU_CBCR_SLEEP_SHIFT: u32 = 4;
const CX_GMU_CBCR_WAKE_MASK: u32 = 0xf;
const CX_GMU_CBCR_WAKE_SHIFT: u32 = 8;

enum {
    P_BI_TCXO,
    P_GPLL0_OUT_MAIN,
    P_GPLL0_OUT_MAIN_DIV,
    P_GPU_CC_PLL1_OUT_MAIN,
}

static gpu_cc_pll1_config: AlphaPllConfig = AlphaPllConfig {
    l: 0x1a,
    alpha: 0xaab,
};

static mut gpu_cc_pll1: ClkAlphaPll = ClkAlphaPll {
    offset: 0x100,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_FABIA],
    clkr: ClkRegmap {
        hw: ClkHw {
            init: &ClkInitData {
                name: "gpu_cc_pll1",
                parent_data: &ClkParentData {
                    fw_name: "bi_tcxo",
                    name: "bi_tcxo",
                },
                num_parents: 1,
                ops: &clk_alpha_pll_fabia_ops,
            },
        },
    },
};

static gpu_cc_parent_map_0: [ParentMap; 4] = [
    ParentMap { src: P_BI_TCXO, cfg: 0 },
    ParentMap { src: P_GPU_CC_PLL1_OUT_MAIN, cfg: 3 },
    ParentMap { src: P_GPLL0_OUT_MAIN, cfg: 5 },
    ParentMap { src: P_GPLL0_OUT_MAIN_DIV, cfg: 6 },
];

static mut gpu_cc_parent_data_0: [ClkParentData; 4] = [
    ClkParentData { fw_name: "bi_tcxo", name: "bi_tcxo" },
    ClkParentData { hw: unsafe { &gpu_cc_pll1.clkr.hw } },
    ClkParentData { fw_name: "gcc_gpu_gpll0_clk_src", name: "gcc_gpu_gpll0_clk_src" },
    ClkParentData { fw_name: "gcc_gpu_gpll0_div_clk_src", name: "gcc_gpu_gpll0_div_clk_src" },
];

static ftbl_gpu_cc_gmu_clk_src: [FreqTbl; 4] = [
    FreqTbl { freq: 19200000, src: P_BI_TCXO, pre_div: 1, m: 0, n: 0 },
    FreqTbl { freq: 200000000, src: P_GPLL0_OUT_MAIN_DIV, pre_div: 1.5, m: 0, n: 0 },
    FreqTbl { freq: 500000000, src: P_GPU_CC_PLL1_OUT_MAIN, pre_div: 1, m: 0, n: 0 },
    FreqTbl::default(),
];

static mut gpu_cc_gmu_clk_src: ClkRcg2 = ClkRcg2 {
    cmd_rcgr: 0x1120,
    mnd_width: 0,
    hid_width: 5,
    parent_map: &gpu_cc_parent_map_0,
    freq_tbl: &ftbl_gpu_cc_gmu_clk_src,
    clkr: ClkRegmap {
        hw: ClkHw {
            init: &ClkInitData {
                name: "gpu_cc_gmu_clk_src",
                parent_data: unsafe { &gpu_cc_parent_data_0 },
                num_parents: gpu_cc_parent_data_0.len(),
                ops: &clk_rcg2_shared_ops,
            },
        },
    },
};

static mut gpu_cc_cx_gmu_clk: ClkBranch = ClkBranch {
    halt_reg: 0x1098,
    halt_check: BRANCH_HALT,
    clkr: ClkRegmap {
        enable_reg: 0x1098,
        enable_mask: BIT(0),
        hw: ClkHw {
            init: &ClkInitData {
                name: "gpu_cc_cx_gmu_clk",
                parent_hws: [&gpu_cc_gmu_clk_src.clkr.hw],
                num_parents: 1,
                flags: CLK_SET_RATE_PARENT,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut gpu_cc_cxo_clk: ClkBranch = ClkBranch {
    halt_reg: 0x109c,
    halt_check: BRANCH_HALT,
    clkr: ClkRegmap {
        enable_reg: 0x109c,
        enable_mask: BIT(0),
        hw: ClkHw {
            init: &ClkInitData {
                name: "gpu_cc_cxo_clk",
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut gpu_cx_gdsc: Gdsc = Gdsc {
    gdscr: 0x106c,
    gds_hw_ctrl: 0x1540,
    clk_dis_wait_val: 0x8,
    pd: GenericPowerDomain { name: "gpu_cx_gdsc" },
    pwrsts: PWRSTS_OFF_ON,
    flags: VOTABLE,
};

static mut gpu_gx_gdsc: Gdsc = Gdsc {
    gdscr: 0x100c,
    clamp_io_ctrl: 0x1508,
    pd: GenericPowerDomain {
        name: "gpu_gx_gdsc",
        power_on: Some(gdsc_gx_do_nothing_enable),
    },
    pwrsts: PWRSTS_OFF_ON,
    flags: CLAMP_IO | AON_RESET | POLL_CFG_GDSCR,
};

static mut gpu_cc_sdm845_clocks: [*mut ClkRegmap; 4] = [
    [GPU_CC_CXO_CLK] = unsafe { &mut gpu_cc_cxo_clk.clkr },
    [GPU_CC_CX_GMU_CLK] = unsafe { &mut gpu_cc_cx_gmu_clk.clkr },
    [GPU_CC_GMU_CLK_SRC] = unsafe { &mut gpu_cc_gmu_clk_src.clkr },
    [GPU_CC_PLL1] = unsafe { &mut gpu_cc_pll1.clkr },
];

static mut gpu_cc_sdm845_gdscs: [*mut Gdsc; 2] = [
    [GPU_CX_GDSC] = unsafe { &mut gpu_cx_gdsc },
    [GPU_GX_GDSC] = unsafe { &mut gpu_gx_gdsc },
];

static gpu_cc_sdm845_regmap_config: RegmapConfig = RegmapConfig {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x8008,
    fast_io: true,
};

static gpu_cc_sdm845_desc: QcomCcDesc = QcomCcDesc {
    config: &gpu_cc_sdm845_regmap_config,
    clks: &gpu_cc_sdm845_clocks,
    num_clks: gpu_cc_sdm845_clocks.len(),
    gdscs: &gpu_cc_sdm845_gdscs,
    num_gdscs: gpu_cc_sdm845_gdscs.len(),
};

static gpu_cc_sdm845_match_table: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "qcom,sdm845-gpucc" },
    OfDeviceId::default(),
];

unsafe fn gpu_cc_sdm845_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap: *mut Regmap;
    let mut value: u32;
    let mut mask: u32;

    regmap = qcom_cc_map(pdev, &gpu_cc_sdm845_desc);
    if is_err(regmap) {
        return ptr_err(regmap);
    }

    clk_fabia_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);

    /*
     * Configure gpu_cc_cx_gmu_clk with recommended
     * wakeup/sleep settings
     */
    mask = CX_GMU_CBCR_WAKE_MASK << CX_GMU_CBCR_WAKE_SHIFT;
    mask |= CX_GMU_CBCR_SLEEP_MASK << CX_GMU_CBCR_SLEEP_SHIFT;
    value = 0xf << CX_GMU_CBCR_WAKE_SHIFT | 0xf << CX_GMU_CBCR_SLEEP_SHIFT;
    regmap_update_bits(regmap, 0x1098, mask, value);

    qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_sdm845_desc, regmap)
}

static mut gpu_cc_sdm845_driver: PlatformDriver = PlatformDriver {
    probe: Some(gpu_cc_sdm845_probe),
    driver: Driver {
        name: "sdm845-gpucc",
        of_match_table: &gpu_cc_sdm845_match_table,
    },
};

module_platform_driver!(gpu_cc_sdm845_driver);

module_description!("QTI GPUCC SDM845 Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
