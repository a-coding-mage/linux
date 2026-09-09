// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh3/clock-sh7706.c
 *
 * SH7706 support for the clock framework
 *
 * Copyright (C) 2006 Takashi YOSHII
 *
 * Based on arch/sh/kernel/cpu/sh3/clock-sh7709.c
 * Copyright (C) 2005 Andriy Skulysh
 */

// External declarations supplied by the kernel clock and I/O interfaces.
#[repr(C)]
pub struct clk {
    pub rate: libc::c_ulong,
    pub parent: *mut clk,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk)>,
    pub recalc: Option<unsafe extern "C" fn(*mut clk) -> libc::c_ulong>,
}

unsafe extern "C" {
    static FRQCR: usize;
    fn __raw_readw(addr: usize) -> u16;
}

static mut stc_multipliers: [libc::c_int; 8] = [1, 2, 4, 1, 3, 6, 1, 1];
static mut ifc_divisors: [libc::c_int; 8] = [1, 2, 4, 1, 3, 1, 1, 1];
static mut pfc_divisors: [libc::c_int; 8] = [1, 2, 4, 1, 3, 6, 1, 1];

unsafe extern "C" fn master_clk_init(clk: *mut clk) {
    let frqcr: libc::c_int = __raw_readw(FRQCR) as libc::c_int;
    let idx = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*clk).rate = (*clk).rate.wrapping_mul(pfc_divisors[idx] as libc::c_ulong);
}

static mut sh7706_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe extern "C" fn module_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let frqcr: libc::c_int = __raw_readw(FRQCR) as libc::c_int;
    let idx = (((frqcr & 0x2000) >> 11) | (frqcr & 0x0003)) as usize;

    (*(*clk).parent).rate / pfc_divisors[idx] as libc::c_ulong
}

static mut sh7706_module_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe extern "C" fn bus_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let frqcr: libc::c_int = __raw_readw(FRQCR) as libc::c_int;
    let idx = (((frqcr & 0x8000) >> 13) | ((frqcr & 0x0030) >> 4)) as usize;

    (*(*clk).parent).rate / stc_multipliers[idx] as libc::c_ulong
}

static mut sh7706_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

unsafe extern "C" fn cpu_clk_recalc(clk: *mut clk) -> libc::c_ulong {
    let frqcr: libc::c_int = __raw_readw(FRQCR) as libc::c_int;
    let idx = (((frqcr & 0x4000) >> 12) | ((frqcr & 0x000c) >> 2)) as usize;

    (*(*clk).parent).rate / ifc_divisors[idx] as libc::c_ulong
}

static mut sh7706_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(cpu_clk_recalc),
};

static mut sh7706_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7706_master_clk_ops,
    &raw mut sh7706_module_clk_ops,
    &raw mut sh7706_bus_clk_ops,
    &raw mut sh7706_cpu_clk_ops,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: libc::c_int) {
    if idx < sh7706_clk_ops.len() as libc::c_int {
        *ops = sh7706_clk_ops[idx as usize];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
