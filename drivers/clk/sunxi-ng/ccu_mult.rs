// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Declarations supplied by the Linux clock framework and the CCU headers.
#[repr(C)]
pub struct ccu_common {
    pub base: *mut u8,
    pub reg: usize,
    pub lock: *mut u8,
}

#[repr(C)]
pub struct ccu_mult_data {
    pub min: usize,
    pub max: usize,
    pub width: u32,
    pub shift: u32,
    pub offset: usize,
}

#[repr(C)]
pub struct ccu_mult {
    pub common: ccu_common,
    pub mult: ccu_mult_data,
    pub mux: u8,
    pub frac: u8,
    pub enable: u32,
    pub lock: u32,
}

#[repr(C)]
pub struct clk_hw;
#[repr(C)]
pub struct ccu_mux_internal;
#[repr(C)]
pub struct clk_rate_request {
    pub rate: usize,
    pub best_parent_rate: usize,
}

extern "C" {
    fn hw_to_ccu_mult(hw: *mut clk_hw) -> *mut ccu_mult;
    fn ccu_gate_helper_disable(common: *mut ccu_common, enable: u32);
    fn ccu_gate_helper_enable(common: *mut ccu_common, enable: u32) -> i32;
    fn ccu_gate_helper_is_enabled(common: *mut ccu_common, enable: u32) -> i32;
    fn ccu_frac_helper_is_enabled(common: *mut ccu_common, frac: *mut u8) -> bool;
    fn ccu_frac_helper_read_rate(common: *mut ccu_common, frac: *mut u8) -> usize;
    fn ccu_mux_helper_apply_prediv(common: *mut ccu_common, mux: *mut u8, index: i32, rate: usize) -> usize;
    fn ccu_mux_helper_determine_rate(common: *mut ccu_common, mux: *mut u8, req: *mut clk_rate_request, helper: unsafe extern "C" fn(*mut ccu_mux_internal, *mut clk_rate_request, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn ccu_frac_helper_has_rate(common: *mut ccu_common, frac: *mut u8, rate: usize) -> bool;
    fn ccu_frac_helper_enable(common: *mut ccu_common, frac: *mut u8);
    fn ccu_frac_helper_set_rate(common: *mut ccu_common, frac: *mut u8, rate: usize, lock: u32) -> i32;
    fn ccu_frac_helper_disable(common: *mut ccu_common, frac: *mut u8);
    fn ccu_mux_helper_get_parent(common: *mut ccu_common, mux: *mut u8) -> u8;
    fn ccu_mux_helper_set_parent(common: *mut ccu_common, mux: *mut u8, index: u8) -> i32;
    fn ccu_helper_wait_for_lock(common: *mut ccu_common, lock: u32);
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut u8, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut u8, flags: usize);
}

#[repr(C)]
struct _ccu_mult {
    mult: usize,
    min: usize,
    max: usize,
}

unsafe fn ccu_mult_find_best(parent: usize, rate: usize, mult: *mut _ccu_mult) {
    let mut _mult = (rate / parent) as i32;
    if _mult < (*mult).min as i32 { _mult = (*mult).min as i32; }
    if _mult > (*mult).max as i32 { _mult = (*mult).max as i32; }
    (*mult).mult = _mult as usize;
}

unsafe extern "C" fn ccu_mult_determine_rate_helper(_mux: *mut ccu_mux_internal, req: *mut clk_rate_request, data: *mut core::ffi::c_void) -> i32 {
    let cm = data as *mut ccu_mult;
    let mut _cm = _ccu_mult { mult: 0, min: (*cm).mult.min, max: 0 };
    _cm.max = if (*cm).mult.max != 0 { (*cm).mult.max } else { (1usize << (*cm).mult.width) + (*cm).mult.offset - 1 };
    ccu_mult_find_best((*req).best_parent_rate, (*req).rate, &mut _cm);
    (*req).rate = (*req).best_parent_rate * _cm.mult;
    0
}

unsafe extern "C" fn ccu_mult_disable(hw: *mut clk_hw) { let cm = hw_to_ccu_mult(hw); ccu_gate_helper_disable(&mut (*cm).common, (*cm).enable); }
unsafe extern "C" fn ccu_mult_enable(hw: *mut clk_hw) -> i32 { let cm = hw_to_ccu_mult(hw); ccu_gate_helper_enable(&mut (*cm).common, (*cm).enable) }
unsafe extern "C" fn ccu_mult_is_enabled(hw: *mut clk_hw) -> i32 { let cm = hw_to_ccu_mult(hw); ccu_gate_helper_is_enabled(&mut (*cm).common, (*cm).enable) }

unsafe extern "C" fn ccu_mult_recalc_rate(hw: *mut clk_hw, mut parent_rate: usize) -> usize {
    let cm = hw_to_ccu_mult(hw);
    if ccu_frac_helper_is_enabled(&mut (*cm).common, &mut (*cm).frac) { return ccu_frac_helper_read_rate(&mut (*cm).common, &mut (*cm).frac); }
    let reg = readl((*cm).common.base.add((*cm).common.reg));
    let val = ((reg >> (*cm).mult.shift) & ((1u32 << (*cm).mult.width) - 1)) as usize;
    parent_rate = ccu_mux_helper_apply_prediv(&mut (*cm).common, &mut (*cm).mux, -1, parent_rate);
    parent_rate * (val + (*cm).mult.offset)
}

unsafe extern "C" fn ccu_mult_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let cm = hw_to_ccu_mult(hw); ccu_mux_helper_determine_rate(&mut (*cm).common, &mut (*cm).mux, req, ccu_mult_determine_rate_helper, cm.cast()) }

