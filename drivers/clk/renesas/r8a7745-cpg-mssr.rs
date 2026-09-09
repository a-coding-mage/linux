// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7745 Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2016 Cogent Embedded Inc.
 */

// Linux headers and the Renesas clock bindings provide the external types,
// constants, and helper macros referenced below.

#[repr(C)]
enum ClkIds {
    LastDtCoreClk = R8A7745_CLK_OSC,
    ClkExtal,
    ClkUsbExtal,
    ClkMain,
    ClkPll0,
    ClkPll1,
    ClkPll3,
    ClkPll1Div2,
    ModClkBase,
}

static R8A7745_CORE_CLKS: &[CpgCoreClk] = &[
    DEF_INPUT!("extal", ClkIds::ClkExtal),
    DEF_INPUT!("usb_extal", ClkIds::ClkUsbExtal),
    DEF_BASE!(".main", ClkIds::ClkMain, CLK_TYPE_GEN2_MAIN, ClkIds::ClkExtal),
    DEF_BASE!(".pll0", ClkIds::ClkPll0, CLK_TYPE_GEN2_PLL0, ClkIds::ClkMain),
    DEF_BASE!(".pll1", ClkIds::ClkPll1, CLK_TYPE_GEN2_PLL1, ClkIds::ClkMain),
    DEF_BASE!(".pll3", ClkIds::ClkPll3, CLK_TYPE_GEN2_PLL3, ClkIds::ClkMain),
    DEF_FIXED!(".pll1_div2", ClkIds::ClkPll1Div2, ClkIds::ClkPll1, 2, 1),
    DEF_BASE!("sdh", R8A7745_CLK_SDH, CLK_TYPE_GEN2_SDH, ClkIds::ClkPll1),
    DEF_BASE!("sd0", R8A7745_CLK_SD0, CLK_TYPE_GEN2_SD0, ClkIds::ClkPll1),
    DEF_BASE!("qspi", R8A7745_CLK_QSPI, CLK_TYPE_GEN2_QSPI, ClkIds::ClkPll1Div2),
    DEF_BASE!("rcan", R8A7745_CLK_RCAN, CLK_TYPE_GEN2_RCAN, ClkIds::ClkUsbExtal),
    DEF_FIXED!("z2", R8A7745_CLK_Z2, ClkIds::ClkPll0, 1, 1),
    DEF_FIXED!("zg", R8A7745_CLK_ZG, ClkIds::ClkPll1, 6, 1),
    DEF_FIXED!("zx", R8A7745_CLK_ZX, ClkIds::ClkPll1, 3, 1),
    DEF_FIXED!("zs", R8A7745_CLK_ZS, ClkIds::ClkPll1, 6, 1),
    DEF_FIXED!("hp", R8A7745_CLK_HP, ClkIds::ClkPll1, 12, 1),
    DEF_FIXED!("b", R8A7745_CLK_B, ClkIds::ClkPll1, 12, 1),
    DEF_FIXED!("lb", R8A7745_CLK_LB, ClkIds::ClkPll1, 24, 1),
    DEF_FIXED!("p", R8A7745_CLK_P, ClkIds::ClkPll1, 24, 1),
    DEF_FIXED!("cl", R8A7745_CLK_CL, ClkIds::ClkPll1, 48, 1),
    DEF_FIXED!("cp", R8A7745_CLK_CP, ClkIds::ClkPll1, 48, 1),
    DEF_FIXED!("m2", R8A7745_CLK_M2, ClkIds::ClkPll1, 8, 1),
    DEF_FIXED!("zb3", R8A7745_CLK_ZB3, ClkIds::ClkPll3, 4, 1),
    DEF_FIXED!("zb3d2", R8A7745_CLK_ZB3D2, ClkIds::ClkPll3, 8, 1),
    DEF_FIXED!("ddr", R8A7745_CLK_DDR, ClkIds::ClkPll3, 8, 1),
    DEF_FIXED!("mp", R8A7745_CLK_MP, ClkIds::ClkPll1Div2, 15, 1),
    DEF_FIXED!("cpex", R8A7745_CLK_CPEX, ClkIds::ClkExtal, 2, 1),
    DEF_FIXED!("r", R8A7745_CLK_R, ClkIds::ClkPll1, 49152, 1),
    DEF_FIXED!("osc", R8A7745_CLK_OSC, ClkIds::ClkPll1, 12288, 1),
    DEF_DIV6P1!("sd2", R8A7745_CLK_SD2, ClkIds::ClkPll1Div2, 0x078),
    DEF_DIV6P1!("sd3", R8A7745_CLK_SD3, ClkIds::ClkPll1Div2, 0x26c),
    DEF_DIV6P1!("mmc0", R8A7745_CLK_MMC0, ClkIds::ClkPll1Div2, 0x240),
];

