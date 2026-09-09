// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7724.c
 *
 * SH7724 clock framework support
 *
 * Copyright (C) 2009 Magnus Damm
 */

/* External Linux clock-framework types, constants, macros, and functions are
 * supplied by the translated kernel dependencies. */

const FRQCRA: usize = 0xa4150000;
const FRQCRB: usize = 0xa4150004;
const VCLKCR: usize = 0xa4150048;
const FCLKACR: usize = 0xa4150008;
const FCLKBCR: usize = 0xa415000c;
const IRDACLKCR: usize = 0xa4150018;
const PLLCR: usize = 0xa4150024;
const MSTPCR0: usize = 0xa4150030;
const MSTPCR1: usize = 0xa4150034;
const MSTPCR2: usize = 0xa4150038;
const SPUCLKCR: usize = 0xa415003c;
const FLLFRQ: usize = 0xa4150050;
const LSTATS: usize = 0xa4150060;

/* Fixed 32 KHz root clock for RTC and Power Management purposes */
static mut r_clk: clk = clk { rate: 32768, ..clk::default() };

/* Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code. */
static mut extal_clk: clk = clk { rate: 33333333, ..clk::default() };

unsafe fn fll_recalc(clk: *mut clk) -> ulong {
    let mut mult: ulong = 0;
    let mut div: ulong = 1;
    if __raw_readl(PLLCR) & 0x1000 != 0 { mult = __raw_readl(FLLFRQ) & 0x3ff; }
    if __raw_readl(FLLFRQ) & 0x4000 != 0 { div = 2; }
    ((*(*clk).parent).rate * mult) / div
}

static mut fll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(fll_recalc), ..sh_clk_ops::default() };
static mut fll_clk: clk = clk { ops: &mut fll_clk_ops, parent: &mut r_clk, flags: CLK_ENABLE_ON_INIT, ..clk::default() };

unsafe fn pll_recalc(clk: *mut clk) -> ulong {
    let mut mult: ulong = 1;
    if __raw_readl(PLLCR) & 0x4000 != 0 { mult = (((__raw_readl(FRQCRA) >> 24) & 0x3f) + 1) * 2; }
    (*(*clk).parent).rate * mult
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(pll_recalc), ..sh_clk_ops::default() };
static mut pll_clk: clk = clk { ops: &mut pll_clk_ops, flags: CLK_ENABLE_ON_INIT, ..clk::default() };

/* A fixed divide-by-3 block use by the div6 clocks */
unsafe fn div3_recalc(clk: *mut clk) -> ulong { (*(*clk).parent).rate / 3 }
static mut div3_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(div3_recalc), ..sh_clk_ops::default() };
static mut div3_clk: clk = clk { ops: &mut div3_clk_ops, parent: &mut pll_clk, ..clk::default() };

/* External input clock (pin name: FSIMCKA/FSIMCKB/DV_CLKI ) */
#[no_mangle] pub static mut sh7724_fsimcka_clk: clk = clk::default();
#[no_mangle] pub static mut sh7724_fsimckb_clk: clk = clk::default();
#[no_mangle] pub static mut sh7724_dv_clki: clk = clk::default();

static mut main_clks: [*mut clk; 8] = [
    &mut r_clk, &mut extal_clk, &mut fll_clk, &mut pll_clk, &mut div3_clk,
    &mut sh7724_fsimcka_clk, &mut sh7724_fsimckb_clk, &mut sh7724_dv_clki,
];

unsafe fn div4_kick(_clk: *mut clk) {
    /* set KICK bit in FRQCRA to update hardware setting */
    let mut value = __raw_readl(FRQCRA);
    value |= 1 << 31;
    __raw_writel(value, FRQCRA);
}

static mut divisors: [i32; 14] = [2, 3, 4, 6, 8, 12, 16, 0, 24, 32, 36, 48, 0, 72];
static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: divisors.as_ptr(), nr_divisors: divisors.len(),
};
static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &mut div4_div_mult_table, kick: Some(div4_kick),
};

enum { DIV4_I, DIV4_SH, DIV4_B, DIV4_P, DIV4_M1, DIV4_NR }
macro_rules! DIV4 { ($reg:expr, $bit:expr, $mask:expr, $flags:expr) => { SH_CLK_DIV4!(&mut pll_clk, $reg, $bit, $mask, $flags) }; }
#[no_mangle] pub static mut div4_clks: [clk; DIV4_NR] = [
    DIV4!(FRQCRA, 20, 0x2f7d, CLK_ENABLE_ON_INIT), DIV4!(FRQCRA, 12, 0x2f7c, CLK_ENABLE_ON_INIT),
    DIV4!(FRQCRA, 8, 0x2f7c, CLK_ENABLE_ON_INIT), DIV4!(FRQCRA, 0, 0x2f7c, 0),
    DIV4!(FRQCRB, 4, 0x2f7c, CLK_ENABLE_ON_INIT),
];

