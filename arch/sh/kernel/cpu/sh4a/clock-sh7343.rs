// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7343.c
 *
 * SH7343 clock framework support
 *
 * Copyright (C) 2009 Magnus Damm
 */

// Linux and SH clock declarations supplied by the surrounding kernel.
use core::ffi::c_char;

const FRQCR: u32 = 0xa4150000;
const VCLKCR: u32 = 0xa4150004;
const SCLKACR: u32 = 0xa4150008;
const SCLKBCR: u32 = 0xa415000c;
const PLLCR: u32 = 0xa4150024;
const MSTPCR0: u32 = 0xa4150030;
const MSTPCR1: u32 = 0xa4150034;
const MSTPCR2: u32 = 0xa4150038;
const DLLFRQ: u32 = 0xa4150050;

extern "C" {
    static mut r_clk: clk;
    static mut extal_clk: clk;
    static mut dll_clk: clk;
    static mut pll_clk: clk;
    static mut main_clks: [*mut clk; 4];
    static mut div4_clks: [clk; DIV4_NR];
    static mut div6_clks: [clk; DIV6_NR];
    static mut mstp_clks: [clk; MSTP_NR];
    static mut lookups: [clk_lookup; 49];

    fn __raw_readl(addr: u32) -> u32;
    fn clk_register(clk: *mut clk) -> i32;
    fn clkdev_add_table(lookups: *mut clk_lookup, n: usize);
    fn sh_clk_div4_register(clks: *mut clk, n: usize, table: *mut clk_div4_table) -> i32;
    fn sh_clk_div6_register(clks: *mut clk, n: usize) -> i32;
    fn sh_clk_mstp_register(clks: *mut clk, n: usize) -> i32;
    fn SH_CLK_DIV4(parent: *mut clk, reg: u32, bit: u32, mask: u32, flags: u32) -> clk;
    fn SH_CLK_DIV6(parent: *mut clk, reg: u32, bit: u32) -> clk;
    fn SH_CLK_MSTP32(parent: *mut clk, reg: u32, bit: u32, flags: u32) -> clk;
}

#[repr(C)] pub struct clk { pub rate: usize, pub ops: *mut sh_clk_ops, pub parent: *mut clk, pub flags: u32 }
#[repr(C)] pub struct sh_clk_ops { pub recalc: Option<unsafe extern "C" fn(*mut clk) -> usize> }
#[repr(C)] pub struct clk_div_mult_table { pub divisors: *mut i32, pub nr_divisors: usize, pub multipliers: *mut i32, pub nr_multipliers: usize }
#[repr(C)] pub struct clk_div4_table { pub div_mult_table: *mut clk_div_mult_table }
#[repr(C)] pub struct clk_lookup { _private: [u8; 0] }

const CLK_ENABLE_ON_INIT: u32 = 1;

static mut r_clk_local: clk = clk { rate: 32768, ops: core::ptr::null_mut(), parent: core::ptr::null_mut(), flags: 0 };
#[no_mangle] pub static mut extal_clk_local: clk = clk { rate: 33333333, ops: core::ptr::null_mut(), parent: core::ptr::null_mut(), flags: 0 };

unsafe extern "C" fn dll_recalc(clk: *mut clk) -> usize {
    let mult: usize;
    if (__raw_readl(PLLCR) & 0x1000) != 0 { mult = __raw_readl(DLLFRQ) as usize; } else { mult = 0; }
    (*(*clk).parent).rate.wrapping_mul(mult)
}
static mut dll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(dll_recalc) };
static mut dll_clk_local: clk = clk { rate: 0, ops: &raw mut dll_clk_ops, parent: &raw mut r_clk_local, flags: CLK_ENABLE_ON_INIT };

unsafe extern "C" fn pll_recalc(clk: *mut clk) -> usize {
    let mut mult: usize = 1;
    if (__raw_readl(PLLCR) & 0x4000) != 0 { mult = (((__raw_readl(FRQCR) >> 24) & 0x1f) + 1) as usize; }
    (*(*clk).parent).rate.wrapping_mul(mult)
}
static mut pll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(pll_recalc) };
static mut pll_clk_local: clk = clk { rate: 0, ops: &raw mut pll_clk_ops, parent: core::ptr::null_mut(), flags: CLK_ENABLE_ON_INIT };

