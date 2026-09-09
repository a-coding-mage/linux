// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7770.c
 *
 * SH7770 support for the clock framework
 *
 *  Copyright (C) 2005  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::{__raw_readl, clk, sh_clk_ops, ARRAY_SIZE, FRQCR};

static mut ifc_divisors: [i32; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
static mut bfc_divisors: [i32; 8] = [1, 1, 1, 1, 1, 8, 12, 1];
static mut pfc_divisors: [i32; 8] = [1, 8, 1, 10, 12, 16, 1, 1];

unsafe fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= pfc_divisors[((__raw_readl(FRQCR) >> 28) & 0x000f) as usize] as _;
}

static mut sh7770_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> u64 {
    let idx = ((__raw_readl(FRQCR) >> 28) & 0x000f) as usize;
    (*clk).parent.as_ref().unwrap().rate / pfc_divisors[idx] as _
}

static mut sh7770_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> u64 {
    let idx = (__raw_readl(FRQCR) & 0x000f) as usize;
    (*clk).parent.as_ref().unwrap().rate / bfc_divisors[idx] as _
}

static mut sh7770_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> u64 {
    let idx = ((__raw_readl(FRQCR) >> 24) & 0x000f) as usize;
    (*clk).parent.as_ref().unwrap().rate / ifc_divisors[idx] as _
}

static mut sh7770_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut sh7770_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7770_master_clk_ops,
    &raw mut sh7770_module_clk_ops,
    &raw mut sh7770_bus_clk_ops,
    &raw mut sh7770_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < ARRAY_SIZE(sh7770_clk_ops) as i32 {
        *ops = sh7770_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
