// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7742 Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2020 Renesas Electronics Corp.
 */

// Linux dependencies supplied by other translation units.

#[repr(C)]
enum ClkIds {
    // Core Clock Outputs exported to DT
    LAST_DT_CORE_CLK = R8A7742_CLK_OSC,
    // External Input Clocks
    CLK_EXTAL,
    CLK_USB_EXTAL,
    // Internal Core Clocks
    CLK_MAIN,
    CLK_PLL0,
    CLK_PLL1,
    CLK_PLL3,
    CLK_PLL1_DIV2,
    // Module Clocks
    MOD_CLK_BASE,
}

static R8A7742_CORE_CLKS: &[CpgCoreClk] = &[
    DEF_INPUT!("extal", CLK_EXTAL),
    DEF_INPUT!("usb_extal", CLK_USB_EXTAL),
    DEF_BASE!(".main", CLK_MAIN, CLK_TYPE_GEN2_MAIN, CLK_EXTAL),
    DEF_BASE!(".pll0", CLK_PLL0, CLK_TYPE_GEN2_PLL0, CLK_MAIN),
    DEF_BASE!(".pll1", CLK_PLL1, CLK_TYPE_GEN2_PLL1, CLK_MAIN),
    DEF_BASE!(".pll3", CLK_PLL3, CLK_TYPE_GEN2_PLL3, CLK_MAIN),
    DEF_FIXED!(".pll1_div2", CLK_PLL1_DIV2, CLK_PLL1, 2, 1),
    DEF_BASE!("z", R8A7742_CLK_Z, CLK_TYPE_GEN2_Z, CLK_PLL0),
    DEF_BASE!("lb", R8A7742_CLK_LB, CLK_TYPE_GEN2_LB, CLK_PLL1),
    DEF_BASE!("sdh", R8A7742_CLK_SDH, CLK_TYPE_GEN2_SDH, CLK_PLL1),
    DEF_BASE!("sd0", R8A7742_CLK_SD0, CLK_TYPE_GEN2_SD0, CLK_PLL1),
    DEF_BASE!("sd1", R8A7742_CLK_SD1, CLK_TYPE_GEN2_SD1, CLK_PLL1),
    DEF_BASE!("qspi", R8A7742_CLK_QSPI, CLK_TYPE_GEN2_QSPI, CLK_PLL1_DIV2),
    DEF_BASE!("rcan", R8A7742_CLK_RCAN, CLK_TYPE_GEN2_RCAN, CLK_USB_EXTAL),
    DEF_FIXED!("z2", R8A7742_CLK_Z2, CLK_PLL1, 2, 1),
    DEF_FIXED!("zg", R8A7742_CLK_ZG, CLK_PLL1, 3, 1),
    DEF_FIXED!("zx", R8A7742_CLK_ZX, CLK_PLL1, 3, 1),
    DEF_FIXED!("zs", R8A7742_CLK_ZS, CLK_PLL1, 6, 1),
    DEF_FIXED!("hp", R8A7742_CLK_HP, CLK_PLL1, 12, 1),
    DEF_FIXED!("b", R8A7742_CLK_B, CLK_PLL1, 12, 1),
    DEF_FIXED!("p", R8A7742_CLK_P, CLK_PLL1, 24, 1),
    DEF_FIXED!("cl", R8A7742_CLK_CL, CLK_PLL1, 48, 1),
    DEF_FIXED!("m2", R8A7742_CLK_M2, CLK_PLL1, 8, 1),
    DEF_FIXED!("zb3", R8A7742_CLK_ZB3, CLK_PLL3, 4, 1),
    DEF_FIXED!("zb3d2", R8A7742_CLK_ZB3D2, CLK_PLL3, 8, 1),
    DEF_FIXED!("ddr", R8A7742_CLK_DDR, CLK_PLL3, 8, 1),
    DEF_FIXED!("mp", R8A7742_CLK_MP, CLK_PLL1_DIV2, 15, 1),
    DEF_FIXED!("cp", R8A7742_CLK_CP, CLK_EXTAL, 2, 1),
    DEF_FIXED!("r", R8A7742_CLK_R, CLK_PLL1, 49152, 1),
    DEF_FIXED!("osc", R8A7742_CLK_OSC, CLK_PLL1, 12288, 1),
    DEF_DIV6P1!("sd2", R8A7742_CLK_SD2, CLK_PLL1_DIV2, 0x078),
    DEF_DIV6P1!("sd3", R8A7742_CLK_SD3, CLK_PLL1_DIV2, 0x26c),
    DEF_DIV6P1!("mmc0", R8A7742_CLK_MMC0, CLK_PLL1_DIV2, 0x240),
    DEF_DIV6P1!("mmc1", R8A7742_CLK_MMC1, CLK_PLL1_DIV2, 0x244),
];

