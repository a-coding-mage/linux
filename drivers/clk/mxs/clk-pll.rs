// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

type c_char = i8;
type c_ulong = usize;

extern "C" {
    fn writel_relaxed(value: u32, address: *mut c_void);
    fn udelay(usecs: u32);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn clk_register(dev: *mut c_void, hw: *mut clk_hw) -> *mut clk;
    fn err_ptr(error: isize) -> *mut clk;
}

// The SET and CLR register offsets are supplied by clk.h.
extern "C" {
    static SET: usize;
    static CLR: usize;
}

/**
 * struct clk_pll - mxs pll clock
 * @hw: clk_hw for the pll
 * @base: base address of the pll
 * @power: the shift of power bit
 * @rate: the clock rate of the pll
 *
 * The mxs pll is a fixed rate clock with power and gate control,
 * and the shift of gate bit is always 31.
 */
#[repr(C)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub base: *mut c_void,
    pub power: u8,
    pub rate: c_ulong,
}

unsafe fn to_clk_pll(hw: *mut clk_hw) -> *mut clk_pll {
    hw as *mut clk_pll
}

unsafe extern "C" fn clk_pll_prepare(hw: *mut clk_hw) -> i32 {
    let pll = &mut *to_clk_pll(hw);

    writel_relaxed(1u32.wrapping_shl(pll.power as u32),
                   pll.base.wrapping_add(SET as usize));

    udelay(10);

    0
}

unsafe extern "C" fn clk_pll_unprepare(hw: *mut clk_hw) {
    let pll = &mut *to_clk_pll(hw);

    writel_relaxed(1u32.wrapping_shl(pll.power as u32),
                   pll.base.wrapping_add(CLR as usize));
}

unsafe extern "C" fn clk_pll_enable(hw: *mut clk_hw) -> i32 {
    let pll = &mut *to_clk_pll(hw);

    writel_relaxed(1u32 << 31, pll.base.wrapping_add(CLR as usize));

    0
}

unsafe extern "C" fn clk_pll_disable(hw: *mut clk_hw) {
    let pll = &mut *to_clk_pll(hw);

    writel_relaxed(1u32 << 31, pll.base.wrapping_add(SET as usize));
}

unsafe extern "C" fn clk_pll_recalc_rate(hw: *mut clk_hw,
                                           _parent_rate: c_ulong) -> c_ulong {
    let pll = &mut *to_clk_pll(hw);

    pll.rate
}

static CLK_PLL_OPS: clk_ops = clk_ops {
    prepare: Some(clk_pll_prepare),
    unprepare: Some(clk_pll_unprepare),
    enable: Some(clk_pll_enable),
    disable: Some(clk_pll_disable),
    recalc_rate: Some(clk_pll_recalc_rate),
};

#[no_mangle]
pub unsafe extern "C" fn mxs_clk_pll(name: *const c_char,
                                      parent_name: *const c_char,
                                      base: *mut c_void,
                                      power: u8,
                                      rate: c_ulong) -> *mut clk {
    let pll: *mut clk_pll = kzalloc_obj();
    let mut init: clk_init_data;
    let clk: *mut clk;

    if pll.is_null() {
        return err_ptr(-12);
    }

    init = clk_init_data {
        name,
        ops: &CLK_PLL_OPS,
        flags: 0,
        parent_names: if !parent_name.is_null() {
            &parent_name
        } else {
            core::ptr::null()
        },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*pll).base = base;
    (*pll).rate = rate;
    (*pll).power = power;
    // The surrounding clk_hw definition supplies the init field.
    clk = clk_register(core::ptr::null_mut(), &mut (*pll).hw);
    if clk.is_null() {
        kfree(pll as *mut c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
