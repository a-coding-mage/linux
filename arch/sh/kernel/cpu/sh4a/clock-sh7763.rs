// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/clock-sh7763.c
 *
 * SH7763 support for the clock framework
 *
 *  Copyright (C) 2005  Paul Mundt
 *  Copyright (C) 2007  Yoshihiro Shimoda
 */

// Linux and SH clock-framework declarations are supplied by other translation units.

#[repr(C)]
pub struct clk {
    pub flags: u32,
    pub ops: *mut sh_clk_ops,
    pub parent: *mut clk,
    pub rate: c_ulong,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk)>,
    pub recalc: Option<unsafe extern "C" fn(*mut clk) -> c_ulong>,
}

type c_ulong = usize;

extern "C" {
    static mut FRQCR: usize;
    static CLK_ENABLE_ON_INIT: u32;

    fn __raw_readl(addr: usize) -> u32;
    fn followparent_recalc(clk: *mut clk) -> c_ulong;
    fn cpg_clk_init();
    fn clk_get(dev: *mut core::ffi::c_void, id: *const u8) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_register(clk: *mut clk) -> i32;
    fn clkdev_add_table(lookups: *mut clk_lookup, num: usize);
}

#[repr(C)]
pub struct clk_lookup {
    pub con_id: *const u8,
    pub clk: *mut clk,
}

static mut BFC_DIVISORS: [i32; 8] = [1, 1, 1, 8, 1, 1, 1, 1];
static mut P0FC_DIVISORS: [i32; 8] = [1, 1, 1, 8, 1, 1, 1, 1];
static mut CFC_DIVISORS: [i32; 8] = [1, 1, 4, 1, 1, 1, 1, 1];

unsafe extern "C" fn master_clk_init(clk: *mut clk) {
    (*clk).rate = (*clk).rate.wrapping_mul(
        P0FC_DIVISORS[((__raw_readl(FRQCR) >> 4) & 0x07) as usize] as usize,
    );
}

static mut SH7763_MASTER_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: Some(master_clk_init),
    recalc: None,
};

unsafe extern "C" fn module_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readl(FRQCR) >> 4) & 0x07) as usize;
    (*(*clk).parent).rate / P0FC_DIVISORS[idx] as usize
}

static mut SH7763_MODULE_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(module_clk_recalc),
};

unsafe extern "C" fn bus_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readl(FRQCR) >> 16) & 0x07) as usize;
    (*(*clk).parent).rate / BFC_DIVISORS[idx] as usize
}

static mut SH7763_BUS_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(bus_clk_recalc),
};

static mut SH7763_CPU_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(followparent_recalc),
};

static mut SH7763_CLK_OPS: [*mut sh_clk_ops; 4] = [
    &raw mut SH7763_MASTER_CLK_OPS,
    &raw mut SH7763_MODULE_CLK_OPS,
    &raw mut SH7763_BUS_CLK_OPS,
    &raw mut SH7763_CPU_CLK_OPS,
];

pub unsafe extern "C" fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, idx: i32) {
    if idx < SH7763_CLK_OPS.len() as i32 {
        *ops = SH7763_CLK_OPS[idx as usize];
    }
}

unsafe extern "C" fn shyway_clk_recalc(clk: *mut clk) -> c_ulong {
    let idx = ((__raw_readl(FRQCR) >> 20) & 0x07) as usize;
    (*(*clk).parent).rate / CFC_DIVISORS[idx] as usize
}

static mut SH7763_SHYWAY_CLK_OPS: sh_clk_ops = sh_clk_ops {
    init: None,
    recalc: Some(shyway_clk_recalc),
};

static mut SH7763_SHYWAY_CLK: clk = clk {
    flags: 0,
    ops: &raw mut SH7763_SHYWAY_CLK_OPS,
    parent: core::ptr::null_mut(),
    rate: 0,
};

/*
 * Additional SH7763-specific on-chip clocks that aren't already part of the
 * clock framework
 */
static mut SH7763_ONCHIP_CLOCKS: [*mut clk; 1] = [&raw mut SH7763_SHYWAY_CLK];

static mut LOOKUPS: [clk_lookup; 1] = [clk_lookup {
    con_id: b"shyway_clk\0".as_ptr(),
    clk: &raw mut SH7763_SHYWAY_CLK,
}];

pub unsafe extern "C" fn arch_clk_init() -> i32 {
    let mut clk: *mut clk;
    let mut ret: i32 = 0;

    cpg_clk_init();

    clk = clk_get(core::ptr::null_mut(), b"master_clk\0".as_ptr());
    for i in 0..SH7763_ONCHIP_CLOCKS.len() {
        let clkp = SH7763_ONCHIP_CLOCKS[i];

        (*clkp).parent = clk;
        ret |= clk_register(clkp);
    }

    clk_put(clk);

    clkdev_add_table(LOOKUPS.as_mut_ptr(), LOOKUPS.len());

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
