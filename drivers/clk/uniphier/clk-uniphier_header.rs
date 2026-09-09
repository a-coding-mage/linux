/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// C header guard: __CLK_UNIPHIER_H__

pub struct clk_hw;
pub struct device;
pub struct regmap;

pub const UNIPHIER_CLK_CPUGEAR_MAX_PARENTS: usize = 16;
pub const UNIPHIER_CLK_MUX_MAX_PARENTS: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum uniphier_clk_type {
    UNIPHIER_CLK_TYPE_CPUGEAR,
    UNIPHIER_CLK_TYPE_FIXED_FACTOR,
    UNIPHIER_CLK_TYPE_FIXED_RATE,
    UNIPHIER_CLK_TYPE_GATE,
    UNIPHIER_CLK_TYPE_MUX,
}

#[repr(C)]
pub struct uniphier_clk_cpugear_data {
    pub parent_names: [*const ::std::os::raw::c_char; UNIPHIER_CLK_CPUGEAR_MAX_PARENTS],
    pub num_parents: ::std::os::raw::c_uint,
    pub regbase: ::std::os::raw::c_uint,
    pub mask: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct uniphier_clk_fixed_factor_data {
    pub parent_name: *const ::std::os::raw::c_char,
    pub mult: ::std::os::raw::c_uint,
    pub div: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct uniphier_clk_fixed_rate_data {
    pub fixed_rate: ::std::os::raw::c_ulong,
}

#[repr(C)]
pub struct uniphier_clk_gate_data {
    pub parent_name: *const ::std::os::raw::c_char,
    pub reg: ::std::os::raw::c_uint,
    pub bit: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct uniphier_clk_mux_data {
    pub parent_names: [*const ::std::os::raw::c_char; UNIPHIER_CLK_MUX_MAX_PARENTS],
    pub num_parents: ::std::os::raw::c_uint,
    pub reg: ::std::os::raw::c_uint,
    pub masks: [::std::os::raw::c_uint; UNIPHIER_CLK_MUX_MAX_PARENTS],
    pub vals: [::std::os::raw::c_uint; UNIPHIER_CLK_MUX_MAX_PARENTS],
}

#[repr(C)]
pub union uniphier_clk_data_union {
    pub cpugear: ::std::mem::ManuallyDrop<uniphier_clk_cpugear_data>,
    pub factor: ::std::mem::ManuallyDrop<uniphier_clk_fixed_factor_data>,
    pub rate: ::std::mem::ManuallyDrop<uniphier_clk_fixed_rate_data>,
    pub gate: ::std::mem::ManuallyDrop<uniphier_clk_gate_data>,
    pub mux: ::std::mem::ManuallyDrop<uniphier_clk_mux_data>,
}

#[repr(C)]
pub struct uniphier_clk_data {
    pub name: *const ::std::os::raw::c_char,
    pub type_: uniphier_clk_type,
    pub idx: ::std::os::raw::c_int,
    pub data: uniphier_clk_data_union,
}

#[macro_export]
macro_rules! UNIPHIER_CLK_CPUGEAR {
    ($name:expr, $idx:expr, $regbase:expr, $mask:expr, $num_parents:expr, $($parent:expr),* $(,)?) => {
        uniphier_clk_data { name: $name, type_: uniphier_clk_type::UNIPHIER_CLK_TYPE_CPUGEAR, idx: $idx,
            data: uniphier_clk_data_union { cpugear: ::std::mem::ManuallyDrop::new(uniphier_clk_cpugear_data {
                parent_names: [$($parent),*], num_parents: $num_parents, regbase: $regbase, mask: $mask }) } }
    };
}

#[macro_export]
macro_rules! UNIPHIER_CLK_FACTOR {
    ($name:expr, $idx:expr, $parent:expr, $mult:expr, $div:expr) => {
        uniphier_clk_data { name: $name, type_: uniphier_clk_type::UNIPHIER_CLK_TYPE_FIXED_FACTOR, idx: $idx,
            data: uniphier_clk_data_union { factor: ::std::mem::ManuallyDrop::new(uniphier_clk_fixed_factor_data {
                parent_name: $parent, mult: $mult, div: $div }) } }
    };
}

#[macro_export]
macro_rules! UNIPHIER_CLK_GATE {
    ($name:expr, $idx:expr, $parent:expr, $reg:expr, $bit:expr) => {
        uniphier_clk_data { name: $name, type_: uniphier_clk_type::UNIPHIER_CLK_TYPE_GATE, idx: $idx,
            data: uniphier_clk_data_union { gate: ::std::mem::ManuallyDrop::new(uniphier_clk_gate_data {
                parent_name: $parent, reg: $reg, bit: $bit }) } }
    };
}

#[macro_export]
macro_rules! UNIPHIER_CLK_DIV {
    ($parent:expr, $div:expr) => { UNIPHIER_CLK_FACTOR!(concat!($parent, "/", stringify!($div)), -1, $parent, 1, $div) };
}
#[macro_export]
macro_rules! UNIPHIER_CLK_DIV2 { ($parent:expr, $div0:expr, $div1:expr) => { UNIPHIER_CLK_DIV!($parent, $div0), UNIPHIER_CLK_DIV!($parent, $div1) }; }
#[macro_export]
macro_rules! UNIPHIER_CLK_DIV3 { ($parent:expr, $div0:expr, $div1:expr, $div2:expr) => { UNIPHIER_CLK_DIV2!($parent, $div0, $div1), UNIPHIER_CLK_DIV!($parent, $div2) }; }
#[macro_export]
macro_rules! UNIPHIER_CLK_DIV4 { ($parent:expr, $div0:expr, $div1:expr, $div2:expr, $div3:expr) => { UNIPHIER_CLK_DIV2!($parent, $div0, $div1), UNIPHIER_CLK_DIV2!($parent, $div2, $div3) }; }
#[macro_export]
macro_rules! UNIPHIER_CLK_DIV5 { ($parent:expr, $div0:expr, $div1:expr, $div2:expr, $div3:expr, $div4:expr) => { UNIPHIER_CLK_DIV4!($parent, $div0, $div1, $div2, $div3), UNIPHIER_CLK_DIV!($parent, $div4) }; }

extern "C" {
    pub fn uniphier_clk_register_cpugear(dev: *mut device, regmap: *mut regmap, name: *const ::std::os::raw::c_char, data: *const uniphier_clk_cpugear_data) -> *mut clk_hw;
    pub fn uniphier_clk_register_fixed_factor(dev: *mut device, name: *const ::std::os::raw::c_char, data: *const uniphier_clk_fixed_factor_data) -> *mut clk_hw;
    pub fn uniphier_clk_register_fixed_rate(dev: *mut device, name: *const ::std::os::raw::c_char, data: *const uniphier_clk_fixed_rate_data) -> *mut clk_hw;
    pub fn uniphier_clk_register_gate(dev: *mut device, regmap: *mut regmap, name: *const ::std::os::raw::c_char, data: *const uniphier_clk_gate_data) -> *mut clk_hw;
    pub fn uniphier_clk_register_mux(dev: *mut device, regmap: *mut regmap, name: *const ::std::os::raw::c_char, data: *const uniphier_clk_mux_data) -> *mut clk_hw;

    pub static uniphier_ld4_sys_clk_data: uniphier_clk_data;
    pub static uniphier_pro4_sys_clk_data: uniphier_clk_data;
    pub static uniphier_sld8_sys_clk_data: uniphier_clk_data;
    pub static uniphier_pro5_sys_clk_data: uniphier_clk_data;
    pub static uniphier_pxs2_sys_clk_data: uniphier_clk_data;
    pub static uniphier_ld11_sys_clk_data: uniphier_clk_data;
    pub static uniphier_ld20_sys_clk_data: uniphier_clk_data;
    pub static uniphier_pxs3_sys_clk_data: uniphier_clk_data;
    pub static uniphier_nx1_sys_clk_data: uniphier_clk_data;
    pub static uniphier_ld4_mio_clk_data: uniphier_clk_data;
    pub static uniphier_pro5_sd_clk_data: uniphier_clk_data;
    pub static uniphier_ld4_peri_clk_data: uniphier_clk_data;
    pub static uniphier_pro4_peri_clk_data: uniphier_clk_data;
    pub static uniphier_pro4_sg_clk_data: uniphier_clk_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
