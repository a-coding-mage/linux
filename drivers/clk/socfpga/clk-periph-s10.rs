// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017, Intel Corporation
 */

// Dependencies supplied by the surrounding kernel clock implementation:
// linux/slab.h, linux/clk-provider.h, linux/io.h, stratix10-clk.h, and clk.h

const CLK_MGR_FREE_SHIFT: u32 = 16;
const CLK_MGR_FREE_MASK: u32 = 0x7;
const SWCTRLBTCLKSEN_SHIFT: u32 = 8;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn kfree(ptr: *mut socfpga_periph_clk);
    fn warn_on(condition: bool) -> bool;
    fn err_ptr(error: i32) -> *mut clk_hw;
}

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct socfpga_periph_clk_hw {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct socfpga_periph_clk {
    pub hw: socfpga_periph_clk_hw,
    pub shift: u32,
    pub bypass_reg: *mut core::ffi::c_void,
    pub bypass_shift: u32,
    pub fixed_div: usize,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub num_parents: u8,
    pub parent_names: *mut *const core::ffi::c_char,
    pub parent_data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct stratix10_perip_c_clock {
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub parent_data: *const core::ffi::c_void,
    pub offset: usize,
    pub flags: u32,
    pub num_parents: u8,
}

#[repr(C)]
pub struct n5x_perip_c_clock {
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub offset: usize,
    pub shift: u32,
    pub flags: u32,
    pub num_parents: u8,
}

#[repr(C)]
pub struct stratix10_perip_cnt_clock {
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub parent_data: *const core::ffi::c_void,
    pub offset: usize,
    pub bypass_reg: usize,
    pub bypass_shift: u32,
    pub fixed_divider: usize,
    pub flags: u32,
    pub num_parents: u8,
}

#[repr(C)]
pub struct agilex5_perip_cnt_clock {
    pub name: *const core::ffi::c_char,
    pub parent_names: *mut *const core::ffi::c_char,
    pub offset: usize,
    pub bypass_reg: usize,
    pub bypass_shift: u32,
    pub fixed_divider: usize,
    pub flags: u32,
    pub num_parents: u8,
}

unsafe extern "C" fn n5x_clk_peri_c_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let socfpgaclk = (hwclk as *mut u8).sub(core::mem::offset_of!(socfpga_periph_clk, hw)) as *mut socfpga_periph_clk;
    let shift = (*socfpgaclk).shift;
    let mut val = readl((*socfpgaclk).hw.reg);
    val &= 0x1f_u32 << shift;
    let div = (val >> shift) as usize + 1;
    parent_rate / div
}

unsafe extern "C" fn clk_peri_c_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let socfpgaclk = (hwclk as *mut u8).sub(core::mem::offset_of!(socfpga_periph_clk, hw)) as *mut socfpga_periph_clk;
    let mut val = readl((*socfpgaclk).hw.reg);
    val &= (1_u32 << SWCTRLBTCLKSEN_SHIFT) - 1;
    parent_rate / val as usize
}

unsafe extern "C" fn clk_peri_cnt_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let socfpgaclk = (hwclk as *mut u8).sub(core::mem::offset_of!(socfpga_periph_clk, hw)) as *mut socfpga_periph_clk;
    let div = if (*socfpgaclk).fixed_div != 0 {
        (*socfpgaclk).fixed_div
    } else if !(*socfpgaclk).hw.reg.is_null() {
        (readl((*socfpgaclk).hw.reg) & 0x7ff) as usize + 1
    } else { 1 };
    parent_rate / div
}

unsafe extern "C" fn clk_periclk_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = (hwclk as *mut u8).sub(core::mem::offset_of!(socfpga_periph_clk, hw)) as *mut socfpga_periph_clk;
    if !(*socfpgaclk).bypass_reg.is_null() {
        let parent = ((readl((*socfpgaclk).bypass_reg) & (1 << (*socfpgaclk).bypass_shift)) >> (*socfpgaclk).bypass_shift) as u8;
        if parent != 0 { return parent; }
    }
    if !(*socfpgaclk).hw.reg.is_null() {
        return ((readl((*socfpgaclk).hw.reg) >> CLK_MGR_FREE_SHIFT) & CLK_MGR_FREE_MASK) as u8;
    }
    0
}

static N5X_PERI_C_CLK_OPS: clk_ops = clk_ops { recalc_rate: Some(n5x_clk_peri_c_clk_recalc_rate), get_parent: Some(clk_periclk_get_parent) };
static PERI_C_CLK_OPS: clk_ops = clk_ops { recalc_rate: Some(clk_peri_c_clk_recalc_rate), get_parent: Some(clk_periclk_get_parent) };
static PERI_CNT_CLK_OPS: clk_ops = clk_ops { recalc_rate: Some(clk_peri_cnt_clk_recalc_rate), get_parent: Some(clk_periclk_get_parent) };

