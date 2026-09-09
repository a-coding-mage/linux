/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
 * All rights reserved.
 *
 * Authors:
 *	Yifeng Huang <huangyifeng@eswincomputing.com>
 *	Xuyang Dong <dongxuyang@eswincomputing.com>
 */

pub const APLL_HIGH_FREQ: u32 = 983040000;
pub const APLL_LOW_FREQ: u32 = 225792000;
pub const PLL_HIGH_FREQ: u32 = 1800000000;
pub const PLL_LOW_FREQ: u32 = 24000000;

/* ESWIN_PRIV_DIV_MIN_2: minimum register value and division ratio is 2. */
pub const ESWIN_PRIV_DIV_MIN_2: u32 = BIT(0);

#[repr(C)]
pub enum eswin_clk_type {
    CLK_FIXED_FACTOR,
    CLK_MUX,
    CLK_DIVIDER,
    CLK_GATE,
}

#[repr(C)]
pub struct eswin_clock_data {
    pub base: *mut core::ffi::c_void,
    pub original_clk: *mut clk_hw,
    pub pll_nb: notifier_block,
    pub lock: spinlock_t,
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
pub struct eswin_divider_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub ctrl_reg: *mut core::ffi::c_void,
    pub flags: usize,
    pub reg: usize,
    pub shift: u8,
    pub width: u8,
    pub div_flags: usize,
    pub priv_flag: usize,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct eswin_fixed_rate_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub flags: usize,
    pub rate: usize,
}

#[repr(C)]
pub struct eswin_fixed_factor_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub mult: usize,
    pub div: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct eswin_gate_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub flags: usize,
    pub reg: usize,
    pub bit_idx: u8,
    pub gate_flags: u8,
}

#[repr(C)]
pub struct eswin_mux_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: usize,
    pub reg: usize,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
    pub table: *mut u32,
}

#[repr(C)]
pub struct eswin_pll_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub ctrl_reg0: u32,
    pub fbdiv_shift: u8,
    pub ctrl_reg1: u32,
    pub frac_shift: u8,
    pub ctrl_reg2: u32,
    pub status_reg: u32,
    pub lock_shift: u8,
    pub lock_width: u8,
    pub max_rate: u64,
    pub min_rate: u64,
}

#[repr(C)]
pub struct eswin_clk_pll {
    pub hw: clk_hw,
    pub id: u32,
    pub ctrl_reg0: *mut core::ffi::c_void,
    pub fbdiv_shift: u8,
    pub ctrl_reg1: *mut core::ffi::c_void,
    pub frac_shift: u8,
    pub ctrl_reg2: *mut core::ffi::c_void,
    pub status_reg: *mut core::ffi::c_void,
    pub lock_shift: u8,
    pub lock_width: u8,
    pub max_rate: u64,
    pub min_rate: u64,
}

#[repr(C)]
pub union eswin_clk_info_data {
    pub div: eswin_divider_clock,
    pub factor: eswin_fixed_factor_clock,
    pub gate: eswin_gate_clock,
    pub mux: eswin_mux_clock,
}

#[repr(C)]
pub struct eswin_clk_info {
    pub type_: u32,
    pub pid: u32,
    pub id: u32,
    pub hw: clk_hw,
    pub data: eswin_clk_info_data,
}

extern "C" {
    pub fn eswin_clk_init(pdev: *mut platform_device, nr_clks: usize) -> *mut eswin_clock_data;
    pub fn eswin_clk_register_fixed_rate(dev: *mut device, clks: *mut eswin_fixed_rate_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_pll(dev: *mut device, clks: *mut eswin_pll_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_fixed_factor(dev: *mut device, clks: *mut eswin_fixed_factor_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_mux(dev: *mut device, clks: *mut eswin_mux_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_divider(dev: *mut device, clks: *mut eswin_divider_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_gate(dev: *mut device, clks: *mut eswin_gate_clock, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_clk_register_clks(dev: *mut device, clks: *mut eswin_clk_info, nums: i32, data: *mut eswin_clock_data) -> i32;
    pub fn eswin_register_clkdiv(dev: *mut device, id: u32, name: *const core::ffi::c_char, parent_hw: *const clk_hw, flags: usize, reg: *mut core::ffi::c_void, shift: u8, width: u8, clk_divider_flags: usize, priv_flag: usize, lock: *mut spinlock_t) -> *mut clk_hw;
}

macro_rules! ESWIN_DIV { ($id:expr, $name:expr, $pdata:expr, $flags:expr, $reg:expr, $shift:expr, $width:expr, $dflags:expr, $pflag:expr) => { eswin_divider_clock { hw: unsafe { core::mem::zeroed() }, id: $id, name: $name, parent_data: $pdata, ctrl_reg: core::ptr::null_mut(), flags: $flags, reg: $reg, shift: $shift, width: $width, div_flags: $dflags, priv_flag: $pflag, lock: core::ptr::null_mut() } }; }
macro_rules! ESWIN_FACTOR { ($id:expr, $name:expr, $pdata:expr, $mult:expr, $div:expr, $flags:expr) => { eswin_fixed_factor_clock { hw: unsafe { core::mem::zeroed() }, id: $id, name: $name, parent_data: $pdata, mult: $mult, div: $div, flags: $flags } }; }
macro_rules! ESWIN_FIXED { ($id:expr, $name:expr, $flags:expr, $rate:expr) => { eswin_fixed_rate_clock { hw: unsafe { core::mem::zeroed() }, id: $id, name: $name, flags: $flags, rate: $rate } }; }

/* The remaining initializer macros preserve the original C designated-field intent. */
macro_rules! ESWIN_DIV_TYPE { ($($args:tt)*) => { compile_error!("ESWIN_DIV_TYPE requires C-style aggregate initialization") }; }
macro_rules! ESWIN_FACTOR_TYPE { ($($args:tt)*) => { compile_error!("ESWIN_FACTOR_TYPE requires C-style aggregate initialization") }; }
macro_rules! ESWIN_GATE { ($($args:tt)*) => { compile_error!("ESWIN_GATE requires C-style aggregate initialization") }; }
macro_rules! ESWIN_GATE_TYPE { ($($args:tt)*) => { compile_error!("ESWIN_GATE_TYPE requires C-style aggregate initialization") }; }
macro_rules! ESWIN_MUX { ($($args:tt)*) => { compile_error!("ESWIN_MUX requires C-style aggregate initialization") }; }
macro_rules! ESWIN_MUX_TBL { ($($args:tt)*) => { compile_error!("ESWIN_MUX_TBL requires C-style aggregate initialization") }; }
macro_rules! ESWIN_MUX_TYPE { ($($args:tt)*) => { compile_error!("ESWIN_MUX_TYPE requires C-style aggregate initialization") }; }
macro_rules! ESWIN_PLL { ($($args:tt)*) => { compile_error!("ESWIN_PLL requires C-style aggregate initialization") }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
