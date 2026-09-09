// SPDX-License-Identifier: GPL-2.0-or-later
// Driver for IDT Versaclock 5

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C kernel dependencies are supplied by the surrounding kernel translation. */
use core::ffi::{c_char, c_int, c_ulong, c_void};

macro_rules! bit { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! genmask { ($hi:expr,$lo:expr) => { (((1u32 << (($hi)+1)) - 1) & !((1u32 << ($lo)) - 1)) }; }
macro_rules! div_round_up { ($a:expr,$b:expr) => { (($a) + ($b) - 1) / ($b) }; }
macro_rules! div_round_closest { ($a:expr,$b:expr) => { (($a) + ($b)/2) / ($b) }; }

const VC5_OTP_CONTROL:u32=0x00; const VC5_RSVD_DEVICE_ID:u32=0x01;
const VC5_RSVD_ADC_GAIN_7_0:u32=0x02; const VC5_RSVD_ADC_GAIN_15_8:u32=0x03;
const VC5_RSVD_ADC_OFFSET_7_0:u32=0x04; const VC5_RSVD_ADC_OFFSET_15_8:u32=0x05;
const VC5_RSVD_TEMPY:u32=0x06; const VC5_RSVD_OFFSET_TBIN:u32=0x07; const VC5_RSVD_GAIN:u32=0x08;
const VC5_RSVD_TEST_NP:u32=0x09; const VC5_RSVD_UNUSED:u32=0x0a; const VC5_RSVD_BANDGAP_TRIM_UP:u32=0x0b;
const VC5_RSVD_BANDGAP_TRIM_DN:u32=0x0c; const VC5_RSVD_CLK_R_12_CLK_AMP_4:u32=0x0d;
const VC5_RSVD_CLK_R_34_CLK_AMP_4:u32=0x0e; const VC5_RSVD_CLK_AMP_123:u32=0x0f;
const VC5_PRIM_SRC_SHDN:u32=0x10; const VC5_PRIM_SRC_SHDN_EN_XTAL:u32=bit!(7); const VC5_PRIM_SRC_SHDN_EN_CLKIN:u32=bit!(6);
const VC5_PRIM_SRC_SHDN_EN_DOUBLE_XTAL_FREQ:u32=bit!(3); const VC5_PRIM_SRC_SHDN_SP:u32=bit!(1); const VC5_PRIM_SRC_SHDN_EN_GBL_SHDN:u32=bit!(0);
const VC5_VCO_BAND:u32=0x11; const VC5_XTAL_X1_LOAD_CAP:u32=0x12; const VC5_XTAL_X2_LOAD_CAP:u32=0x13;
const VC5_REF_DIVIDER:u32=0x15; const VC5_REF_DIVIDER_SEL_PREDIV2:u32=bit!(7); const VC5_VCO_CTRL_AND_PREDIV:u32=0x16;
const VC5_VCO_CTRL_AND_PREDIV_BYPASS_PREDIV:u32=bit!(7); const VC5_FEEDBACK_INT_DIV:u32=0x17; const VC5_FEEDBACK_INT_DIV_BITS:u32=0x18;
const VC5_RC_CONTROL0:u32=0x1e; const VC5_RC_CONTROL1:u32=0x1f; const VC5_MAX_CLK_OUT_NUM:usize=5; const VC5_MAX_FOD_NUM:usize=4;
const VC5_PLL_VCO_MIN:u64=2500000000; const VC5_MUX_IN_XIN:u8=bit!(0) as u8; const VC5_MUX_IN_CLKIN:u8=bit!(1) as u8;
const VC5_HAS_INTERNAL_XTAL:u32=bit!(0); const VC5_HAS_PFD_FREQ_DBL:u32=bit!(1); const VC5_HAS_BYPASS_SYNC_BIT:u32=bit!(2);

#[inline] const fn vc5_ref_divider_ref_div(n:u32)->u32 { n & 0x3f }
#[inline] const fn vc5_feedback_frac_div(n:u32)->u32 { 0x19+n }
#[inline] const fn vc5_reserved_x0(i:u32)->u32 { 0x20+i*0x10 }
#[inline] const fn vc5_out_div_control(i:u32)->u32 { 0x21+i*0x10 }
#[inline] const fn vc5_out_div_frac(i:u32,n:u32)->u32 { 0x22+i*0x10+n }
#[inline] const fn vc5_out_div_step_spread(i:u32,n:u32)->u32 { 0x26+i*0x10+n }
#[inline] const fn vc5_out_div_spread_mod(i:u32,n:u32)->u32 { 0x29+i*0x10+n }
#[inline] const fn vc5_out_div_skew_int(i:u32,n:u32)->u32 { 0x2b+i*0x10+n }
#[inline] const fn vc5_out_div_int(i:u32,n:u32)->u32 { 0x2d+i*0x10+n }
#[inline] const fn vc5_out_div_skew_frac(i:u32)->u32 { 0x2f+i*0x10 }
#[inline] const fn vc5_clk_output_cfg(i:u32,n:u32)->u32 { 0x60+i*2+n }
const VC5_OUT_DIV_CONTROL_RESET:u32=bit!(7); const VC5_OUT_DIV_CONTROL_SELB_NORM:u32=bit!(3); const VC5_OUT_DIV_CONTROL_SEL_EXT:u32=bit!(2); const VC5_OUT_DIV_CONTROL_EN_FOD:u32=bit!(0);
const VC5_RESERVED_X0_BYPASS_SYNC:u32=bit!(7); const VC5_CLK_OUTPUT_CFG1_EN_CLKBUF:u32=bit!(0); const VC5_GLOBAL_REGISTER:u32=0x76; const VC5_GLOBAL_REGISTER_GLOBAL_RESET:u32=bit!(5);

#[repr(C)] #[derive(Copy,Clone)] pub enum vc5_model { IDT_VC5_5P49V5923, IDT_VC5_5P49V5925, IDT_VC5_5P49V5933, IDT_VC5_5P49V5935, IDT_VC6_5P49V60, IDT_VC6_5P49V6901, IDT_VC6_5P49V6965, IDT_VC6_5P49V6975 }
#[repr(C)] pub struct vc5_chip_info { pub model:vc5_model,pub clk_fod_cnt:u32,pub clk_out_cnt:u32,pub flags:u32,pub vco_max:u64 }
#[repr(C)] pub struct clk_hw { pub init:*mut c_void }
#[repr(C)] pub struct vc5_hw_data { pub hw:clk_hw,pub vc5:*mut vc5_driver_data,pub div_int:u32,pub div_frc:u32,pub num:u32 }
#[repr(C)] pub struct vc5_out_data { pub hw:clk_hw,pub vc5:*mut vc5_driver_data,pub num:u32,pub clk_output_cfg0:u32,pub clk_output_cfg0_mask:u32 }
#[repr(C)] pub struct vc5_driver_data { pub client:*mut c_void,pub regmap:*mut c_void,pub chip_info:*const vc5_chip_info,pub pin_xin:*mut c_void,pub pin_clkin:*mut c_void,pub clk_mux_ins:u8,pub clk_mux:clk_hw,pub clk_mul:clk_hw,pub clk_pfd:clk_hw,pub clk_pll:vc5_hw_data,pub clk_fod:[vc5_hw_data;VC5_MAX_FOD_NUM],pub clk_out:[vc5_out_data;VC5_MAX_CLK_OUT_NUM] }

extern "C" { fn regmap_read(*mut c_void,u32,*mut u32)->c_int; fn regmap_update_bits(*mut c_void,u32,u32,u32)->c_int; fn regmap_bulk_read(*mut c_void,u32,*mut u8,usize)->c_int; fn regmap_bulk_write(*mut c_void,u32,*const u8,usize)->c_int; fn regmap_set_bits(*mut c_void,u32,u32)->c_int; fn regmap_clear_bits(*mut c_void,u32,u32)->c_int; }

pub fn vc5_map_index_to_output(model:vc5_model,n:u32)->u32 { match model { vc5_model::IDT_VC5_5P49V5933 => if n==0 {0} else {3}, _=>n } }
pub fn vc5_map_cap_value(femtofarads:u32)->c_int { if femtofarads<9000 || femtofarads>22760 { return -22; } let mut v=div_round_closest!(femtofarads-9000,430); if v>31 {v=0x3f} else {v<<=1}; v as c_int }

pub unsafe fn vc5_mux_get_parent(vc5:*mut vc5_driver_data)->u8 { let mut src=0; if regmap_read((*vc5).regmap,VC5_PRIM_SRC_SHDN,&mut src)!=0{return 0}; match src&(VC5_PRIM_SRC_SHDN_EN_XTAL|VC5_PRIM_SRC_SHDN_EN_CLKIN) { VC5_PRIM_SRC_SHDN_EN_CLKIN=>1,_=>0 } }
pub unsafe fn vc5_mux_set_parent(vc5:*mut vc5_driver_data,index:u8)->c_int { if index>1||(*vc5).clk_mux_ins==0{return -22}; let src=if (*vc5).clk_mux_ins==(VC5_MUX_IN_XIN|VC5_MUX_IN_CLKIN) {if index==0{VC5_PRIM_SRC_SHDN_EN_XTAL}else{VC5_PRIM_SRC_SHDN_EN_CLKIN}} else if index!=0{return -22} else if (*vc5).clk_mux_ins==VC5_MUX_IN_XIN{VC5_PRIM_SRC_SHDN_EN_XTAL}else if (*vc5).clk_mux_ins==VC5_MUX_IN_CLKIN{VC5_PRIM_SRC_SHDN_EN_CLKIN}else{return -22}; regmap_update_bits((*vc5).regmap,VC5_PRIM_SRC_SHDN,VC5_PRIM_SRC_SHDN_EN_XTAL|VC5_PRIM_SRC_SHDN_EN_CLKIN,src) }

pub unsafe fn vc5_dbl_recalc_rate(vc5:*mut vc5_driver_data,parent:u64)->u64 { let mut v=0;if regmap_read((*vc5).regmap,VC5_PRIM_SRC_SHDN,&mut v)!=0{return 0}; if v&VC5_PRIM_SRC_SHDN_EN_DOUBLE_XTAL_FREQ!=0{parent*2}else{parent} }
pub fn vc5_dbl_determine_rate(best:u64,rate:u64)->c_int { if best==rate||best*2==rate{0}else{-22} }
pub unsafe fn vc5_dbl_set_rate(vc5:*mut vc5_driver_data,rate:u64,parent:u64)->c_int { regmap_update_bits((*vc5).regmap,VC5_PRIM_SRC_SHDN,VC5_PRIM_SRC_SHDN_EN_DOUBLE_XTAL_FREQ,if parent*2==rate{VC5_PRIM_SRC_SHDN_EN_DOUBLE_XTAL_FREQ}else{0}) }

pub fn vc5_pfd_determine_rate(best:u64,rate:u64)->Result<u64,c_int>{if rate>50000000{return Err(-22)};if best<=50000000{return Ok(best)};let d=div_round_up!(best,rate);if d>127{Err(-22)}else{Ok(best/d)}}
pub unsafe fn vc5_pfd_set_rate(vc5:*mut vc5_driver_data,rate:u64,parent:u64)->c_int {if parent<=50000000{let r=regmap_set_bits((*vc5).regmap,VC5_VCO_CTRL_AND_PREDIV,VC5_VCO_CTRL_AND_PREDIV_BYPASS_PREDIV);if r!=0{return r}return regmap_update_bits((*vc5).regmap,VC5_REF_DIVIDER,0xff,0)} let d=div_round_up!(parent,rate);let v=if d==2{0x80}else{vc5_ref_divider_ref_div(d as u32)};let r=regmap_update_bits((*vc5).regmap,VC5_REF_DIVIDER,0xff,v);if r!=0{return r} regmap_clear_bits((*vc5).regmap,VC5_VCO_CTRL_AND_PREDIV,VC5_VCO_CTRL_AND_PREDIV_BYPASS_PREDIV)}

pub unsafe fn vc5_pll_recalc_rate(vc5:*mut vc5_driver_data,parent:u64)->u64 {let mut fb=[0u8;5];regmap_bulk_read((*vc5).regmap,VC5_FEEDBACK_INT_DIV,fb.as_mut_ptr(),5);let i=((fb[0] as u64)<<4)|((fb[1] as u64)>>4);let f=((fb[2] as u64)<<16)|((fb[3] as u64)<<8)|fb[4] as u64;(parent*i)+((parent*f)>>24)}
pub fn vc5_fod_rate(f_in:u64,div_int:u64,div_frc:u64)->u64 {if div_int==0&&div_frc==0{0}else{(f_in<<24)/((div_int<<24)+div_frc)}}
pub fn vc5_map_output_config(value:u32,shift:u32,mask:u32,allowed:&[u32],out:&mut u32,outmask:&mut u32)->c_int {if !allowed.contains(&value){return -22};*outmask|=mask;*out|=value<<shift;0}

/* Remaining kernel registration and device-management routines retain their C ABI-facing roles. */
extern "C" { pub fn vc5_probe(client:*mut c_void)->c_int; pub fn vc5_remove(client:*mut c_void); pub fn vc5_suspend(dev:*mut c_void)->c_int; pub fn vc5_resume(dev:*mut c_void)->c_int; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
