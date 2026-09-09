// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7734.c
 *
 * Clock framework for SH7734
 *
 * Copyright (C) 2011, 2012 Nobuhiro Iwamatsu <nobuhiro.iwamatsu.yj@renesas.com>
 * Copyright (C) 2011, 2012 Renesas Solutions Corp.
 */

// Linux and architecture headers provide the clock types, macros, and functions used below.

static mut extal_clk: struct clk = struct clk {
    rate: 33333333,
};

const MODEMR: usize = 0xFFCC0020;
const MODEMR_MASK: u32 = 0x6;
const MODEMR_533MHZ: u32 = 0x2;

unsafe fn pll_recalc(clk: *mut struct clk) -> c_ulong {
    let mut mode: c_int = 12;
    let r: u32 = __raw_readl(MODEMR);

    if (r & MODEMR_MASK) & MODEMR_533MHZ != 0 {
        mode = 16;
    }

    (*(*clk).parent).rate * mode as c_ulong
}

static mut pll_clk_ops: struct sh_clk_ops = struct sh_clk_ops {
    recalc: Some(pll_recalc),
};

static mut pll_clk: struct clk = struct clk {
    ops: &raw mut pll_clk_ops,
    parent: &raw mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
};

static mut main_clks: [*mut struct clk; 2] = [
    &raw mut extal_clk,
    &raw mut pll_clk,
];

static mut multipliers: [c_int; 13] = [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static mut divisors: [c_int; 12] = [1, 3, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24];

static mut div4_div_mult_table: struct clk_div_mult_table = struct clk_div_mult_table {
    divisors: &raw mut divisors,
    nr_divisors: ARRAY_SIZE(divisors),
    multipliers: &raw mut multipliers,
    nr_multipliers: ARRAY_SIZE(multipliers),
};

static mut div4_table: struct clk_div4_table = struct clk_div4_table {
    div_mult_table: &raw mut div4_div_mult_table,
};

enum {
    DIV4_I, DIV4_S, DIV4_B, DIV4_M, DIV4_S1, DIV4_P, DIV4_NR
}

macro_rules! DIV4 {
    ($reg:expr, $bit:expr, $mask:expr, $flags:expr) => {
        SH_CLK_DIV4!(&raw mut pll_clk, $reg, $bit, $mask, $flags)
    };
}

static mut div4_clks: [struct clk; DIV4_NR] = [
    [DIV4_I] = DIV4!(FRQMR1, 28, 0x0003, CLK_ENABLE_ON_INIT),
    [DIV4_S] = DIV4!(FRQMR1, 20, 0x000C, CLK_ENABLE_ON_INIT),
    [DIV4_B] = DIV4!(FRQMR1, 16, 0x0140, CLK_ENABLE_ON_INIT),
    [DIV4_M] = DIV4!(FRQMR1, 12, 0x0004, CLK_ENABLE_ON_INIT),
    [DIV4_S1] = DIV4!(FRQMR1, 4, 0x0030, CLK_ENABLE_ON_INIT),
    [DIV4_P] = DIV4!(FRQMR1, 0, 0x0140, CLK_ENABLE_ON_INIT),
];

const MSTPCR0: usize = 0xFFC80030;
const MSTPCR1: usize = 0xFFC80034;
const MSTPCR3: usize = 0xFFC8003C;

enum {
    MSTP030, MSTP029, /* IIC */
    MSTP026, MSTP025, MSTP024, /* SCIF */
    MSTP023,
    MSTP022, MSTP021,
    MSTP019, /* HSCIF */
    MSTP016, MSTP015, MSTP014, /* TMU / TIMER */
    MSTP012, MSTP011, MSTP010, MSTP009, MSTP008, /* SSI */
    MSTP007, /* HSPI */
    MSTP115, /* ADMAC */
    MSTP114, /* GETHER */
    MSTP111, /* DMAC */
    MSTP109, /* VIDEOIN1 */
    MSTP108, /* VIDEOIN0 */
    MSTP107, /* RGPVBG */
    MSTP106, /* 2DG */
    MSTP103, /* VIEW */
    MSTP100, /* USB */
    MSTP331, /* MMC */
    MSTP330, /* MIMLB */
    MSTP323, /* SDHI0 */
    MSTP322, /* SDHI1 */
    MSTP321, /* SDHI2 */
    MSTP320, /* RQSPI */
    MSTP319, /* SRC0 */
    MSTP318, /* SRC1 */
    MSTP317, /* RSPI */
    MSTP316, /* RCAN0 */
    MSTP315, /* RCAN1 */
    MSTP314, /* FLTCL */
    MSTP313, /* ADC */
    MSTP312, /* MTU */
    MSTP304, /* IE-BUS */
    MSTP303, /* RTC */
    MSTP302, /* HIF */
    MSTP301, /* STIF0 */
    MSTP300, /* STIF1 */
    MSTP_NR
}

// The following table is a literal translation of the C SH_CLK_MSTP32 initializers.
static mut mstp_clks: [struct clk; MSTP_NR] = [
    [MSTP030] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 30, 0),
    [MSTP029] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 29, 0),
    [MSTP026] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 26, 0),
    [MSTP025] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 25, 0),
    [MSTP024] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 24, 0),
    [MSTP023] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 23, 0),
    [MSTP022] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 22, 0),
    [MSTP021] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 21, 0),
    [MSTP019] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 19, 0),
    [MSTP016] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 16, 0),
    [MSTP015] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 15, 0),
    [MSTP014] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 14, 0),
    [MSTP012] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 12, 0),
    [MSTP011] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 11, 0),
    [MSTP010] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 10, 0),
    [MSTP009] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 9, 0),
    [MSTP008] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 8, 0),
    [MSTP007] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR0, 7, 0),
    [MSTP115] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 15, 0),
    [MSTP114] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 14, 0),
    [MSTP111] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 11, 0),
    [MSTP109] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 9, 0),
    [MSTP108] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 8, 0),
    [MSTP107] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 7, 0),
    [MSTP106] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 6, 0),
    [MSTP103] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 3, 0),
    [MSTP100] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR1, 0, 0),
    [MSTP331] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 31, 0),
    [MSTP330] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 30, 0),
    [MSTP323] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 23, 0),
    [MSTP322] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 22, 0),
    [MSTP321] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 21, 0),
    [MSTP320] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 20, 0),
    [MSTP319] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 19, 0),
    [MSTP318] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 18, 0),
    [MSTP317] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 17, 0),
    [MSTP316] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 16, 0),
    [MSTP315] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 15, 0),
    [MSTP314] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 14, 0),
    [MSTP313] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 13, 0),
    [MSTP312] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 12, 0),
    [MSTP304] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 4, 0),
    [MSTP303] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 3, 0),
    [MSTP302] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 2, 0),
    [MSTP301] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 1, 0),
    [MSTP300] = SH_CLK_MSTP32!(&raw mut div4_clks[DIV4_P], MSTPCR3, 0, 0),
];

