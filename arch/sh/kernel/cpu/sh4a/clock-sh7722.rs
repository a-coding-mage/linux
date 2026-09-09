// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7722.c
 *
 * SH7722 clock framework support
 *
 * Copyright (C) 2009 Magnus Damm
 */

// External kernel dependencies supplied by the surrounding build.

/* SH7722 registers */
const FRQCR: usize = 0xa4150000;
const VCLKCR: usize = 0xa4150004;
const SCLKACR: usize = 0xa4150008;
const SCLKBCR: usize = 0xa415000c;
const IRDACLKCR: usize = 0xa4150018;
const PLLCR: usize = 0xa4150024;
const MSTPCR0: usize = 0xa4150030;
const MSTPCR1: usize = 0xa4150034;
const MSTPCR2: usize = 0xa4150038;
const DLLFRQ: usize = 0xa4150050;

/* Fixed 32 KHz root clock for RTC and Power Management purposes */
static mut r_clk: clk = clk {
    rate: 32768,
    ..unsafe { core::mem::zeroed() }
};

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
pub static mut extal_clk: clk = clk {
    rate: 33333333,
    ..unsafe { core::mem::zeroed() }
};

/* The dll block multiplies the 32khz r_clk, may be used instead of extal */
unsafe fn dll_recalc(clk: *mut clk) -> c_ulong {
    let mult: c_ulong;

    if __raw_readl(PLLCR) & 0x1000 != 0 {
        mult = __raw_readl(DLLFRQ) as c_ulong;
    } else {
        mult = 0;
    }

    (*(*clk).parent).rate.wrapping_mul(mult)
}

static mut dll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(dll_recalc),
    ..unsafe { core::mem::zeroed() }
};

static mut dll_clk: clk = clk {
    ops: &raw mut dll_clk_ops,
    parent: &raw mut r_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    let mut mult: c_ulong = 1;
    let mut div: c_ulong = 1;

    if __raw_readl(PLLCR) & 0x4000 != 0 {
        mult = (((__raw_readl(FRQCR) >> 24) & 0x1f) + 1) as c_ulong;
    } else {
        div = 2;
    }

    ((*(*clk).parent).rate.wrapping_mul(mult)) / div
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
    ..unsafe { core::mem::zeroed() }
};

static mut pll_clk: clk = clk {
    ops: &raw mut pll_clk_ops,
    flags: CLK_ENABLE_ON_INIT,
    ..unsafe { core::mem::zeroed() }
};

pub static mut main_clks: [*mut clk; 4] = [
    &raw mut r_clk,
    &raw mut extal_clk,
    &raw mut dll_clk,
    &raw mut pll_clk,
];

static mut multipliers: [c_int; 13] = [1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static mut divisors: [c_int; 13] = [1, 3, 2, 5, 3, 4, 5, 6, 8, 10, 12, 16, 20];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: &raw mut divisors,
    nr_divisors: divisors.len(),
    multipliers: &raw mut multipliers,
    nr_multipliers: multipliers.len(),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &raw mut div4_div_mult_table,
};

macro_rules! DIV4 {
    ($reg:expr, $bit:expr, $mask:expr, $flags:expr) => {
        SH_CLK_DIV4!(&raw mut pll_clk, $reg, $bit, $mask, $flags)
    };
}

const DIV4_I: usize = 0;
const DIV4_U: usize = 1;
const DIV4_SH: usize = 2;
const DIV4_B: usize = 3;
const DIV4_B3: usize = 4;
const DIV4_P: usize = 5;
const DIV4_NR: usize = 6;

pub static mut div4_clks: [clk; DIV4_NR] = [
    [DIV4!(FRQCR, 20, 0x1fef, CLK_ENABLE_ON_INIT), DIV4!(FRQCR, 16, 0x1fff, CLK_ENABLE_ON_INIT), DIV4!(FRQCR, 12, 0x1fff, CLK_ENABLE_ON_INIT), DIV4!(FRQCR, 8, 0x1fff, CLK_ENABLE_ON_INIT), DIV4!(FRQCR, 4, 0x1fff, CLK_ENABLE_ON_INIT), DIV4!(FRQCR, 0, 0x1fff, 0)]
];

