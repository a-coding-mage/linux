// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4/clock-sh4.c
 *
 * Generic SH-4 support for the clock framework
 *
 *  Copyright (C) 2005  Paul Mundt
 *
 * FRQCR parsing hacked out of arch/sh/kernel/time.c
 *
 *  Copyright (C) 1999  Tetsuya Okada & Niibe Yutaka
 *  Copyright (C) 2000  Philipp Rumpf <prumpf@tux.org>
 *  Copyright (C) 2002, 2003, 2004  Paul Mundt
 *  Copyright (C) 2002  M. R. Brown  <mrbrown@linux-sh.org>
 */

use core::ffi::{c_int, c_ulong};

// Supplied by the surrounding clock and I/O implementation.
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

unsafe extern "C" {
    static FRQCR: u16;
    fn __raw_readw(addr: *const u16) -> u16;
}

static mut ifc_divisors: [c_int; 8] = [1, 2, 3, 4, 6, 8, 1, 1];
// Same as ifc_divisors.
// #define bfc_divisors ifc_divisors
static mut pfc_divisors: [c_int; 8] = [2, 3, 4, 6, 8, 2, 2, 2];

unsafe extern "C" fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= pfc_divisors[(__raw_readw(&raw const FRQCR) & 0x0007) as usize] as c_ulong;
}

static mut sh4_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe extern "C" fn module_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = (__raw_readw(&raw const FRQCR) & 0x0007) as usize;
    (*(*clk).parent).rate / pfc_divisors[idx] as c_ulong
}

static mut sh4_module_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe extern "C" fn bus_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readw(&raw const FRQCR) >> 3) & 0x0007) as usize;
    (*(*clk).parent).rate / ifc_divisors[idx] as c_ulong
}

static mut sh4_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

unsafe extern "C" fn cpu_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readw(&raw const FRQCR) >> 6) & 0x0007) as usize;
    (*(*clk).parent).rate / ifc_divisors[idx] as c_ulong
}

static mut sh4_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(cpu_clk_recalc),
};

static mut sh4_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh4_master_clk_ops,
    &raw mut sh4_module_clk_ops,
    &raw mut sh4_bus_clk_ops,
    &raw mut sh4_cpu_clk_ops,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: c_int) {
    if idx < sh4_clk_ops.len() as c_int {
        *ops = sh4_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
