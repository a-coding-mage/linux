// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7780.c
 *
 * SH7780 support for the clock framework
 *
 *  Copyright (C) 2005  Paul Mundt
 */

// Linux/architecture headers provide these types, constants, and functions.
#[repr(C)]
pub struct clk {
    pub flags: u32,
    pub ops: *mut sh_clk_ops,
    pub parent: *mut clk,
    pub rate: usize,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk)>,
    pub recalc: Option<unsafe extern "C" fn(*mut clk) -> usize>,
}

#[repr(C)]
pub struct clk_lookup {
    pub con_id: *const u8,
    pub clk: *mut clk,
}

const CLK_ENABLE_ON_INIT: u32 = 1 << 0;
const FRQCR: usize = 0;

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn cpg_clk_init();
    fn clk_get(dev: *const core::ffi::c_void, id: *const u8) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_register(clk: *mut clk) -> i32;
    fn clkdev_add_table(table: *mut clk_lookup, num: usize);
}

static mut ifc_divisors: [i32; 2] = [2, 4];
static mut bfc_divisors: [i32; 8] = [1, 1, 1, 8, 12, 16, 24, 1];
static mut pfc_divisors: [i32; 4] = [1, 24, 24, 1];
static mut cfc_divisors: [i32; 8] = [1, 1, 4, 1, 6, 1, 1, 1];

unsafe extern "C" fn master_clk_init(clk: *mut clk) {
    (*clk).rate *= pfc_divisors[(__raw_readl(FRQCR) & 0x0003) as usize] as usize;
}

static mut sh7780_master_clk_ops: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe extern "C" fn module_clk_recalc(clk: *mut clk) -> usize {
    let idx = (__raw_readl(FRQCR) & 0x0003) as usize;
    (*(*clk).parent).rate / pfc_divisors[idx] as usize
}

static mut sh7780_module_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe extern "C" fn bus_clk_recalc(clk: *mut clk) -> usize {
    let idx = ((__raw_readl(FRQCR) >> 16) & 0x0007) as usize;
    (*(*clk).parent).rate / bfc_divisors[idx] as usize
}

static mut sh7780_bus_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

unsafe extern "C" fn cpu_clk_recalc(clk: *mut clk) -> usize {
    let idx = ((__raw_readl(FRQCR) >> 24) & 0x0001) as usize;
    (*(*clk).parent).rate / ifc_divisors[idx] as usize
}

static mut sh7780_cpu_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(cpu_clk_recalc),
};

static mut sh7780_clk_ops: [*mut sh_clk_ops; 4] = [
    &raw mut sh7780_master_clk_ops,
    &raw mut sh7780_module_clk_ops,
    &raw mut sh7780_bus_clk_ops,
    &raw mut sh7780_cpu_clk_ops,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < sh7780_clk_ops.len() as i32 {
        *ops = sh7780_clk_ops[idx as usize];
    }
}

unsafe extern "C" fn shyway_clk_recalc(clk: *mut clk) -> usize {
    let idx = ((__raw_readl(FRQCR) >> 20) & 0x0007) as usize;
    (*(*clk).parent).rate / cfc_divisors[idx] as usize
}

static mut sh7780_shyway_clk_ops: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(shyway_clk_recalc),
};

static mut sh7780_shyway_clk: clk = clk {
    flags: CLK_ENABLE_ON_INIT,
    ops: &raw mut sh7780_shyway_clk_ops,
    parent: core::ptr::null_mut(),
    rate: 0,
};

/* Additional SH7780-specific on-chip clocks that aren't already part of the
 * clock framework */
static mut sh7780_onchip_clocks: [*mut clk; 1] = [&raw mut sh7780_shyway_clk];

static mut lookups: [clk_lookup; 1] = [clk_lookup {
    con_id: b"shyway_clk\0".as_ptr(),
    clk: &raw mut sh7780_shyway_clk,
}];

pub unsafe extern "C" fn arch_clk_init() -> i32 {
    let mut clk: *mut clk;
    let mut i: usize;
    let mut ret: i32 = 0;

    cpg_clk_init();

    clk = clk_get(core::ptr::null(), b"master_clk\0".as_ptr());
    i = 0;
    while i < sh7780_onchip_clocks.len() {
        let clkp = sh7780_onchip_clocks[i];

        (*clkp).parent = clk;
        ret |= clk_register(clkp);
        i += 1;
    }

    clk_put(clk);

    clkdev_add_table(&raw mut lookups, lookups.len());

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