const DIV4_IRDA: usize = 0;
const DIV4_ENABLE_NR: usize = 1;
pub static mut div4_enable_clks: [clk; DIV4_ENABLE_NR] = [DIV4!(IRDACLKCR, 0, 0x1fff, 0)];

const DIV4_SIUA: usize = 0;
const DIV4_SIUB: usize = 1;
const DIV4_REPARENT_NR: usize = 2;
pub static mut div4_reparent_clks: [clk; DIV4_REPARENT_NR] = [
    DIV4!(SCLKACR, 0, 0x1fff, 0),
    DIV4!(SCLKBCR, 0, 0x1fff, 0),
];

const DIV6_V: usize = 0;
const DIV6_NR: usize = 1;
pub static mut div6_clks: [clk; DIV6_NR] = [SH_CLK_DIV6!(&raw mut pll_clk, VCLKCR, 0)];

static mut mstp_clks: [clk; HWBLK_NR] = [
    [HWBLK_URAM] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_U], MSTPCR0, 28, CLK_ENABLE_ON_INIT),
    [HWBLK_XYMEM] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR0, 26, CLK_ENABLE_ON_INIT),
    [HWBLK_TMU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 15, 0),
    [HWBLK_CMT] = SH_CLK_MSTP32!(&raw mut r_clk, MSTPCR0, 14, 0),
    [HWBLK_RWDT] = SH_CLK_MSTP32!(&raw mut r_clk, MSTPCR0, 13, 0),
    [HWBLK_FLCTL] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 10, 0),
    [HWBLK_SCIF0] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 7, 0),
    [HWBLK_SCIF1] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 6, 0),
    [HWBLK_SCIF2] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 5, 0),
    [HWBLK_IIC] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 9, 0),
    [HWBLK_RTC] = SH_CLK_MSTP32!(&raw mut r_clk, MSTPCR1, 8, 0),
    [HWBLK_SDHI] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR2, 18, 0),
    [HWBLK_KEYSC] = SH_CLK_MSTP32!(&raw mut r_clk, MSTPCR2, 14, 0),
    [HWBLK_USBF] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR2, 11, 0),
    [HWBLK_2DG] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 9, 0),
    [HWBLK_SIU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 8, 0),
    [HWBLK_JPU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 6, 0),
    [HWBLK_VOU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 5, 0),
    [HWBLK_BEU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 4, 0),
    [HWBLK_CEU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 3, 0),
    [HWBLK_VEU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 2, 0),
    [HWBLK_VPU] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_B], MSTPCR2, 1, 0),
    [HWBLK_LCDC] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR2, 0, 0),
];

