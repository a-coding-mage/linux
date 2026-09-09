// SPDX-License-Identifier: GPL-2.0-only
/* TI CDCE706 programmable 3-PLL clock synthesizer driver. */

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::{c_char, c_void};

const CDCE706_CLKIN_CLOCK: u32 = 10;
const CDCE706_CLKIN_SOURCE: u32 = 11;
const CDCE706_PLL_M_LOW: fn(u32) -> u32 = |pll| 1 + 3 * pll;
const CDCE706_PLL_N_LOW: fn(u32) -> u32 = |pll| 2 + 3 * pll;
const CDCE706_PLL_HI: fn(u32) -> u32 = |pll| 3 + 3 * pll;
const CDCE706_PLL_MUX: u32 = 3;
const CDCE706_PLL_FVCO: u32 = 6;
const CDCE706_DIVIDER: fn(u32) -> u32 = |div| 13 + div;
const CDCE706_CLKOUT: fn(u32) -> u32 = |out| 19 + out;
const CDCE706_CLKIN_CLOCK_MASK: u32 = 0x10;
const CDCE706_CLKIN_SOURCE_SHIFT: u32 = 6;
const CDCE706_CLKIN_SOURCE_MASK: u32 = 0xc0;
const CDCE706_CLKIN_SOURCE_LVCMOS: u32 = 0x40;
const CDCE706_PLL_MUX_MASK: fn(u32) -> u32 = |pll| 0x80 >> pll;
const CDCE706_PLL_LOW_M_MASK: u32 = 0xff;
const CDCE706_PLL_LOW_N_MASK: u32 = 0xff;
const CDCE706_PLL_HI_M_MASK: u32 = 0x1;
const CDCE706_PLL_HI_N_MASK: u32 = 0x1e;
const CDCE706_PLL_HI_N_SHIFT: u32 = 1;
const CDCE706_PLL_M_MAX: u32 = 0x1ff;
const CDCE706_PLL_N_MAX: u32 = 0xfff;
const CDCE706_PLL_FVCO_MASK: fn(u32) -> u32 = |pll| 0x80 >> pll;
const CDCE706_PLL_FREQ_MIN: u64 = 80000000;
const CDCE706_PLL_FREQ_MAX: u64 = 300000000;
const CDCE706_PLL_FREQ_HI: u64 = 180000000;
const CDCE706_DIVIDER_PLL: fn(u32) -> u32 = |div| 9 + div - (div > 2) as u32 - (div > 4) as u32;
const CDCE706_DIVIDER_PLL_SHIFT: fn(u32) -> u32 = |div| if div < 2 { 5 } else { 3 * (div & 1) };
const CDCE706_DIVIDER_PLL_MASK: fn(u32) -> u32 = |div| 0x7 << CDCE706_DIVIDER_PLL_SHIFT(div);
const CDCE706_DIVIDER_DIVIDER_MASK: u32 = 0x7f;
const CDCE706_DIVIDER_DIVIDER_MAX: u32 = 0x7f;
const CDCE706_CLKOUT_DIVIDER_MASK: u32 = 0x7;
const CDCE706_CLKOUT_ENABLE_MASK: u32 = 0x8;

#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub val_bits: u32, pub val_format_endian: u32 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub adapter: *mut i2c_adapter }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct clk_init_data { pub ops: *const clk_ops, pub parent_names: *const *const c_char, pub num_parents: usize, pub flags: u32, pub name: *const c_char }
#[repr(C)] pub struct clk_rate_request { pub rate: u64, pub best_parent_rate: u64 }
#[repr(C)] pub struct clk_ops { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 1] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { pub name: *const c_char }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }

#[repr(C)] struct cdce706_hw_data { dev_data: *mut cdce706_dev_data, idx: u32, parent: u32, hw: clk_hw, div: u32, mul: u32, mux: u32 }
#[repr(C)] struct cdce706_dev_data { client: *mut i2c_client, regmap: *mut regmap, clkin_clk: [*mut clk; 2], clkin_name: [*const c_char; 2], clkin: [cdce706_hw_data; 1], pll: [cdce706_hw_data; 3], divider: [cdce706_hw_data; 6], clkout: [cdce706_hw_data; 6] }

