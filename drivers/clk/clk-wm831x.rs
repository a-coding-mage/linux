// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * WM831x clock control
 *
 * Copyright 2011-2 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_char;

#[repr(C)]
pub struct wm831x {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
    pub num_parents: usize,
    pub flags: u32,
}
#[repr(C)]
pub struct clk_rate_request {
    pub rate: usize,
}
#[repr(C)]
pub struct clk_ops {
    pub is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
}

extern "C" {
    fn wm831x_reg_read(wm831x: *mut wm831x, reg: u32) -> i32;
    fn wm831x_set_bits(wm831x: *mut wm831x, reg: u32, mask: u32, val: u32) -> i32;
    fn wm831x_reg_unlock(wm831x: *mut wm831x) -> i32;
    fn wm831x_reg_lock(wm831x: *mut wm831x);
    fn usleep_range(min: u32, max: u32);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut wm831x;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn clk_hw_determine_rate_no_reparent(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32;
}

const WM831X_FLL_CONTROL_1: u32 = 0;
const WM831X_CLOCK_CONTROL_1: u32 = 0;
const WM831X_CLOCK_CONTROL_2: u32 = 0;
const WM831X_FLL_CONTROL_5: u32 = 0;
const WM831X_FLL_ENA: u32 = 0;
const WM831X_FLL_AUTO: u32 = 0;
const WM831X_FLL_AUTO_FREQ_MASK: u32 = 0;
const WM831X_FLL_CLK_SRC_MASK: u32 = 0;
const WM831X_CLKOUT_ENA: u32 = 0;
const WM831X_CLKOUT_SRC: u32 = 0;
const WM831X_CLKOUT_SRC_SHIFT: u32 = 0;
const WM831X_XTAL_ENA: u32 = 0;
const CLK_SET_RATE_GATE: u32 = 0;
const CLK_SET_RATE_PARENT: u32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EPERM: i32 = 1;

#[repr(C)]
struct wm831x_clk {
    wm831x: *mut wm831x,
    xtal_hw: clk_hw,
    fll_hw: clk_hw,
    clkout_hw: clk_hw,
    xtal_ena: bool,
}

unsafe fn wm831x_xtal_is_prepared(hw: *mut clk_hw) -> i32 {
    let clkdata = (hw as *mut wm831x_clk).as_ref().unwrap();
    clkdata.xtal_ena as i32
}

unsafe fn wm831x_xtal_recalc_rate(hw: *mut clk_hw, _parent_rate: usize) -> usize {
    let clkdata = (hw as *mut wm831x_clk).as_ref().unwrap();
    if clkdata.xtal_ena { 32768 } else { 0 }
}

static WM831X_XTAL_OPS: clk_ops = clk_ops { is_prepared: Some(wm831x_xtal_is_prepared), prepare: None, unprepare: None, determine_rate: None, recalc_rate: Some(wm831x_xtal_recalc_rate), set_rate: None, get_parent: None, set_parent: None };
static WM831X_XTAL_INIT: clk_init_data = clk_init_data { name: b"xtal\0".as_ptr() as *const c_char, ops: &WM831X_XTAL_OPS, parent_names: core::ptr::null(), num_parents: 0, flags: 0 };
static WM831X_FLL_AUTO_RATES: [usize; 8] = [2048000, 11289600, 12000000, 12288000, 19200000, 22579600, 24000000, 24576000];

unsafe fn wm831x_fll_is_prepared(hw: *mut clk_hw) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_read(c.wm831x, WM831X_FLL_CONTROL_1); if r < 0 { return 1; } ((r as u32 & WM831X_FLL_ENA) != 0) as i32 }
unsafe fn wm831x_fll_prepare(hw: *mut clk_hw) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_set_bits(c.wm831x, WM831X_FLL_CONTROL_1, WM831X_FLL_ENA, WM831X_FLL_ENA); usleep_range(2000, 3000); r }
unsafe fn wm831x_fll_unprepare(hw: *mut clk_hw) { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let _ = wm831x_set_bits(c.wm831x, WM831X_FLL_CONTROL_1, WM831X_FLL_ENA, 0); }
unsafe fn wm831x_fll_recalc_rate(hw: *mut clk_hw, _parent_rate: usize) -> usize { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_read(c.wm831x, WM831X_CLOCK_CONTROL_2); if r < 0 { return 0; } if r as u32 & WM831X_FLL_AUTO != 0 { return WM831X_FLL_AUTO_RATES[(r as u32 & WM831X_FLL_AUTO_FREQ_MASK) as usize]; } 0 }
unsafe fn wm831x_fll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let mut best = 0usize; for i in 0..WM831X_FLL_AUTO_RATES.len() { if WM831X_FLL_AUTO_RATES[i].abs_diff((*req).rate) < WM831X_FLL_AUTO_RATES[best].abs_diff((*req).rate) { best = i; } } (*req).rate = WM831X_FLL_AUTO_RATES[best]; 0 }
unsafe fn wm831x_fll_set_rate(hw: *mut clk_hw, rate: usize, _parent_rate: usize) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let mut i = 0; while i < WM831X_FLL_AUTO_RATES.len() && WM831X_FLL_AUTO_RATES[i] != rate { i += 1; } if i == WM831X_FLL_AUTO_RATES.len() { return -EINVAL; } if wm831x_fll_is_prepared(hw) != 0 { return -EPERM; } wm831x_set_bits(c.wm831x, WM831X_CLOCK_CONTROL_2, WM831X_FLL_AUTO_FREQ_MASK, i as u32) }
unsafe fn wm831x_fll_get_parent(hw: *mut clk_hw) -> u8 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_read(c.wm831x, WM831X_CLOCK_CONTROL_2); if r >= 0 && r as u32 & WM831X_FLL_AUTO != 0 { return 0; } let r = wm831x_reg_read(c.wm831x, WM831X_FLL_CONTROL_5); if r >= 0 && (r as u32 & WM831X_FLL_CLK_SRC_MASK) == 1 { 1 } else { 0 } }

