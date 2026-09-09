// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/clock-sh7206.c
 *
 * SH7206 support for the clock framework
 *
 *  Copyright (C) 2006  Yoshinori Sato
 *
 * Based on clock-sh4.c
 *  Copyright (C) 2005  Paul Mundt
 */
// Dependencies supplied by the surrounding kernel environment:
// linux/init.h, linux/kernel.h, asm/clock.h, asm/freq.h, asm/io.h

static const pll1rate: [i32; 6] = [1, 2, 3, 4, 6, 8];
static const pfc_divisors: [i32; 7] = [1, 2, 3, 4, 6, 8, 12];
// #define ifc_divisors pfc_divisors
static const ifc_divisors: [i32; 7] = pfc_divisors;

static mut pll2_mult: u32 = 0;

unsafe fn master_clk_init(clk: *mut clk) {
    (*clk).rate = (*clk).rate.wrapping_mul(
        pll2_mult.wrapping_mul(
            pll1rate[((__raw_readw(FREQCR) >> 8) & 0x0007) as usize] as u32,
        ),
    );
}

static mut sh7206_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> unsigned_long {
    let idx: i32 = (__raw_readw(FREQCR) & 0x0007) as i32;
    (*(*clk).parent).rate / pfc_divisors[idx as usize] as unsigned_long
}

static mut sh7206_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> unsigned_long {
    (*(*clk).parent).rate
        / pll1rate[((__raw_readw(FREQCR) >> 8) & 0x0007) as usize] as unsigned_long
}

static mut sh7206_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> unsigned_long {
    let idx: i32 = (__raw_readw(FREQCR) & 0x0007) as i32;
    (*(*clk).parent).rate / ifc_divisors[idx as usize] as unsigned_long
}

static mut sh7206_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut sh7206_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7206_master_clk_ops,
    &raw mut sh7206_module_clk_ops,
    &raw mut sh7206_bus_clk_ops,
    &raw mut sh7206_cpu_clk_ops,
];

pub unsafe fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if test_mode_pin(MODE_PIN2 | MODE_PIN1 | MODE_PIN0) != 0 {
        pll2_mult = 1;
    } else if test_mode_pin(MODE_PIN2 | MODE_PIN1) != 0 {
        pll2_mult = 2;
    } else if test_mode_pin(MODE_PIN1) != 0 {
        pll2_mult = 4;
    }

    if idx < sh7206_clk_ops.len() as i32 {
        *ops = sh7206_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
