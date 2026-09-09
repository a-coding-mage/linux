/* SPDX-License-Identifier:    GPL-2.0 */
/*
 * Copyright (C) 2017, Intel Corporation
 */

use core::ffi::{c_char, c_ulong, c_void};

/* External kernel types supplied by other dependencies. */
#[repr(C)]
pub struct clk_hw_onecell_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_parent_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stratix10_clock_data {
    pub base: *mut c_void,
    /* Must be last */
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
pub struct stratix10_pll_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct stratix10_perip_c_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct n5x_perip_c_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: c_ulong,
}

#[repr(C)]
pub struct stratix10_perip_cnt_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub fixed_divider: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: c_ulong,
}

#[repr(C)]
pub struct stratix10_gate_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub gate_reg: c_ulong,
    pub gate_idx: u8,
    pub div_reg: c_ulong,
    pub div_offset: u8,
    pub div_width: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: u8,
    pub fixed_div: u8,
}

#[repr(C)]
pub struct agilex5_pll_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct agilex5_perip_cnt_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub fixed_divider: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: c_ulong,
}

#[repr(C)]
pub struct agilex5_gate_clock {
    pub id: ::core::ffi::c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub gate_reg: c_ulong,
    pub gate_idx: u8,
    pub div_reg: c_ulong,
    pub div_offset: u8,
    pub div_width: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: u8,
    pub fixed_div: u8,
}

extern "C" {
    pub fn s10_register_pll(clks: *const stratix10_pll_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn agilex_register_pll(clks: *const stratix10_pll_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn n5x_register_pll(clks: *const stratix10_pll_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn agilex5_register_pll(clks: *const agilex5_pll_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn agilex5_register_cnt_periph(clks: *const agilex5_perip_cnt_clock, regbase: *mut c_void) -> *mut clk_hw;
    pub fn agilex5_register_gate(clks: *const agilex5_gate_clock, regbase: *mut c_void) -> *mut clk_hw;
    pub fn s10_register_periph(clks: *const stratix10_perip_c_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn n5x_register_periph(clks: *const n5x_perip_c_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn s10_register_cnt_periph(clks: *const stratix10_perip_cnt_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn s10_register_gate(clks: *const stratix10_gate_clock, reg: *mut c_void) -> *mut clk_hw;
    pub fn agilex_register_gate(clks: *const stratix10_gate_clock, reg: *mut c_void) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
