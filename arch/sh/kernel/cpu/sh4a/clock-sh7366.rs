// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7366.c
 *
 * SH7366 clock framework support
 *
 * Copyright (C) 2009 Magnus Damm
 */
// External kernel dependencies are supplied by the surrounding translation.

/* SH7366 registers */
const FRQCR: usize = 0xa4150000;
const VCLKCR: usize = 0xa4150004;
const SCLKACR: usize = 0xa4150008;
const SCLKBCR: usize = 0xa415000c;
const PLLCR: usize = 0xa4150024;
const MSTPCR0: usize = 0xa4150030;
const MSTPCR1: usize = 0xa4150034;
const MSTPCR2: usize = 0xa4150038;
const DLLFRQ: usize = 0xa4150050;

/* Fixed 32 KHz root clock for RTC and Power Management purposes */
static mut r_clk: clk = clk { rate: 32768, ..clk::default() };

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
pub static mut extal_clk: clk = clk { rate: 33333333, ..clk::default() };

/* The dll block multiplies the 32khz r_clk, may be used instead of extal */
unsafe fn dll_recalc(clk: *mut clk) -> c_ulong {
    let mult: c_ulong;
    if __raw_readl(PLLCR) & 0x1000 != 0 {
        mult = __raw_readl(DLLFRQ) as c_ulong;
    } else {
        mult = 0;
    }
    (*(*clk).parent).rate * mult
}

static mut dll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(dll_recalc), ..sh_clk_ops::default() };
static mut dll_clk: clk = clk {
    ops: &mut dll_clk_ops,
    parent: &mut r_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..clk::default()
};

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    let mut mult: c_ulong = 1;
    let mut div: c_ulong = 1;
    if __raw_readl(PLLCR) & 0x4000 != 0 {
        mult = (((__raw_readl(FRQCR) >> 24) & 0x1f) + 1) as c_ulong;
    } else {
        div = 2;
    }
    ((*(*clk).parent).rate * mult) / div
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(pll_recalc), ..sh_clk_ops::default() };
static mut pll_clk: clk = clk { ops: &mut pll_clk_ops, flags: CLK_ENABLE_ON_INIT, ..clk::default() };

pub static mut main_clks: [*mut clk; 4] = [
    &mut r_clk, &mut extal_clk, &mut dll_clk, &mut pll_clk,
];

static mut multipliers: [c_int; 13] = [1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static mut divisors: [c_int; 13] = [1, 3, 2, 5, 3, 4, 5, 6, 8, 10, 12, 16, 20];
static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: divisors.as_mut_ptr(), nr_divisors: divisors.len(),
    multipliers: multipliers.as_mut_ptr(), nr_multipliers: multipliers.len(),
};
static mut div4_table: clk_div4_table = clk_div4_table { div_mult_table: &mut div4_div_mult_table };

pub const DIV4_I: i32 = 0;
pub const DIV4_U: i32 = DIV4_I + 1;
pub const DIV4_SH: i32 = DIV4_U + 1;
pub const DIV4_B: i32 = DIV4_SH + 1;
pub const DIV4_B3: i32 = DIV4_B + 1;
pub const DIV4_P: i32 = DIV4_B3 + 1;
pub const DIV4_SIUA: i32 = DIV4_P + 1;
pub const DIV4_SIUB: i32 = DIV4_SIUA + 1;
pub const DIV4_NR: i32 = DIV4_SIUB + 1;

pub static mut div4_clks: [clk; DIV4_NR] = [
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 20, 0x1fef, CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 16, 0x1fff, CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 12, 0x1fff, CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 8, 0x1fff, CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 4, 0x1fff, CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4!(&mut pll_clk, FRQCR, 0, 0x1fff, 0),
    SH_CLK_DIV4!(&mut pll_clk, SCLKACR, 0, 0x1fff, 0),
    SH_CLK_DIV4!(&mut pll_clk, SCLKBCR, 0, 0x1fff, 0),
];

pub const DIV6_V: i32 = 0;
pub const DIV6_NR: i32 = DIV6_V + 1;
pub static mut div6_clks: [clk; DIV6_NR] = [SH_CLK_DIV6!(&mut pll_clk, VCLKCR, 0)];

