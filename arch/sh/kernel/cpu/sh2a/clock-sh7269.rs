// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/clock-sh7269.c
 *
 * SH7269 clock framework support
 *
 * Copyright (C) 2012  Phil Edworthy
 */

// Linux dependencies supplied by the surrounding kernel translation.

/* SH7269 registers */
const FRQCR: usize = 0xfffe0010;
const STBCR3: usize = 0xfffe0408;
const STBCR4: usize = 0xfffe040c;
const STBCR5: usize = 0xfffe0410;
const STBCR6: usize = 0xfffe0414;
const STBCR7: usize = 0xfffe0418;

const PLL_RATE: u64 = 20;

/* Fixed 32 KHz root clock for RTC */
static mut r_clk: clk = clk { rate: 32768, ..clk::ZERO };

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
static mut extal_clk: clk = clk { rate: 13340000, ..clk::ZERO };

unsafe extern "C" fn pll_recalc(clk: *mut clk) -> c_ulong {
    (*(*clk).parent).rate * PLL_RATE as c_ulong
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
    ..sh_clk_ops::ZERO
};

static mut pll_clk: clk = clk {
    ops: &mut pll_clk_ops,
    parent: &mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..clk::ZERO
};

unsafe extern "C" fn peripheral0_recalc(clk: *mut clk) -> c_ulong {
    (*(*clk).parent).rate / 8
}

static mut peripheral0_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(peripheral0_recalc),
    ..sh_clk_ops::ZERO
};

static mut peripheral0_clk: clk = clk {
    ops: &mut peripheral0_clk_ops,
    parent: &mut pll_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..clk::ZERO
};

unsafe extern "C" fn peripheral1_recalc(clk: *mut clk) -> c_ulong {
    (*(*clk).parent).rate / 4
}

static mut peripheral1_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(peripheral1_recalc),
    ..sh_clk_ops::ZERO
};

static mut peripheral1_clk: clk = clk {
    ops: &mut peripheral1_clk_ops,
    parent: &mut pll_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..clk::ZERO
};

static mut main_clks: [*mut clk; 5] = unsafe {
    [&mut r_clk, &mut extal_clk, &mut pll_clk, &mut peripheral0_clk, &mut peripheral1_clk]
};

static mut div2: [c_uint; 4] = [1, 2, 0, 4];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: div2.as_mut_ptr(),
    nr_divisors: div2.len(),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &mut div4_div_mult_table,
};

enum { DIV4_I, DIV4_B, DIV4_NR }

// DIV4(_reg, _bit, _mask, _flags) expands to SH_CLK_DIV4(&pll_clk, ...).
static mut div4_clks: [clk; DIV4_NR] = unsafe {
    [
        SH_CLK_DIV4!(&mut pll_clk, FRQCR, 8, 0xB,
            CLK_ENABLE_REG_16BIT | CLK_ENABLE_ON_INIT),
        SH_CLK_DIV4!(&mut pll_clk, FRQCR, 4, 0xA,
            CLK_ENABLE_REG_16BIT | CLK_ENABLE_ON_INIT),
    ]
};

enum {
    MSTP72,
    MSTP60,
    MSTP47, MSTP46, MSTP45, MSTP44, MSTP43, MSTP42, MSTP41, MSTP40,
    MSTP35, MSTP32, MSTP30,
    MSTP_NR,
}

static mut mstp_clks: [clk; MSTP_NR] = unsafe {
    [
        SH_CLK_MSTP8!(&mut peripheral0_clk, STBCR7, 2, 0), // CMT
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR6, 0, 0), // USB
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 7, 0), // SCIF0
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 6, 0), // SCIF1
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 5, 0), // SCIF2
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 4, 0), // SCIF3
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 3, 0), // SCIF4
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 2, 0), // SCIF5
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 1, 0), // SCIF6
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR4, 0, 0), // SCIF7
        SH_CLK_MSTP8!(&mut peripheral0_clk, STBCR3, 5, 0), // MTU2
        SH_CLK_MSTP8!(&mut peripheral1_clk, STBCR3, 2, 0), // ADC
        SH_CLK_MSTP8!(&mut r_clk, STBCR3, 0, 0), // RTC
    ]
};

static mut lookups: [clk_lookup; 19] = unsafe {
    [
        CLKDEV_CON_ID!("rclk", &mut r_clk),
        CLKDEV_CON_ID!("extal", &mut extal_clk),
        CLKDEV_CON_ID!("pll_clk", &mut pll_clk),
        CLKDEV_CON_ID!("peripheral_clk", &mut peripheral1_clk),
        CLKDEV_CON_ID!("cpu_clk", &mut div4_clks[DIV4_I]),
        CLKDEV_CON_ID!("bus_clk", &mut div4_clks[DIV4_B]),
        CLKDEV_ICK_ID!("fck", "sh-sci.0", &mut mstp_clks[MSTP47]),
        CLKDEV_ICK_ID!("fck", "sh-sci.1", &mut mstp_clks[MSTP46]),
        CLKDEV_ICK_ID!("fck", "sh-sci.2", &mut mstp_clks[MSTP45]),
        CLKDEV_ICK_ID!("fck", "sh-sci.3", &mut mstp_clks[MSTP44]),
        CLKDEV_ICK_ID!("fck", "sh-sci.4", &mut mstp_clks[MSTP43]),
        CLKDEV_ICK_ID!("fck", "sh-sci.5", &mut mstp_clks[MSTP42]),
        CLKDEV_ICK_ID!("fck", "sh-sci.6", &mut mstp_clks[MSTP41]),
        CLKDEV_ICK_ID!("fck", "sh-sci.7", &mut mstp_clks[MSTP40]),
        CLKDEV_ICK_ID!("fck", "sh-cmt-16.0", &mut mstp_clks[MSTP72]),
        CLKDEV_CON_ID!("usb0", &mut mstp_clks[MSTP60]),
        CLKDEV_ICK_ID!("fck", "sh-mtu2", &mut mstp_clks[MSTP35]),
        CLKDEV_CON_ID!("adc0", &mut mstp_clks[MSTP32]),
        CLKDEV_CON_ID!("rtc0", &mut mstp_clks[MSTP30]),
    ]
};

pub unsafe extern "C" fn arch_clk_init() -> c_int {
    let mut k: usize = 0;
    let mut ret: c_int = 0;
    while ret == 0 && k < main_clks.len() {
        ret = clk_register(main_clks[k]);
        k += 1;
    }
    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());
    if ret == 0 {
        ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), DIV4_NR, &mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), MSTP_NR);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
