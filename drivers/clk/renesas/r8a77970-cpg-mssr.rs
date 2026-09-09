// SPDX-License-Identifier: GPL-2.0
/*
 * r8a77970 Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2017-2018 Cogent Embedded Inc.
 *
 * Based on r8a7795-cpg-mssr.c
 *
 * Copyright (C) 2015 Glider bvba
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const CPG_SD0CKCR: usize = 0x0074;

#[repr(C)]
enum R8a77970ClkTypes {
    ClkTypeR8a77970Sd0h = CLK_TYPE_GEN3_SOC_BASE,
    ClkTypeR8a77970Sd0,
}

enum ClkIds {
    LastDtCoreClk = R8A77970_CLK_OSC,
    ClkExtal,
    ClkExtalr,
    ClkMain,
    ClkPll0,
    ClkPll1,
    ClkPll3,
    ClkPll1Div2,
    ClkPll1Div4,
    ModClkBase,
}

static CPG_SD0H_DIV_TABLE: [ClkDivTable; 12] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 3 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 6 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 16 }, ClkDivTable { val: 7, div: 18 },
    ClkDivTable { val: 8, div: 24 }, ClkDivTable { val: 10, div: 36 },
    ClkDivTable { val: 11, div: 48 }, ClkDivTable { val: 0, div: 0 },
];

static CPG_SD0_DIV_TABLE: [ClkDivTable; 9] = [
    ClkDivTable { val: 4, div: 8 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 16 }, ClkDivTable { val: 7, div: 18 },
    ClkDivTable { val: 8, div: 24 }, ClkDivTable { val: 10, div: 36 },
    ClkDivTable { val: 11, div: 48 }, ClkDivTable { val: 12, div: 10 },
    ClkDivTable { val: 0, div: 0 },
];

static R8A77970_CORE_CLKS: &[CpgCoreClk] = &[
    DEF_INPUT!("extal", ClkExtal), DEF_INPUT!("extalr", ClkExtalr),
    DEF_BASE!(".main", ClkMain, CLK_TYPE_GEN3_MAIN, ClkExtal),
    DEF_BASE!(".pll0", ClkPll0, CLK_TYPE_GEN3_PLL0, ClkMain),
    DEF_BASE!(".pll1", ClkPll1, CLK_TYPE_GEN3_PLL1, ClkMain),
    DEF_BASE!(".pll3", ClkPll3, CLK_TYPE_GEN3_PLL3, ClkMain),
    DEF_FIXED!(".pll1_div2", ClkPll1Div2, ClkPll1, 2, 1),
    DEF_FIXED!(".pll1_div4", ClkPll1Div4, ClkPll1Div2, 2, 1),
    DEF_FIXED!("z2", R8A77970_CLK_Z2, ClkPll1Div4, 1, 1),
    DEF_FIXED!("ztr", R8A77970_CLK_ZTR, ClkPll1Div2, 6, 1),
    DEF_FIXED!("ztrd2", R8A77970_CLK_ZTRD2, ClkPll1Div2, 12, 1),
    DEF_FIXED!("zt", R8A77970_CLK_ZT, ClkPll1Div2, 4, 1),
    DEF_FIXED!("zx", R8A77970_CLK_ZX, ClkPll1Div2, 3, 1),
    DEF_FIXED!("s1d1", R8A77970_CLK_S1D1, ClkPll1Div2, 4, 1),
    DEF_FIXED!("s1d2", R8A77970_CLK_S1D2, ClkPll1Div2, 8, 1),
    DEF_FIXED!("s1d4", R8A77970_CLK_S1D4, ClkPll1Div2, 16, 1),
    DEF_FIXED!("s2d1", R8A77970_CLK_S2D1, ClkPll1Div2, 6, 1),
    DEF_FIXED!("s2d2", R8A77970_CLK_S2D2, ClkPll1Div2, 12, 1),
    DEF_FIXED!("s2d4", R8A77970_CLK_S2D4, ClkPll1Div2, 24, 1),
    DEF_BASE!("sd0h", R8A77970_CLK_SD0H, ClkTypeR8a77970Sd0h, ClkPll1Div2),
    DEF_BASE!("sd0", R8A77970_CLK_SD0, ClkTypeR8a77970Sd0, ClkPll1Div2),
    DEF_FIXED!("rpc", R8A77970_CLK_RPC, ClkPll1Div2, 5, 1),
    DEF_FIXED!("rpcd2", R8A77970_CLK_RPCD2, ClkPll1Div2, 10, 1),
    DEF_FIXED!("cl", R8A77970_CLK_CL, ClkPll1Div2, 48, 1),
    DEF_FIXED!("cp", R8A77970_CLK_CP, ClkExtal, 2, 1),
    DEF_FIXED!("cpex", R8A77970_CLK_CPEX, ClkExtal, 2, 1),
    DEF_DIV6P1!("canfd", R8A77970_CLK_CANFD, ClkPll1Div4, 0x244),
    DEF_DIV6P1!("mso", R8A77970_CLK_MSO, ClkPll1Div4, 0x014),
    DEF_DIV6P1!("csi0", R8A77970_CLK_CSI0, ClkPll1Div4, 0x00c),
    DEF_FIXED!("osc", R8A77970_CLK_OSC, ClkPll1Div2, 12 * 1024, 1),
    DEF_FIXED!("r", R8A77970_CLK_R, ClkExtalr, 1, 1),
];

static R8A77970_MOD_CLKS: &[MssrModClk] = &[
    DEF_MOD!("tmu4", 121, R8A77970_CLK_S2D2), DEF_MOD!("tmu3", 122, R8A77970_CLK_S2D2),
    DEF_MOD!("tmu2", 123, R8A77970_CLK_S2D2), DEF_MOD!("tmu1", 124, R8A77970_CLK_S2D2),
    DEF_MOD!("tmu0", 125, R8A77970_CLK_CP), DEF_MOD!("ivcp1e", 127, R8A77970_CLK_S2D1),
    DEF_MOD!("scif4", 203, R8A77970_CLK_S2D4), DEF_MOD!("scif3", 204, R8A77970_CLK_S2D4),
    DEF_MOD!("scif1", 206, R8A77970_CLK_S2D4), DEF_MOD!("scif0", 207, R8A77970_CLK_S2D4),
    DEF_MOD!("msiof3", 208, R8A77970_CLK_MSO), DEF_MOD!("msiof2", 209, R8A77970_CLK_MSO),
    DEF_MOD!("msiof1", 210, R8A77970_CLK_MSO), DEF_MOD!("msiof0", 211, R8A77970_CLK_MSO),
    DEF_MOD!("mfis", 213, R8A77970_CLK_S2D2), DEF_MOD!("sys-dmac2", 217, R8A77970_CLK_S2D1),
    DEF_MOD!("sys-dmac1", 218, R8A77970_CLK_S2D1), DEF_MOD!("cmt3", 300, R8A77970_CLK_R),
    DEF_MOD!("cmt2", 301, R8A77970_CLK_R), DEF_MOD!("cmt1", 302, R8A77970_CLK_R),
    DEF_MOD!("cmt0", 303, R8A77970_CLK_R), DEF_MOD!("tpu0", 304, R8A77970_CLK_S2D4),
    DEF_MOD!("sd-if", 314, R8A77970_CLK_SD0), DEF_MOD!("rwdt", 402, R8A77970_CLK_R),
    DEF_MOD!("intc-ex", 407, R8A77970_CLK_CP), DEF_MOD!("intc-ap", 408, R8A77970_CLK_S2D1),
    DEF_MOD!("hscif3", 517, R8A77970_CLK_S2D1), DEF_MOD!("hscif2", 518, R8A77970_CLK_S2D1),
    DEF_MOD!("hscif1", 519, R8A77970_CLK_S2D1), DEF_MOD!("hscif0", 520, R8A77970_CLK_S2D1),
    DEF_MOD!("thermal", 522, R8A77970_CLK_CP), DEF_MOD!("pwm", 523, R8A77970_CLK_S2D4),
    DEF_MOD!("fcpvd0", 603, R8A77970_CLK_S2D1), DEF_MOD!("vspd0", 623, R8A77970_CLK_S2D1),
    DEF_MOD!("csi40", 716, R8A77970_CLK_CSI0), DEF_MOD!("du0", 724, R8A77970_CLK_S2D1),
    DEF_MOD!("lvds", 727, R8A77970_CLK_S2D1), DEF_MOD!("vin3", 808, R8A77970_CLK_S2D1),
    DEF_MOD!("vin2", 809, R8A77970_CLK_S2D1), DEF_MOD!("vin1", 810, R8A77970_CLK_S2D1),
    DEF_MOD!("vin0", 811, R8A77970_CLK_S2D1), DEF_MOD!("etheravb", 812, R8A77970_CLK_S2D2),
    DEF_MOD!("gpio5", 907, R8A77970_CLK_CP), DEF_MOD!("gpio4", 908, R8A77970_CLK_CP),
    DEF_MOD!("gpio3", 909, R8A77970_CLK_CP), DEF_MOD!("gpio2", 910, R8A77970_CLK_CP),
    DEF_MOD!("gpio1", 911, R8A77970_CLK_CP), DEF_MOD!("gpio0", 912, R8A77970_CLK_CP),
    DEF_MOD!("can-fd", 914, R8A77970_CLK_S2D2), DEF_MOD!("rpc-if", 917, R8A77970_CLK_RPC),
    DEF_MOD!("i2c4", 927, R8A77970_CLK_S2D2), DEF_MOD!("i2c3", 928, R8A77970_CLK_S2D2),
    DEF_MOD!("i2c2", 929, R8A77970_CLK_S2D2), DEF_MOD!("i2c1", 930, R8A77970_CLK_S2D2),
    DEF_MOD!("i2c0", 931, R8A77970_CLK_S2D2),
];

static R8A77970_CRIT_MOD_CLKS: &[u32] = &[MOD_CLK_ID!(402), MOD_CLK_ID!(408)];

// CPG Clock Data; the table corresponds to the eight MD14/MD13/MD19 modes.
macro_rules! CPG_PLL_CONFIG_INDEX { ($md:expr) => { ((($md & BIT(14)) >> 12) | (($md & BIT(13)) >> 12) | (($md & BIT(19)) >> 19)) }; }

static CPG_PLL_CONFIGS: [RcarGen3CpgPllConfig; 8] = [
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 192, pll1_div: 1, pll3_mult: 96, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 192, pll1_div: 1, pll3_mult: 80, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 160, pll1_div: 1, pll3_mult: 80, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 160, pll1_div: 1, pll3_mult: 66, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 2, pll1_mult: 236, pll1_div: 1, pll3_mult: 118, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 2, pll1_mult: 236, pll1_div: 1, pll3_mult: 98, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 2, pll1_mult: 192, pll1_div: 1, pll3_mult: 96, pll3_div: 1 },
    RcarGen3CpgPllConfig { extal_div: 2, pll1_mult: 192, pll1_div: 1, pll3_mult: 80, pll3_div: 1 },
];

unsafe fn r8a77970_cpg_mssr_init(dev: *mut Device) -> i32 {
    let mut cpg_mode: u32 = 0;
    let error = rcar_rst_read_mode_pins(&mut cpg_mode);
    if error != 0 { return error; }
    let cpg_pll_config = &CPG_PLL_CONFIGS[CPG_PLL_CONFIG_INDEX!(cpg_mode) as usize];
    rcar_gen3_cpg_init(cpg_pll_config, ClkExtalr as i32, cpg_mode)
}

unsafe fn r8a77970_cpg_clk_register(dev: *mut Device, core: *const CpgCoreClk, info: *const CpgMssrInfo, pub_: *mut CpgMssrPub) -> *mut Clk {
    let (table, shift) = match (*core).type_ {
        ClkTypeR8a77970Sd0h => (&CPG_SD0H_DIV_TABLE as *const _, 8),
        ClkTypeR8a77970Sd0 => (&CPG_SD0_DIV_TABLE as *const _, 4),
        _ => return rcar_gen3_cpg_clk_register(dev, core, info, pub_),
    };
    let base = (*pub_).base0;
    let clks = (*pub_).clks;
    let parent = *clks.add((*core).parent as usize);
    if IS_ERR(parent) { return ERR_CAST(parent); }
    clk_register_divider_table(core_null!(), (*core).name, __clk_get_name(parent), 0,
        base.add(CPG_SD0CKCR), shift, 4, 0, table, &mut cpg_lock)
}

#[no_mangle]
pub static R8A77970_CPG_MSSR_INFO: CpgMssrInfo = CpgMssrInfo {
    core_clks: R8A77970_CORE_CLKS.as_ptr(), num_core_clks: R8A77970_CORE_CLKS.len(),
    last_dt_core_clk: LastDtCoreClk as usize, num_total_core_clks: ModClkBase as usize,
    mod_clks: R8A77970_MOD_CLKS.as_ptr(), num_mod_clks: R8A77970_MOD_CLKS.len(), num_hw_mod_clks: 12 * 32,
    crit_mod_clks: R8A77970_CRIT_MOD_CLKS.as_ptr(), num_crit_mod_clks: R8A77970_CRIT_MOD_CLKS.len(),
    init: Some(r8a77970_cpg_mssr_init), cpg_clk_register: Some(r8a77970_cpg_clk_register),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
