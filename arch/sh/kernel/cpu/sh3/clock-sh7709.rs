// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh7709.c
 *
 * SH7709 support for the clock framework
 *
 *  Copyright (C) 2005  Andriy Skulysh
 *
 * Based on arch/sh/kernel/cpu/sh3/clock-sh7705.c
 *  Copyright (C) 2005  Paul Mundt
 */
// Dependencies supplied by the surrounding kernel translation:
// linux/init.h, linux/kernel.h, asm/clock.h, asm/freq.h, asm/io.h

static mut stc_multipliers: [i32; 8] = [1, 2, 4, 8, 3, 6, 1, 1];
static mut ifc_divisors: [i32; 8] = [1, 2, 4, 1, 3, 1, 1, 1];
static mut pfc_divisors: [i32; 8] = [1, 2, 4, 1, 3, 6, 1, 1];

unsafe fn master_clk_init(clk: *mut sh_clk) {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*clk).rate *= stc_multipliers[idx] as _;
}

static mut sh7709_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut sh_clk) -> u64 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*(*clk).parent).rate / pfc_divisors[idx] as u64
}

static mut sh7709_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut sh_clk) -> u64 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = if (frqcr & 0x0080) != 0 {
        (((frqcr & 0x8000) >> 13) | ((frqcr & 0x0030) >> 4)) as usize
    } else {
        1
    };

    (*(*clk).parent).rate * stc_multipliers[idx] as u64
}

static mut sh7709_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut sh_clk) -> u64 {
    let frqcr: i32 = __raw_readw(FRQCR) as i32;
    let idx: usize = (((frqcr & 0x4000) >> 12) | ((frqcr & 0x000c) >> 2)) as usize;

    (*(*clk).parent).rate / ifc_divisors[idx] as u64
}

static mut sh7709_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut sh7709_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7709_master_clk_ops,
    &raw mut sh7709_module_clk_ops,
    &raw mut sh7709_bus_clk_ops,
    &raw mut sh7709_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < sh7709_clk_ops.len() as i32 {
        *ops = sh7709_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
