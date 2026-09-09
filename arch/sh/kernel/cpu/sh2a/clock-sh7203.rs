// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/clock-sh7203.c
 *
 * SH7203 support for the clock framework
 *
 *  Copyright (C) 2007 Kieran Bingham (MPC-Data Ltd)
 *
 * Based on clock-sh7263.c
 *  Copyright (C) 2006  Yoshinori Sato
 *
 * Based on clock-sh4.c
 *  Copyright (C) 2005  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static FREQCR: usize;
    fn __raw_readw(addr: usize) -> u16;
    fn test_mode_pin(pin: i32) -> bool;
    fn followparent_recalc(clk: *mut clk) -> u64;
}

// MODE_PIN0 and MODE_PIN1 are supplied by the platform headers.
const MODE_PIN0: i32 = 0;
const MODE_PIN1: i32 = 1;

static PLL1RATE: [i32; 4] = [8, 12, 16, 0];
static PFC_DIVISORS: [i32; 7] = [1, 2, 3, 4, 6, 8, 12];
// #define ifc_divisors pfc_divisors
static IFC_DIVISORS: &[i32; 7] = &PFC_DIVISORS;

static mut pll2_mult: u32 = 0;

unsafe fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= (PLL1RATE[((__raw_readw(FREQCR) >> 8) & 0x0003) as usize]
        as u64)
        * (pll2_mult as u64);
}

static mut sh7203_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> u64 {
    let idx = (__raw_readw(FREQCR) & 0x0007) as usize;
    (*(*clk).parent).rate / (PFC_DIVISORS[idx] as u64)
}

static mut sh7203_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> u64 {
    let idx = (__raw_readw(FREQCR) & 0x0007) as usize;
    (*(*clk).parent).rate / (PFC_DIVISORS[idx - 2] as u64)
}

static mut sh7203_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

static mut sh7203_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(followparent_recalc),
};

static mut sh7203_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7203_master_clk_ops,
    &raw mut sh7203_module_clk_ops,
    &raw mut sh7203_bus_clk_ops,
    &raw mut sh7203_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if test_mode_pin(MODE_PIN1) {
        pll2_mult = 4;
    } else if test_mode_pin(MODE_PIN0) {
        pll2_mult = 2;
    } else {
        pll2_mult = 1;
    }

    if idx < sh7203_clk_ops.len() as i32 {
        *ops = sh7203_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
