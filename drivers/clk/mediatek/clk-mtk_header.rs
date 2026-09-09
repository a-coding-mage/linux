/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const MAX_MUX_GATE_BIT: i32 = 31;
pub const INVALID_MUX_GATE_BIT: i32 = MAX_MUX_GATE_BIT + 1;
pub const MHZ: i32 = 1000 * 1000;
pub const MTK_WAIT_HWV_DONE_US: i32 = 30;

pub const CLK_DUMMY: i32 = 0;

extern "C" {
    pub static mtk_clk_dummy_ops: clk_ops;
    pub static cg_regs_dummy: mtk_gate_regs;
}

#[repr(C)]
pub struct mtk_fixed_clk {
    pub id: i32,
    pub name: *const ::std::os::raw::c_char,
    pub parent: *const ::std::os::raw::c_char,
    pub rate: ::std::os::raw::c_ulong,
}

#[macro_export]
macro_rules! GATE_DUMMY {
    ($id:expr, $name:expr) => { mtk_gate { id: $id, name: $name, regs: unsafe { &cg_regs_dummy }, ops: unsafe { &mtk_clk_dummy_ops } } };
}

#[macro_export]
macro_rules! FIXED_CLK {
    ($id:expr, $name:expr, $parent:expr, $rate:expr) => { mtk_fixed_clk { id: $id, name: $name, parent: $parent, rate: $rate } };
}

extern "C" {
    pub fn mtk_clk_register_fixed_clks(clks: *const mtk_fixed_clk, num: i32, clk_data: *mut clk_hw_onecell_data) -> i32;
    pub fn mtk_clk_unregister_fixed_clks(clks: *const mtk_fixed_clk, num: i32, clk_data: *mut clk_hw_onecell_data);
}

#[repr(C)]
pub struct mtk_fixed_factor {
    pub id: i32,
    pub name: *const ::std::os::raw::c_char,
    pub parent_name: *const ::std::os::raw::c_char,
    pub mult: i32,
    pub div: i32,
    pub flags: ::std::os::raw::c_ulong,
}

#[macro_export]
macro_rules! FACTOR_FLAGS {
    ($id:expr, $name:expr, $parent:expr, $mult:expr, $div:expr, $fl:expr) => { mtk_fixed_factor { id: $id, name: $name, parent_name: $parent, mult: $mult, div: $div, flags: $fl } };
}

#[macro_export]
macro_rules! FACTOR {
    ($id:expr, $name:expr, $parent:expr, $mult:expr, $div:expr) => { FACTOR_FLAGS!($id, $name, $parent, $mult, $div, CLK_SET_RATE_PARENT) };
}

extern "C" {
    pub fn mtk_clk_register_factors(clks: *const mtk_fixed_factor, num: i32, clk_data: *mut clk_hw_onecell_data) -> i32;
    pub fn mtk_clk_unregister_factors(clks: *const mtk_fixed_factor, num: i32, clk_data: *mut clk_hw_onecell_data);
}

#[repr(C)]
pub struct mtk_composite {
    pub id: i32,
    pub name: *const ::std::os::raw::c_char,
    pub parent_names: *const *const ::std::os::raw::c_char,
    pub parent: *const ::std::os::raw::c_char,
    pub flags: u32,
    pub mux_reg: u32,
    pub divider_reg: u32,
    pub gate_reg: u32,
    pub mux_shift: i8,
    pub mux_width: i8,
    pub gate_shift: i8,
    pub divider_shift: i8,
    pub divider_width: i8,
    pub mux_flags: u8,
    pub num_parents: i8,
}

#[macro_export]
macro_rules! MUX_GATE_FLAGS_2 {
    ($id:expr, $name:expr, $parents:expr, $reg:expr, $shift:expr, $width:expr, $gate:expr, $flags:expr, $muxflags:expr) => { mtk_composite { id: $id, name: $name, mux_reg: $reg, mux_shift: $shift, mux_width: $width, gate_reg: $reg, gate_shift: $gate, divider_shift: -1, parent_names: $parents.as_ptr(), parent: ::std::ptr::null(), divider_reg: 0, divider_width: 0, num_parents: $parents.len() as i8, flags: $flags, mux_flags: $muxflags } };
}

