// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: ::core::ffi::c_ulong,
    pub best_parent_rate: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const ::core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const ::core::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, ::core::ffi::c_ulong) -> ::core::ffi::c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, ::core::ffi::c_ulong, ::core::ffi::c_ulong) -> i32>,
}

extern "C" {
    static mut mxs_lock: u8;
    fn readl_relaxed(reg: *mut ::core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, reg: *mut ::core::ffi::c_void);
    fn mxs_clk_wait(reg: *mut ::core::ffi::c_void, busy: u8) -> i32;
    fn clk_register(dev: *mut ::core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
    fn kfree(ptr: *mut clk_frac);
    fn kzalloc_obj<T>() -> *mut T;
    fn err_ptr(error: isize) -> *mut clk;
    fn spin_lock_irqsave(lock: *mut u8, flags: *mut ::core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut u8, flags: ::core::ffi::c_ulong);
    fn is_err(ptr: *mut clk) -> bool;
}

#[repr(C)]
pub struct clk_frac {
    pub hw: clk_hw,
    pub reg: *mut ::core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub busy: u8,
}

unsafe fn to_clk_frac(hw: *mut clk_hw) -> *mut clk_frac {
    hw as *mut clk_frac
}

unsafe extern "C" fn clk_frac_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let frac = &*to_clk_frac(hw);
    let mut div = readl_relaxed(frac.reg) >> frac.shift;
    div &= (1u32 << frac.width) - 1;

    let tmp_rate = (parent_rate as u64).wrapping_mul(div as u64);
    (tmp_rate >> frac.width) as ::core::ffi::c_ulong
}

unsafe extern "C" fn clk_frac_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let frac = &*to_clk_frac(hw);
    let parent_rate = (*req).best_parent_rate;
    let mut tmp = (*req).rate as u64;

    if (*req).rate > parent_rate {
        return -22;
    }

    tmp <<= frac.width;
    tmp /= parent_rate as u64;
    let div = tmp as u32;

    if div == 0 {
        return -22;
    }

    let tmp_rate = (parent_rate as u64).wrapping_mul(div as u64);
    let mut result = tmp_rate >> frac.width;
    if (result << frac.width) < tmp_rate {
        result += 1;
    }
    (*req).rate = result as ::core::ffi::c_ulong;

    0
}

unsafe extern "C" fn clk_frac_set_rate(
    hw: *mut clk_hw,
    rate: ::core::ffi::c_ulong,
    parent_rate: ::core::ffi::c_ulong,
) -> i32 {
    let frac = &*to_clk_frac(hw);
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut tmp = rate as u64;

    if rate > parent_rate {
        return -22;
    }

    tmp <<= frac.width;
    tmp /= parent_rate as u64;
    let div = tmp as u32;

    if div == 0 {
        return -22;
    }

    spin_lock_irqsave(&raw mut mxs_lock, &mut flags);

    let mut val = readl_relaxed(frac.reg);
    val &= !(((1u32 << frac.width) - 1) << frac.shift);
    val |= div << frac.shift;
    writel_relaxed(val, frac.reg);

    spin_unlock_irqrestore(&raw mut mxs_lock, flags);

    mxs_clk_wait(frac.reg, frac.busy)
}

static clk_frac_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_frac_recalc_rate),
    determine_rate: Some(clk_frac_determine_rate),
    set_rate: Some(clk_frac_set_rate),
};

pub unsafe extern "C" fn mxs_clk_frac(
    name: *const ::core::ffi::c_char,
    parent_name: *const ::core::ffi::c_char,
    reg: *mut ::core::ffi::c_void,
    shift: u8,
    width: u8,
    busy: u8,
) -> *mut clk {
    let frac = kzalloc_obj::<clk_frac>();
    if frac.is_null() {
        return err_ptr(-12);
    }

    let mut init = clk_init_data {
        name,
        ops: &clk_frac_ops,
        flags: 1 << 2,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*frac).reg = reg;
    (*frac).shift = shift;
    (*frac).width = width;
    (*frac).busy = busy;
    (*frac).hw.init = &mut init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*frac).hw);
    if is_err(clk) {
        kfree(frac);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
