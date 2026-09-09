// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7786.c
 *
 * SH7786 support for the clock framework
 *
 *  Copyright (C) 2010  Paul Mundt
 */
// Dependencies supplied by the surrounding kernel translation.

static mut extal_clk: clk = clk { rate: 33333333 };

unsafe fn pll_recalc(clk: *mut clk) -> c_ulong {
    let multiplier: c_int;

    /*
     * Clock modes 0, 1, and 2 use an x64 multiplier against PLL1,
     * while modes 3, 4, and 5 use an x32.
     */
    multiplier = if (sh_mv.mv_mode_pins)() & 0xf < 3 { 64 } else { 32 };

    (*(*clk).parent).rate * multiplier as c_ulong
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(pll_recalc),
};

static mut pll_clk: clk = clk {
    ops: &mut pll_clk_ops,
    parent: &mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
};

static mut clks: [*mut clk; 2] = [&mut extal_clk, &mut pll_clk];

static mut div2: [c_uint; 12] = [1, 2, 4, 6, 8, 12, 16, 18, 24, 32, 36, 48];

static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: div2.as_mut_ptr(),
    nr_divisors: ARRAY_SIZE(&div2),
};

static mut div4_table: clk_div4_table = clk_div4_table {
    div_mult_table: &mut div4_div_mult_table,
};

enum {
    DIV4_I, DIV4_SH, DIV4_B, DIV4_DDR, DIV4_DU, DIV4_P, DIV4_NR,
}

macro_rules! DIV4 {
    ($bit:expr, $mask:expr, $flags:expr) => {
        SH_CLK_DIV4(&mut pll_clk, FRQMR1, $bit, $mask, $flags)
    };
}

static mut div4_clks: [clk; DIV4_NR] = [
    [DIV4_P] = DIV4!(0, 0x0b40, 0),
    [DIV4_DU] = DIV4!(4, 0x0010, 0),
    [DIV4_DDR] = DIV4!(12, 0x0002, CLK_ENABLE_ON_INIT),
    [DIV4_B] = DIV4!(16, 0x0360, CLK_ENABLE_ON_INIT),
    [DIV4_SH] = DIV4!(20, 0x0002, CLK_ENABLE_ON_INIT),
    [DIV4_I] = DIV4!(28, 0x0006, CLK_ENABLE_ON_INIT),
];

const MSTPCR0: usize = 0xffc40030;
const MSTPCR1: usize = 0xffc40034;

enum {
    MSTP029, MSTP028, MSTP027, MSTP026, MSTP025, MSTP024,
    MSTP023, MSTP022, MSTP021, MSTP020, MSTP017, MSTP016,
    MSTP015, MSTP014, MSTP011, MSTP010, MSTP009, MSTP008,
    MSTP005, MSTP004, MSTP002,
    MSTP112, MSTP110, MSTP109, MSTP108,
    MSTP105, MSTP104, MSTP103, MSTP102,
    MSTP_NR,
}

static mut mstp_clks: [clk; MSTP_NR] = [
    [MSTP029] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 29, 0),
    [MSTP028] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 28, 0),
    [MSTP027] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 27, 0),
    [MSTP026] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 26, 0),
    [MSTP025] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 25, 0),
    [MSTP024] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 24, 0),
    [MSTP023] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 23, 0),
    [MSTP022] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 22, 0),
    [MSTP021] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 21, 0),
    [MSTP020] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 20, 0),
    [MSTP017] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 17, 0),
    [MSTP016] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 16, 0),
    [MSTP015] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 15, 0),
    [MSTP014] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 14, 0),
    [MSTP011] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 11, 0),
    [MSTP010] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 10, 0),
    [MSTP009] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 9, 0),
    [MSTP008] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 8, 0),
    [MSTP005] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 5, 0),
    [MSTP004] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 4, 0),
    [MSTP002] = SH_CLK_MSTP32(&mut div4_clks[DIV4_P], MSTPCR0, 2, 0),
    [MSTP112] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 12, 0),
    [MSTP110] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 10, 0),
    [MSTP109] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 9, 0),
    [MSTP108] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 8, 0),
    [MSTP105] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 5, 0),
    [MSTP104] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 4, 0),
    [MSTP103] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 3, 0),
    [MSTP102] = SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 2, 0),
];

