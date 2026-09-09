/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Google, Inc.
 */

/* Dependency supplied by the Linux clock-provider environment. */

#[repr(C)]
pub struct pistachio_gate {
    pub id: ::core::ffi::c_uint,
    pub reg: ::core::ffi::c_ulong,
    pub shift: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent: *const ::core::ffi::c_char,
}

#[macro_export]
macro_rules! GATE {
    ($id:expr, $name:expr, $pname:expr, $reg:expr, $shift:expr) => {
        pistachio_gate { id: $id, reg: $reg, shift: $shift, name: $name, parent: $pname }
    };
}

#[repr(C)]
pub struct pistachio_mux {
    pub id: ::core::ffi::c_uint,
    pub reg: ::core::ffi::c_ulong,
    pub shift: ::core::ffi::c_uint,
    pub num_parents: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parents: *const *const ::core::ffi::c_char,
}

#[macro_export]
macro_rules! PNAME {
    ($name:ident) => {
        static $name: &[*const ::core::ffi::c_char] = &[];
    };
}

#[macro_export]
macro_rules! MUX {
    ($id:expr, $name:expr, $pnames:expr, $reg:expr, $shift:expr) => {
        pistachio_mux { id: $id, reg: $reg, shift: $shift, name: $name,
            parents: $pnames.as_ptr(), num_parents: $pnames.len() as ::core::ffi::c_uint }
    };
}

#[repr(C)]
pub struct pistachio_div {
    pub id: ::core::ffi::c_uint,
    pub reg: ::core::ffi::c_ulong,
    pub width: ::core::ffi::c_uint,
    pub div_flags: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent: *const ::core::ffi::c_char,
}

#[macro_export]
macro_rules! DIV {
    ($id:expr, $name:expr, $pname:expr, $reg:expr, $width:expr) => {
        pistachio_div { id: $id, reg: $reg, width: $width, div_flags: 0, name: $name, parent: $pname }
    };
}

#[macro_export]
macro_rules! DIV_F {
    ($id:expr, $name:expr, $pname:expr, $reg:expr, $width:expr, $div_flags:expr) => {
        pistachio_div { id: $id, reg: $reg, width: $width, div_flags: $div_flags, name: $name, parent: $pname }
    };
}

#[repr(C)]
pub struct pistachio_fixed_factor {
    pub id: ::core::ffi::c_uint,
    pub div: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent: *const ::core::ffi::c_char,
}

#[macro_export]
macro_rules! FIXED_FACTOR {
    ($id:expr, $name:expr, $pname:expr, $div:expr) => {
        pistachio_fixed_factor { id: $id, div: $div, name: $name, parent: $pname }
    };
}

#[repr(C)]
pub struct pistachio_pll_rate_table {
    pub fref: u64,
    pub fout: u64,
    pub refdiv: u64,
    pub fbdiv: u64,
    pub postdiv1: u64,
    pub postdiv2: u64,
    pub frac: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pistachio_pll_type {
    PLL_GF40LP_LAINT,
    PLL_GF40LP_FRAC,
}

#[repr(C)]
pub struct pistachio_pll {
    pub id: ::core::ffi::c_uint,
    pub reg_base: ::core::ffi::c_ulong,
    pub type_: pistachio_pll_type,
    pub rates: *mut pistachio_pll_rate_table,
    pub nr_rates: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent: *const ::core::ffi::c_char,
}

#[macro_export]
macro_rules! PLL {
    ($id:expr, $name:expr, $pname:expr, $type:expr, $reg:expr, $rates:expr) => {
        pistachio_pll { id: $id, reg_base: $reg, type_: $type, rates: $rates.as_mut_ptr(),
            nr_rates: $rates.len() as ::core::ffi::c_uint, name: $name, parent: $pname }
    };
}

#[macro_export]
macro_rules! PLL_FIXED {
    ($id:expr, $name:expr, $pname:expr, $type:expr, $reg:expr) => {
        pistachio_pll { id: $id, reg_base: $reg, type_: $type, rates: ::core::ptr::null_mut(),
            nr_rates: 0, name: $name, parent: $pname }
    };
}

#[repr(C)]
pub struct pistachio_clk_provider {
    pub node: *mut device_node,
    pub base: *mut ::core::ffi::c_void,
    pub clk_data: clk_onecell_data,
}

/* External Linux kernel types supplied by dependent headers. */
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_onecell_data { _private: [u8; 0] }

extern "C" {
    pub fn pistachio_clk_alloc_provider(node: *mut device_node, num_clks: ::core::ffi::c_uint) -> *mut pistachio_clk_provider;
    pub fn pistachio_clk_register_provider(p: *mut pistachio_clk_provider);
    pub fn pistachio_clk_register_gate(p: *mut pistachio_clk_provider, gate: *mut pistachio_gate, num: ::core::ffi::c_uint);
    pub fn pistachio_clk_register_mux(p: *mut pistachio_clk_provider, mux: *mut pistachio_mux, num: ::core::ffi::c_uint);
    pub fn pistachio_clk_register_div(p: *mut pistachio_clk_provider, div: *mut pistachio_div, num: ::core::ffi::c_uint);
    pub fn pistachio_clk_register_fixed_factor(p: *mut pistachio_clk_provider, ff: *mut pistachio_fixed_factor, num: ::core::ffi::c_uint);
    pub fn pistachio_clk_register_pll(p: *mut pistachio_clk_provider, pll: *mut pistachio_pll, num: ::core::ffi::c_uint);
    pub fn pistachio_clk_force_enable(p: *mut pistachio_clk_provider, clk_ids: *mut ::core::ffi::c_uint, num: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
