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

enum { DIV4_I, DIV4_U, DIV4_SH, DIV4_B, DIV4_B3, DIV4_P, DIV4_SIUA, DIV4_SIUB, DIV4_NR }

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

enum { DIV6_V, DIV6_NR }
pub static mut div6_clks: [clk; DIV6_NR] = [SH_CLK_DIV6!(&mut pll_clk, VCLKCR, 0)];

enum { MSTP031, MSTP030, MSTP029, MSTP028, MSTP026, MSTP023, MSTP022, MSTP021, MSTP020, MSTP019, MSTP018, MSTP017, MSTP016, MSTP015, MSTP014, MSTP013, MSTP012, MSTP011, MSTP010, MSTP007, MSTP006, MSTP005, MSTP002, MSTP001, MSTP109, MSTP100, MSTP227, MSTP226, MSTP224, MSTP223, MSTP222, MSTP218, MSTP217, MSTP211, MSTP207, MSTP205, MSTP204, MSTP203, MSTP202, MSTP201, MSTP200, MSTP_NR }

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