static mut main_clks_local: [*mut clk; 4] = [
    &raw mut r_clk_local, &raw mut extal_clk_local, &raw mut dll_clk_local, &raw mut pll_clk_local,
];

static mut multipliers: [i32; 13] = [1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static mut divisors: [i32; 13] = [1, 3, 2, 5, 3, 4, 5, 6, 8, 10, 12, 16, 20];
static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table { divisors: &raw mut divisors as *mut _, nr_divisors: 13, multipliers: &raw mut multipliers as *mut _, nr_multipliers: 13 };
static mut div4_table: clk_div4_table = clk_div4_table { div_mult_table: &raw mut div4_div_mult_table };

const DIV4_I: usize = 0; const DIV4_U: usize = 1; const DIV4_SH: usize = 2; const DIV4_B: usize = 3; const DIV4_B3: usize = 4; const DIV4_P: usize = 5; const DIV4_SIUA: usize = 6; const DIV4_SIUB: usize = 7; const DIV4_NR: usize = 8;
const DIV6_V: usize = 0; const DIV6_NR: usize = 1;
const MSTP_NR: usize = 47;

static mut div4_clks_local: [clk; DIV4_NR] = [const { clk { rate: 0, ops: core::ptr::null_mut(), parent: core::ptr::null_mut(), flags: 0 } }; DIV4_NR];
static mut div6_clks_local: [clk; DIV6_NR] = [const { clk { rate: 0, ops: core::ptr::null_mut(), parent: core::ptr::null_mut(), flags: 0 } }; DIV6_NR];
static mut mstp_clks_local: [clk; MSTP_NR] = [const { clk { rate: 0, ops: core::ptr::null_mut(), parent: core::ptr::null_mut(), flags: 0 } }; MSTP_NR];

#[no_mangle]
pub unsafe extern "C" fn arch_clk_init() -> i32 {
    div4_clks_local[DIV4_I] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 20, 0x1fff, CLK_ENABLE_ON_INIT);
    div4_clks_local[DIV4_U] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 16, 0x1fff, CLK_ENABLE_ON_INIT);
    div4_clks_local[DIV4_SH] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 12, 0x1fff, CLK_ENABLE_ON_INIT);
    div4_clks_local[DIV4_B] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 8, 0x1fff, CLK_ENABLE_ON_INIT);
    div4_clks_local[DIV4_B3] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 4, 0x1fff, CLK_ENABLE_ON_INIT);
    div4_clks_local[DIV4_P] = SH_CLK_DIV4(&raw mut pll_clk_local, FRQCR, 0, 0x1fff, 0);
    div4_clks_local[DIV4_SIUA] = SH_CLK_DIV4(&raw mut pll_clk_local, SCLKACR, 0, 0x1fff, 0);
    div4_clks_local[DIV4_SIUB] = SH_CLK_DIV4(&raw mut pll_clk_local, SCLKBCR, 0, 0x1fff, 0);
    div6_clks_local[DIV6_V] = SH_CLK_DIV6(&raw mut pll_clk_local, VCLKCR, 0);
    if (__raw_readl(PLLCR) & 0x1000) != 0 { pll_clk_local.parent = &raw mut dll_clk_local; } else { pll_clk_local.parent = &raw mut extal_clk_local; }
    let mut k = 0usize; let mut ret = 0i32;
    while ret == 0 && k < main_clks_local.len() { ret = clk_register(main_clks_local[k]); k += 1; }
    clkdev_add_table(core::ptr::null_mut(), 49);
    if ret == 0 { ret = sh_clk_div4_register(&raw mut div4_clks_local as *mut _, DIV4_NR, &raw mut div4_table); }
    if ret == 0 { ret = sh_clk_div6_register(&raw mut div6_clks_local as *mut _, DIV6_NR); }
    if ret == 0 { ret = sh_clk_mstp_register(&raw mut mstp_clks_local as *mut _, MSTP_NR); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