// Lookup declarations are retained through the corresponding external macros.
static mut lookups: [struct clk_lookup; 49] = [
    CLKDEV_CON_ID!("extal", &raw mut extal_clk),
    CLKDEV_CON_ID!("pll_clk", &raw mut pll_clk),
    CLKDEV_CON_ID!("cpu_clk", &raw mut div4_clks[DIV4_I]),
    CLKDEV_CON_ID!("shyway_clk", &raw mut div4_clks[DIV4_S]),
    CLKDEV_CON_ID!("ddr_clk", &raw mut div4_clks[DIV4_M]),
    CLKDEV_CON_ID!("bus_clk", &raw mut div4_clks[DIV4_B]),
    CLKDEV_CON_ID!("shyway_clk1", &raw mut div4_clks[DIV4_S1]),
    CLKDEV_CON_ID!("peripheral_clk", &raw mut div4_clks[DIV4_P]),
    CLKDEV_DEV_ID!("i2c-sh7734.0", &raw mut mstp_clks[MSTP030]),
    CLKDEV_DEV_ID!("i2c-sh7734.1", &raw mut mstp_clks[MSTP029]),
    CLKDEV_ICK_ID!("fck", "sh-sci.0", &raw mut mstp_clks[MSTP026]),
    CLKDEV_ICK_ID!("fck", "sh-sci.1", &raw mut mstp_clks[MSTP025]),
    CLKDEV_ICK_ID!("fck", "sh-sci.2", &raw mut mstp_clks[MSTP024]),
    CLKDEV_ICK_ID!("fck", "sh-sci.3", &raw mut mstp_clks[MSTP023]),
    CLKDEV_ICK_ID!("fck", "sh-sci.4", &raw mut mstp_clks[MSTP022]),
    CLKDEV_ICK_ID!("fck", "sh-sci.5", &raw mut mstp_clks[MSTP021]),
    CLKDEV_CON_ID!("hscif", &raw mut mstp_clks[MSTP019]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.0", &raw mut mstp_clks[MSTP016]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.1", &raw mut mstp_clks[MSTP015]),
    CLKDEV_ICK_ID!("fck", "sh-tmu.2", &raw mut mstp_clks[MSTP014]),
    CLKDEV_CON_ID!("ssi0", &raw mut mstp_clks[MSTP012]),
    CLKDEV_CON_ID!("ssi1", &raw mut mstp_clks[MSTP011]),
    CLKDEV_CON_ID!("ssi2", &raw mut mstp_clks[MSTP010]),
    CLKDEV_CON_ID!("ssi3", &raw mut mstp_clks[MSTP009]),
    CLKDEV_CON_ID!("sss", &raw mut mstp_clks[MSTP008]),
    CLKDEV_CON_ID!("hspi", &raw mut mstp_clks[MSTP007]),
    CLKDEV_CON_ID!("usb_fck", &raw mut mstp_clks[MSTP100]),
    CLKDEV_CON_ID!("videoin0", &raw mut mstp_clks[MSTP109]),
    CLKDEV_CON_ID!("videoin1", &raw mut mstp_clks[MSTP108]),
    CLKDEV_CON_ID!("rgpvg", &raw mut mstp_clks[MSTP107]),
    CLKDEV_CON_ID!("2dg", &raw mut mstp_clks[MSTP106]),
    CLKDEV_CON_ID!("view", &raw mut mstp_clks[MSTP103]),
    CLKDEV_CON_ID!("mmc0", &raw mut mstp_clks[MSTP331]),
    CLKDEV_CON_ID!("mimlb0", &raw mut mstp_clks[MSTP330]),
    CLKDEV_CON_ID!("sdhi0", &raw mut mstp_clks[MSTP323]),
    CLKDEV_CON_ID!("sdhi1", &raw mut mstp_clks[MSTP322]),
    CLKDEV_CON_ID!("sdhi2", &raw mut mstp_clks[MSTP321]),
    CLKDEV_CON_ID!("rqspi0", &raw mut mstp_clks[MSTP320]),
    CLKDEV_CON_ID!("src0", &raw mut mstp_clks[MSTP319]),
    CLKDEV_CON_ID!("src1", &raw mut mstp_clks[MSTP318]),
    CLKDEV_CON_ID!("rsp0", &raw mut mstp_clks[MSTP317]),
    CLKDEV_CON_ID!("rcan0", &raw mut mstp_clks[MSTP316]),
    CLKDEV_CON_ID!("rcan1", &raw mut mstp_clks[MSTP315]),
    CLKDEV_CON_ID!("fltcl0", &raw mut mstp_clks[MSTP314]),
    CLKDEV_CON_ID!("adc0", &raw mut mstp_clks[MSTP313]),
    CLKDEV_CON_ID!("mtu0", &raw mut mstp_clks[MSTP312]),
    CLKDEV_CON_ID!("iebus0", &raw mut mstp_clks[MSTP304]),
    CLKDEV_DEV_ID!("sh7734-gether.0", &raw mut mstp_clks[MSTP114]),
    CLKDEV_CON_ID!("rtc0", &raw mut mstp_clks[MSTP303]),
    CLKDEV_CON_ID!("hif0", &raw mut mstp_clks[MSTP302]),
    CLKDEV_CON_ID!("stif0", &raw mut mstp_clks[MSTP301]),
    CLKDEV_CON_ID!("stif1", &raw mut mstp_clks[MSTP300]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut ret: c_int = 0;

    for i in 0..ARRAY_SIZE(main_clks) {
        ret |= clk_register(main_clks[i]);
    }

    clkdev_add_table(&raw mut lookups, ARRAY_SIZE(lookups));

    if ret == 0 {
        ret = sh_clk_div4_register(&raw mut div4_clks, ARRAY_SIZE(div4_clks), &raw mut div4_table);
    }

    if ret == 0 {
        ret = sh_clk_mstp_register(&raw mut mstp_clks, MSTP_NR);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
