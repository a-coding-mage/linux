// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh3.c
 *
 * Generic SH-3 support for the clock framework
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

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn __raw_readw(addr: usize) -> u16;
    static FRQCR: usize;
}

#[repr(C)]
pub struct clk {
    pub rate: u32,
    pub parent: *mut clk,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub init: Option<unsafe fn(*mut clk)>,
    pub recalc: Option<unsafe fn(*mut clk) -> u32>,
}

static mut stc_multipliers: [i32; 8] = [1, 2, 3, 4, 6, 1, 1, 1];
static mut ifc_divisors: [i32; 8] = [1, 2, 3, 4, 1, 1, 1, 1];
static mut pfc_divisors: [i32; 8] = [1, 2, 3, 4, 6, 1, 1, 1];

unsafe fn master_clk_init(clk: *mut clk) {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*clk).rate = (*clk).rate.wrapping_mul(pfc_divisors[idx] as u32);
}

static mut sh3_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe fn module_clk_recalc(clk: *mut clk) -> u32 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*(*clk).parent).rate / pfc_divisors[idx] as u32
}

static mut sh3_module_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> u32 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx = (((frqcr & 0x8000) >> 13) | ((frqcr & 0x0030) >> 4)) as usize;

    (*(*clk).parent).rate / stc_multipliers[idx] as u32
}

static mut sh3_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> u32 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx = (((frqcr & 0x4000) >> 12) | ((frqcr & 0x000c) >> 2)) as usize;

    (*(*clk).parent).rate / ifc_divisors[idx] as u32
}

static mut sh3_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(cpu_clk_recalc),
};

static mut sh3_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh3_master_clk_ops,
    &raw mut sh3_module_clk_ops,
    &raw mut sh3_bus_clk_ops,
    &raw mut sh3_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < sh3_clk_ops.len() as i32 {
        *ops = sh3_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
