// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/clock-sh7201.c
 *
 * SH7201 support for the clock framework
 *
 *  Copyright (C) 2008 Peter Griffin  <pgriffin@mpc-data.co.uk>
 *
 * Based on clock-sh4.c
 *  Copyright (C) 2005  Paul Mundt
 */

use core::ffi::{c_int, c_ulong};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static FREQCR: usize;
    fn __raw_readw(addr: usize) -> u16;
    fn test_mode_pin(pin: c_int) -> c_int;
}

#[repr(C)]
pub struct clk {
    pub rate: c_ulong,
    pub parent: *mut clk,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk)>,
    pub recalc: Option<unsafe extern "C" fn(*mut clk) -> c_ulong>,
}

static PLL1RATE: [c_int; 6] = [1, 2, 3, 4, 6, 8];
static PFC_DIVISORS: [c_int; 7] = [1, 2, 3, 4, 6, 8, 12];
// #define ifc_divisors pfc_divisors

static mut pll2_mult: c_int = 0;

unsafe extern "C" fn master_clk_init(clk: *mut clk) {
    (*clk).rate = (10000000 * pll2_mult
        * PLL1RATE[((__raw_readw(FREQCR) >> 8) & 0x0007) as usize]) as c_ulong;
}

static mut sh7201_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe extern "C" fn module_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = (__raw_readw(FREQCR) & 0x0007) as usize;
    (*clk).parent.as_ref().unwrap().rate / PFC_DIVISORS[idx] as c_ulong
}

static mut sh7201_module_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe extern "C" fn bus_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = (__raw_readw(FREQCR) & 0x0007) as usize;
    (*clk).parent.as_ref().unwrap().rate / PFC_DIVISORS[idx] as c_ulong
}

static mut sh7201_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

unsafe extern "C" fn cpu_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readw(FREQCR) >> 4) & 0x0007) as usize;
    (*clk).parent.as_ref().unwrap().rate / PFC_DIVISORS[idx] as c_ulong
}

static mut sh7201_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(cpu_clk_recalc),
};

static mut sh7201_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7201_master_clk_ops,
    &raw mut sh7201_module_clk_ops,
    &raw mut sh7201_bus_clk_ops,
    &raw mut sh7201_cpu_clk_ops,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: c_int) {
    if test_mode_pin(0x0002 | 0x0001) != 0 {
        pll2_mult = 1;
    } else if test_mode_pin(0x0002) != 0 {
        pll2_mult = 2;
    } else {
        pll2_mult = 4;
    }

    if idx >= 0 && (idx as usize) < sh7201_clk_ops.len() {
        *ops = sh7201_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