enum { DIV6_V, DIV6_I, DIV6_S, DIV6_FA, DIV6_FB, DIV6_NR }
static mut common_parent: [*mut clk; 2] = [&mut div3_clk, core::ptr::null_mut()];
static mut vclkcr_parent: [*mut clk; 8] = [&mut div3_clk, core::ptr::null_mut(), &mut sh7724_dv_clki, core::ptr::null_mut(), &mut extal_clk, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()];
static mut fclkacr_parent: [*mut clk; 4] = [&mut div3_clk, core::ptr::null_mut(), &mut sh7724_fsimcka_clk, core::ptr::null_mut()];
static mut fclkbcr_parent: [*mut clk; 4] = [&mut div3_clk, core::ptr::null_mut(), &mut sh7724_fsimckb_clk, core::ptr::null_mut()];
static mut div6_clks: [clk; DIV6_NR] = [
    SH_CLK_DIV6_EXT!(VCLKCR, 0, vclkcr_parent, vclkcr_parent.len(), 12, 3),
    SH_CLK_DIV6_EXT!(IRDACLKCR, 0, common_parent, common_parent.len(), 6, 1),
    SH_CLK_DIV6_EXT!(SPUCLKCR, CLK_ENABLE_ON_INIT, common_parent, common_parent.len(), 6, 1),
    SH_CLK_DIV6_EXT!(FCLKACR, 0, fclkacr_parent, fclkacr_parent.len(), 6, 2),
    SH_CLK_DIV6_EXT!(FCLKBCR, 0, fclkbcr_parent, fclkbcr_parent.len(), 6, 2),
];

/* Hardware block clock table. The indexed entries and framework constructors
 * are preserved exactly; HWBLK_* and SH_CLK_MSTP32! come from dependencies. */
static mut mstp_clks: [clk; HWBLK_NR] = [
    [HWBLK_TLB] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 31, CLK_ENABLE_ON_INIT),
    [HWBLK_IC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 30, CLK_ENABLE_ON_INIT),
    [HWBLK_OC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 29, CLK_ENABLE_ON_INIT),
    [HWBLK_RSMEM] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 28, CLK_ENABLE_ON_INIT),
    [HWBLK_ILMEM] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 27, CLK_ENABLE_ON_INIT),
    [HWBLK_L2C] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_SH], MSTPCR0, 26, CLK_ENABLE_ON_INIT),
    [HWBLK_FPU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 24, CLK_ENABLE_ON_INIT),
    [HWBLK_INTC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 22, CLK_ENABLE_ON_INIT),
    [HWBLK_DMAC0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 21, 0),
    [HWBLK_SHYWAY] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_SH], MSTPCR0, 20, CLK_ENABLE_ON_INIT),
    [HWBLK_HUDI] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 19, 0),
    [HWBLK_UBC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_I], MSTPCR0, 17, 0),
    [HWBLK_TMU0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 15, 0),
    [HWBLK_CMT] = SH_CLK_MSTP32!(&mut r_clk, MSTPCR0, 14, 0), [HWBLK_RWDT] = SH_CLK_MSTP32!(&mut r_clk, MSTPCR0, 13, 0),
    [HWBLK_DMAC1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 12, 0), [HWBLK_TMU1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 10, 0),
    [HWBLK_SCIF0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 9, 0), [HWBLK_SCIF1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 8, 0),
    [HWBLK_SCIF2] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR0, 7, 0), [HWBLK_SCIF3] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 6, 0),
    [HWBLK_SCIF4] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 5, 0), [HWBLK_SCIF5] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 4, 0),
    [HWBLK_MSIOF0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 2, 0), [HWBLK_MSIOF1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR0, 1, 0),
    [HWBLK_KEYSC] = SH_CLK_MSTP32!(&mut r_clk, MSTPCR1, 12, 0), [HWBLK_RTC] = SH_CLK_MSTP32!(&mut r_clk, MSTPCR1, 11, 0),
    [HWBLK_IIC0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR1, 9, 0), [HWBLK_IIC1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR1, 8, 0),
    [HWBLK_MMC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 29, 0), [HWBLK_ETHER] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 28, 0),
    [HWBLK_ATAPI] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 26, 0), [HWBLK_TPU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 25, 0),
    [HWBLK_IRDA] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_P], MSTPCR2, 24, 0), [HWBLK_TSIF] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 22, 0),
    [HWBLK_USB1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 21, 0), [HWBLK_USB0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 20, 0),
    [HWBLK_2DG] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 19, 0), [HWBLK_SDHI0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 18, 0),
    [HWBLK_SDHI1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 17, 0), [HWBLK_VEU1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 15, 0),
    [HWBLK_CEU1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 13, 0), [HWBLK_BEU1] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 12, 0),
    [HWBLK_2DDMAC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_SH], MSTPCR2, 10, 0), [HWBLK_SPU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 9, 0),
    [HWBLK_JPU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 6, 0), [HWBLK_VOU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 5, 0),
    [HWBLK_BEU0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 4, 0), [HWBLK_CEU0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 3, 0),
    [HWBLK_VEU0] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 2, 0), [HWBLK_VPU] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 1, 0),
    [HWBLK_LCDC] = SH_CLK_MSTP32!(&mut div4_clks[DIV4_B], MSTPCR2, 0, 0),
];

/* CLKDEV lookup declarations are preserved as framework macro invocations. */
static mut lookups: [clk_lookup; 0] = [];

#[no_mangle]
pub unsafe fn arch_clk_init() -> i32 {
    let mut k: usize = 0;
    let mut ret: i32 = 0;
    /* autodetect extal or fll configuration */
    if __raw_readl(PLLCR) & 0x1000 != 0 { pll_clk.parent = &mut fll_clk; }
    else { pll_clk.parent = &mut extal_clk; }
    while ret == 0 && k < main_clks.len() { ret = clk_register(main_clks[k]); k += 1; }
    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());
    if ret == 0 { ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), DIV4_NR, &mut div4_table); }
    if ret == 0 { ret = sh_clk_div6_reparent_register(div6_clks.as_mut_ptr(), DIV6_NR); }
    if ret == 0 { ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), HWBLK_NR); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
