// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Linux clock-provider, module, platform-device, regmap, Qualcomm clock and
// device-tree declarations are supplied by the surrounding kernel bindings.

enum Dt { BiTcxo, GccGpuGpll0ClkSrc }

enum Parent { BiTcxo, Gpll0OutMain, GpuCcPll0_2xClk, GpuCcPll0OutAux2, GpuCcPll1OutAux, GpuCcPll1OutAux2 }

static GPU_CC_PLL_VCO: [PllVco; 2] = [
    PllVco { min_freq: 1_000_000_000, max_freq: 2_000_000_000, val: 0 },
    PllVco { min_freq: 500_000_000, max_freq: 1_000_000_000, val: 2 },
];

// 1020MHz configuration
static GPU_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x35, config_ctl_val: 0x4001055b, alpha_hi: 0x20, alpha: 0x00,
    alpha_en_mask: 1 << 24, vco_val: 0x0 << 20, vco_mask: 0x3 << 20,
    aux2_output_mask: 1 << 2,
};

// 930MHz configuration
static GPU_PLL1_CONFIG: AlphaPllConfig = AlphaPllConfig {
    l: 0x30, config_ctl_val: 0x4001055b, alpha_hi: 0x70, alpha: 0x00,
    alpha_en_mask: 1 << 24, vco_val: 0x2 << 20, vco_mask: 0x3 << 20,
    aux2_output_mask: 1 << 2,
};

static mut GPU_CC_PLL0_OUT_AUX2: ClkAlphaPll = ClkAlphaPll {
    offset: 0x0, vco_table: GPU_CC_PLL_VCO.as_ptr(), num_vco: 2,
    regs: CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_DEFAULT], flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "gpu_cc_pll0_out_aux2", parent_data: &ClkParentData { index: Dt::BiTcxo },
        num_parents: 1, ops: &CLK_ALPHA_PLL_OPS,
    } } },
};
static mut GPU_CC_PLL1_OUT_AUX2: ClkAlphaPll = ClkAlphaPll {
    offset: 0x100, vco_table: GPU_CC_PLL_VCO.as_ptr(), num_vco: 2,
    regs: CLK_ALPHA_PLL_REGS[CLK_ALPHA_PLL_TYPE_DEFAULT], flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: ClkRegmap { hw: ClkHw { init: &ClkInitData {
        name: "gpu_cc_pll1_out_aux2", parent_data: &ClkParentData { index: Dt::BiTcxo },
        num_parents: 1, ops: &CLK_ALPHA_PLL_OPS,
    } } },
};

static GPU_CC_PARENT_MAP_0: [ParentMap; 2] = [ParentMap { parent: Parent::BiTcxo, sel: 0 }, ParentMap { parent: Parent::Gpll0OutMain, sel: 5 }];
static GPU_CC_PARENT_DATA_0: [ClkParentData; 2] = [ClkParentData { index: Dt::BiTcxo }, ClkParentData { index: Dt::GccGpuGpll0ClkSrc }];
static GPU_CC_PARENT_MAP_1: [ParentMap; 3] = [ParentMap { parent: Parent::BiTcxo, sel: 0 }, ParentMap { parent: Parent::GpuCcPll0OutAux2, sel: 2 }, ParentMap { parent: Parent::GpuCcPll1OutAux2, sel: 4 }];
static GPU_CC_PARENT_DATA_1: [ClkParentData; 3] = [ClkParentData { index: Dt::BiTcxo }, ClkParentData { hw: unsafe { &GPU_CC_PLL0_OUT_AUX2.clkr.hw } }, ClkParentData { hw: unsafe { &GPU_CC_PLL1_OUT_AUX2.clkr.hw } }];

static FTBL_GPU_CC_GMU_CLK_SRC: [FreqTbl; 2] = [FreqTbl { freq: 200_000_000, parent: Parent::Gpll0OutMain, pre_div: 3, m: 0, n: 0 }, FreqTbl::EMPTY];
static mut GPU_CC_GMU_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x1120, mnd_width: 0, hid_width: 5, parent_map: GPU_CC_PARENT_MAP_0.as_ptr(), freq_tbl: FTBL_GPU_CC_GMU_CLK_SRC.as_ptr(), clkr: rcg_init("gpu_cc_gmu_clk_src", GPU_CC_PARENT_DATA_0.as_ptr(), 2, &CLK_RCG2_SHARED_OPS) };

