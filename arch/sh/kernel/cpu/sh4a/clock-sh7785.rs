// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7785.c
 *
 * SH7785 support for the clock framework
 *
 * Copyright (C) 2007 - 2010 Paul Mundt
 */

// Linux and architecture dependencies supplied by the surrounding tree.

static mut extal_clk: struct_clk = struct_clk {
    rate: 33333333,
};

unsafe fn pll_recalc(clk: *mut struct_clk) -> c_ulong {
    let multiplier: c_int = if test_mode_pin(MODE_PIN4) != 0 { 36 } else { 72 };
    (*(*clk).parent).rate.wrapping_mul(multiplier as c_ulong)
}

static mut pll_clk_ops: struct_sh_clk_ops = struct_sh_clk_ops {
    recalc: Some(pll_recalc),
};

static mut pll_clk: struct_clk = struct_clk {
    ops: &mut pll_clk_ops,
    parent: &mut extal_clk,
    flags: CLK_ENABLE_ON_INIT,
};

static mut clks: [*mut struct_clk; 2] = [
    &mut extal_clk,
    &mut pll_clk,
];

static mut div2: [c_uint; 12] = [1, 2, 4, 6, 8, 12, 16, 18, 24, 32, 36, 48];

static mut div4_div_mult_table: struct_clk_div_mult_table = struct_clk_div_mult_table {
    divisors: div2.as_ptr(),
    nr_divisors: div2.len(),
};

static mut div4_table: struct_clk_div4_table = struct_clk_div4_table {
    div_mult_table: &mut div4_div_mult_table,
};

enum Div4 {
    DIV4_I,
    DIV4_U,
    DIV4_SH,
    DIV4_B,
    DIV4_DDR,
    DIV4_GA,
    DIV4_DU,
    DIV4_P,
    DIV4_NR,
}

static mut div4_clks: [struct_clk; Div4::DIV4_NR as usize] = [
    /* DIV4_P */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 0, 0x0f80, 0),
    /* DIV4_DU */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 4, 0x0ff0, 0),
    /* DIV4_GA */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 8, 0x0030, 0),
    /* DIV4_DDR */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 12, 0x000c, CLK_ENABLE_ON_INIT),
    /* DIV4_B */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 16, 0x0fe0, CLK_ENABLE_ON_INIT),
    /* DIV4_SH */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 20, 0x000c, CLK_ENABLE_ON_INIT),
    /* DIV4_U */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 24, 0x000c, CLK_ENABLE_ON_INIT),
    /* DIV4_I */ SH_CLK_DIV4(&mut pll_clk, FRQMR1, 28, 0x000e, CLK_ENABLE_ON_INIT),
];

const MSTPCR0: usize = 0xffc80030;
const MSTPCR1: usize = 0xffc80034;

enum Mstp {
    MSTP029, MSTP028, MSTP027, MSTP026, MSTP025, MSTP024,
    MSTP021, MSTP020, MSTP017, MSTP016,
    MSTP013, MSTP012, MSTP009, MSTP008, MSTP003, MSTP002,
    MSTP119, MSTP117, MSTP105, MSTP104, MSTP100,
    MSTP_NR,
}

static mut mstp_clks: [struct_clk; Mstp::MSTP_NR as usize] = [
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 29, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 28, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 27, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 26, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 25, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 24, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 21, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 20, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 17, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 16, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 13, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 12, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 9, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 8, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 3, 0),
    SH_CLK_MSTP32(&mut div4_clks[Div4::DIV4_P as usize], MSTPCR0, 2, 0),
    SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 19, 0),
    SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 17, 0),
    SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 5, 0),
    SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 4, 0),
    SH_CLK_MSTP32(core::ptr::null_mut(), MSTPCR1, 0, 0),
];

// Clock lookup table entries retain the source's registration order.
static mut lookups: [struct_clk_lookup; 31] = [
    CLKDEV_CON_ID("extal", &mut extal_clk),
    CLKDEV_CON_ID("pll_clk", &mut pll_clk),
    CLKDEV_CON_ID("peripheral_clk", &mut div4_clks[Div4::DIV4_P as usize]),
    CLKDEV_CON_ID("du_clk", &mut div4_clks[Div4::DIV4_DU as usize]),
    CLKDEV_CON_ID("ga_clk", &mut div4_clks[Div4::DIV4_GA as usize]),
    CLKDEV_CON_ID("ddr_clk", &mut div4_clks[Div4::DIV4_DDR as usize]),
    CLKDEV_CON_ID("bus_clk", &mut div4_clks[Div4::DIV4_B as usize]),
    CLKDEV_CON_ID("shyway_clk", &mut div4_clks[Div4::DIV4_SH as usize]),
    CLKDEV_CON_ID("umem_clk", &mut div4_clks[Div4::DIV4_U as usize]),
    CLKDEV_CON_ID("cpu_clk", &mut div4_clks[Div4::DIV4_I as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.5", &mut mstp_clks[Mstp::MSTP029 as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.4", &mut mstp_clks[Mstp::MSTP028 as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.3", &mut mstp_clks[Mstp::MSTP027 as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.2", &mut mstp_clks[Mstp::MSTP026 as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.1", &mut mstp_clks[Mstp::MSTP025 as usize]),
    CLKDEV_ICK_ID("fck", "sh-sci.0", &mut mstp_clks[Mstp::MSTP024 as usize]),
    CLKDEV_CON_ID("ssi1_fck", &mut mstp_clks[Mstp::MSTP021 as usize]),
    CLKDEV_CON_ID("ssi0_fck", &mut mstp_clks[Mstp::MSTP020 as usize]),
    CLKDEV_CON_ID("hac1_fck", &mut mstp_clks[Mstp::MSTP017 as usize]),
    CLKDEV_CON_ID("hac0_fck", &mut mstp_clks[Mstp::MSTP016 as usize]),
    CLKDEV_CON_ID("mmcif_fck", &mut mstp_clks[Mstp::MSTP013 as usize]),
    CLKDEV_CON_ID("flctl_fck", &mut mstp_clks[Mstp::MSTP012 as usize]),
    CLKDEV_ICK_ID("fck", "sh-tmu.0", &mut mstp_clks[Mstp::MSTP008 as usize]),
    CLKDEV_ICK_ID("fck", "sh-tmu.1", &mut mstp_clks[Mstp::MSTP009 as usize]),
    CLKDEV_CON_ID("siof_fck", &mut mstp_clks[Mstp::MSTP003 as usize]),
    CLKDEV_CON_ID("hspi_fck", &mut mstp_clks[Mstp::MSTP002 as usize]),
    CLKDEV_CON_ID("hudi_fck", &mut mstp_clks[Mstp::MSTP119 as usize]),
    CLKDEV_CON_ID("ubc0", &mut mstp_clks[Mstp::MSTP117 as usize]),
    CLKDEV_CON_ID("dmac_11_6_fck", &mut mstp_clks[Mstp::MSTP105 as usize]),
    CLKDEV_CON_ID("dmac_5_0_fck", &mut mstp_clks[Mstp::MSTP104 as usize]),
    CLKDEV_CON_ID("gdta_fck", &mut mstp_clks[Mstp::MSTP100 as usize]),
];

unsafe fn arch_clk_init() -> c_int {
    let mut ret: c_int = 0;
    for i in 0..clks.len() {
        ret |= clk_register(clks[i]);
    }
    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());
    if ret == 0 {
        ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), div4_clks.len(), &mut div4_table);
    }
    if ret == 0 {
        ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), Mstp::MSTP_NR as usize);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