unsafe extern "C" fn ccu_mult_set_rate(hw: *mut clk_hw, rate: usize, mut parent_rate: usize) -> i32 {
    let cm = hw_to_ccu_mult(hw);
    if ccu_frac_helper_has_rate(&mut (*cm).common, &mut (*cm).frac, rate) { ccu_frac_helper_enable(&mut (*cm).common, &mut (*cm).frac); return ccu_frac_helper_set_rate(&mut (*cm).common, &mut (*cm).frac, rate, (*cm).lock); } else { ccu_frac_helper_disable(&mut (*cm).common, &mut (*cm).frac); }
    parent_rate = ccu_mux_helper_apply_prediv(&mut (*cm).common, &mut (*cm).mux, -1, parent_rate);
    let mut _cm = _ccu_mult { mult: 0, min: (*cm).mult.min, max: if (*cm).mult.max != 0 { (*cm).mult.max } else { (1usize << (*cm).mult.width) + (*cm).mult.offset - 1 } };
    ccu_mult_find_best(parent_rate, rate, &mut _cm);
    let mut flags = 0usize; spin_lock_irqsave((*cm).common.lock, &mut flags);
    let mut reg = readl((*cm).common.base.add((*cm).common.reg));
    reg &= !(((1u32 << (*cm).mult.width) - 1) << (*cm).mult.shift);
    reg |= (((_cm.mult - (*cm).mult.offset) as u32) << (*cm).mult.shift);
    writel(reg, (*cm).common.base.add((*cm).common.reg)); spin_unlock_irqrestore((*cm).common.lock, flags);
    ccu_helper_wait_for_lock(&mut (*cm).common, (*cm).lock); 0
}

unsafe extern "C" fn ccu_mult_get_parent(hw: *mut clk_hw) -> u8 { let cm = hw_to_ccu_mult(hw); ccu_mux_helper_get_parent(&mut (*cm).common, &mut (*cm).mux) }
unsafe extern "C" fn ccu_mult_set_parent(hw: *mut clk_hw, index: u8) -> i32 { let cm = hw_to_ccu_mult(hw); ccu_mux_helper_set_parent(&mut (*cm).common, &mut (*cm).mux, index) }

#[repr(C)]
pub struct clk_ops {
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
}

#[no_mangle]
pub static ccu_mult_ops: clk_ops = clk_ops { disable: Some(ccu_mult_disable), enable: Some(ccu_mult_enable), is_enabled: Some(ccu_mult_is_enabled), get_parent: Some(ccu_mult_get_parent), set_parent: Some(ccu_mult_set_parent), determine_rate: Some(ccu_mult_determine_rate), recalc_rate: Some(ccu_mult_recalc_rate), set_rate: Some(ccu_mult_set_rate) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