pub const MSTP031: i32 = 0;
pub const MSTP030: i32 = MSTP031 + 1;
pub const MSTP029: i32 = MSTP030 + 1;
pub const MSTP028: i32 = MSTP029 + 1;
pub const MSTP026: i32 = MSTP028 + 1;
pub const MSTP023: i32 = MSTP026 + 1;
pub const MSTP022: i32 = MSTP023 + 1;
pub const MSTP021: i32 = MSTP022 + 1;
pub const MSTP020: i32 = MSTP021 + 1;
pub const MSTP019: i32 = MSTP020 + 1;
pub const MSTP018: i32 = MSTP019 + 1;
pub const MSTP017: i32 = MSTP018 + 1;
pub const MSTP016: i32 = MSTP017 + 1;
pub const MSTP015: i32 = MSTP016 + 1;
pub const MSTP014: i32 = MSTP015 + 1;
pub const MSTP013: i32 = MSTP014 + 1;
pub const MSTP012: i32 = MSTP013 + 1;
pub const MSTP011: i32 = MSTP012 + 1;
pub const MSTP010: i32 = MSTP011 + 1;
pub const MSTP007: i32 = MSTP010 + 1;
pub const MSTP006: i32 = MSTP007 + 1;
pub const MSTP005: i32 = MSTP006 + 1;
pub const MSTP002: i32 = MSTP005 + 1;
pub const MSTP001: i32 = MSTP002 + 1;
pub const MSTP109: i32 = MSTP001 + 1;
pub const MSTP100: i32 = MSTP109 + 1;
pub const MSTP227: i32 = MSTP100 + 1;
pub const MSTP226: i32 = MSTP227 + 1;
pub const MSTP224: i32 = MSTP226 + 1;
pub const MSTP223: i32 = MSTP224 + 1;
pub const MSTP222: i32 = MSTP223 + 1;
pub const MSTP218: i32 = MSTP222 + 1;
pub const MSTP217: i32 = MSTP218 + 1;
pub const MSTP211: i32 = MSTP217 + 1;
pub const MSTP207: i32 = MSTP211 + 1;
pub const MSTP205: i32 = MSTP207 + 1;
pub const MSTP204: i32 = MSTP205 + 1;
pub const MSTP203: i32 = MSTP204 + 1;
pub const MSTP202: i32 = MSTP203 + 1;
pub const MSTP201: i32 = MSTP202 + 1;
pub const MSTP200: i32 = MSTP201 + 1;
pub const MSTP_NR: i32 = MSTP200 + 1;

static mut mstp_clks: [clk; MSTP_NR] = [
    MSTP!(&mut div4_clks[DIV4_I], MSTPCR0, 31, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_I], MSTPCR0, 30, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_I], MSTPCR0, 29, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_SH], MSTPCR0, 28, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_B], MSTPCR0, 26, CLK_ENABLE_ON_INIT),
    MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 23, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 22, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 21, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 20, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 19, 0),
    MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 17, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 15, 0), MSTP!(&mut r_clk, MSTPCR0, 14, 0), MSTP!(&mut r_clk, MSTPCR0, 13, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 11, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 10, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 7, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 6, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 5, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 2, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR0, 1, 0),
    MSTP!(&mut div4_clks[DIV4_P], MSTPCR1, 9, 0),
    MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 27, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 26, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 24, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 23, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 22, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 18, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 17, 0), MSTP!(&mut div4_clks[DIV4_P], MSTPCR2, 11, 0), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 7, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 5, 0), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 4, 0), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 3, 0), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 2, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 1, CLK_ENABLE_ON_INIT), MSTP!(&mut div4_clks[DIV4_B], MSTPCR2, 0, 0),
];

