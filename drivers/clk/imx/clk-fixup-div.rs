// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here by their source-level names.

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_divider {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong,
    pub determine_rate: unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int,
    pub set_rate: unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int,
}

#[repr(C)]
pub struct clk_rate_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

pub type c_int = i32;
pub type c_ulong = usize;
pub type c_char = i8;
pub type u32 = core::ffi::c_uint;
pub type u8 = core::ffi::c_uchar;
pub type spinlock_t = core::ffi::c_void;

#[repr(C)]
pub struct clk_fixup_div {
    pub divider: clk_divider,
    pub ops: *const clk_ops,
    pub fixup: Option<unsafe extern "C" fn(*mut u32)>,
}

extern "C" {
    pub static imx_ccm_lock: spinlock_t;
    pub static clk_divider_ops: clk_ops;
    pub const CLK_SET_RATE_PARENT: u32;
    pub fn to_clk_divider(hw: *mut clk_hw) -> *mut clk_divider;
    pub fn readl(reg: *mut core::ffi::c_void) -> u32;
    pub fn writel(val: u32, reg: *mut core::ffi::c_void);
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    pub fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> c_int;
    pub fn kzalloc_obj<T>() -> *mut T;
    pub fn kfree(ptr: *mut core::ffi::c_void);
}

#[inline]
unsafe fn div_mask(d: *mut clk_divider) -> u32 {
    (1u32 << (*d).width) - 1
}

unsafe fn to_clk_fixup_div(hw: *mut clk_hw) -> *mut clk_fixup_div {
    let divider = to_clk_divider(hw);
    divider as *mut clk_fixup_div
}

unsafe extern "C" fn clk_fixup_div_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let fixup_div = to_clk_fixup_div(hw);
    ((*(*fixup_div).ops).recalc_rate)(&mut (*fixup_div).divider.hw, parent_rate)
}

unsafe extern "C" fn clk_fixup_div_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let fixup_div = to_clk_fixup_div(hw);
    ((*(*fixup_div).ops).determine_rate)(&mut (*fixup_div).divider.hw, req)
}

unsafe extern "C" fn clk_fixup_div_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let fixup_div = to_clk_fixup_div(hw);
    let div = to_clk_divider(hw);
    let divider = parent_rate / rate;
    let mut value = divider - 1;
    let mut flags: c_ulong = 0;
    let mut val: u32;

    if value > div_mask(div) as c_ulong {
        value = div_mask(div) as c_ulong;
    }

    spin_lock_irqsave((*div).lock, &mut flags);
    val = readl((*div).reg);
    val &= !(div_mask(div) << (*div).shift);
    val |= (value as u32) << (*div).shift;
    ((*fixup_div).fixup.unwrap())(&mut val);
    writel(val, (*div).reg);
    spin_unlock_irqrestore((*div).lock, flags);

    0
}

static clk_fixup_div_ops: clk_ops = clk_ops {
    recalc_rate: clk_fixup_div_recalc_rate,
    determine_rate: clk_fixup_div_determine_rate,
    set_rate: clk_fixup_div_set_rate,
};

pub unsafe extern "C" fn imx_clk_hw_fixup_divider(
    name: *const c_char,
    parent: *const c_char,
    reg: *mut core::ffi::c_void,
    shift: u8,
    width: u8,
    fixup: Option<unsafe extern "C" fn(*mut u32)>,
) -> *mut clk_hw {
    if fixup.is_none() {
        return (-22isize) as *mut clk_hw;
    }

    let fixup_div = kzalloc_obj::<clk_fixup_div>();
    if fixup_div.is_null() {
        return (-12isize) as *mut clk_hw;
    }

    let init = clk_init_data {
        name,
        ops: &clk_fixup_div_ops,
        flags: CLK_SET_RATE_PARENT,
        parent_names: if !parent.is_null() { &parent } else { core::ptr::null() },
        num_parents: if !parent.is_null() { 1 } else { 0 },
    };

    (*fixup_div).divider.reg = reg;
    (*fixup_div).divider.shift = shift;
    (*fixup_div).divider.width = width;
    (*fixup_div).divider.lock = &imx_ccm_lock as *const _ as *mut _;
    (*fixup_div).divider.hw.init = &init;
    (*fixup_div).ops = &clk_divider_ops;
    (*fixup_div).fixup = fixup;

    let hw = &mut (*fixup_div).divider.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(fixup_div as *mut core::ffi::c_void);
        return (ret as isize) as *mut clk_hw;
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