unsafe fn wm831x_clkout_is_prepared(hw: *mut clk_hw) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_read(c.wm831x, WM831X_CLOCK_CONTROL_1); if r < 0 { return 0; } ((r as u32 & WM831X_CLKOUT_ENA) != 0) as i32 }
unsafe fn wm831x_clkout_prepare(hw: *mut clk_hw) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_unlock(c.wm831x); if r != 0 { return r; } let r = wm831x_set_bits(c.wm831x, WM831X_CLOCK_CONTROL_1, WM831X_CLKOUT_ENA, WM831X_CLKOUT_ENA); wm831x_reg_lock(c.wm831x); r }
unsafe fn wm831x_clkout_unprepare(hw: *mut clk_hw) { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); if wm831x_reg_unlock(c.wm831x) == 0 { let _ = wm831x_set_bits(c.wm831x, WM831X_CLOCK_CONTROL_1, WM831X_CLKOUT_ENA, 0); wm831x_reg_lock(c.wm831x); } }
unsafe fn wm831x_clkout_get_parent(hw: *mut clk_hw) -> u8 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); let r = wm831x_reg_read(c.wm831x, WM831X_CLOCK_CONTROL_1); if r >= 0 && r as u32 & WM831X_CLKOUT_SRC != 0 { 1 } else { 0 } }
unsafe fn wm831x_clkout_set_parent(hw: *mut clk_hw, parent: u8) -> i32 { let c = (hw as *mut wm831x_clk).as_ref().unwrap(); wm831x_set_bits(c.wm831x, WM831X_CLOCK_CONTROL_1, WM831X_CLKOUT_SRC, (parent as u32) << WM831X_CLKOUT_SRC_SHIFT) }

unsafe fn wm831x_clk_probe(pdev: *mut platform_device) -> i32 {
    let wm831x = dev_get_drvdata(&mut (*pdev).dev);
    let clkdata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<wm831x_clk>(), GFP_KERNEL) as *mut wm831x_clk;
    if clkdata.is_null() { return -ENOMEM; }
    (*clkdata).wm831x = wm831x;
    let ret = wm831x_reg_read(wm831x, WM831X_CLOCK_CONTROL_2);
    if ret < 0 { return ret; }
    (*clkdata).xtal_ena = ret as u32 & WM831X_XTAL_ENA != 0;
    (*clkdata).xtal_hw.init = &WM831X_XTAL_INIT;
    let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clkdata).xtal_hw); if ret != 0 { return ret; }
    platform_set_drvdata(pdev, clkdata as *mut core::ffi::c_void);
    0
}

// C module registration and metadata are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
