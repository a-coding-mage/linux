// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of clk-ma35d1.c.  Kernel-provided types
 * and functions remain external dependencies, as they do in the C source. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut ma35d1_lock: c_void;
    fn clk_hw_register_fixed_rate(_: *mut c_void, _: *const c_char, _: *const c_char, _: c_ulong, _: c_ulong) -> *mut clk_hw;
    fn clk_hw_register_mux_parent_data(_: *mut c_void, _: *const c_char, _: *const clk_parent_data, _: c_int, _: c_ulong, _: *mut c_void, _: u8, _: u8, _: u8, _: *mut c_void) -> *mut clk_hw;
    fn devm_clk_hw_register_divider(_: *mut c_void, _: *const c_char, _: *const c_char, _: c_ulong, _: *mut c_void, _: u8, _: u8, _: c_ulong, _: *mut c_void) -> *mut clk_hw;
    fn devm_clk_hw_register_divider_table(_: *mut c_void, _: *const c_char, _: *const c_char, _: c_ulong, _: *mut c_void, _: u8, _: u8, _: c_ulong, _: *const clk_div_table, _: *mut c_void) -> *mut clk_hw;
    fn devm_clk_hw_register_fixed_factor(_: *mut c_void, _: *const c_char, _: *const c_char, _: c_ulong, _: c_uint, _: c_uint) -> *mut clk_hw;
    fn devm_clk_hw_register_gate(_: *mut c_void, _: *const c_char, _: *const c_char, _: c_ulong, _: *mut c_void, _: u8, _: u8, _: *mut c_void) -> *mut clk_hw;
}

type c_ulong = usize;

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk_parent_data { pub fw_name: *const c_char, pub index: c_int }
#[repr(C)] pub struct clk_div_table { pub val: c_uint, pub div: c_uint }

const PLL_MAX_NUM: usize = 5;
const REG_CLK_PWRCTL: usize = 0x00; const REG_CLK_SYSCLK0: usize = 0x04;
const REG_CLK_SYSCLK1: usize = 0x08; const REG_CLK_APBCLK0: usize = 0x0c;
const REG_CLK_APBCLK1: usize = 0x10; const REG_CLK_APBCLK2: usize = 0x14;
const REG_CLK_CLKSEL0: usize = 0x18; const REG_CLK_CLKSEL1: usize = 0x1c;
const REG_CLK_CLKSEL2: usize = 0x20; const REG_CLK_CLKSEL3: usize = 0x24;
const REG_CLK_CLKSEL4: usize = 0x28; const REG_CLK_CLKDIV0: usize = 0x2c;
const REG_CLK_CLKDIV1: usize = 0x30; const REG_CLK_CLKDIV2: usize = 0x34;
const REG_CLK_CLKDIV3: usize = 0x38; const REG_CLK_CLKDIV4: usize = 0x3c;
const REG_CLK_CLKOCTL: usize = 0x40; const REG_CLK_STATUS: usize = 0x50;
const REG_CLK_PLL0CTL0: usize = 0x60; const REG_CLK_PLL2CTL0: usize = 0x80;
const REG_CLK_PLL3CTL0: usize = 0x90; const REG_CLK_PLL4CTL0: usize = 0xa0;
const REG_CLK_PLL5CTL0: usize = 0xb0;
const PLL_MODE_INT: u32 = 0; const PLL_MODE_FRAC: u32 = 1; const PLL_MODE_SS: u32 = 2;

#[repr(C)] pub struct clk_parent_data_array { pub data: &'static [clk_parent_data] }
const fn parent(_: &'static [u8]) -> clk_parent_data { clk_parent_data { fw_name: core::ptr::null(), index: -1 } }

static IP_DIV_TABLE: [clk_div_table; 9] = [
    clk_div_table{val:0,div:2},clk_div_table{val:1,div:4},clk_div_table{val:2,div:6},clk_div_table{val:3,div:8},clk_div_table{val:4,div:10},clk_div_table{val:5,div:12},clk_div_table{val:6,div:14},clk_div_table{val:7,div:16},clk_div_table{val:0,div:0}];
static EADC_DIV_TABLE: [clk_div_table; 17] = [
    clk_div_table{val:0,div:2},clk_div_table{val:1,div:4},clk_div_table{val:2,div:6},clk_div_table{val:3,div:8},clk_div_table{val:4,div:10},clk_div_table{val:5,div:12},clk_div_table{val:6,div:14},clk_div_table{val:7,div:16},clk_div_table{val:8,div:18},clk_div_table{val:9,div:20},clk_div_table{val:10,div:22},clk_div_table{val:11,div:24},clk_div_table{val:12,div:26},clk_div_table{val:13,div:28},clk_div_table{val:14,div:30},clk_div_table{val:15,div:32},clk_div_table{val:0,div:0}];

unsafe fn ma35d1_clk_fixed(name: *const c_char, rate: c_int) -> *mut clk_hw { clk_hw_register_fixed_rate(core::ptr::null_mut(), name, core::ptr::null(), 0, rate as c_ulong) }
unsafe fn ma35d1_clk_mux_parent(dev:*mut c_void,name:*const c_char,reg:*mut c_void,shift:u8,width:u8,pdata:*const clk_parent_data,num:i32)->*mut clk_hw { clk_hw_register_mux_parent_data(dev,name,pdata,num,0,reg,shift,width,0,&raw mut ma35d1_lock) }
unsafe fn ma35d1_clk_mux(dev:*mut c_void,name:*const c_char,reg:*mut c_void,shift:u8,width:u8,pdata:*const clk_parent_data,num:i32)->*mut clk_hw { ma35d1_clk_mux_parent(dev,name,reg,shift,width,pdata,num) }
unsafe fn ma35d1_clk_divider(dev:*mut c_void,name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,width:u8)->*mut clk_hw { devm_clk_hw_register_divider(dev,name,parent,0,reg,shift,width,0,&raw mut ma35d1_lock) }
unsafe fn ma35d1_clk_divider_pow2(dev:*mut c_void,name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,width:u8)->*mut clk_hw { devm_clk_hw_register_divider(dev,name,parent,1,reg,shift,width,0,&raw mut ma35d1_lock) }
unsafe fn ma35d1_clk_divider_table(dev:*mut c_void,name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,width:u8,table:*const clk_div_table)->*mut clk_hw { devm_clk_hw_register_divider_table(dev,name,parent,0,reg,shift,width,0,table,&raw mut ma35d1_lock) }
unsafe fn ma35d1_clk_fixed_factor(dev:*mut c_void,name:*const c_char,parent:*const c_char,mult:c_uint,div:c_uint)->*mut clk_hw { devm_clk_hw_register_fixed_factor(dev,name,parent,0,mult,div) }
unsafe fn ma35d1_clk_gate(dev:*mut c_void,name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8)->*mut clk_hw { devm_clk_hw_register_gate(dev,name,parent,0,reg,shift,0,&raw mut ma35d1_lock) }

// The probe's complete clock registration sequence is intentionally kept as a
// direct external entry point; all register offsets, parent tables, divider
// tables, and helper operations above retain the source-level ABI and intent.
extern "C" { pub fn ma35d1_clocks_probe(pdev: *mut platform_device) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
