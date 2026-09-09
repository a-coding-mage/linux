// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn pr_warn(fmt: *const c_char, ...);
    fn clk_hw_get_name(hw: *const clk_hw) -> *const c_char;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> c_int;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}

#[repr(C)]
pub struct berlin2_pll_map {
    pub fbdiv_shift: u32,
    pub rfdiv_shift: u32,
    pub divsel_shift: u32,
    pub vcodiv: [u32; 16],
    pub mult: u32,
}

#[repr(C)]
pub struct berlin2_pll {
    pub hw: clk_hw,
    pub base: *mut c_void,
    pub map: berlin2_pll_map,
}

const SPLL_CTRL0: usize = 0x00;
const SPLL_CTRL1: usize = 0x04;
const SPLL_CTRL2: usize = 0x08;
const SPLL_CTRL3: usize = 0x0c;
const SPLL_CTRL4: usize = 0x10;

const FBDIV_MASK: u32 = 0x1ff;
const RFDIV_MASK: u32 = 0x1f;
const DIVSEL_MASK: u32 = 0xf;

// The output frequency formula for the pll is:
// clkout = fbdiv / refdiv * parent / vcodiv
unsafe extern "C" fn berlin2_pll_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let pll = (hw as *mut u8).sub(core::mem::offset_of!(berlin2_pll, hw)) as *mut berlin2_pll;
    let map = &(*pll).map;
    let mut val: u32;
    let fbdiv: u32;
    let rfdiv: u32;
    let vcodivsel: u32;
    let mut vcodiv: u32;
    let mut rate = parent_rate as u64;

    val = readl_relaxed((*pll).base.add(SPLL_CTRL0));
    fbdiv = (val >> map.fbdiv_shift) & FBDIV_MASK;
    let mut rfdiv_value = (val >> map.rfdiv_shift) & RFDIV_MASK;
    if rfdiv_value == 0 {
        pr_warn(b"%s has zero rfdiv\0".as_ptr() as *const c_char, clk_hw_get_name(hw));
        rfdiv_value = 1;
    }
    rfdiv = rfdiv_value;

    val = readl_relaxed((*pll).base.add(SPLL_CTRL1));
    vcodivsel = (val >> map.divsel_shift) & DIVSEL_MASK;
    vcodiv = map.vcodiv[vcodivsel as usize];
    if vcodiv == 0 {
        pr_warn(b"%s has zero vcodiv (index %d)\n\0".as_ptr() as *const c_char,
                clk_hw_get_name(hw), vcodivsel as c_int);
        vcodiv = 1;
    }

    rate = rate.wrapping_mul((fbdiv.wrapping_mul(map.mult)) as u64);
    rate /= (rfdiv.wrapping_mul(vcodiv)) as u64;
    rate as c_ulong
}

static berlin2_pll_ops: clk_ops = clk_ops {
    recalc_rate: Some(berlin2_pll_recalc_rate),
};

pub unsafe extern "C" fn berlin2_pll_register(
    map: *const berlin2_pll_map,
    base: *mut c_void,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
) -> c_int {
    let pll = kzalloc(core::mem::size_of::<berlin2_pll>(), 0) as *mut berlin2_pll;
    if pll.is_null() {
        return -12;
    }

    memcpy(&mut (*pll).map as *mut berlin2_pll_map as *mut c_void,
           map as *const c_void, core::mem::size_of::<berlin2_pll_map>());
    (*pll).base = base;

    let mut init = clk_init_data {
        name,
        ops: &berlin2_pll_ops,
        parent_names: &parent_name,
        num_parents: 1,
        flags,
    };
    (*pll).hw.init = &mut init;

    clk_hw_register(core::ptr::null_mut(), &mut (*pll).hw)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
