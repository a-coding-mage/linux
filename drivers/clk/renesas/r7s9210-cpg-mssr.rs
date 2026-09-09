// SPDX-License-Identifier: GPL-2.0
/*
 * R7S9210 Clock Pulse Generator / Module Standby
 *
 * Based on r8a7795-cpg-mssr.c
 *
 * Copyright (C) 2018 Chris Brandt
 * Copyright (C) 2018 Renesas Electronics Corp.
 */

// Linux clock, I/O, device-tree bindings, and Renesas CPG/MSSR dependencies
// are supplied externally.

const CPG_FRQCR: usize = 0x00;

static mut cpg_mode: u8 = 0;

/* Internal Clock ratio table */
#[repr(C)]
struct RatioTab {
    i: u32,
    g: u32,
    b: u32,
    p1: u32,
    /* p0 is always 32 */
}

static ratio_tab: [RatioTab; 5] = [
    RatioTab { i: 2, g: 4, b: 8, p1: 16 },
    RatioTab { i: 4, g: 4, b: 8, p1: 16 },
    RatioTab { i: 8, g: 4, b: 8, p1: 16 },
    RatioTab { i: 16, g: 8, b: 16, p1: 16 },
    RatioTab { i: 16, g: 16, b: 32, p1: 32 },
];

enum RzClkTypes {
    CLK_TYPE_RZA_MAIN = CLK_TYPE_CUSTOM,
    CLK_TYPE_RZA_PLL,
}

enum ClkIds {
    /* Core Clock Outputs exported to DT */
    LAST_DT_CORE_CLK = R7S9210_CLK_P0,
    /* External Input Clocks */
    CLK_EXTAL,
    /* Internal Core Clocks */
    CLK_MAIN,
    CLK_PLL,
    /* Module Clocks */
    MOD_CLK_BASE,
}

static mut r7s9210_early_core_clks: [cpg_core_clk; 4] = [
    DEF_INPUT!("extal", CLK_EXTAL),
    DEF_BASE!(".main", CLK_MAIN, CLK_TYPE_RZA_MAIN, CLK_EXTAL),
    DEF_BASE!(".pll", CLK_PLL, CLK_TYPE_RZA_PLL, CLK_MAIN),
    DEF_FIXED!("p1c", R7S9210_CLK_P1C, CLK_PLL, 16, 1),
];

static r7s9210_early_mod_clks: [mssr_mod_clk; 3] = [
    DEF_MOD_STB!("ostm2", 34, R7S9210_CLK_P1C),
    DEF_MOD_STB!("ostm1", 35, R7S9210_CLK_P1C),
    DEF_MOD_STB!("ostm0", 36, R7S9210_CLK_P1C),
];

static mut r7s9210_core_clks: [cpg_core_clk; 5] = [
    DEF_FIXED!("i", R7S9210_CLK_I, CLK_PLL, 2, 1),
    DEF_FIXED!("g", R7S9210_CLK_G, CLK_PLL, 4, 1),
    DEF_FIXED!("b", R7S9210_CLK_B, CLK_PLL, 8, 1),
    DEF_FIXED!("p1", R7S9210_CLK_P1, CLK_PLL, 16, 1),
    DEF_FIXED!("p0", R7S9210_CLK_P0, CLK_PLL, 32, 1),
];

static r7s9210_mod_clks: [mssr_mod_clk; 21] = [
    DEF_MOD_STB!("scif4", 43, R7S9210_CLK_P1C),
    DEF_MOD_STB!("scif3", 44, R7S9210_CLK_P1C),
    DEF_MOD_STB!("scif2", 45, R7S9210_CLK_P1C),
    DEF_MOD_STB!("scif1", 46, R7S9210_CLK_P1C),
    DEF_MOD_STB!("scif0", 47, R7S9210_CLK_P1C),
    DEF_MOD_STB!("usb1", 60, R7S9210_CLK_B),
    DEF_MOD_STB!("usb0", 61, R7S9210_CLK_B),
    DEF_MOD_STB!("ether1", 64, R7S9210_CLK_B),
    DEF_MOD_STB!("ether0", 65, R7S9210_CLK_B),
    DEF_MOD_STB!("spibsc", 83, R7S9210_CLK_P1),
    DEF_MOD_STB!("i2c3", 84, R7S9210_CLK_P1),
    DEF_MOD_STB!("i2c2", 85, R7S9210_CLK_P1),
    DEF_MOD_STB!("i2c1", 86, R7S9210_CLK_P1),
    DEF_MOD_STB!("i2c0", 87, R7S9210_CLK_P1),
    DEF_MOD_STB!("spi2", 95, R7S9210_CLK_P1),
    DEF_MOD_STB!("spi1", 96, R7S9210_CLK_P1),
    DEF_MOD_STB!("spi0", 97, R7S9210_CLK_P1),
    DEF_MOD_STB!("sdhi11", 100, R7S9210_CLK_B),
    DEF_MOD_STB!("sdhi10", 101, R7S9210_CLK_B),
    DEF_MOD_STB!("sdhi01", 102, R7S9210_CLK_B),
    DEF_MOD_STB!("sdhi00", 103, R7S9210_CLK_B),
];

