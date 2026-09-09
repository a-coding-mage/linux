// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh7705.c
 *
 * SH7705 support for the clock framework
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

// C dependencies: linux/init.h, linux/kernel.h, asm/clock.h, asm/freq.h,
// and asm/io.h provide the declarations used below.

/*
 * SH7705 uses the same divisors as the generic SH-3 case, it's just the
 * FRQCR layout that is a bit different..
 */
static mut stc_multipliers: [i32; 8] = [1, 2, 3, 4, 6, 1, 1, 1];
static mut ifc_divisors: [i32; 8] = [1, 2, 3, 4, 1, 1, 1, 1];
static mut pfc_divisors: [i32; 8] = [1, 2, 3, 4, 6, 1, 1, 1];

unsafe fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= pfc_divisors[(__raw_readw(FRQCR) & 0x0003) as usize] as _;
}

static mut sh7705_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
};

unsafe fn module_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let idx = (__raw_readw(FRQCR) & 0x0003) as usize;
    (*clk).parent.as_ref().unwrap().rate / pfc_divisors[idx] as _
}

static mut sh7705_module_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(module_clk_recalc),
};

unsafe fn bus_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let idx = ((__raw_readw(FRQCR) & 0x0300) >> 8) as usize;
    (*clk).parent.as_ref().unwrap().rate / stc_multipliers[idx] as _
}

static mut sh7705_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(bus_clk_recalc),
};

unsafe fn cpu_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let idx = ((__raw_readw(FRQCR) & 0x0030) >> 4) as usize;
    (*clk).parent.as_ref().unwrap().rate / ifc_divisors[idx] as _
}

static mut sh7705_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(cpu_clk_recalc),
};

static mut sh7705_clk_ops: [*mut sh_clk_ops; 4] = [
    &mut sh7705_master_clk_ops,
    &mut sh7705_module_clk_ops,
    &mut sh7705_bus_clk_ops,
    &mut sh7705_cpu_clk_ops,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < sh7705_clk_ops.len() as i32 {
        *ops = sh7705_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
