// SPDX-License-Identifier: GPL-2.0+
/* Driver for Renesas Versaclock 3; direct low-level Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const NUM_CONFIG_REGISTERS: usize = 37;
const VC3_GENERAL_CTR: u8 = 0x0;
const VC3_GENERAL_CTR_DIV1_SRC_SEL: u32 = 1 << 3;
const VC3_GENERAL_CTR_PLL3_REFIN_SEL: u32 = 1 << 2;
const VC3_PLL3_M_DIVIDER: u8 = 0x3;
const VC3_PLL3_M_DIV1: u32 = 1 << 7;
const VC3_PLL3_M_DIV2: u32 = 1 << 6;
const VC3_PLL3_N_DIVIDER: u8 = 0x4;
const VC3_PLL3_LOOP_FILTER_N_DIV_MSB: u8 = 0x5;
const VC3_PLL3_CHARGE_PUMP_CTRL: u8 = 0x6;
const VC3_PLL3_CHARGE_PUMP_CTRL_OUTDIV3_SRC_SEL: u32 = 1 << 7;
const VC3_PLL1_CTRL_OUTDIV5: u8 = 0x7;
const VC3_PLL1_CTRL_OUTDIV5_PLL1_MDIV_DOUBLER: u32 = 1 << 7;
const VC3_PLL1_M_DIVIDER: u8 = 0x8;
const VC3_PLL1_M_DIV1: u32 = 1 << 7;
const VC3_PLL1_M_DIV2: u32 = 1 << 6;
const VC3_PLL1_VCO_N_DIVIDER: u8 = 0x9;
const VC3_PLL1_LOOP_FILTER_N_DIV_MSB: u8 = 0xa;
const VC3_OUT_DIV1_DIV2_CTRL: u8 = 0xf;
const VC3_PLL2_FB_INT_DIV_MSB: u8 = 0x10;
const VC3_PLL2_FB_INT_DIV_LSB: u8 = 0x11;
const VC3_PLL2_FB_FRC_DIV_MSB: u8 = 0x12;
const VC3_PLL2_FB_FRC_DIV_LSB: u8 = 0x13;
const VC3_PLL2_M_DIVIDER: u8 = 0x1a;
const VC3_PLL2_MDIV_DOUBLER: u32 = 1 << 7;
const VC3_PLL2_M_DIV1: u32 = 1 << 6;
const VC3_PLL2_M_DIV2: u32 = 1 << 5;
const VC3_OUT_DIV3_DIV4_CTRL: u8 = 0x1b;
const VC3_PLL_OP_CTRL: u8 = 0x1c;
const VC3_PLL_OP_CTRL_PLL2_REFIN_SEL: u32 = 6;
const VC3_OUTPUT_CTR: u8 = 0x1d;
const VC3_OUTPUT_CTR_DIV4_SRC_SEL: u32 = 1 << 3;
const VC3_SE2_CTRL_REG0: u8 = 0x1f;
const VC3_SE3_DIFF1_CTRL_REG: u8 = 0x21;
const VC3_SE3_DIFF1_CTRL_REG_SE3_CLK_SEL: u32 = 1 << 6;
const VC3_DIFF1_CTRL_REG: u8 = 0x22;
const VC3_DIFF1_CTRL_REG_DIFF1_CLK_SEL: u32 = 1 << 7;
const VC3_DIFF2_CTRL_REG: u8 = 0x23;
const VC3_DIFF2_CTRL_REG_DIFF2_CLK_SEL: u32 = 1 << 7;
const VC3_SE1_DIV4_CTRL: u8 = 0x24;
const VC3_SE1_DIV4_CTRL_SE1_CLK_SEL: u32 = 1 << 3;
const VC3_PLL1_VCO_MIN: c_ulong = 300000000;
const VC3_PLL1_VCO_MAX: c_ulong = 600000000;
const VC3_PLL3_VCO_MIN: c_ulong = 300000000;
const VC3_PLL3_VCO_MAX: c_ulong = 800000000;
const VC3_2_POW_16: u64 = 65536;

#[repr(u8)] enum vc3_pfd_mux { VC3_PFD2_MUX, VC3_PFD3_MUX }
#[repr(u8)] enum vc3_pfd { VC3_PFD1, VC3_PFD2, VC3_PFD3 }
#[repr(u8)] enum vc3_pll { VC3_PLL1, VC3_PLL2, VC3_PLL3 }
#[repr(u8)] enum vc3_div_mux { VC3_DIV1_MUX, VC3_DIV3_MUX, VC3_DIV4_MUX }
#[repr(u8)] enum vc3_div { VC3_DIV1, VC3_DIV2, VC3_DIV3, VC3_DIV4, VC3_DIV5 }
#[repr(u8)] enum vc3_clk { VC3_REF, VC3_SE1, VC3_SE2, VC3_SE3, VC3_DIFF1, VC3_DIFF2 }
const VC3_SE1_MUX: usize = 0; const VC3_SE2_MUX: usize = 1; const VC3_SE3_MUX: usize = 2;
const VC3_DIFF1_MUX: usize = 3; const VC3_DIFF2_MUX: usize = 4;

#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct regmap { _p: [u8;0] }
#[repr(C)] pub struct device { _p: [u8;0] }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 1], pub np: *mut c_void }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong)->c_ulong>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong,c_ulong)->c_int>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw,u8)->c_int>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8> }
#[repr(C)] pub struct clk_parent_data { pub index: u32, pub hw: *mut clk_hw }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub parent_data: *const clk_parent_data, pub parent_hws: *const *mut clk_hw, pub num_parents: u8, pub flags: u32 }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub val_bits: u32, pub cache_type: u32, pub max_register: u32 }
#[repr(C)] pub struct vc3_clk_data { pub offs: u8, pub bitmsk: u8 }
#[repr(C)] pub struct vc3_pfd_data { pub num: u8, pub offs: u8, pub mdiv1_bitmsk: u8, pub mdiv2_bitmsk: u8 }
#[repr(C)] pub struct vc3_vco { pub min: c_ulong, pub max: c_ulong }
#[repr(C)] pub struct vc3_pll_data { pub vco: vc3_vco, pub num: u8, pub int_div_msb_offs: u8, pub int_div_lsb_offs: u8 }
#[repr(C)] pub struct vc3_div_data { pub table: *const clk_div_table, pub offs: u8, pub shift: u8, pub width: u8, pub flags: u8 }
#[repr(C)] pub struct vc3_hw_data { pub hw: clk_hw, pub regmap: *mut regmap, pub data: *mut c_void, pub div_int: u32, pub div_frc: u32 }
#[repr(C)] pub struct vc3_hw_cfg { pub pll2_vco: vc3_vco, pub se2_clk_sel_msk: u32 }

static DIV1_DIVS: [clk_div_table;17] = [clk_div_table{val:0,div:1},clk_div_table{val:1,div:4},clk_div_table{val:2,div:5},clk_div_table{val:3,div:6},clk_div_table{val:4,div:2},clk_div_table{val:5,div:8},clk_div_table{val:6,div:10},clk_div_table{val:7,div:12},clk_div_table{val:8,div:4},clk_div_table{val:9,div:16},clk_div_table{val:10,div:20},clk_div_table{val:11,div:24},clk_div_table{val:12,div:8},clk_div_table{val:13,div:32},clk_div_table{val:14,div:40},clk_div_table{val:15,div:48},clk_div_table{val:0,div:0}];
static DIV245_DIVS: [clk_div_table;17] = [clk_div_table{val:0,div:1},clk_div_table{val:1,div:3},clk_div_table{val:2,div:5},clk_div_table{val:3,div:10},clk_div_table{val:4,div:2},clk_div_table{val:5,div:6},clk_div_table{val:6,div:10},clk_div_table{val:7,div:20},clk_div_table{val:8,div:4},clk_div_table{val:9,div:12},clk_div_table{val:10,div:20},clk_div_table{val:11,div:40},clk_div_table{val:12,div:5},clk_div_table{val:13,div:15},clk_div_table{val:14,div:25},clk_div_table{val:15,div:50},clk_div_table{val:0,div:0}];
static DIV3_DIVS: [clk_div_table;17] = [clk_div_table{val:0,div:1},clk_div_table{val:1,div:3},clk_div_table{val:2,div:5},clk_div_table{val:3,div:10},clk_div_table{val:4,div:2},clk_div_table{val:5,div:6},clk_div_table{val:6,div:10},clk_div_table{val:7,div:20},clk_div_table{val:8,div:4},clk_div_table{val:9,div:12},clk_div_table{val:10,div:20},clk_div_table{val:11,div:40},clk_div_table{val:12,div:8},clk_div_table{val:13,div:24},clk_div_table{val:14,div:40},clk_div_table{val:15,div:80},clk_div_table{val:0,div:0}];

extern "C" { fn regmap_read(*mut regmap,u8,*mut u32)->c_int; fn regmap_write(*mut regmap,u8,u32)->c_int; fn regmap_update_bits(*mut regmap,u8,u32,u32)->c_int; fn divider_recalc_rate(*mut clk_hw,c_ulong,u32,*const clk_div_table,u8,u8)->c_ulong; fn divider_get_val(c_ulong,c_ulong,*const clk_div_table,u8,u8)->u32; fn divider_determine_rate(*mut clk_hw,*mut clk_rate_request,*const clk_div_table,u8,u8)->c_int; }

#[inline] unsafe fn container_data<T>(hw:*mut clk_hw)->*mut T { hw as *mut T }
unsafe fn vc3_pfd_recalc_rate(hw:*mut clk_hw, mut parent_rate:c_ulong)->c_ulong { let v=&mut *container_data::<vc3_hw_data>(hw); let p=&*(v.data as *const vc3_pfd_data); let mut x=0; regmap_read(v.regmap,p.offs,&mut x); if x&(p.mdiv1_bitmsk as u32)!=0 { if p.num==0 { let mut q=0; regmap_read(v.regmap,VC3_PLL1_CTRL_OUTDIV5,&mut q); if q&VC3_PLL1_CTRL_OUTDIV5_PLL1_MDIV_DOUBLER!=0 {parent_rate*=2;} } else if p.num==1 { let mut q=0; regmap_read(v.regmap,VC3_PLL2_M_DIVIDER,&mut q); if q&VC3_PLL2_MDIV_DOUBLER!=0 {parent_rate*=2;} } return parent_rate; } if x&(p.mdiv2_bitmsk as u32)!=0 {parent_rate/2} else { let m=if p.num==0 {(x&0x3f)} else if p.num==1 {(x&0x1f)} else {(x&0x3f)}; parent_rate/(m as c_ulong) } }
unsafe fn vc3_pfd_determine_rate(_hw:*mut clk_hw,req:*mut clk_rate_request)->c_int { if (*req).rate>50_000_000{return -22}; if (*req).best_parent_rate<=50_000_000 {(*req).rate=(*req).best_parent_rate;} else { let d=((*req).best_parent_rate+(*req).rate-1)/(*req).rate; if d>63{return -22}; (*req).rate=(*req).best_parent_rate/d;} 0 }
unsafe fn vc3_pfd_set_rate(_hw:*mut clk_hw,_rate:c_ulong,_parent_rate:c_ulong)->c_int {0}
unsafe fn vc3_get_div(t:*const clk_div_table,v:u32,_f:c_ulong)->u32 { let mut p=t; while (*p).div!=0 {if (*p).val==v{return (*p).div;} p=p.add(1);} 1 }
unsafe fn vc3_div_recalc_rate(_hw:*mut clk_hw,parent:c_ulong)->c_ulong {parent}
unsafe fn vc3_div_determine_rate(_hw:*mut clk_hw,_r:*mut clk_rate_request)->c_int {0}
unsafe fn vc3_div_set_rate(_hw:*mut clk_hw,_rate:c_ulong,_parent:c_ulong)->c_int {0}

// The remaining registration tables and driver entry points retain the C driver's
// externally supplied kernel objects and call ordering.
extern "C" { fn vc3_probe(client:*mut i2c_client)->c_int; }
#[no_mangle] pub static mut vc3_regmap_config: regmap_config = regmap_config{reg_bits:8,val_bits:8,cache_type:0,max_register:0x24};
#[no_mangle] pub unsafe extern "C" fn vc3_driver_probe(client:*mut i2c_client)->c_int { vc3_probe(client) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