extern "C" {
    fn regmap_read(r: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(r: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(r: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn rational_best_approximation(a: u64, b: u64, max_a: u32, max_b: u32, best_a: *mut u64, best_b: *mut u64);
}

static CDCE706_SOURCE_NAME: [&[u8]; 2] = [b"clk_in0\0", b"clk_in1\0"];
static CDCE706_CLKIN_NAME: [&[u8]; 1] = [b"clk_in\0"];
static CDCE706_PLL_NAME: [&[u8]; 3] = [b"pll1\0", b"pll2\0", b"pll3\0"];
static CDCE706_DIVIDER_PARENT_NAME: [&[u8]; 5] = [b"clk_in\0", b"pll1\0", b"pll2\0", b"pll2\0", b"pll3\0"];
static CDCE706_DIVIDER_NAME: [&[u8]; 6] = [b"p0\0", b"p1\0", b"p2\0", b"p3\0", b"p4\0", b"p5\0"];
static CDCE706_CLKOUT_NAME: [&[u8]; 6] = [b"clk_out0\0", b"clk_out1\0", b"clk_out2\0", b"clk_out3\0", b"clk_out4\0", b"clk_out5\0"];

unsafe fn cdce706_reg_read(d: *mut cdce706_dev_data, reg: u32, val: *mut u32) -> i32 { regmap_read((*d).regmap, reg | 0x80, val) }
unsafe fn cdce706_reg_write(d: *mut cdce706_dev_data, reg: u32, val: u32) -> i32 { regmap_write((*d).regmap, reg | 0x80, val) }
unsafe fn cdce706_reg_update(d: *mut cdce706_dev_data, reg: u32, mask: u32, val: u32) -> i32 { regmap_update_bits((*d).regmap, reg | 0x80, mask, val) }

unsafe fn cdce706_clkin_set_parent(hw: *mut clk_hw, index: u8) -> i32 { let h = hw as *mut cdce706_hw_data; (*h).parent = index as u32; 0 }
unsafe fn cdce706_clkin_get_parent(hw: *mut clk_hw) -> u8 { (*(hw as *mut cdce706_hw_data)).parent as u8 }
unsafe fn cdce706_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 { let h=hw as *mut cdce706_hw_data; if (*h).mux==0 && (*h).div!=0 && (*h).mul!=0 { parent_rate.wrapping_mul((*h).mul as u64)/(*h).div as u64 } else if (*h).mux!=0 && (*h).div!=0 { parent_rate/(*h).div as u64 } else { 0 } }
unsafe fn cdce706_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let h=hw as *mut cdce706_hw_data; let mut m=0u64; let mut d=0u64; rational_best_approximation((*req).rate,(*req).best_parent_rate,CDCE706_PLL_N_MAX,CDCE706_PLL_M_MAX,&mut m,&mut d); (*h).mul=m as u32; (*h).div=d as u32; (*req).rate=(*req).best_parent_rate.wrapping_mul(m)/d; 0 }
unsafe fn cdce706_pll_set_rate(hw:*mut clk_hw, rate:u64, _parent_rate:u64)->i32 { let h=hw as *mut cdce706_hw_data; let e=cdce706_reg_update((*h).dev_data,CDCE706_PLL_HI((*h).idx),CDCE706_PLL_HI_M_MASK|CDCE706_PLL_HI_N_MASK,(((*h).div>>8)&CDCE706_PLL_HI_M_MASK)|(((*h).mul>>(8-CDCE706_PLL_HI_N_SHIFT))&CDCE706_PLL_HI_N_MASK)); if e<0{return e}; let e=cdce706_reg_write((*h).dev_data,CDCE706_PLL_M_LOW((*h).idx),(*h).div&CDCE706_PLL_LOW_M_MASK); if e<0{return e}; let e=cdce706_reg_write((*h).dev_data,CDCE706_PLL_N_LOW((*h).idx),(*h).mul&CDCE706_PLL_LOW_N_MASK); if e<0{return e}; cdce706_reg_update((*h).dev_data,CDCE706_PLL_FVCO,CDCE706_PLL_FVCO_MASK((*h).idx),if rate>CDCE706_PLL_FREQ_HI{CDCE706_PLL_FVCO_MASK((*h).idx)}else{0}) }
unsafe fn cdce706_divider_set_parent(hw:*mut clk_hw,index:u8)->i32 { let h=hw as *mut cdce706_hw_data; if (*h).parent==index as u32{return 0}; (*h).parent=index as u32; cdce706_reg_update((*h).dev_data,CDCE706_DIVIDER_PLL((*h).idx),CDCE706_DIVIDER_PLL_MASK((*h).idx),(index as u32)<<CDCE706_DIVIDER_PLL_SHIFT((*h).idx)) }
unsafe fn cdce706_divider_get_parent(hw:*mut clk_hw)->u8{(*(hw as *mut cdce706_hw_data)).parent as u8}
unsafe fn cdce706_divider_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64{let h=hw as *mut cdce706_hw_data;if (*h).div!=0{parent_rate/(*h).div as u64}else{0}}
unsafe fn cdce706_divider_set_rate(hw:*mut clk_hw,_rate:u64,_parent_rate:u64)->i32{let h=hw as *mut cdce706_hw_data;cdce706_reg_update((*h).dev_data,CDCE706_DIVIDER((*h).idx),CDCE706_DIVIDER_DIVIDER_MASK,(*h).div)}
unsafe fn cdce706_clkout_prepare(hw:*mut clk_hw)->i32{let h=hw as *mut cdce706_hw_data;cdce706_reg_update((*h).dev_data,CDCE706_CLKOUT((*h).idx),CDCE706_CLKOUT_ENABLE_MASK,CDCE706_CLKOUT_ENABLE_MASK)}
unsafe fn cdce706_clkout_unprepare(hw:*mut clk_hw){let h=hw as *mut cdce706_hw_data;let _=cdce706_reg_update((*h).dev_data,CDCE706_CLKOUT((*h).idx),CDCE706_CLKOUT_ENABLE_MASK,0);}
unsafe fn cdce706_clkout_set_parent(hw:*mut clk_hw,index:u8)->i32{let h=hw as *mut cdce706_hw_data;if (*h).parent==index as u32{return 0};(*h).parent=index as u32;cdce706_reg_update((*h).dev_data,CDCE706_CLKOUT((*h).idx),CDCE706_CLKOUT_ENABLE_MASK,index as u32)}
unsafe fn cdce706_clkout_get_parent(hw:*mut clk_hw)->u8{(*(hw as *mut cdce706_hw_data)).parent as u8}
unsafe fn cdce706_clkout_recalc_rate(_hw:*mut clk_hw,parent_rate:u64)->u64{parent_rate}
unsafe fn cdce706_clkout_determine_rate(_hw:*mut clk_hw,req:*mut clk_rate_request)->i32{(*req).best_parent_rate=(*req).rate;0}
unsafe fn cdce706_clkout_set_rate(_hw:*mut clk_hw,_rate:u64,_parent_rate:u64)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