static FTBL_GPU_CC_GX_GFX3D_CLK_SRC: [FreqTbl; 8] = [
    FreqTbl { freq: 320_000_000, parent: Parent::GpuCcPll1OutAux2, pre_div: 2, m: 0, n: 0 }, FreqTbl { freq: 465_000_000, parent: Parent::GpuCcPll1OutAux2, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 600_000_000, parent: Parent::GpuCcPll0OutAux2, pre_div: 2, m: 0, n: 0 }, FreqTbl { freq: 745_000_000, parent: Parent::GpuCcPll0OutAux2, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 820_000_000, parent: Parent::GpuCcPll0OutAux2, pre_div: 2, m: 0, n: 0 }, FreqTbl { freq: 900_000_000, parent: Parent::GpuCcPll0OutAux2, pre_div: 2, m: 0, n: 0 },
    FreqTbl { freq: 950_000_000, parent: Parent::GpuCcPll0OutAux2, pre_div: 2, m: 0, n: 0 }, FreqTbl::EMPTY,
];
static mut GPU_CC_GX_GFX3D_CLK_SRC: ClkRcg2 = ClkRcg2 { cmd_rcgr: 0x101c, mnd_width: 0, hid_width: 5, parent_map: GPU_CC_PARENT_MAP_1.as_ptr(), freq_tbl: FTBL_GPU_CC_GX_GFX3D_CLK_SRC.as_ptr(), clkr: rcg_init_flags("gpu_cc_gx_gfx3d_clk_src", GPU_CC_PARENT_DATA_1.as_ptr(), 3, CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE, &CLK_RCG2_OPS) };

macro_rules! branch { ($name:ident, $halt:expr, $check:expr, $parent:expr, $flags:expr) => { static mut $name: ClkBranch = ClkBranch { halt_reg: $halt, halt_check: $check, clkr: branch_init(stringify!($name), $halt, $parent, $flags) }; }; }
branch!(GPU_CC_CRC_AHB_CLK, 0x107c, BRANCH_HALT_DELAY, None, 0);
branch!(GPU_CC_CX_APB_CLK, 0x1088, BRANCH_HALT_DELAY, None, 0);
branch!(GPU_CC_GX_GFX3D_CLK, 0x1054, BRANCH_HALT_SKIP, Some(unsafe { &GPU_CC_GX_GFX3D_CLK_SRC.clkr.hw }), CLK_SET_RATE_PARENT);
branch!(GPU_CC_CX_GFX3D_CLK, 0x10a4, BRANCH_HALT_DELAY, Some(unsafe { &GPU_CC_GX_GFX3D_CLK.clkr.hw }), CLK_SET_RATE_PARENT);
branch!(GPU_CC_CX_GMU_CLK, 0x1098, BRANCH_HALT, Some(unsafe { &GPU_CC_GMU_CLK_SRC.clkr.hw }), CLK_SET_RATE_PARENT);
branch!(GPU_CC_CX_SNOC_DVM_CLK, 0x108c, BRANCH_HALT_DELAY, None, 0);
branch!(GPU_CC_CXO_AON_CLK, 0x1004, BRANCH_HALT_DELAY, None, 0);
branch!(GPU_CC_CXO_CLK, 0x109c, BRANCH_HALT, None, 0);
branch!(GPU_CC_SLEEP_CLK, 0x1090, BRANCH_HALT_DELAY, None, 0);
branch!(GPU_CC_AHB_CLK, 0x1078, BRANCH_HALT_DELAY, None, CLK_IS_CRITICAL);
branch!(GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK, 0x5000, BRANCH_VOTED, None, 0);

