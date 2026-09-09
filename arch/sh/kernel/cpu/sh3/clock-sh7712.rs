// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh7712.c
 *
 * SH7712 support for the clock framework
 *
 *  Copyright (C) 2007  Andrew Murray <amurray@mpc-data.co.uk>
 *
 * Based on arch/sh/kernel/cpu/sh3/clock-sh3.c
 *  Copyright (C) 2005  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/kernel.h, asm/clock.h, asm/freq.h, asm/io.h

static mut multipliers: [i32; 3] = [1, 2, 3];
static mut divisors: [i32; 5] = [1, 2, 3, 4, 6];

unsafe fn master_clk_init(clk: *mut clk) {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = ((frqcr & 0x0300) >> 8) as usize;

    (*clk).rate *= multipliers[idx] as _;
}

static mut sh7712_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> c_ulong {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = (frqcr & 0x0007) as usize;

    (*clk).parent.as_ref().unwrap().rate / divisors[idx] as _
}

static mut sh7712_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> c_ulong {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = ((frqcr & 0x0030) >> 4) as usize;

    (*clk).parent.as_ref().unwrap().rate / divisors[idx] as _
}

static mut sh7712_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut sh7712_clk_ops: [*mut sh_clk_ops; 3] = [
    &raw mut sh7712_master_clk_ops,
    &raw mut sh7712_module_clk_ops,
    &raw mut sh7712_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < sh7712_clk_ops.len() as i32 {
        *ops = sh7712_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
