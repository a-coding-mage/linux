// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2/clock-sh7619.c
 *
 * SH7619 support for the clock framework
 *
 *  Copyright (C) 2006  Yoshinori Sato
 *
 * Based on clock-sh4.c
 *  Copyright (C) 2005  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::{
    array_size, followparent_recalc, test_mode_pin, Clk, ShClkOps, FREQCR,
    MODE_PIN0, MODE_PIN1, MODE_PIN2,
};

extern "C" {
    fn __raw_readw(addr: usize) -> u16;
    fn bug_on(condition: bool);
}

static PLL1RATE: [i32; 2] = [1, 2];
static PFC_DIVISORS: [i32; 4] = [1, 2, 0, 4];
static mut PLL2_MULT: u32 = 0;

unsafe fn master_clk_init(clk: *mut Clk) {
    (*clk).rate *= PLL2_MULT * PLL1RATE[((__raw_readw(FREQCR) >> 8) & 7) as usize] as u32;
}

static mut SH7619_MASTER_CLK_OPS: ShClkOps = ShClkOps {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut Clk) -> u32 {
    let idx = (__raw_readw(FREQCR) & 0x0007) as usize;
    (*clk).parent.rate / PFC_DIVISORS[idx] as u32
}

static mut SH7619_MODULE_CLK_OPS: ShClkOps = ShClkOps {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut Clk) -> u32 {
    (*clk).parent.rate / PLL1RATE[((__raw_readw(FREQCR) >> 8) & 7) as usize] as u32
}

static mut SH7619_BUS_CLK_OPS: ShClkOps = ShClkOps {
    recalc: Some(bus_clk_recalc),
};

static mut SH7619_CPU_CLK_OPS: ShClkOps = ShClkOps {
    recalc: Some(followparent_recalc),
};

static mut SH7619_CLK_OPS: [*mut ShClkOps; 4] = [
    &raw mut SH7619_MASTER_CLK_OPS,
    &raw mut SH7619_MODULE_CLK_OPS,
    &raw mut SH7619_BUS_CLK_OPS,
    &raw mut SH7619_CPU_CLK_OPS,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut ShClkOps, idx: i32) {
    if test_mode_pin(MODE_PIN2 | MODE_PIN0) || test_mode_pin(MODE_PIN2 | MODE_PIN1) {
        PLL2_MULT = 2;
    } else if test_mode_pin(MODE_PIN0) || test_mode_pin(MODE_PIN1) {
        PLL2_MULT = 4;
    }

    bug_on(PLL2_MULT == 0);

    if idx < array_size(&SH7619_CLK_OPS) as i32 {
        *ops = SH7619_CLK_OPS[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
