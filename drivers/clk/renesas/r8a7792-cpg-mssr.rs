// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7792 Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2017 Glider bvba
 *
 * Based on clk-rcar-gen2.c
 *
 * Copyright (C) 2013 Ideas On Board SPRL
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(i32)]
enum ClkIds {
    // Core Clock Outputs exported to DT
    LAST_DT_CORE_CLK = R8A7792_CLK_OSC,
    // External Input Clocks
    CLK_EXTAL,
    // Internal Core Clocks
    CLK_MAIN,
    CLK_PLL0,
    CLK_PLL1,
    CLK_PLL3,
    CLK_PLL1_DIV2,
    // Module Clocks
    MOD_CLK_BASE,
}

static R8A7792_CORE_CLKS: [cpg_core_clk; 26] = [
    DEF_INPUT!("extal", CLK_EXTAL),
    DEF_BASE!(".main", CLK_MAIN, CLK_TYPE_GEN2_MAIN, CLK_EXTAL),
    DEF_BASE!(".pll0", CLK_PLL0, CLK_TYPE_GEN2_PLL0, CLK_MAIN),
    DEF_BASE!(".pll1", CLK_PLL1, CLK_TYPE_GEN2_PLL1, CLK_MAIN),
    DEF_BASE!(".pll3", CLK_PLL3, CLK_TYPE_GEN2_PLL3, CLK_MAIN),
    DEF_FIXED!(".pll1_div2", CLK_PLL1_DIV2, CLK_PLL1, 2, 1),
    DEF_BASE!("qspi", R8A7792_CLK_QSPI, CLK_TYPE_GEN2_QSPI, CLK_PLL1_DIV2),
    DEF_FIXED!("z", R8A7792_CLK_Z, CLK_PLL0, 1, 1),
    DEF_FIXED!("zg", R8A7792_CLK_ZG, CLK_PLL1, 5, 1),
    DEF_FIXED!("zx", R8A7792_CLK_ZX, CLK_PLL1, 3, 1),
    DEF_FIXED!("zs", R8A7792_CLK_ZS, CLK_PLL1, 6, 1),
    DEF_FIXED!("hp", R8A7792_CLK_HP, CLK_PLL1, 12, 1),
    DEF_FIXED!("i", R8A7792_CLK_I, CLK_PLL1, 3, 1),
    DEF_FIXED!("b", R8A7792_CLK_B, CLK_PLL1, 12, 1),
    DEF_FIXED!("lb", R8A7792_CLK_LB, CLK_PLL1, 24, 1),
    DEF_FIXED!("p", R8A7792_CLK_P, CLK_PLL1, 24, 1),
    DEF_FIXED!("cl", R8A7792_CLK_CL, CLK_PLL1, 48, 1),
    DEF_FIXED!("m2", R8A7792_CLK_M2, CLK_PLL1, 8, 1),
    DEF_FIXED!("imp", R8A7792_CLK_IMP, CLK_PLL1, 4, 1),
    DEF_FIXED!("zb3", R8A7792_CLK_ZB3, CLK_PLL3, 4, 1),
    DEF_FIXED!("zb3d2", R8A7792_CLK_ZB3D2, CLK_PLL3, 8, 1),
    DEF_FIXED!("ddr", R8A7792_CLK_DDR, CLK_PLL3, 8, 1),
    DEF_FIXED!("sd", R8A7792_CLK_SD, CLK_PLL1_DIV2, 8, 1),
    DEF_FIXED!("mp", R8A7792_CLK_MP, CLK_PLL1_DIV2, 15, 1),
    DEF_FIXED!("cp", R8A7792_CLK_CP, CLK_PLL1, 48, 1),
    DEF_FIXED!("cpex", R8A7792_CLK_CPEX, CLK_EXTAL, 2, 1),
    DEF_FIXED!("rcan", R8A7792_CLK_RCAN, CLK_PLL1_DIV2, 49, 1),
    DEF_FIXED!("r", R8A7792_CLK_R, CLK_PLL1, 49152, 1),
    DEF_FIXED!("osc", R8A7792_CLK_OSC, CLK_PLL1, 12288, 1),
];