static R8A7745_MOD_CLKS: &[MssrModClk] = &[
    DEF_MOD!("msiof0", 0, R8A7745_CLK_MP), DEF_MOD!("vcp0", 101, R8A7745_CLK_ZS), DEF_MOD!("vpc0", 103, R8A7745_CLK_ZS),
    DEF_MOD!("tmu1", 111, R8A7745_CLK_P), DEF_MOD!("3dg", 112, R8A7745_CLK_ZG), DEF_MOD!("2d-dmac", 115, R8A7745_CLK_ZS),
    DEF_MOD!("fdp1-0", 119, R8A7745_CLK_ZS), DEF_MOD!("tmu3", 121, R8A7745_CLK_P), DEF_MOD!("tmu2", 122, R8A7745_CLK_P),
    DEF_MOD!("cmt0", 124, R8A7745_CLK_R), DEF_MOD!("tmu0", 125, R8A7745_CLK_CP), DEF_MOD!("vsp1du0", 128, R8A7745_CLK_ZS),
    DEF_MOD!("vsps", 131, R8A7745_CLK_ZS), DEF_MOD!("scifa2", 202, R8A7745_CLK_MP), DEF_MOD!("scifa1", 203, R8A7745_CLK_MP),
    DEF_MOD!("scifa0", 204, R8A7745_CLK_MP), DEF_MOD!("msiof2", 205, R8A7745_CLK_MP), DEF_MOD!("scifb0", 206, R8A7745_CLK_MP),
    DEF_MOD!("scifb1", 207, R8A7745_CLK_MP), DEF_MOD!("msiof1", 208, R8A7745_CLK_MP), DEF_MOD!("scifb2", 216, R8A7745_CLK_MP),
    DEF_MOD!("sys-dmac1", 218, R8A7745_CLK_ZS), DEF_MOD!("sys-dmac0", 219, R8A7745_CLK_ZS), DEF_MOD!("tpu0", 304, R8A7745_CLK_CP),
    DEF_MOD!("sdhi3", 311, R8A7745_CLK_SD3), DEF_MOD!("sdhi2", 312, R8A7745_CLK_SD2), DEF_MOD!("sdhi0", 314, R8A7745_CLK_SD0),
    DEF_MOD!("mmcif0", 315, R8A7745_CLK_MMC0), DEF_MOD!("iic0", 318, R8A7745_CLK_HP), DEF_MOD!("iic1", 323, R8A7745_CLK_HP),
    DEF_MOD!("cmt1", 329, R8A7745_CLK_R), DEF_MOD!("usbhs-dmac0", 330, R8A7745_CLK_HP), DEF_MOD!("usbhs-dmac1", 331, R8A7745_CLK_HP),
    DEF_MOD!("rwdt", 402, R8A7745_CLK_R), DEF_MOD!("irqc", 407, R8A7745_CLK_CP), DEF_MOD!("intc-sys", 408, R8A7745_CLK_ZS),
    DEF_MOD!("audio-dmac0", 502, R8A7745_CLK_HP), DEF_MOD!("pwm", 523, R8A7745_CLK_P), DEF_MOD!("usb-ehci", 703, R8A7745_CLK_MP),
    DEF_MOD!("usbhs", 704, R8A7745_CLK_HP), DEF_MOD!("hscif2", 713, R8A7745_CLK_ZS), DEF_MOD!("scif5", 714, R8A7745_CLK_P),
    DEF_MOD!("scif4", 715, R8A7745_CLK_P), DEF_MOD!("hscif1", 716, R8A7745_CLK_ZS), DEF_MOD!("hscif0", 717, R8A7745_CLK_ZS),
    DEF_MOD!("scif3", 718, R8A7745_CLK_P), DEF_MOD!("scif2", 719, R8A7745_CLK_P), DEF_MOD!("scif1", 720, R8A7745_CLK_P), DEF_MOD!("scif0", 721, R8A7745_CLK_P),
    DEF_MOD!("du1", 723, R8A7745_CLK_ZX), DEF_MOD!("du0", 724, R8A7745_CLK_ZX), DEF_MOD!("ipmmu-sgx", 800, R8A7745_CLK_ZX),
    DEF_MOD!("vin1", 810, R8A7745_CLK_ZG), DEF_MOD!("vin0", 811, R8A7745_CLK_ZG), DEF_MOD!("etheravb", 812, R8A7745_CLK_HP), DEF_MOD!("ether", 813, R8A7745_CLK_P),
    DEF_MOD!("gpio6", 905, R8A7745_CLK_CP), DEF_MOD!("gpio5", 907, R8A7745_CLK_CP), DEF_MOD!("gpio4", 908, R8A7745_CLK_CP), DEF_MOD!("gpio3", 909, R8A7745_CLK_CP), DEF_MOD!("gpio2", 910, R8A7745_CLK_CP), DEF_MOD!("gpio1", 911, R8A7745_CLK_CP), DEF_MOD!("gpio0", 912, R8A7745_CLK_CP),
    DEF_MOD!("can1", 915, R8A7745_CLK_P), DEF_MOD!("can0", 916, R8A7745_CLK_P), DEF_MOD!("qspi_mod", 917, R8A7745_CLK_QSPI),
    DEF_MOD!("i2c5", 925, R8A7745_CLK_HP), DEF_MOD!("i2c4", 927, R8A7745_CLK_HP), DEF_MOD!("i2c3", 928, R8A7745_CLK_HP), DEF_MOD!("i2c2", 929, R8A7745_CLK_HP), DEF_MOD!("i2c1", 930, R8A7745_CLK_HP), DEF_MOD!("i2c0", 931, R8A7745_CLK_HP),
    DEF_MOD!("ssi-all", 1005, R8A7745_CLK_P), DEF_MOD!("ssi9", 1006, MOD_CLK_ID!(1005)), DEF_MOD!("ssi8", 1007, MOD_CLK_ID!(1005)), DEF_MOD!("ssi7", 1008, MOD_CLK_ID!(1005)), DEF_MOD!("ssi6", 1009, MOD_CLK_ID!(1005)), DEF_MOD!("ssi5", 1010, MOD_CLK_ID!(1005)), DEF_MOD!("ssi4", 1011, MOD_CLK_ID!(1005)), DEF_MOD!("ssi3", 1012, MOD_CLK_ID!(1005)), DEF_MOD!("ssi2", 1013, MOD_CLK_ID!(1005)), DEF_MOD!("ssi1", 1014, MOD_CLK_ID!(1005)), DEF_MOD!("ssi0", 1015, MOD_CLK_ID!(1005)),
    DEF_MOD!("scu-all", 1017, R8A7745_CLK_P), DEF_MOD!("scu-dvc1", 1018, MOD_CLK_ID!(1017)), DEF_MOD!("scu-dvc0", 1019, MOD_CLK_ID!(1017)), DEF_MOD!("scu-ctu1-mix1", 1020, MOD_CLK_ID!(1017)), DEF_MOD!("scu-ctu0-mix0", 1021, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src6", 1025, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src5", 1026, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src4", 1027, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src3", 1028, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src2", 1029, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src1", 1030, MOD_CLK_ID!(1017)),
    DEF_MOD!("scifa3", 1106, R8A7745_CLK_MP), DEF_MOD!("scifa4", 1107, R8A7745_CLK_MP), DEF_MOD!("scifa5", 1108, R8A7745_CLK_MP),
];

