/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 MaxLinear, Inc.
 * Copyright (C) 2020 Intel Corporation.
 * Zhu Yixin <yzhu@maxlinear.com>
 * Rahul Tanwar <rtanwar@maxlinear.com>
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub struct lgm_clk_mux {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct lgm_clk_divider {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
    pub shift_gate: u8,
    pub width_gate: u8,
    pub flags: c_ulong,
    pub table: *const clk_div_table,
}

#[repr(C)]
pub struct lgm_clk_ddiv {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: u32,
    pub shift0: u8,
    pub width0: u8,
    pub shift1: u8,
    pub width1: u8,
    pub shift2: u8,
    pub width2: u8,
    pub shift_gate: u8,
    pub width_gate: u8,
    pub mult: u32,
    pub div: u32,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct lgm_clk_gate {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: u32,
    pub shift: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lgm_clk_type {
    CLK_TYPE_FIXED,
    CLK_TYPE_MUX,
    CLK_TYPE_DIVIDER,
    CLK_TYPE_FIXED_FACTOR,
    CLK_TYPE_GATE,
    CLK_TYPE_NONE,
}

#[repr(C)]
pub struct lgm_clk_provider {
    pub membase: *mut regmap,
    pub np: *mut device_node,
    pub dev: *mut device,
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pll_type { TYPE_ROPLL, TYPE_LJPLL, TYPE_NONE }

#[repr(C)]
pub struct lgm_clk_pll {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: u32,
    pub flags: c_ulong,
    pub type_: pll_type,
}

#[repr(C)]
pub struct lgm_pll_clk_data {
    pub id: u32,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub type_: pll_type,
    pub reg: i32,
}

#[repr(C)]
pub struct lgm_clk_ddiv_data {
    pub id: u32,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub flags: u8,
    pub div_flags: c_ulong,
    pub reg: u32,
    pub shift0: u8, pub width0: u8,
    pub shift1: u8, pub width1: u8,
    pub shift_gate: u8, pub width_gate: u8,
    pub ex_shift: u8, pub ex_width: u8,
}

#[repr(C)]
pub struct lgm_clk_branch {
    pub id: u32,
    pub type_: lgm_clk_type,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub mux_off: u32, pub mux_shift: u8, pub mux_width: u8,
    pub mux_flags: c_ulong, pub mux_val: u32,
    pub div_off: u32, pub div_shift: u8, pub div_width: u8,
    pub div_shift_gate: u8, pub div_width_gate: u8,
    pub div_flags: c_ulong, pub div_val: u32,
    pub div_table: *const clk_div_table,
    pub gate_off: u32, pub gate_shift: u8, pub gate_flags: c_ulong,
    pub gate_val: u32,
    pub mult: u32, pub div: u32,
}

pub const CLOCK_FLAG_VAL_INIT: c_ulong = 1 << 16;
pub const MUX_CLK_SW: c_ulong = 1 << 17;
pub const GATE_CLK_HW: c_ulong = 1 << 18;
pub const DIV_CLK_NO_MASK: c_ulong = 1 << 19;

// C initializer macros, retained as Rust macros for source-level use.
#[macro_export]
macro_rules! LGM_PLL { ($id:expr, $name:expr, $pdata:expr, $flags:expr, $reg:expr, $type_:expr) => { lgm_pll_clk_data { id:$id, name:$name, parent_data:$pdata, num_parents:$pdata.len() as u8, flags:$flags, reg:$reg, type_:$type_ } }; }
#[macro_export]
macro_rules! LGM_DDIV { ($id:expr,$name:expr,$pdata:expr,$flags:expr,$reg:expr,$s0:expr,$w0:expr,$s1:expr,$w1:expr,$sg:expr,$wg:expr,$xs:expr,$df:expr) => { lgm_clk_ddiv_data { id:$id,name:$name,parent_data:$pdata,flags:$flags,reg:$reg,shift0:$s0,width0:$w0,shift1:$s1,width1:$w1,shift_gate:$sg,width_gate:$wg,ex_shift:$xs,ex_width:1,div_flags:$df } }; }

#[macro_export]
macro_rules! LGM_MUX { ($id:expr,$name:expr,$pdata:expr,$f:expr,$reg:expr,$shift:expr,$width:expr,$cf:expr,$v:expr) => { lgm_clk_branch { id:$id,type_:lgm_clk_type::CLK_TYPE_MUX,name:$name,parent_data:$pdata,num_parents:$pdata.len() as u8,flags:$f,mux_off:$reg,mux_shift:$shift,mux_width:$width,mux_flags:$cf,mux_val:$v, ..unsafe { core::mem::zeroed() } } }; }

pub const fn lgm_set_clk_val(membase: *mut regmap, reg: u32, shift: u8, width: u8, set_val: u32) {
    let mask = (((1u32 << (width - 1)) * 2 - 1) << shift);
    unsafe { regmap_update_bits(membase, reg, mask, set_val << shift); }
}

pub unsafe fn lgm_get_clk_val(membase: *mut regmap, reg: u32, shift: u8, width: u8) -> u32 {
    let mask = (((1u32 << (width - 1)) * 2 - 1) << shift);
    let mut val = 0u32;
    if regmap_read(membase, reg, &mut val) != 0 { return 0; }
    (val & mask) >> shift
}

extern "C" {
    pub fn lgm_clk_register_branches(ctx: *mut lgm_clk_provider, list: *const lgm_clk_branch, nr_clk: u32) -> i32;
    pub fn lgm_clk_register_plls(ctx: *mut lgm_clk_provider, list: *const lgm_pll_clk_data, nr_clk: u32) -> i32;
    pub fn lgm_clk_register_ddiv(ctx: *mut lgm_clk_provider, list: *const lgm_clk_ddiv_data, nr_clk: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