static R8A7792_MOD_CLKS: [mssr_mod_clk; 67] = [
    DEF_MOD!("msiof0", 0, R8A7792_CLK_MP), DEF_MOD!("jpu", 106, R8A7792_CLK_M2),
    DEF_MOD!("tmu1", 111, R8A7792_CLK_P), DEF_MOD!("3dg", 112, R8A7792_CLK_ZG),
    DEF_MOD!("2d-dmac", 115, R8A7792_CLK_ZS), DEF_MOD!("tmu3", 121, R8A7792_CLK_P),
    DEF_MOD!("tmu2", 122, R8A7792_CLK_P), DEF_MOD!("cmt0", 124, R8A7792_CLK_R),
    DEF_MOD!("tmu0", 125, R8A7792_CLK_CP), DEF_MOD!("vsp1du1", 127, R8A7792_CLK_ZS),
    DEF_MOD!("vsp1du0", 128, R8A7792_CLK_ZS), DEF_MOD!("vsps", 131, R8A7792_CLK_ZS),
    DEF_MOD!("msiof1", 208, R8A7792_CLK_MP), DEF_MOD!("sys-dmac1", 218, R8A7792_CLK_ZS),
    DEF_MOD!("sys-dmac0", 219, R8A7792_CLK_ZS), DEF_MOD!("tpu0", 304, R8A7792_CLK_CP),
    DEF_MOD!("sdhi0", 314, R8A7792_CLK_SD), DEF_MOD!("cmt1", 329, R8A7792_CLK_R),
    DEF_MOD!("rwdt", 402, R8A7792_CLK_R), DEF_MOD!("irqc", 407, R8A7792_CLK_CP),
    DEF_MOD!("intc-sys", 408, R8A7792_CLK_ZS), DEF_MOD!("audio-dmac0", 502, R8A7792_CLK_HP),
    DEF_MOD!("thermal", 522, CLK_EXTAL), DEF_MOD!("pwm", 523, R8A7792_CLK_P),
    DEF_MOD!("hscif1", 716, R8A7792_CLK_ZS), DEF_MOD!("hscif0", 717, R8A7792_CLK_ZS),
    DEF_MOD!("scif3", 718, R8A7792_CLK_P), DEF_MOD!("scif2", 719, R8A7792_CLK_P),
    DEF_MOD!("scif1", 720, R8A7792_CLK_P), DEF_MOD!("scif0", 721, R8A7792_CLK_P),
    DEF_MOD!("du1", 723, R8A7792_CLK_ZX), DEF_MOD!("du0", 724, R8A7792_CLK_ZX),
    DEF_MOD!("vin5", 804, R8A7792_CLK_ZG), DEF_MOD!("vin4", 805, R8A7792_CLK_ZG),
    DEF_MOD!("vin3", 808, R8A7792_CLK_ZG), DEF_MOD!("vin2", 809, R8A7792_CLK_ZG),
    DEF_MOD!("vin1", 810, R8A7792_CLK_ZG), DEF_MOD!("vin0", 811, R8A7792_CLK_ZG),
    DEF_MOD!("etheravb", 812, R8A7792_CLK_HP), DEF_MOD!("imr-lx3", 821, R8A7792_CLK_ZG),
    DEF_MOD!("imr-lsx3-1", 822, R8A7792_CLK_ZG), DEF_MOD!("imr-lsx3-0", 823, R8A7792_CLK_ZG),
    DEF_MOD!("imr-lsx3-5", 825, R8A7792_CLK_ZG), DEF_MOD!("imr-lsx3-4", 826, R8A7792_CLK_ZG),
    DEF_MOD!("imr-lsx3-3", 827, R8A7792_CLK_ZG), DEF_MOD!("imr-lsx3-2", 828, R8A7792_CLK_ZG),
    DEF_MOD!("gyro-adc", 901, R8A7792_CLK_P), DEF_MOD!("gpio7", 904, R8A7792_CLK_CP),
    DEF_MOD!("gpio6", 905, R8A7792_CLK_CP), DEF_MOD!("gpio5", 907, R8A7792_CLK_CP),
    DEF_MOD!("gpio4", 908, R8A7792_CLK_CP), DEF_MOD!("gpio3", 909, R8A7792_CLK_CP),
    DEF_MOD!("gpio2", 910, R8A7792_CLK_CP), DEF_MOD!("gpio1", 911, R8A7792_CLK_CP),
    DEF_MOD!("gpio0", 912, R8A7792_CLK_CP), DEF_MOD!("gpio11", 913, R8A7792_CLK_CP),
    DEF_MOD!("gpio10", 914, R8A7792_CLK_CP), DEF_MOD!("can1", 915, R8A7792_CLK_P),
    DEF_MOD!("can0", 916, R8A7792_CLK_P), DEF_MOD!("qspi_mod", 917, R8A7792_CLK_QSPI),
    DEF_MOD!("gpio9", 919, R8A7792_CLK_CP), DEF_MOD!("gpio8", 921, R8A7792_CLK_CP),
    DEF_MOD!("i2c5", 925, R8A7792_CLK_HP), DEF_MOD!("iicdvfs", 926, R8A7792_CLK_CP),
    DEF_MOD!("i2c4", 927, R8A7792_CLK_HP), DEF_MOD!("i2c3", 928, R8A7792_CLK_HP),
    DEF_MOD!("i2c2", 929, R8A7792_CLK_HP), DEF_MOD!("i2c1", 930, R8A7792_CLK_HP),
    DEF_MOD!("i2c0", 931, R8A7792_CLK_HP), DEF_MOD!("ssi-all", 1005, R8A7792_CLK_P),
    DEF_MOD!("ssi4", 1011, MOD_CLK_ID!(1005)), DEF_MOD!("ssi3", 1012, MOD_CLK_ID!(1005)),
];