static R8A7745_CRIT_MOD_CLKS: &[u32] = &[MOD_CLK_ID!(402), MOD_CLK_ID!(408)];

// CPG_PLL_CONFIG_INDEX(md) = ((((md) & BIT(14)) >> 13) | (((md) & BIT(13)) >> 13))
const fn cpg_pll_config_index(md: u32) -> usize { (((md & (1 << 14)) >> 13) | ((md & (1 << 13)) >> 13)) as usize }

static CPG_PLL_CONFIGS: [RcarGen2CpgPllConfig; 8] = [
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 208, pll3_mult: 88, pll0_mult: 200 },
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 156, pll3_mult: 66, pll0_mult: 150 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 240, pll3_mult: 102, pll0_mult: 230 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 208, pll3_mult: 88, pll0_mult: 200 },
    RcarGen2CpgPllConfig { extal_div: 0, pll1_mult: 0, pll3_mult: 0, pll0_mult: 0 },
    RcarGen2CpgPllConfig { extal_div: 0, pll1_mult: 0, pll3_mult: 0, pll0_mult: 0 },
    RcarGen2CpgPllConfig { extal_div: 0, pll1_mult: 0, pll3_mult: 0, pll0_mult: 0 },
    RcarGen2CpgPllConfig { extal_div: 0, pll1_mult: 0, pll3_mult: 0, pll0_mult: 0 },
];

unsafe fn r8a7745_cpg_mssr_init(dev: *mut Device) -> i32 {
    let mut cpg_mode: u32 = 0;
    let error = rcar_rst_read_mode_pins(&mut cpg_mode);
    if error != 0 { return error; }
    let cpg_pll_config = &CPG_PLL_CONFIGS[cpg_pll_config_index(cpg_mode)];
    rcar_gen2_cpg_init(cpg_pll_config, 3, cpg_mode)
}

static R8A7745_CPG_MSSR_INFO: CpgMssrInfo = CpgMssrInfo {
    core_clks: R8A7745_CORE_CLKS.as_ptr(), num_core_clks: R8A7745_CORE_CLKS.len(),
    last_dt_core_clk: R8A7745_CLK_OSC, num_total_core_clks: ClkIds::ModClkBase as usize,
    mod_clks: R8A7745_MOD_CLKS.as_ptr(), num_mod_clks: R8A7745_MOD_CLKS.len(), num_hw_mod_clks: 12 * 32,
    crit_mod_clks: R8A7745_CRIT_MOD_CLKS.as_ptr(), num_crit_mod_clks: R8A7745_CRIT_MOD_CLKS.len(),
    init: Some(r8a7745_cpg_mssr_init), cpg_clk_register: Some(rcar_gen2_cpg_clk_register),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