#[macro_export]
macro_rules! MUX_GATE_FLAGS { ($($args:tt)*) => { MUX_GATE_FLAGS_2!($($args)*, 0) }; }
#[macro_export]
macro_rules! MUX_GATE { ($($args:tt)*) => { MUX_GATE_FLAGS!($($args)*, CLK_SET_RATE_PARENT) }; }
#[macro_export]
macro_rules! MUX { ($id:expr, $name:expr, $parents:expr, $reg:expr, $shift:expr, $width:expr) => { MUX_FLAGS!($id, $name, $parents, $reg, $shift, $width, CLK_SET_RATE_PARENT) }; }
#[macro_export]
macro_rules! MUX_FLAGS { ($id:expr, $name:expr, $parents:expr, $reg:expr, $shift:expr, $width:expr, $flags:expr) => { mtk_composite { id: $id, name: $name, mux_reg: $reg, mux_shift: $shift, mux_width: $width, gate_shift: -1, divider_shift: -1, parent_names: $parents.as_ptr(), parent: ::std::ptr::null(), divider_reg: 0, divider_width: 0, gate_reg: 0, num_parents: $parents.len() as i8, flags: $flags, mux_flags: 0 } }; }

#[macro_export]
macro_rules! DIV_GATE { ($id:expr, $name:expr, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_width:expr, $div_shift:expr) => { mtk_composite { id: $id, parent: $parent, name: $name, divider_reg: $div_reg, divider_shift: $div_shift, divider_width: $div_width, gate_reg: $gate_reg, gate_shift: $gate_shift, mux_shift: -1, flags: 0, parent_names: ::std::ptr::null(), mux_reg: 0, mux_width: 0, num_parents: 0, mux_flags: 0 } }; }

#[macro_export]
macro_rules! MUX_DIV_GATE {
    ($id:expr, $name:expr, $parents:expr, $mux_reg:expr, $mux_shift:expr, $mux_width:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $gate_reg:expr, $gate_shift:expr) => { mtk_composite { id: $id, name: $name, parent_names: $parents.as_ptr(), num_parents: $parents.len() as i8, mux_reg: $mux_reg, mux_shift: $mux_shift, mux_width: $mux_width, divider_reg: $div_reg, divider_shift: $div_shift, divider_width: $div_width, gate_reg: $gate_reg, gate_shift: $gate_shift, flags: CLK_SET_RATE_PARENT, parent: ::std::ptr::null(), mux_flags: 0 } };
}

extern "C" {
    pub fn mtk_clk_register_composites(dev: *mut device, mcs: *const mtk_composite, num: i32, base: *mut ::std::ffi::c_void, lock: *mut spinlock_t, clk_data: *mut clk_hw_onecell_data) -> i32;
    pub fn mtk_clk_unregister_composites(mcs: *const mtk_composite, num: i32, clk_data: *mut clk_hw_onecell_data);
}

#[repr(C)]
pub struct mtk_clk_divider {
    pub id: i32,
    pub name: *const ::std::os::raw::c_char,
    pub parent_name: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_ulong,
    pub div_reg: u32,
    pub div_shift: u8,
    pub div_width: u8,
    pub clk_divider_flags: u8,
    pub clk_div_table: *const clk_div_table,
}

#[macro_export]
macro_rules! DIV_ADJ { ($id:expr, $name:expr, $parent:expr, $reg:expr, $shift:expr, $width:expr) => { mtk_clk_divider { id: $id, name: $name, parent_name: $parent, div_reg: $reg, div_shift: $shift, div_width: $width, flags: 0, clk_divider_flags: 0, clk_div_table: ::std::ptr::null() } }; }

extern "C" {
    pub fn mtk_clk_register_dividers(dev: *mut device, mcds: *const mtk_clk_divider, num: i32, base: *mut ::std::ffi::c_void, lock: *mut spinlock_t, clk_data: *mut clk_hw_onecell_data) -> i32;
    pub fn mtk_clk_unregister_dividers(mcds: *const mtk_clk_divider, num: i32, clk_data: *mut clk_hw_onecell_data);
    pub fn mtk_alloc_clk_data(clk_num: u32) -> *mut clk_hw_onecell_data;
    pub fn mtk_devm_alloc_clk_data(dev: *mut device, clk_num: u32) -> *mut clk_hw_onecell_data;
    pub fn mtk_free_clk_data(clk_data: *mut clk_hw_onecell_data);
    pub fn mtk_clk_register_ref2usb_tx(name: *const ::std::os::raw::c_char, parent_name: *const ::std::os::raw::c_char, reg: *mut ::std::ffi::c_void) -> *mut clk_hw;
    pub fn mtk_clk_unregister_ref2usb_tx(hw: *mut clk_hw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