static R8A7792_CRIT_MOD_CLKS: [u32; 2] = [MOD_CLK_ID!(402), MOD_CLK_ID!(408)];

macro_rules! CPG_PLL_CONFIG_INDEX { ($md:expr) => { ((($md & BIT!(14)) >> 12) | (($md & BIT!(13)) >> 12) | (($md & BIT!(19)) >> 19)) }; }

static CPG_PLL_CONFIGS: [rcar_gen2_cpg_pll_config; 8] = [
    rcar_gen2_cpg_pll_config { extal_div: 1, pll1_mult: 208, pll3_mult: 106, pll0_mult: 200 },
    rcar_gen2_cpg_pll_config { extal_div: 1, pll1_mult: 208, pll3_mult: 88, pll0_mult: 200 },
    rcar_gen2_cpg_pll_config { extal_div: 1, pll1_mult: 156, pll3_mult: 80, pll0_mult: 150 },
    rcar_gen2_cpg_pll_config { extal_div: 1, pll1_mult: 156, pll3_mult: 66, pll0_mult: 150 },
    rcar_gen2_cpg_pll_config { extal_div: 2, pll1_mult: 240, pll3_mult: 122, pll0_mult: 230 },
    rcar_gen2_cpg_pll_config { extal_div: 2, pll1_mult: 240, pll3_mult: 102, pll0_mult: 230 },
    rcar_gen2_cpg_pll_config { extal_div: 2, pll1_mult: 208, pll3_mult: 106, pll0_mult: 200 },
    rcar_gen2_cpg_pll_config { extal_div: 2, pll1_mult: 208, pll3_mult: 88, pll0_mult: 200 },
];

unsafe extern "C" {
    fn rcar_rst_read_mode_pins(cpg_mode: *mut u32) -> i32;
    fn rcar_gen2_cpg_init(config: *const rcar_gen2_cpg_pll_config, pll3_mult: i32, cpg_mode: u32) -> i32;
    static rcar_gen2_cpg_clk_register: unsafe extern "C" fn();
}

unsafe extern "C" fn r8a7792_cpg_mssr_init(_dev: *mut device) -> i32 {
    let mut cpg_mode: u32 = 0;
    let error = rcar_rst_read_mode_pins(&mut cpg_mode);
    if error != 0 { return error; }
    let cpg_pll_config = &CPG_PLL_CONFIGS[CPG_PLL_CONFIG_INDEX!(cpg_mode) as usize];
    rcar_gen2_cpg_init(cpg_pll_config, 3, cpg_mode)
}

static R8A7792_CPG_MSSR_INFO: cpg_mssr_info = cpg_mssr_info {
    core_clks: R8A7792_CORE_CLKS.as_ptr(),
    num_core_clks: R8A7792_CORE_CLKS.len(),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    mod_clks: R8A7792_MOD_CLKS.as_ptr(),
    num_mod_clks: R8A7792_MOD_CLKS.len(),
    num_hw_mod_clks: 12 * 32,
    crit_mod_clks: R8A7792_CRIT_MOD_CLKS.as_ptr(),
    num_crit_mod_clks: R8A7792_CRIT_MOD_CLKS.len(),
    init: Some(r8a7792_cpg_mssr_init),
    cpg_clk_register: Some(rcar_gen2_cpg_clk_register),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