static mut GPU_CX_GDSC: Gdsc = Gdsc { gdscr: 0x106c, gds_hw_ctrl: 0x1540, pd: GenericPowerDomain { name: "gpu_cx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };
static mut GPU_GX_GDSC: Gdsc = Gdsc { gdscr: 0x100c, gds_hw_ctrl: 0, pd: GenericPowerDomain { name: "gpu_gx_gdsc" }, pwrsts: PWRSTS_OFF_ON, flags: VOTABLE };

static mut GPU_CC_SM6125_CLOCKS: [*mut ClkRegmap; 15] = [
    unsafe { &mut GPU_CC_CRC_AHB_CLK.clkr }, unsafe { &mut GPU_CC_CX_APB_CLK.clkr }, unsafe { &mut GPU_CC_CX_GFX3D_CLK.clkr }, unsafe { &mut GPU_CC_CX_GMU_CLK.clkr }, unsafe { &mut GPU_CC_CX_SNOC_DVM_CLK.clkr }, unsafe { &mut GPU_CC_CXO_AON_CLK.clkr }, unsafe { &mut GPU_CC_CXO_CLK.clkr }, unsafe { &mut GPU_CC_GMU_CLK_SRC.clkr }, unsafe { &mut GPU_CC_PLL0_OUT_AUX2.clkr }, unsafe { &mut GPU_CC_PLL1_OUT_AUX2.clkr }, unsafe { &mut GPU_CC_SLEEP_CLK.clkr }, unsafe { &mut GPU_CC_GX_GFX3D_CLK.clkr }, unsafe { &mut GPU_CC_GX_GFX3D_CLK_SRC.clkr }, unsafe { &mut GPU_CC_AHB_CLK.clkr }, unsafe { &mut GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK.clkr },
];
static mut GPUCC_SM6125_GDSCS: [*mut Gdsc; 2] = [unsafe { &mut GPU_CX_GDSC }, unsafe { &mut GPU_GX_GDSC }];
static GPU_CC_SM6125_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x9000, fast_io: true };
static GPU_CC_SM6125_DESC: QcomCcDesc = QcomCcDesc { config: &GPU_CC_SM6125_REGMAP_CONFIG, clks: unsafe { GPU_CC_SM6125_CLOCKS.as_ptr() }, num_clks: 15, gdscs: unsafe { GPUCC_SM6125_GDSCS.as_ptr() }, num_gdscs: 2 };
static GPU_CC_SM6125_MATCH_TABLE: [OfDeviceId; 2] = [OfDeviceId { compatible: "qcom,sm6125-gpucc" }, OfDeviceId::EMPTY];

unsafe fn gpu_cc_sm6125_probe(pdev: *mut PlatformDevice) -> i32 {
    let regmap = qcom_cc_map(pdev, &GPU_CC_SM6125_DESC);
    if is_err(regmap) { return ptr_err(regmap); }
    clk_alpha_pll_configure(&mut GPU_CC_PLL0_OUT_AUX2, regmap, &GPU_PLL0_CONFIG);
    clk_alpha_pll_configure(&mut GPU_CC_PLL1_OUT_AUX2, regmap, &GPU_PLL1_CONFIG);
    // Set recommended WAKEUP/SLEEP settings for the gpu_cc_cx_gmu_clk
    qcom_branch_set_wakeup(regmap, &mut GPU_CC_CX_GMU_CLK, 0xf);
    qcom_branch_set_sleep(regmap, &mut GPU_CC_CX_GMU_CLK, 0xf);
    qcom_branch_set_force_mem_core(regmap, &mut GPU_CC_GX_GFX3D_CLK, true);
    qcom_branch_set_force_periph_on(regmap, &mut GPU_CC_GX_GFX3D_CLK, true);
    qcom_cc_really_probe(&mut (*pdev).dev, &GPU_CC_SM6125_DESC, regmap)
}

static mut GPU_CC_SM6125_DRIVER: PlatformDriver = PlatformDriver { probe: Some(gpu_cc_sm6125_probe), driver: Driver { name: "gpucc-sm6125", of_match_table: GPU_CC_SM6125_MATCH_TABLE.as_ptr() } };
module_platform_driver!(GPU_CC_SM6125_DRIVER);
module_description!("QTI GPUCC SM6125 Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
