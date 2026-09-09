// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4/clock-sh7757.c
 *
 * SH7757 support for the clock framework
 *
 * Copyright (C) 2009-2010  Renesas Solutions Corp.
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
static mut extal_clk: clk = clk {
    rate: 48_000_000,
};

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    let multiplier: c_int = if test_mode_pin(MODE_PIN0) != 0 { 24 } else { 16 };
    (*(*clk).parent).rate.wrapping_mul(multiplier as c_ulong)
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
};

static mut pll_clk: clk = clk {
    ops: &raw mut pll_clk_ops,
    parent: &raw mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
};

static mut clks: [*mut clk; 2] = [
    &raw mut extal_clk,
    &raw mut pll_clk,
];

static mut div2: [c_uint; 16] = [1, 1, 2, 1, 1, 4, 1, 6,
                                  1, 1, 1, 16, 1, 24, 1, 1];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: &raw mut div2,
    nr_divisors: ARRAY_SIZE(div2),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &raw mut div4_div_mult_table,
};

enum { DIV4_I, DIV4_SH, DIV4_P, DIV4_NR }

// #define DIV4(_bit, _mask, _flags) SH_CLK_DIV4(&pll_clk, FRQCR, _bit, _mask, _flags)

#[allow(non_upper_case_globals)]
static mut div4_clks: [clk; DIV4_NR] = [
    /* P clock is always enable, because some P clock modules is used
     * by Host PC.
     */
    [DIV4_P] = SH_CLK_DIV4!(&raw mut pll_clk, FRQCR, 0, 0x2800, CLK_ENABLE_ON_INIT),
    [DIV4_SH] = SH_CLK_DIV4!(&raw mut pll_clk, FRQCR, 12, 0x00a0, CLK_ENABLE_ON_INIT),
    [DIV4_I] = SH_CLK_DIV4!(&raw mut pll_clk, FRQCR, 20, 0x0004, CLK_ENABLE_ON_INIT),
];

const MSTPCR0: c_ulong = 0xffc8_0030;
const MSTPCR1: c_ulong = 0xffc8_0034;
const MSTPCR2: c_ulong = 0xffc1_0028;

enum { MSTP004, MSTP000, MSTP127, MSTP114, MSTP113, MSTP112,
       MSTP111, MSTP110, MSTP103, MSTP102, MSTP220, MSTP_NR }

static mut mstp_clks: [clk; MSTP_NR] = [
    /* MSTPCR0 */
    [MSTP004] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 4, 0),
    [MSTP000] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 0, 0),

    /* MSTPCR1 */
    [MSTP127] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 27, 0),
    [MSTP114] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 14, 0),
    [MSTP113] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 13, 0),
    [MSTP112] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 12, 0),
    [MSTP111] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 11, 0),
    [MSTP110] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 10, 0),
    [MSTP103] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 3, 0),
    [MSTP102] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 2, 0),

    /* MSTPCR2 */
    [MSTP220] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR2, 20, 0),
];

static mut lookups: [clk_lookup; 25] = [
    /* main clocks */
    CLKDEV_CON_ID!("extal", &raw mut extal_clk),
    CLKDEV_CON_ID!("pll_clk", &raw mut pll_clk),
    /* DIV4 clocks */
    CLKDEV_CON_ID!("peripheral_clk", &raw mut div4_clks[DIV4_P]),
    CLKDEV_CON_ID!("shyway_clk", &raw mut div4_clks[DIV4_SH]),
    CLKDEV_CON_ID!("cpu_clk", &raw mut div4_clks[DIV4_I]),
    /* MSTP32 clocks */
    CLKDEV_DEV_ID!("sh_mobile_sdhi.0", &raw mut mstp_clks[MSTP004]),
    CLKDEV_CON_ID!("riic0", &raw mut mstp_clks[MSTP000]), CLKDEV_CON_ID!("riic1", &raw mut mstp_clks[MSTP000]),
    CLKDEV_CON_ID!("riic2", &raw mut mstp_clks[MSTP000]), CLKDEV_CON_ID!("riic3", &raw mut mstp_clks[MSTP000]),
    CLKDEV_CON_ID!("riic4", &raw mut mstp_clks[MSTP000]), CLKDEV_CON_ID!("riic5", &raw mut mstp_clks[MSTP000]),
    CLKDEV_CON_ID!("riic6", &raw mut mstp_clks[MSTP000]), CLKDEV_CON_ID!("riic7", &raw mut mstp_clks[MSTP000]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.0", &raw mut mstp_clks[MSTP113]), CLKDEV_ICK_ID!("fck", "sh-tmu.1", &raw mut mstp_clks[MSTP114]),
    CLKDEV_ICK_ID!("fck", "sh-sci.2", &raw mut mstp_clks[MSTP112]), CLKDEV_ICK_ID!("fck", "sh-sci.1", &raw mut mstp_clks[MSTP111]),
    CLKDEV_ICK_ID!("fck", "sh-sci.0", &raw mut mstp_clks[MSTP110]), CLKDEV_CON_ID!("usb_fck", &raw mut mstp_clks[MSTP103]),
    CLKDEV_DEV_ID!("renesas_usbhs.0", &raw mut mstp_clks[MSTP102]), CLKDEV_CON_ID!("mmc0", &raw mut mstp_clks[MSTP220]),
    CLKDEV_DEV_ID!("rspi.2", &raw mut mstp_clks[MSTP127]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut ret: c_int = 0;
    let mut i = 0;
    while i < ARRAY_SIZE(clks) {
        ret |= clk_register(clks[i]);
        i += 1;
    }
    clkdev_add_table(&raw mut lookups, ARRAY_SIZE(lookups));
    if ret == 0 { ret = sh_clk_div4_register(&raw mut div4_clks, ARRAY_SIZE(div4_clks), &raw mut div4_table); }
    if ret == 0 { ret = sh_clk_mstp_register(&raw mut mstp_clks, MSTP_NR); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