static R8A7742_MOD_CLKS: &[MssrModClk] = &[
    DEF_MOD!("msiof0", 0, R8A7742_CLK_MP), DEF_MOD!("vcp1", 100, R8A7742_CLK_ZS),
    DEF_MOD!("vcp0", 101, R8A7742_CLK_ZS), DEF_MOD!("vpc1", 102, R8A7742_CLK_ZS),
    DEF_MOD!("vpc0", 103, R8A7742_CLK_ZS), DEF_MOD!("tmu1", 111, R8A7742_CLK_P),
    DEF_MOD!("3dg", 112, R8A7742_CLK_ZG), DEF_MOD!("2d-dmac", 115, R8A7742_CLK_ZS),
    DEF_MOD!("fdp1-2", 117, R8A7742_CLK_ZS), DEF_MOD!("fdp1-1", 118, R8A7742_CLK_ZS),
    DEF_MOD!("fdp1-0", 119, R8A7742_CLK_ZS), DEF_MOD!("tmu3", 121, R8A7742_CLK_P),
    DEF_MOD!("tmu2", 122, R8A7742_CLK_P), DEF_MOD!("cmt0", 124, R8A7742_CLK_R),
    DEF_MOD!("tmu0", 125, R8A7742_CLK_CP), DEF_MOD!("vsp1du1", 127, R8A7742_CLK_ZS),
    DEF_MOD!("vsp1du0", 128, R8A7742_CLK_ZS), DEF_MOD!("vspr", 130, R8A7742_CLK_ZS),
    DEF_MOD!("vsps", 131, R8A7742_CLK_ZS), DEF_MOD!("scifa2", 202, R8A7742_CLK_MP),
    DEF_MOD!("scifa1", 203, R8A7742_CLK_MP), DEF_MOD!("scifa0", 204, R8A7742_CLK_MP),
    DEF_MOD!("msiof2", 205, R8A7742_CLK_MP), DEF_MOD!("scifb0", 206, R8A7742_CLK_MP),
    DEF_MOD!("scifb1", 207, R8A7742_CLK_MP), DEF_MOD!("msiof1", 208, R8A7742_CLK_MP),
    DEF_MOD!("msiof3", 215, R8A7742_CLK_MP), DEF_MOD!("scifb2", 216, R8A7742_CLK_MP),
    DEF_MOD!("sys-dmac1", 218, R8A7742_CLK_ZS), DEF_MOD!("sys-dmac0", 219, R8A7742_CLK_ZS),
    DEF_MOD!("iic2", 300, R8A7742_CLK_HP), DEF_MOD!("tpu0", 304, R8A7742_CLK_CP),
    DEF_MOD!("mmcif1", 305, R8A7742_CLK_MMC1), DEF_MOD!("scif2", 310, R8A7742_CLK_P),
    DEF_MOD!("sdhi3", 311, R8A7742_CLK_SD3), DEF_MOD!("sdhi2", 312, R8A7742_CLK_SD2),
    DEF_MOD!("sdhi1", 313, R8A7742_CLK_SD1), DEF_MOD!("sdhi0", 314, R8A7742_CLK_SD0),
    DEF_MOD!("mmcif0", 315, R8A7742_CLK_MMC0), DEF_MOD!("iic0", 318, R8A7742_CLK_HP),
    DEF_MOD!("pciec", 319, R8A7742_CLK_MP), DEF_MOD!("iic1", 323, R8A7742_CLK_HP),
    DEF_MOD!("usb3.0", 328, R8A7742_CLK_MP), DEF_MOD!("cmt1", 329, R8A7742_CLK_R),
    DEF_MOD!("usbhs-dmac0", 330, R8A7742_CLK_HP), DEF_MOD!("usbhs-dmac1", 331, R8A7742_CLK_HP),
    DEF_MOD!("rwdt", 402, R8A7742_CLK_R), DEF_MOD!("irqc", 407, R8A7742_CLK_CP),
    DEF_MOD!("intc-sys", 408, R8A7742_CLK_ZS), DEF_MOD!("audio-dmac1", 501, R8A7742_CLK_HP),
    DEF_MOD!("audio-dmac0", 502, R8A7742_CLK_HP), DEF_MOD!("thermal", 522, CLK_EXTAL),
    DEF_MOD!("pwm", 523, R8A7742_CLK_P), DEF_MOD!("usb-ehci", 703, R8A7742_CLK_MP),
    DEF_MOD!("usbhs", 704, R8A7742_CLK_HP), DEF_MOD!("hscif1", 716, R8A7742_CLK_ZS),
    DEF_MOD!("hscif0", 717, R8A7742_CLK_ZS), DEF_MOD!("scif1", 720, R8A7742_CLK_P),
    DEF_MOD!("scif0", 721, R8A7742_CLK_P), DEF_MOD!("du2", 722, R8A7742_CLK_ZX),
    DEF_MOD!("du1", 723, R8A7742_CLK_ZX), DEF_MOD!("du0", 724, R8A7742_CLK_ZX),
    DEF_MOD!("lvds1", 725, R8A7742_CLK_ZX), DEF_MOD!("lvds0", 726, R8A7742_CLK_ZX),
    DEF_MOD!("r-gp2d", 807, R8A7742_CLK_ZX), DEF_MOD!("vin3", 808, R8A7742_CLK_ZG),
    DEF_MOD!("vin2", 809, R8A7742_CLK_ZG), DEF_MOD!("vin1", 810, R8A7742_CLK_ZG),
    DEF_MOD!("vin0", 811, R8A7742_CLK_ZG), DEF_MOD!("etheravb", 812, R8A7742_CLK_HP),
    DEF_MOD!("ether", 813, R8A7742_CLK_P), DEF_MOD!("sata1", 814, R8A7742_CLK_ZS),
    DEF_MOD!("sata0", 815, R8A7742_CLK_ZS), DEF_MOD!("imr-x2-1", 820, R8A7742_CLK_ZG),
    DEF_MOD!("imr-x2-0", 821, R8A7742_CLK_HP), DEF_MOD!("imr-lsx2-1", 822, R8A7742_CLK_P),
    DEF_MOD!("imr-lsx2-0", 823, R8A7742_CLK_ZS), DEF_MOD!("gpio5", 907, R8A7742_CLK_CP),
    DEF_MOD!("gpio4", 908, R8A7742_CLK_CP), DEF_MOD!("gpio3", 909, R8A7742_CLK_CP),
    DEF_MOD!("gpio2", 910, R8A7742_CLK_CP), DEF_MOD!("gpio1", 911, R8A7742_CLK_CP),
    DEF_MOD!("gpio0", 912, R8A7742_CLK_CP), DEF_MOD!("can1", 915, R8A7742_CLK_P),
    DEF_MOD!("can0", 916, R8A7742_CLK_P), DEF_MOD!("qspi_mod", 917, R8A7742_CLK_QSPI),
    DEF_MOD!("iicdvfs", 926, R8A7742_CLK_CP), DEF_MOD!("i2c3", 928, R8A7742_CLK_HP),
    DEF_MOD!("i2c2", 929, R8A7742_CLK_HP), DEF_MOD!("i2c1", 930, R8A7742_CLK_HP),
    DEF_MOD!("i2c0", 931, R8A7742_CLK_HP), DEF_MOD!("ssi-all", 1005, R8A7742_CLK_P),
    DEF_MOD!("ssi9", 1006, MOD_CLK_ID!(1005)), DEF_MOD!("ssi8", 1007, MOD_CLK_ID!(1005)),
    DEF_MOD!("ssi7", 1008, MOD_CLK_ID!(1005)), DEF_MOD!("ssi6", 1009, MOD_CLK_ID!(1005)),
    DEF_MOD!("ssi5", 1010, MOD_CLK_ID!(1005)), DEF_MOD!("ssi4", 1011, MOD_CLK_ID!(1005)),
    DEF_MOD!("ssi3", 1012, MOD_CLK_ID!(1005)), DEF_MOD!("ssi2", 1013, MOD_CLK_ID!(1005)),
    DEF_MOD!("ssi1", 1014, MOD_CLK_ID!(1005)), DEF_MOD!("ssi0", 1015, MOD_CLK_ID!(1005)),
    DEF_MOD!("scu-all", 1017, R8A7742_CLK_P), DEF_MOD!("scu-dvc1", 1018, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-dvc0", 1019, MOD_CLK_ID!(1017)), DEF_MOD!("scu-ctu1-mix1", 1020, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-ctu0-mix0", 1021, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src9", 1022, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-src8", 1023, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src7", 1024, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-src6", 1025, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src5", 1026, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-src4", 1027, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src3", 1028, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-src2", 1029, MOD_CLK_ID!(1017)), DEF_MOD!("scu-src1", 1030, MOD_CLK_ID!(1017)),
    DEF_MOD!("scu-src0", 1031, MOD_CLK_ID!(1017)),
];

