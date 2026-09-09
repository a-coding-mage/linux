// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_multiplier {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub flags: libc::c_ulong,
    pub lock: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: libc::c_ulong,
    pub best_parent_rate: libc::c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, libc::c_ulong) -> libc::c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, libc::c_ulong, libc::c_ulong) -> i32>,
}

pub const CLK_MULTIPLIER_BIG_ENDIAN: libc::c_ulong = 1 << 0;
pub const CLK_MULTIPLIER_ROUND_CLOSEST: libc::c_ulong = 1 << 1;
pub const CLK_MULTIPLIER_ZERO_BYPASS: libc::c_ulong = 1 << 2;
pub const CLK_SET_RATE_PARENT: libc::c_ulong = 1 << 0;

extern "C" {
    fn ioread32be(reg: *mut core::ffi::c_void) -> u32;
    fn readl(reg: *mut core::ffi::c_void) -> u32;
    fn iowrite32be(val: u32, reg: *mut core::ffi::c_void);
    fn writel(val: u32, reg: *mut core::ffi::c_void);
    fn clk_hw_get_flags(hw: *mut clk_hw) -> libc::c_ulong;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: libc::c_ulong) -> libc::c_ulong;
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut libc::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: libc::c_ulong);
    fn __acquire(lock: *mut core::ffi::c_void);
    fn __release(lock: *mut core::ffi::c_void);
}

#[inline]
unsafe fn to_clk_multiplier(hw: *mut clk_hw) -> *mut clk_multiplier {
    hw as *mut clk_multiplier
}

#[inline]
fn genmask(width: u8, shift: u8) -> u32 {
    (((1u32 << width) - 1) << shift)
}

#[inline]
unsafe fn clk_mult_readl(mult: *mut clk_multiplier) -> u32 {
    if (*mult).flags & CLK_MULTIPLIER_BIG_ENDIAN != 0 {
        return ioread32be((*mult).reg);
    }
    readl((*mult).reg)
}

#[inline]
unsafe fn clk_mult_writel(mult: *mut clk_multiplier, val: u32) {
    if (*mult).flags & CLK_MULTIPLIER_BIG_ENDIAN != 0 {
        iowrite32be(val, (*mult).reg);
    } else {
        writel(val, (*mult).reg);
    }
}

unsafe fn __get_mult(mult: *mut clk_multiplier, rate: libc::c_ulong, parent_rate: libc::c_ulong) -> libc::c_ulong {
    if (*mult).flags & CLK_MULTIPLIER_ROUND_CLOSEST != 0 {
        return (rate + parent_rate / 2) / parent_rate;
    }
    rate / parent_rate
}

unsafe extern "C" fn clk_multiplier_recalc_rate(hw: *mut clk_hw, parent_rate: libc::c_ulong) -> libc::c_ulong {
    let mult = to_clk_multiplier(hw);
    let mut val = (clk_mult_readl(mult) >> (*mult).shift) & genmask((*mult).width, 0);
    if val == 0 && (*mult).flags & CLK_MULTIPLIER_ZERO_BYPASS != 0 {
        val = 1;
    }
    parent_rate * val as libc::c_ulong
}

fn __is_best_rate(rate: libc::c_ulong, new: libc::c_ulong, best: libc::c_ulong, flags: libc::c_ulong) -> bool {
    if flags & CLK_MULTIPLIER_ROUND_CLOSEST != 0 {
        return rate.abs_diff(new) < rate.abs_diff(best);
    }
    new >= rate && new < best
}

unsafe fn __bestmult(hw: *mut clk_hw, rate: libc::c_ulong, best_parent_rate: *mut libc::c_ulong, width: u8, flags: libc::c_ulong) -> libc::c_ulong {
    let mult = to_clk_multiplier(hw);
    let orig_parent_rate = *best_parent_rate;
    let mut best_rate = libc::c_ulong::MAX;
    let mut bestmult = 0;
    let maxmult = (1u64 << width) - 1;
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        bestmult = rate / orig_parent_rate;
        if bestmult == 0 && (*mult).flags & CLK_MULTIPLIER_ZERO_BYPASS == 0 { bestmult = 1; }
        if bestmult > maxmult as libc::c_ulong { bestmult = maxmult as libc::c_ulong; }
        return bestmult;
    }
    for i in 1..maxmult {
        if rate == orig_parent_rate * i as libc::c_ulong { *best_parent_rate = orig_parent_rate; return i as libc::c_ulong; }
        let parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), rate / i as libc::c_ulong);
        let current_rate = parent_rate * i as libc::c_ulong;
        if __is_best_rate(rate, current_rate, best_rate, flags) {
            bestmult = i as libc::c_ulong;
            best_rate = current_rate;
            *best_parent_rate = parent_rate;
        }
    }
    bestmult
}

unsafe extern "C" fn clk_multiplier_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mult = to_clk_multiplier(hw);
    let factor = __bestmult(hw, (*req).rate, &mut (*req).best_parent_rate, (*mult).width, (*mult).flags);
    (*req).rate = (*req).best_parent_rate * factor;
    0
}

unsafe extern "C" fn clk_multiplier_set_rate(hw: *mut clk_hw, rate: libc::c_ulong, parent_rate: libc::c_ulong) -> i32 {
    let mult = to_clk_multiplier(hw);
    let factor = __get_mult(mult, rate, parent_rate);
    let mut flags = 0;
    if !(*mult).lock.is_null() { spin_lock_irqsave((*mult).lock, &mut flags); } else { __acquire((*mult).lock); }
    let mut val = clk_mult_readl(mult);
    val &= !genmask((*mult).width + (*mult).shift, (*mult).shift);
    val |= (factor as u32) << (*mult).shift;
    clk_mult_writel(mult, val);
    if !(*mult).lock.is_null() { spin_unlock_irqrestore((*mult).lock, flags); } else { __release((*mult).lock); }
    0
}

pub static clk_multiplier_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_multiplier_recalc_rate),
    determine_rate: Some(clk_multiplier_determine_rate),
    set_rate: Some(clk_multiplier_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