static mut lookups: [clk_lookup; 35] = [
    /* main clocks */
    CLKDEV_CON_ID!("rclk", &raw mut r_clk),
    CLKDEV_CON_ID!("extal", &raw mut extal_clk),
    CLKDEV_CON_ID!("dll_clk", &raw mut dll_clk),
    CLKDEV_CON_ID!("pll_clk", &raw mut pll_clk),
    /* DIV4 clocks */
    CLKDEV_CON_ID!("cpu_clk", &raw mut div4_clks[DIV4_I]),
    CLKDEV_CON_ID!("umem_clk", &raw mut div4_clks[DIV4_U]),
    CLKDEV_CON_ID!("shyway_clk", &raw mut div4_clks[DIV4_SH]),
    CLKDEV_CON_ID!("bus_clk", &raw mut div4_clks[DIV4_B]),
    CLKDEV_CON_ID!("b3_clk", &raw mut div4_clks[DIV4_B3]),
    CLKDEV_CON_ID!("peripheral_clk", &raw mut div4_clks[DIV4_P]),
    CLKDEV_CON_ID!("irda_clk", &raw mut div4_enable_clks[DIV4_IRDA]),
    CLKDEV_CON_ID!("siua_clk", &raw mut div4_reparent_clks[DIV4_SIUA]),
    CLKDEV_CON_ID!("siub_clk", &raw mut div4_reparent_clks[DIV4_SIUB]),
    /* DIV6 clocks */
    CLKDEV_CON_ID!("video_clk", &raw mut div6_clks[DIV6_V]),
    /* MSTP clocks */
    CLKDEV_CON_ID!("uram0", &raw mut mstp_clks[HWBLK_URAM]),
    CLKDEV_CON_ID!("xymem0", &raw mut mstp_clks[HWBLK_XYMEM]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.0", &raw mut mstp_clks[HWBLK_TMU]),
    CLKDEV_ICK_ID!("fck", "sh-cmt-32.0", &raw mut mstp_clks[HWBLK_CMT]),
    CLKDEV_DEV_ID!("sh-wdt.0", &raw mut mstp_clks[HWBLK_RWDT]),
    CLKDEV_CON_ID!("flctl0", &raw mut mstp_clks[HWBLK_FLCTL]),
    CLKDEV_DEV_ID!("sh-sci.0", &raw mut mstp_clks[HWBLK_SCIF0]),
    CLKDEV_DEV_ID!("sh-sci.1", &raw mut mstp_clks[HWBLK_SCIF1]),
    CLKDEV_DEV_ID!("sh-sci.2", &raw mut mstp_clks[HWBLK_SCIF2]),
    CLKDEV_DEV_ID!("i2c-sh_mobile.0", &raw mut mstp_clks[HWBLK_IIC]),
    CLKDEV_CON_ID!("rtc0", &raw mut mstp_clks[HWBLK_RTC]),
    CLKDEV_DEV_ID!("sh_mobile_sdhi.0", &raw mut mstp_clks[HWBLK_SDHI]),
    CLKDEV_DEV_ID!("sh_keysc.0", &raw mut mstp_clks[HWBLK_KEYSC]),
    CLKDEV_CON_ID!("usbf0", &raw mut mstp_clks[HWBLK_USBF]),
    CLKDEV_CON_ID!("2dg0", &raw mut mstp_clks[HWBLK_2DG]),
    CLKDEV_DEV_ID!("siu-pcm-audio", &raw mut mstp_clks[HWBLK_SIU]),
    CLKDEV_DEV_ID!("sh-vou.0", &raw mut mstp_clks[HWBLK_VOU]),
    CLKDEV_CON_ID!("jpu0", &raw mut mstp_clks[HWBLK_JPU]),
    CLKDEV_CON_ID!("beu0", &raw mut mstp_clks[HWBLK_BEU]),
    CLKDEV_DEV_ID!("renesas-ceu.0", &raw mut mstp_clks[HWBLK_CEU]),
    CLKDEV_CON_ID!("veu0", &raw mut mstp_clks[HWBLK_VEU]),
    CLKDEV_CON_ID!("vpu0", &raw mut mstp_clks[HWBLK_VPU]),
    CLKDEV_DEV_ID!("sh_mobile_lcdc_fb.0", &raw mut mstp_clks[HWBLK_LCDC]),
];

pub unsafe fn arch_clk_init() -> c_int {
    let mut k: usize;
    let mut ret: c_int = 0;

    /* autodetect extal or dll configuration */
    if __raw_readl(PLLCR) & 0x1000 != 0 {
        pll_clk.parent = &raw mut dll_clk;
    } else {
        pll_clk.parent = &raw mut extal_clk;
    }

    k = 0;
    while ret == 0 && k < main_clks.len() {
        ret = clk_register(main_clks[k]);
        k += 1;
    }

    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());

    if ret == 0 {
        ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), DIV4_NR, &raw mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_div4_enable_register(div4_enable_clks.as_mut_ptr(), DIV4_ENABLE_NR, &raw mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_div4_reparent_register(div4_reparent_clks.as_mut_ptr(), DIV4_REPARENT_NR, &raw mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_div6_register(div6_clks.as_mut_ptr(), DIV6_NR);
    }
    if ret == 0 {
        ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), HWBLK_NR);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