static mut lookups: [clk_lookup; 37] = [
    CLKDEV_CON_ID!("extal", &mut extal_clk),
    CLKDEV_CON_ID!("pll_clk", &mut pll_clk),
    CLKDEV_CON_ID!("peripheral_clk", &mut div4_clks[DIV4_P]),
    CLKDEV_CON_ID!("du_clk", &mut div4_clks[DIV4_DU]),
    CLKDEV_CON_ID!("ddr_clk", &mut div4_clks[DIV4_DDR]),
    CLKDEV_CON_ID!("bus_clk", &mut div4_clks[DIV4_B]),
    CLKDEV_CON_ID!("shyway_clk", &mut div4_clks[DIV4_SH]),
    CLKDEV_CON_ID!("cpu_clk", &mut div4_clks[DIV4_I]),
    CLKDEV_ICK_ID!("fck", "sh-sci.5", &mut mstp_clks[MSTP029]),
    CLKDEV_ICK_ID!("fck", "sh-sci.4", &mut mstp_clks[MSTP028]),
    CLKDEV_ICK_ID!("fck", "sh-sci.3", &mut mstp_clks[MSTP027]),
    CLKDEV_ICK_ID!("fck", "sh-sci.2", &mut mstp_clks[MSTP026]),
    CLKDEV_ICK_ID!("fck", "sh-sci.1", &mut mstp_clks[MSTP025]),
    CLKDEV_ICK_ID!("fck", "sh-sci.0", &mut mstp_clks[MSTP024]),
    CLKDEV_CON_ID!("ssi3_fck", &mut mstp_clks[MSTP023]),
    CLKDEV_CON_ID!("ssi2_fck", &mut mstp_clks[MSTP022]),
    CLKDEV_CON_ID!("ssi1_fck", &mut mstp_clks[MSTP021]),
    CLKDEV_CON_ID!("ssi0_fck", &mut mstp_clks[MSTP020]),
    CLKDEV_CON_ID!("hac1_fck", &mut mstp_clks[MSTP017]),
    CLKDEV_CON_ID!("hac0_fck", &mut mstp_clks[MSTP016]),
    CLKDEV_CON_ID!("i2c1_fck", &mut mstp_clks[MSTP015]),
    CLKDEV_CON_ID!("i2c0_fck", &mut mstp_clks[MSTP014]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.0", &mut mstp_clks[MSTP008]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.1", &mut mstp_clks[MSTP009]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.2", &mut mstp_clks[MSTP010]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.3", &mut mstp_clks[MSTP011]),
    CLKDEV_CON_ID!("sdif1_fck", &mut mstp_clks[MSTP005]),
    CLKDEV_CON_ID!("sdif0_fck", &mut mstp_clks[MSTP004]),
    CLKDEV_CON_ID!("hspi_fck", &mut mstp_clks[MSTP002]),
    CLKDEV_CON_ID!("usb_fck", &mut mstp_clks[MSTP112]),
    CLKDEV_CON_ID!("pcie2_fck", &mut mstp_clks[MSTP110]),
    CLKDEV_CON_ID!("pcie1_fck", &mut mstp_clks[MSTP109]),
    CLKDEV_CON_ID!("pcie0_fck", &mut mstp_clks[MSTP108]),
    CLKDEV_CON_ID!("dmac_11_6_fck", &mut mstp_clks[MSTP105]),
    CLKDEV_CON_ID!("dmac_5_0_fck", &mut mstp_clks[MSTP104]),
    CLKDEV_CON_ID!("du_fck", &mut mstp_clks[MSTP103]),
    CLKDEV_CON_ID!("ether_fck", &mut mstp_clks[MSTP102]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    while i < ARRAY_SIZE(&clks) as c_int {
        ret |= clk_register(clks[i as usize]);
        i += 1;
    }
    clkdev_add_table(lookups.as_mut_ptr(), ARRAY_SIZE(&lookups));
    if ret == 0 {
        ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), ARRAY_SIZE(&div4_clks), &mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), MSTP_NR);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