// CLKDEV lookup declarations are retained as external helper macro calls.
static mut lookups: [clk_lookup; 49] = [
    CLKDEV_CON_ID!("rclk", &mut r_clk), CLKDEV_CON_ID!("extal", &mut extal_clk), CLKDEV_CON_ID!("dll_clk", &mut dll_clk), CLKDEV_CON_ID!("pll_clk", &mut pll_clk),
    CLKDEV_CON_ID!("cpu_clk", &mut div4_clks[DIV4_I]), CLKDEV_CON_ID!("umem_clk", &mut div4_clks[DIV4_U]), CLKDEV_CON_ID!("shyway_clk", &mut div4_clks[DIV4_SH]), CLKDEV_CON_ID!("bus_clk", &mut div4_clks[DIV4_B]), CLKDEV_CON_ID!("b3_clk", &mut div4_clks[DIV4_B3]), CLKDEV_CON_ID!("peripheral_clk", &mut div4_clks[DIV4_P]), CLKDEV_CON_ID!("siua_clk", &mut div4_clks[DIV4_SIUA]), CLKDEV_CON_ID!("siub_clk", &mut div4_clks[DIV4_SIUB]),
    CLKDEV_CON_ID!("video_clk", &mut div6_clks[DIV6_V]),
    CLKDEV_CON_ID!("tlb0", &mut mstp_clks[MSTP031]), CLKDEV_CON_ID!("ic0", &mut mstp_clks[MSTP030]), CLKDEV_CON_ID!("oc0", &mut mstp_clks[MSTP029]), CLKDEV_CON_ID!("rsmem0", &mut mstp_clks[MSTP028]), CLKDEV_CON_ID!("xymem0", &mut mstp_clks[MSTP026]), CLKDEV_CON_ID!("intc3", &mut mstp_clks[MSTP023]), CLKDEV_CON_ID!("intc0", &mut mstp_clks[MSTP022]), CLKDEV_CON_ID!("dmac0", &mut mstp_clks[MSTP021]), CLKDEV_CON_ID!("sh0", &mut mstp_clks[MSTP020]), CLKDEV_CON_ID!("hudi0", &mut mstp_clks[MSTP019]), CLKDEV_CON_ID!("ubc0", &mut mstp_clks[MSTP017]), CLKDEV_CON_ID!("tmu_fck", &mut mstp_clks[MSTP015]), CLKDEV_ICK_ID!("fck", "sh-cmt-32.0", &mut mstp_clks[MSTP014]), CLKDEV_CON_ID!("rwdt0", &mut mstp_clks[MSTP013]), CLKDEV_CON_ID!("mfi0", &mut mstp_clks[MSTP011]), CLKDEV_CON_ID!("flctl0", &mut mstp_clks[MSTP010]),
    CLKDEV_ICK_ID!("fck", "sh-sci.0", &mut mstp_clks[MSTP007]), CLKDEV_ICK_ID!("fck", "sh-sci.1", &mut mstp_clks[MSTP006]), CLKDEV_ICK_ID!("fck", "sh-sci.2", &mut mstp_clks[MSTP005]), CLKDEV_CON_ID!("msiof0", &mut mstp_clks[MSTP002]), CLKDEV_CON_ID!("sbr0", &mut mstp_clks[MSTP001]), CLKDEV_DEV_ID!("i2c-sh_mobile.0", &mut mstp_clks[MSTP109]), CLKDEV_CON_ID!("icb0", &mut mstp_clks[MSTP227]), CLKDEV_CON_ID!("meram0", &mut mstp_clks[MSTP226]), CLKDEV_CON_ID!("dacy1", &mut mstp_clks[MSTP224]), CLKDEV_CON_ID!("dacy0", &mut mstp_clks[MSTP223]), CLKDEV_CON_ID!("tsif0", &mut mstp_clks[MSTP222]), CLKDEV_CON_ID!("sdhi0", &mut mstp_clks[MSTP218]), CLKDEV_CON_ID!("mmcif0", &mut mstp_clks[MSTP217]), CLKDEV_CON_ID!("usbf0", &mut mstp_clks[MSTP211]), CLKDEV_CON_ID!("veu1", &mut mstp_clks[MSTP207]), CLKDEV_CON_ID!("vou0", &mut mstp_clks[MSTP205]), CLKDEV_CON_ID!("beu0", &mut mstp_clks[MSTP204]), CLKDEV_CON_ID!("ceu0", &mut mstp_clks[MSTP203]), CLKDEV_CON_ID!("veu0", &mut mstp_clks[MSTP202]), CLKDEV_CON_ID!("vpu0", &mut mstp_clks[MSTP201]), CLKDEV_CON_ID!("lcdc0", &mut mstp_clks[MSTP200]),
];

pub unsafe fn arch_clk_init() -> c_int {
    let mut k: usize = 0;
    let mut ret: c_int = 0;
    /* autodetect extal or dll configuration */
    if __raw_readl(PLLCR) & 0x1000 != 0 { pll_clk.parent = &mut dll_clk; } else { pll_clk.parent = &mut extal_clk; }
    while ret == 0 && k < main_clks.len() { ret = clk_register(main_clks[k]); k += 1; }
    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());
    if ret == 0 { ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), DIV4_NR, &mut div4_table); }
    if ret == 0 { ret = sh_clk_div6_register(div6_clks.as_mut_ptr(), DIV6_NR); }
    if ret == 0 { ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), MSTP_NR); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
