// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4/clock-shx3.c
 *
 * SH-X3 support for the clock framework
 *
 *  Copyright (C) 2006-2007  Renesas Technology Corp.
 *  Copyright (C) 2006-2007  Renesas Solutions Corp.
 *  Copyright (C) 2006-2010  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel clock framework.

/*
 * Default rate for the root input clock, reset this with clk_set_rate()
 * from the platform code.
 */
static mut extal_clk: clk = clk {
    rate: 16666666,
    ..clk::default()
};

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    /* PLL1 has a fixed x72 multiplier.  */
    (*(*clk).parent).rate * 72
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
    ..sh_clk_ops::default()
};

static mut pll_clk: clk = clk {
    ops: &mut pll_clk_ops,
    parent: &mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
    ..clk::default()
};

static mut clks: [*mut clk; 2] = [
    &mut extal_clk,
    &mut pll_clk,
];

static mut div2: [c_uint; 12] = [1, 2, 4, 6, 8, 12, 16, 18, 24, 32, 36, 48];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: div2.as_mut_ptr(),
    nr_divisors: ARRAY_SIZE!(div2),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &mut div4_div_mult_table,
};

enum Div4 {
    DIV4_I,
    DIV4_SH,
    DIV4_B,
    DIV4_DDR,
    DIV4_SHA,
    DIV4_P,
    DIV4_NR,
}

macro_rules! DIV4 {
    ($bit:expr, $mask:expr, $flags:expr) => {
        SH_CLK_DIV4!(&mut pll_clk, FRQMR1, $bit, $mask, $flags)
    };
}

static mut div4_clks: [clk; Div4::DIV4_NR as usize] = [
    /* DIV4_P */ DIV4!(0, 0x0f80, 0),
    /* DIV4_SHA */ DIV4!(4, 0x0ff0, 0),
    /* DIV4_DDR */ DIV4!(12, 0x000c, CLK_ENABLE_ON_INIT),
    /* DIV4_B */ DIV4!(16, 0x0fe0, CLK_ENABLE_ON_INIT),
    /* DIV4_SH */ DIV4!(20, 0x000c, CLK_ENABLE_ON_INIT),
    /* DIV4_I */ DIV4!(28, 0x000e, CLK_ENABLE_ON_INIT),
];

const MSTPCR0: c_ulong = 0xffc00030;
const MSTPCR1: c_ulong = 0xffc00034;

enum Mstp {
    MSTP027,
    MSTP026,
    MSTP025,
    MSTP024,
    MSTP009,
    MSTP008,
    MSTP003,
    MSTP002,
    MSTP001,
    MSTP000,
    MSTP119,
    MSTP105,
    MSTP104,
    MSTP_NR,
}

static mut mstp_clks: [clk; Mstp::MSTP_NR as usize] = [
    /* MSTPCR0 */
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 27, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 26, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 25, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 24, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 9, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 8, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 3, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 2, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 1, 0),
    SH_CLK_MSTP32!(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 0, 0),
    /* MSTPCR1 */
    SH_CLK_MSTP32!(core::ptr::null_mut(), MSTPCR1, 19, 0),
    SH_CLK_MSTP32!(core::ptr::null_mut(), MSTPCR1, 5, 0),
    SH_CLK_MSTP32!(core::ptr::null_mut(), MSTPCR1, 4, 0),
];

static mut lookups: [clk_lookup; 23] = [
    CLKDEV_CON_ID!("extal", &mut extal_clk),
    CLKDEV_CON_ID!("pll_clk", &mut pll_clk),
    CLKDEV_CON_ID!("peripheral_clk", &mut div4_clks[Div4::DIV4_P as usize]),
    CLKDEV_CON_ID!("shywaya_clk", &mut div4_clks[Div4::DIV4_SHA as usize]),
    CLKDEV_CON_ID!("ddr_clk", &mut div4_clks[Div4::DIV4_DDR as usize]),
    CLKDEV_CON_ID!("bus_clk", &mut div4_clks[Div4::DIV4_B as usize]),
    CLKDEV_CON_ID!("shyway_clk", &mut div4_clks[Div4::DIV4_SH as usize]),
    CLKDEV_CON_ID!("cpu_clk", &mut div4_clks[Div4::DIV4_I as usize]),
    CLKDEV_ICK_ID!("fck", "sh-sci.3", &mut mstp_clks[Mstp::MSTP027 as usize]),
    CLKDEV_ICK_ID!("fck", "sh-sci.2", &mut mstp_clks[Mstp::MSTP026 as usize]),
    CLKDEV_ICK_ID!("fck", "sh-sci.1", &mut mstp_clks[Mstp::MSTP025 as usize]),
    CLKDEV_ICK_ID!("fck", "sh-sci.0", &mut mstp_clks[Mstp::MSTP024 as usize]),
    CLKDEV_CON_ID!("h8ex_fck", &mut mstp_clks[Mstp::MSTP003 as usize]),
    CLKDEV_CON_ID!("csm_fck", &mut mstp_clks[Mstp::MSTP002 as usize]),
    CLKDEV_CON_ID!("fe1_fck", &mut mstp_clks[Mstp::MSTP001 as usize]),
    CLKDEV_CON_ID!("fe0_fck", &mut mstp_clks[Mstp::MSTP000 as usize]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.0", &mut mstp_clks[Mstp::MSTP008 as usize]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.1", &mut mstp_clks[Mstp::MSTP009 as usize]),
    CLKDEV_CON_ID!("hudi_fck", &mut mstp_clks[Mstp::MSTP119 as usize]),
    CLKDEV_CON_ID!("dmac_11_6_fck", &mut mstp_clks[Mstp::MSTP105 as usize]),
    CLKDEV_CON_ID!("dmac_5_0_fck", &mut mstp_clks[Mstp::MSTP104 as usize]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut ret: c_int = 0;
    for i in 0..ARRAY_SIZE!(clks) {
        ret |= clk_register(clks[i]);
    }

    clkdev_add_table(lookups.as_mut_ptr(), ARRAY_SIZE!(lookups));

    if ret == 0 {
        ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), ARRAY_SIZE!(div4_clks), &mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), Mstp::MSTP_NR as usize);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