static R8A7742_CRIT_MOD_CLKS: &[u32] = &[MOD_CLK_ID!(402), MOD_CLK_ID!(408)];

static CPG_PLL_CONFIGS: [RcarGen2CpgPllConfig; 8] = [
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 208, pll3_mult: 106 },
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 208, pll3_mult: 88 },
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 156, pll3_mult: 80 },
    RcarGen2CpgPllConfig { extal_div: 1, pll1_mult: 156, pll3_mult: 66 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 240, pll3_mult: 122 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 240, pll3_mult: 102 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 208, pll3_mult: 106 },
    RcarGen2CpgPllConfig { extal_div: 2, pll1_mult: 208, pll3_mult: 88 },
];

unsafe fn r8a7742_cpg_mssr_init(_dev: *mut Device) -> i32 {
    let mut cpg_mode: u32 = 0;
    let error = rcar_rst_read_mode_pins(&mut cpg_mode);
    if error != 0 { return error; }
    let cpg_pll_config = &CPG_PLL_CONFIGS[CPG_PLL_CONFIG_INDEX!(cpg_mode) as usize];
    rcar_gen2_cpg_init(cpg_pll_config, 2, cpg_mode)
}

static R8A7742_CPG_MSSR_INFO: CpgMssrInfo = CpgMssrInfo {
    core_clks: R8A7742_CORE_CLKS,
    num_core_clks: ARRAY_SIZE!(R8A7742_CORE_CLKS),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    mod_clks: R8A7742_MOD_CLKS,
    num_mod_clks: ARRAY_SIZE!(R8A7742_MOD_CLKS),
    num_hw_mod_clks: 12 * 32,
    crit_mod_clks: R8A7742_CRIT_MOD_CLKS,
    num_crit_mod_clks: ARRAY_SIZE!(R8A7742_CRIT_MOD_CLKS),
    init: Some(r8a7742_cpg_mssr_init),
    cpg_clk_register: Some(rcar_gen2_cpg_clk_register),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