/* The clock dividers in the table vary based on DT and register settings */
unsafe fn r7s9210_update_clk_table(extal_clk: *mut clk, base: *mut core::ffi::c_void) {
    let mut index: usize;
    if clk_get_rate(extal_clk) > 12000000 { cpg_mode = 1; }
    let frqcr: u16 = (readl(base.add(CPG_FRQCR)) & 0xFFF) as u16;
    index = match frqcr {
        0x012 => 0, 0x112 => 1, 0x212 => 2, 0x322 => 3, 0x333 => 4,
        _ => { BUG_ON(1); 0 }
    };
    for i in 0..r7s9210_core_clks.len() {
        match r7s9210_core_clks[i].id {
            R7S9210_CLK_I => r7s9210_core_clks[i].div = ratio_tab[index].i,
            R7S9210_CLK_G => r7s9210_core_clks[i].div = ratio_tab[index].g,
            R7S9210_CLK_B => r7s9210_core_clks[i].div = ratio_tab[index].b,
            R7S9210_CLK_P1 | R7S9210_CLK_P1C => r7s9210_core_clks[i].div = ratio_tab[index].p1,
            R7S9210_CLK_P0 => r7s9210_core_clks[i].div = 32,
            _ => {}
        }
    }
}

unsafe fn rza2_cpg_clk_register(dev: *mut device, core: *const cpg_core_clk,
    info: *const cpg_mssr_info, pub_: *mut cpg_mssr_pub) -> *mut clk {
    let base = (*pub_).base0;
    let clks = (*pub_).clks;
    let mut mult = 1;
    let div = 1;
    let parent = *clks.add((*core).parent as usize);
    if IS_ERR(parent) { return ERR_CAST(parent); }
    match (*core).type_ {
        CLK_TYPE_RZA_MAIN => r7s9210_update_clk_table(parent, base),
        CLK_TYPE_RZA_PLL => { mult = if cpg_mode != 0 { 44 } else { 88 }; },
        _ => return ERR_PTR(-EINVAL),
    }
    clk_register_fixed_factor(core::ptr::null_mut(), (*core).name,
        __clk_get_name(parent), 0, mult, div)
}

static r7s9210_cpg_mssr_info: cpg_mssr_info = cpg_mssr_info {
    early_core_clks: r7s9210_early_core_clks.as_ptr(),
    num_early_core_clks: r7s9210_early_core_clks.len(),
    early_mod_clks: r7s9210_early_mod_clks.as_ptr(),
    num_early_mod_clks: r7s9210_early_mod_clks.len(),
    core_clks: r7s9210_core_clks.as_ptr(),
    num_core_clks: r7s9210_core_clks.len(),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    mod_clks: r7s9210_mod_clks.as_ptr(),
    num_mod_clks: r7s9210_mod_clks.len(),
    num_hw_mod_clks: 11 * 32,
    cpg_clk_register: Some(rza2_cpg_clk_register),
    reg_layout: CLK_REG_LAYOUT_RZ_A,
};

unsafe fn r7s9210_cpg_mssr_early_init(np: *mut device_node) {
    cpg_mssr_early_init(np, &r7s9210_cpg_mssr_info);
}

CLK_OF_DECLARE_DRIVER!(cpg_mstp_clks, "renesas,r7s9210-cpg-mssr",
    r7s9210_cpg_mssr_early_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
