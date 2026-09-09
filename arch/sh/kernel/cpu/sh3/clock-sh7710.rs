// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh7710.c
 *
 * SH7710 support for the clock framework
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

// C dependencies supplied by the surrounding kernel translation.

static mut MD_TABLE: [i32; 7] = [1, 2, 3, 4, 6, 8, 12];

unsafe fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= MD_TABLE[__raw_readw(FRQCR) as usize & 0x0007] as _;
}

static mut SH7710_MASTER_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> ::core::ffi::c_ulong {
    let idx: i32 = (__raw_readw(FRQCR) & 0x0007) as i32;
    (*clk).parent.as_ref().unwrap().rate / MD_TABLE[idx as usize] as _
}

static mut SH7710_MODULE_CLK_OPS: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> ::core::ffi::c_ulong {
    let idx: i32 = ((__raw_readw(FRQCR) & 0x0700) >> 8) as i32;
    (*clk).parent.as_ref().unwrap().rate / MD_TABLE[idx as usize] as _
}

static mut SH7710_BUS_CLK_OPS: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> ::core::ffi::c_ulong {
    let idx: i32 = ((__raw_readw(FRQCR) & 0x0070) >> 4) as i32;
    (*clk).parent.as_ref().unwrap().rate / MD_TABLE[idx as usize] as _
}

static mut SH7710_CPU_CLK_OPS: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut SH7710_CLK_OPS: [*mut sh_clk_ops; 4] = [
    &raw mut SH7710_MASTER_CLK_OPS,
    &raw mut SH7710_MODULE_CLK_OPS,
    &raw mut SH7710_BUS_CLK_OPS,
    &raw mut SH7710_CPU_CLK_OPS,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < SH7710_CLK_OPS.len() as i32 {
        *ops = SH7710_CLK_OPS[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