// The registration functions below preserve the C allocation and clock-registration flow.
pub unsafe fn s10_register_periph(clks: *const stratix10_perip_c_clock, reg: *mut core::ffi::c_void) -> *mut clk_hw {
    let periph_clk = alloc_zeroed::<socfpga_periph_clk>();
    if periph_clk.is_null() { return core::ptr::null_mut(); }
    (*periph_clk).hw.reg = (reg as *mut u8).add((*clks).offset) as *mut core::ffi::c_void;
    let init = clk_init_data { name: (*clks).name, ops: &PERI_C_CLK_OPS, flags: (*clks).flags, num_parents: (*clks).num_parents, parent_names: core::ptr::null_mut(), parent_data: (*clks).parent_data };
    (*periph_clk).hw.hw.init = alloc_value(init);
    let hw_clk = &mut (*periph_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(periph_clk); return err_ptr(ret); }
    hw_clk
}

pub unsafe fn n5x_register_periph(clks: *const n5x_perip_c_clock, regbase: *mut core::ffi::c_void) -> *mut clk_hw {
    let periph_clk = alloc_zeroed::<socfpga_periph_clk>();
    if periph_clk.is_null() { return core::ptr::null_mut(); }
    (*periph_clk).hw.reg = (regbase as *mut u8).add((*clks).offset) as *mut core::ffi::c_void;
    (*periph_clk).shift = (*clks).shift;
    let init = clk_init_data { name: (*clks).name, ops: &N5X_PERI_C_CLK_OPS, flags: (*clks).flags, num_parents: (*clks).num_parents, parent_names: core::ptr::null_mut(), parent_data: core::ptr::null() };
    (*periph_clk).hw.hw.init = alloc_value(init);
    let hw_clk = &mut (*periph_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(periph_clk); return err_ptr(ret); }
    hw_clk
}

pub unsafe fn s10_register_cnt_periph(clks: *const stratix10_perip_cnt_clock, regbase: *mut core::ffi::c_void) -> *mut clk_hw {
    let periph_clk = alloc_zeroed::<socfpga_periph_clk>();
    if periph_clk.is_null() { return core::ptr::null_mut(); }
    (*periph_clk).hw.reg = if (*clks).offset != 0 { (regbase as *mut u8).add((*clks).offset) as _ } else { core::ptr::null_mut() };
    (*periph_clk).bypass_reg = if (*clks).bypass_reg != 0 { (regbase as *mut u8).add((*clks).bypass_reg) as _ } else { core::ptr::null_mut() };
    (*periph_clk).bypass_shift = (*clks).bypass_shift; (*periph_clk).fixed_div = (*clks).fixed_divider;
    let init = clk_init_data { name: (*clks).name, ops: &PERI_CNT_CLK_OPS, flags: (*clks).flags, num_parents: (*clks).num_parents, parent_names: core::ptr::null_mut(), parent_data: (*clks).parent_data };
    (*periph_clk).hw.hw.init = alloc_value(init); let hw_clk = &mut (*periph_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk); if ret != 0 { kfree(periph_clk); return err_ptr(ret); } hw_clk
}

pub unsafe fn agilex5_register_cnt_periph(clks: *const agilex5_perip_cnt_clock, regbase: *mut core::ffi::c_void) -> *mut clk_hw {
    let periph_clk = alloc_zeroed::<socfpga_periph_clk>(); if periph_clk.is_null() { return core::ptr::null_mut(); }
    (*periph_clk).hw.reg = if (*clks).offset != 0 { (regbase as *mut u8).add((*clks).offset) as _ } else { core::ptr::null_mut() };
    (*periph_clk).bypass_reg = if (*clks).bypass_reg != 0 { (regbase as *mut u8).add((*clks).bypass_reg) as _ } else { core::ptr::null_mut() };
    (*periph_clk).bypass_shift = (*clks).bypass_shift; (*periph_clk).fixed_div = (*clks).fixed_divider;
    let init = clk_init_data { name: (*clks).name, ops: &PERI_CNT_CLK_OPS, flags: (*clks).flags, num_parents: (*clks).num_parents, parent_names: (*clks).parent_names, parent_data: core::ptr::null() };
    (*periph_clk).hw.hw.init = alloc_value(init); let hw_clk = &mut (*periph_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk); if ret != 0 { kfree(periph_clk); return err_ptr(ret); } hw_clk
}

unsafe fn alloc_zeroed<T>() -> *mut T { std::alloc::alloc_zeroed(std::alloc::Layout::new::<T>()) as *mut T }
unsafe fn alloc_value<T>(value: T) -> *mut T { let p = std::alloc::alloc(std::alloc::Layout::new::<T>()) as *mut T; p.write(value); p }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
