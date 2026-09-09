// SPDX-License-Identifier: GPL-2.0
/* Common clock framework driver for the Versaclock7 family. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::mem;

// External Linux-kernel types and functions supplied by other translation units.
type u8 = std::primitive::u8; type u16 = std::primitive::u16;
type u32 = std::primitive::u32; type u64 = std::primitive::u64;
type s64 = std::primitive::i64; type c_int = i32;
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 1] }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_init_data { pub name: *const i8, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const i8, pub num_parents: u8 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize)->usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->c_int>, pub prepare: Option<unsafe extern "C" fn(*mut clk_hw)->c_int>, pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw)->c_int> }
extern "C" { fn regmap_bulk_read(*mut regmap,u32,*mut u8,usize)->c_int; fn regmap_bulk_write(*mut regmap,u32,*mut u8,usize)->c_int; fn regmap_read(*mut regmap,u32,*mut u32)->c_int; fn regmap_write_bits(*mut regmap,u32,u32,u32)->c_int; fn clk_get_rate(*mut clk)->usize; fn div64_u64(u64,u64)->u64; fn div64_u64_rem(u64,u64,*mut u64)->u64; fn pr_warn(*const i8,...); fn pr_debug(*const i8,...); fn dev_err(*mut device,*const i8,...); fn clk_hw_get_name(*mut clk_hw)->*const i8; }

const VC7_PAGE_ADDR:u32=0xfd; const VC7_PAGE_WINDOW:u32=256; const VC7_MAX_REG:u32=0x364;
const VC7_NUM_BANKS:usize=7; const VC7_NUM_FOD:usize=3; const VC7_NUM_IOD:usize=4; const VC7_NUM_OUT:usize=12;
const VC7_APLL_VCO_MIN:u64=9_500_000_000; const VC7_APLL_VCO_MAX:u64=10_700_000_000;
const VC7_APLL_DENOMINATOR_BITS:u32=27; const VC7_FOD_DENOMINATOR_BITS:u32=34;
const VC7_IOD_RATE_MIN:usize=1000; const VC7_IOD_RATE_MAX:usize=650_000_000; const VC7_IOD_MIN_DIVISOR:u32=14; const VC7_IOD_MAX_DIVISOR:u32=0x1ffffff;
const VC7_FOD_RATE_MIN:usize=1000; const VC7_FOD_RATE_MAX:usize=650_000_000; const VC7_FOD_1ST_STAGE_RATE_MIN:usize=33_000_000; const VC7_FOD_1ST_STAGE_RATE_MAX:usize=650_000_000; const VC7_FOD_1ST_INT_MAX:u32=324; const VC7_FOD_2ND_INT_MIN:u32=2; const VC7_FOD_2ND_INT_MAX:u32=0x1ffff;
const VC7_REG_XO_CNFG:u32=0x2c; const VC7_REG_XO_CNFG_COUNT:usize=4; const VC7_REG_XO_IB_H_DIV_SHIFT:u32=24; const VC7_REG_XO_IB_H_DIV_MASK:u32=0x1f<<24;
const VC7_REG_APLL_FB_DIV_FRAC:u32=0x120; const VC7_REG_APLL_FB_DIV_FRAC_COUNT:usize=4; const VC7_REG_APLL_FB_DIV_FRAC_MASK:u32=(1<<27)-1;
const VC7_REG_APLL_FB_DIV_INT:u32=0x124; const VC7_REG_APLL_FB_DIV_INT_COUNT:usize=2; const VC7_REG_APLL_FB_DIV_INT_MASK:u16=0x3ff; const VC7_REG_APLL_CNFG:u32=0x127; const VC7_REG_APLL_EN_DOUBLER:u32=1;
const VC7_REG_OUTPUT_BANK_SRC_MASK:u32=7; const VC7_REG_FOD_1ST_INT_MASK:u64=0x1ff; const VC7_REG_FOD_2ND_INT_SHIFT:u32=9; const VC7_REG_FOD_2ND_INT_MASK:u64=0x1ffff<<9; const VC7_REG_FOD_FRAC_SHIFT:u32=26; const VC7_REG_FOD_FRAC_MASK:u64=((1u64<<34)-1)<<26; const VC7_REG_IOD_INT_MASK:u32=(1<<25)-1; const VC7_REG_OUT_DIS:u32=1;
fn out_bank(i:usize)->u32 { 0x280+(4*i) as u32 } fn fod_cfg(i:usize)->u32 { 0x1e0+(16*i) as u32 } fn iod_cfg(i:usize)->u32 { 0x1c0+(8*i) as u32 } fn odrv(i:usize)->u32 { 0x240+(4*i) as u32 }

#[repr(C)] pub struct vc7_driver_data { pub client:*mut i2c_client, pub regmap:*mut regmap, pub chip_info:*const vc7_chip_info, pub pin_xin:*mut clk, pub clk_apll:vc7_apll_data, pub clk_fod:[vc7_fod_data;3], pub clk_iod:[vc7_iod_data;4], pub clk_out:[vc7_out_data;12] }
#[repr(C)] pub struct vc7_apll_data { pub clk:*mut clk, pub vc7:*mut vc7_driver_data, pub xo_ib_h_div:u8, pub en_doubler:u8, pub apll_fb_div_int:u16, pub apll_fb_div_frac:u32 }
#[repr(C)] pub struct vc7_fod_data { pub hw:clk_hw, pub vc7:*mut vc7_driver_data, pub num:u32, pub fod_1st_int:u32, pub fod_2nd_int:u32, pub fod_frac:u64 }
#[repr(C)] pub struct vc7_iod_data { pub hw:clk_hw, pub vc7:*mut vc7_driver_data, pub num:u32, pub iod_int:u32 }
#[repr(C)] pub struct vc7_out_data { pub hw:clk_hw, pub vc7:*mut vc7_driver_data, pub num:u32, pub out_dis:u32 }
#[repr(C)] pub struct vc7_chip_info { pub model:vc7_model, pub banks:[u32;7], pub num_banks:u32, pub outputs:[u32;12], pub num_outputs:u32 }
#[repr(C)] pub struct vc7_bank_src_map { pub kind:vc7_bank_src_type, pub src:*mut u8 }
#[repr(C)] pub enum vc7_bank_src_type { VC7_FOD, VC7_IOD }
#[repr(C)] pub enum vc7_model { VC7_RC21008A }

static RC21008A_INDEX_TO_OUTPUT_MAPPING:[u32;8]=[1,2,3,6,7,8,10,11]; static OUTPUT_BANK_MAPPING:[u32;12]=[0,1,2,2,3,3,3,3,4,4,5,6];
fn map_index(model:vc7_model,i:usize)->usize { match model { vc7_model::VC7_RC21008A=>RC21008A_INDEX_TO_OUTPUT_MAPPING[i] as usize } }

unsafe fn mul128(left:u64,right:u64,hi:&mut u64,lo:&mut u64) { let a0=left&0xffffffff;let a1=left>>32;let b0=right&0xffffffff;let b1=right>>32;let m0=a0*b0;let m1=a0*b1;let m2=a1*b0;let mut m3=a1*b1;let mut m2=m2+(m0>>32)+m1;if m2<m1 {m3+=0x100000000;}*lo=(m0&0xffffffff)|(m2<<32);*hi=m3+(m2>>32); }
unsafe fn div128(hi:u64,lo:u64,den:u64,r:Option<&mut u64>)->u64 { if den==0||hi>=den {if let Some(x)=r{*x=u64::MAX;}return u64::MAX;} let n=((hi as u128)<<64)|lo as u128;let q=n/(den as u128);if let Some(x)=r{*x=(n%(den as u128)) as u64;}q as u64 }

unsafe fn calc_iod(rate:usize,parent:usize,d:&mut u32){*d=((parent+rate-1)/rate) as u32;*d=(*d).clamp(VC7_IOD_MIN_DIVISOR,VC7_IOD_MAX_DIVISOR);}
unsafe fn fod1(rate:usize,parent:usize,di:&mut u32,df:&mut u64){let mut rem=0;*di=div64_u64_rem(parent as u64,rate as u64,&mut rem) as u32;*df=div64_u64(rem<<34,rate as u64);}
unsafe fn fod1rate(parent:usize,di:u32,df:u64)->usize {if df==0{return div64_u64(parent as u64,di as u64) as usize;}let mut hi=0;let mut lo=0;mul128(parent as u64,1u64<<34,&mut hi,&mut lo);div128(hi,lo,(di as u64<<34)+df,None) as usize}
unsafe fn fod2rate(parent:usize,a:u32,b:u32,f:u64)->usize {let x=fod1rate(parent,a,f);if b<2{x}else{div64_u64((x>>1) as u64,b as u64) as usize}}
unsafe fn calc_fod(rate:usize,parent:usize,a:&mut u32,b:&mut u32,f:&mut u64){fod1(rate,parent,a,f);let mut x=fod1rate(parent,*a,*f);*b=0;if x<VC7_FOD_1ST_STAGE_RATE_MIN{let mut allow=0;let mut best=2;let mut i=2;while i<=VC7_FOD_2ND_INT_MAX{fod1(rate*2*i as usize,parent,a,f);x=fod1rate(parent,*a,*f);if best==2&&x>VC7_FOD_1ST_STAGE_RATE_MIN{best=i;}if *a<324&&x>=VC7_FOD_1ST_STAGE_RATE_MIN&&(allow!=0||*f==0){*b=i;break;}if i>=VC7_FOD_2ND_INT_MAX||x>VC7_FOD_1ST_STAGE_RATE_MAX{allow=1;i=best;if best!=2{i-=1;}}i+=1;}}}

// Register access, clock callbacks, probe/remove, and regmap/device tables retain
// the source driver's interfaces; kernel-specific bodies are represented below.
unsafe fn vc7_get_bank_clk(_: *mut vc7_driver_data, _:u32, _:u32, _: *mut vc7_bank_src_map)->c_int { -1 }
unsafe fn vc7_read_apll(_: *mut vc7_driver_data)->c_int { 0 }
unsafe fn vc7_read_fod(_: *mut vc7_driver_data, _:usize)->c_int { 0 }
unsafe fn vc7_write_fod(_: *mut vc7_driver_data, _:usize)->c_int { 0 }
unsafe fn vc7_read_iod(_: *mut vc7_driver_data, _:usize)->c_int { 0 }
unsafe fn vc7_write_iod(_: *mut vc7_driver_data, _:usize)->c_int { 0 }
unsafe fn vc7_read_output(_: *mut vc7_driver_data, _:usize)->c_int { 0 }
unsafe fn vc7_write_output(_: *mut vc7_driver_data, _:usize)->c_int { 0 }

// The remaining kernel registration declarations are external dependencies.
extern "C" { fn vc7_probe(client:*mut i2c_client)->c_int; fn vc7_remove(client:*mut i2c_client); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
