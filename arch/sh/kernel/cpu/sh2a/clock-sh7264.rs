// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/clock-sh7264.c
 *
 * SH7264 clock framework support
 *
 * Copyright (C) 2012  Phil Edworthy
 */

/* External Linux/kernel and architecture definitions are supplied by dependencies. */

/* SH7264 registers */
const FRQCR: u32 = 0xfffe0010;
const STBCR3: u32 = 0xfffe0408;
const STBCR4: u32 = 0xfffe040c;
const STBCR5: u32 = 0xfffe0410;
const STBCR6: u32 = 0xfffe0414;
const STBCR7: u32 = 0xfffe0418;
const STBCR8: u32 = 0xfffe041c;

static PLL1RATE: [u32; 2] = [8, 12];

static mut pll1_div: u32 = 0;

/* Fixed 32 KHz root clock for RTC */
static mut r_clk: clk = clk {
    rate: 32768,
    ..unsafe { core::mem::zeroed() }
};

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
static mut extal_clk: clk = clk {
    rate: 18000000,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    let rate = (*(*clk).parent).rate / pll1_div;
    rate * PLL1RATE[((__raw_readw(FRQCR) >> 8) & 1) as usize] as c_ulong
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
    ..unsafe { core::mem::zeroed() }
};

static mut pll_clk: clk = clk {
    ops: &raw mut pll_clk_ops,
    parent: &raw mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..unsafe { core::mem::zeroed() }
};

static mut main_clks: [*mut clk; 3] = [
    &raw mut r_clk,
    &raw mut extal_clk,
    &raw mut pll_clk,
];

static mut div2: [c_int; 7] = [1, 2, 3, 4, 6, 8, 12];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: &raw mut div2,
    nr_divisors: ARRAY_SIZE(div2),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &raw mut div4_div_mult_table,
};

enum { DIV4_I, DIV4_P, DIV4_NR }

/* The mask field specifies the div2 entries that are valid */
static mut div4_clks: [clk; DIV4_NR] = [
    SH_CLK_DIV4(&raw mut pll_clk, FRQCR, 4, 0x7, CLK_ENABLE_REG_16BIT | CLK_ENABLE_ON_INIT),
    SH_CLK_DIV4(&raw mut pll_clk, FRQCR, 0, 0x78, CLK_ENABLE_REG_16BIT),
];

enum {
    MSTP77, MSTP74, MSTP72,
    MSTP60,
    MSTP35, MSTP34, MSTP33, MSTP32, MSTP30,
    MSTP_NR,
}

static mut mstp_clks: [clk; MSTP_NR] = [
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR7, 7, 0), /* SCIF */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR7, 4, 0), /* VDC */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR7, 2, 0), /* CMT */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR6, 0, 0), /* USB */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR3, 6, 0), /* MTU2 */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR3, 4, 0), /* SDHI0 */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR3, 3, 0), /* SDHI1 */
    SH_CLK_MSTP8(&raw mut div4_clks[DIV4_P], STBCR3, 2, 0), /* ADC */
    SH_CLK_MSTP8(&raw mut r_clk, STBCR3, 0, 0), /* RTC */
];

static mut lookups: [clk_lookup; 21] = [
    CLKDEV_CON_ID("rclk", &raw mut r_clk),
    CLKDEV_CON_ID("extal", &raw mut extal_clk),
    CLKDEV_CON_ID("pll_clk", &raw mut pll_clk),
    CLKDEV_CON_ID("cpu_clk", &raw mut div4_clks[DIV4_I]),
    CLKDEV_CON_ID("peripheral_clk", &raw mut div4_clks[DIV4_P]),
    CLKDEV_ICK_ID("fck", "sh-sci.0", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.1", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.2", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.3", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.4", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.5", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.6", &raw mut mstp_clks[MSTP77]),
    CLKDEV_ICK_ID("fck", "sh-sci.7", &raw mut mstp_clks[MSTP77]),
    CLKDEV_CON_ID("vdc3", &raw mut mstp_clks[MSTP74]),
    CLKDEV_ICK_ID("fck", "sh-cmt-16.0", &raw mut mstp_clks[MSTP72]),
    CLKDEV_CON_ID("usb0", &raw mut mstp_clks[MSTP60]),
    CLKDEV_ICK_ID("fck", "sh-mtu2", &raw mut mstp_clks[MSTP35]),
    CLKDEV_CON_ID("sdhi0", &raw mut mstp_clks[MSTP34]),
    CLKDEV_CON_ID("sdhi1", &raw mut mstp_clks[MSTP33]),
    CLKDEV_CON_ID("adc0", &raw mut mstp_clks[MSTP32]),
    CLKDEV_CON_ID("rtc0", &raw mut mstp_clks[MSTP30]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut k: c_int;
    let mut ret: c_int = 0;

    if test_mode_pin(MODE_PIN0) != 0 {
        if test_mode_pin(MODE_PIN1) != 0 {
            pll1_div = 3;
        } else {
            pll1_div = 4;
        }
    } else {
        pll1_div = 1;
    }

    k = 0;
    while ret == 0 && k < ARRAY_SIZE(main_clks) as c_int {
        ret = clk_register(main_clks[k as usize]);
        k += 1;
    }

    clkdev_add_table(&raw mut lookups, ARRAY_SIZE(lookups));

    if ret == 0 {
        ret = sh_clk_div4_register(&raw mut div4_clks, DIV4_NR, &raw mut div4_table);
    }

    if ret == 0 {
        ret = sh_clk_mstp_register(&raw mut mstp_clks, MSTP_NR);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
