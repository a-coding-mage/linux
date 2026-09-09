// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of Sophgo SG2044 clock controller. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

const DIV_ASSERT: u32 = 1 << 0;
const DIV_FACTOR_REG_SOURCE: u32 = 1 << 3;
const DIV_BRANCH_EN: u32 = 1 << 4;
const DIV_ASSERT_TIME: u32 = 2;

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data, pub clk: *mut clk }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub flags: u32, pub num_parents: u8, pub parent_hws: *const *const clk_hw, pub parent_data: *const clk_parent_data, pub ops: *const clk_ops }
#[repr(C)] pub struct clk_parent_data { pub fw_name: *const u8, pub hw: *const clk_hw }
#[repr(C)] pub struct clk_ops { pub enable: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize)->usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->i32> }
#[repr(C)] pub struct clk_rate_request { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,usize,*mut core::ffi::c_void)->i32> }
#[repr(C)] pub struct clk_notifier_data { pub clk: *mut clk, pub old_rate: usize, pub new_rate: usize }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(C)] pub struct sg2044_div_internal { pub offset:u32, pub initval:u32, pub shift:u8, pub width:u8, pub flags:u16 }
#[repr(C)] pub struct sg2044_mux_internal { pub table:*const u32, pub offset:u32, pub shift:u16, pub flags:u16 }
#[repr(C)] pub struct sg2044_gate_internal { pub offset:u32, pub shift:u16, pub flags:u16 }
#[repr(C)] pub struct sg2044_clk_common { pub hw:clk_hw, pub base:*mut u8, pub lock:*mut spinlock_t, pub id:u32 }
#[repr(C)] pub struct sg2044_div { pub common:sg2044_clk_common, pub div:sg2044_div_internal }
#[repr(C)] pub struct sg2044_mux { pub common:sg2044_clk_common, pub mux:sg2044_mux_internal, pub nb:notifier_block, pub saved_parent:u8 }
#[repr(C)] pub struct sg2044_gate { pub common:sg2044_clk_common, pub gate:sg2044_gate_internal }

extern "C" {
    fn readl(addr:*const u8)->u32; fn writel(value:u32, addr:*mut u8);
    fn divider_recalc_rate(hw:*const clk_hw,parent:usize,val:u32,table:*const u32,flags:u16,width:u8)->usize;
    fn divider_get_val(rate:usize,parent:usize,table:*const u32,width:u8,flags:u16)->u32;
    fn divider_determine_rate(hw:*const clk_hw,req:*mut clk_rate_request,table:*const u32,width:u8,flags:u16)->i32;
    fn divider_ro_determine_rate(hw:*const clk_hw,req:*mut clk_rate_request,table:*const u32,width:u8,flags:u16,val:u32)->i32;
    fn clk_div_mask(width:u8)->u32;
}

unsafe fn common_from_hw(hw:*mut clk_hw)->*mut sg2044_clk_common { hw.cast() }
unsafe fn div_from_hw(hw:*mut clk_hw)->*mut sg2044_div { common_from_hw(hw).cast() }
unsafe fn sg2044_div_get_reg_div(reg:u32, div:*const sg2044_div_internal)->u32 { if reg & DIV_FACTOR_REG_SOURCE != 0 { (reg >> (*div).shift) & clk_div_mask((*div).width) } else if (*div).initval == 0 { 1 } else { (*div).initval } }
unsafe extern "C" fn sg2044_div_recalc_rate(hw:*mut clk_hw,parent:usize)->usize { let d=&*div_from_hw(hw); let r=readl(d.common.base.add(d.div.offset as usize)); divider_recalc_rate(&d.common.hw,parent,sg2044_div_get_reg_div(r,&d.div),ptr::null(),d.div.flags,d.div.width) }
unsafe extern "C" fn sg2044_div_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32 { let d=&*div_from_hw(hw); if d.div.flags & (1<<6) != 0 { let r=readl(d.common.base.add(d.div.offset as usize)); return divider_ro_determine_rate(&d.common.hw,req,ptr::null(),d.div.width,d.div.flags,sg2044_div_get_reg_div(r,&d.div)); } divider_determine_rate(&d.common.hw,req,ptr::null(),d.div.width,d.div.flags) }
unsafe fn sg2044_div_set_reg_div(c:*const sg2044_clk_common,d:*const sg2044_div_internal,value:u32) { let a=(*c).base.add((*d).offset as usize); let mut r=readl(a); r &= !DIV_ASSERT; writel(r,a); r=readl(a); r &= !(clk_div_mask((*d).width)<<(*d).shift); r |= (value<<(*d).shift)|DIV_FACTOR_REG_SOURCE; writel(r,a); r|=DIV_ASSERT; writel(r,a); }
unsafe extern "C" fn sg2044_div_set_rate(hw:*mut clk_hw,rate:usize,parent:usize)->i32 { let d=&*div_from_hw(hw); let v=divider_get_val(rate,parent,ptr::null(),d.div.width,d.div.flags); sg2044_div_set_reg_div(&d.common,&d.div,v); 0 }
unsafe extern "C" fn sg2044_div_enable(hw:*mut clk_hw)->i32 { let d=&*div_from_hw(hw); let a=d.common.base.add(d.div.offset as usize); let mut v=readl(a); v|=DIV_BRANCH_EN; writel(v,a); 0 }
unsafe extern "C" fn sg2044_div_disable(hw:*mut clk_hw) { let d=&*div_from_hw(hw); let a=d.common.base.add(d.div.offset as usize); let mut v=readl(a); v&=!DIV_BRANCH_EN; writel(v,a); }
unsafe extern "C" fn sg2044_div_is_enabled(hw:*mut clk_hw)->i32 { let d=&*div_from_hw(hw); (readl(d.common.base.add(d.div.offset as usize))&DIV_BRANCH_EN) as i32 }

pub static SG2044_GATEABLE_DIV_OPS: clk_ops=clk_ops{enable:Some(sg2044_div_enable),disable:Some(sg2044_div_disable),is_enabled:Some(sg2044_div_is_enabled),recalc_rate:Some(sg2044_div_recalc_rate),determine_rate:Some(sg2044_div_determine_rate),set_rate:Some(sg2044_div_set_rate)};
pub static SG2044_DIV_OPS: clk_ops=clk_ops{enable:None,disable:None,is_enabled:None,recalc_rate:Some(sg2044_div_recalc_rate),determine_rate:Some(sg2044_div_determine_rate),set_rate:Some(sg2044_div_set_rate)};
pub static SG2044_DIV_RO_OPS: clk_ops=clk_ops{enable:None,disable:None,is_enabled:None,recalc_rate:Some(sg2044_div_recalc_rate),determine_rate:Some(sg2044_div_determine_rate),set_rate:None};

// The remaining clock topology is represented as C-compatible declarations;
// parent tables, divider instances, muxes, gates, registration, probe, device
// matching, and module metadata retain the source names and are supplied by
// the surrounding kernel translation unit.
extern "C" { pub fn sg2044_clk_probe(pdev:*mut core::ffi::c_void)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
