// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Linux headers and clk-regmap.h provide the declarations used below.

extern "C" {
    fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap;
    fn clk_hw_get_dev(hw: *mut clk_hw) -> *mut device;
    fn dev_get_regmap(dev: *mut device, name: *const core::ffi::c_char) -> *mut regmap;
    fn clk_hw_get_of_node(hw: *mut clk_hw) -> *mut device_node;
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn of_node_put(np: *mut device_node);
    fn regmap_update_bits(map: *mut regmap, offset: u32, mask: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, offset: u32, val: *mut u32) -> i32;
    fn clk_get_regmap_gate_data(clk: *mut clk_regmap) -> *mut clk_regmap_gate_data;
    fn clk_get_regmap_div_data(clk: *mut clk_regmap) -> *mut clk_regmap_div_data;
    fn clk_get_regmap_mux_data(clk: *mut clk_regmap) -> *mut clk_regmap_mux_data;
    fn clk_div_mask(width: u8) -> u32;
    fn divider_recalc_rate(hw: *mut clk_hw, prate: usize, val: u32, table: *const core::ffi::c_void, flags: u8, width: u8) -> usize;
    fn divider_ro_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request, table: *const core::ffi::c_void, width: u8, flags: u8, val: u32) -> i32;
    fn divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request, table: *const core::ffi::c_void, width: u8, flags: u8) -> i32;
    fn divider_get_val(rate: usize, parent_rate: usize, table: *const core::ffi::c_void, width: u8, flags: u8) -> i32;
    fn clk_mux_val_to_index(hw: *mut clk_hw, table: *const u32, flags: u32, val: u32) -> u8;
    fn clk_mux_index_to_val(table: *const u32, flags: u32, index: u8) -> u32;
    fn clk_mux_determine_rate_flags(hw: *mut clk_hw, req: *mut clk_rate_request, flags: u32) -> i32;
}

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { pub map: *mut regmap }
#[repr(C)] pub struct clk_regmap_gate_data { pub flags: u32, pub offset: u32, pub bit_idx: u8 }
#[repr(C)] pub struct clk_regmap_div_data { pub flags: u8, pub offset: u32, pub shift: u8, pub width: u8, pub table: *const core::ffi::c_void }
#[repr(C)] pub struct clk_regmap_mux_data { pub flags: u32, pub offset: u32, pub shift: u8, pub mask: u32, pub table: *const u32 }
#[repr(C)] pub struct clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
}

const EINVAL: i32 = 22;
const CLK_GATE_SET_TO_DISABLE: u32 = 1 << 0;
const CLK_DIVIDER_READ_ONLY: u8 = 1 << 0;

#[inline] unsafe fn bit(n: u8) -> u32 { 1u32.wrapping_shl(n as u32) }

#[no_mangle]
pub unsafe extern "C" fn clk_regmap_init(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    if !(*clk).map.is_null() { return 0; }
    let dev = clk_hw_get_dev(hw);
    if !dev.is_null() {
        (*clk).map = dev_get_regmap(dev, core::ptr::null());
        if !(*clk).map.is_null() { return 0; }
    }
    let np = clk_hw_get_of_node(hw);
    if !np.is_null() {
        let parent_np = of_get_parent(np);
        (*clk).map = syscon_node_to_regmap(parent_np);
        of_node_put(parent_np);
        if !(*clk).map.is_null() { return 0; }
    }
    -EINVAL
}

unsafe fn clk_regmap_gate_endisable(hw: *mut clk_hw, enable: i32) -> i32 {
    let clk = to_clk_regmap(hw); let gate = clk_get_regmap_gate_data(clk);
    let mut set = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { 1 } else { 0 };
    set ^= enable;
    regmap_update_bits((*clk).map, (*gate).offset, bit((*gate).bit_idx), if set != 0 { bit((*gate).bit_idx) } else { 0 })
}
unsafe extern "C" fn clk_regmap_gate_enable(hw: *mut clk_hw) -> i32 { clk_regmap_gate_endisable(hw, 1) }
unsafe extern "C" fn clk_regmap_gate_disable(hw: *mut clk_hw) { let _ = clk_regmap_gate_endisable(hw, 0); }
unsafe extern "C" fn clk_regmap_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw); let gate = clk_get_regmap_gate_data(clk); let mut val = 0;
    let _ = regmap_read((*clk).map, (*gate).offset, &mut val);
    if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { val ^= bit((*gate).bit_idx); }
    if val & bit((*gate).bit_idx) != 0 { 1 } else { 0 }
}

pub static clk_regmap_gate_ops: clk_ops = clk_ops { init: Some(clk_regmap_init), enable: Some(clk_regmap_gate_enable), disable: Some(clk_regmap_gate_disable), is_enabled: Some(clk_regmap_gate_is_enabled), recalc_rate: None, determine_rate: None, set_rate: None, get_parent: None, set_parent: None };
pub static clk_regmap_gate_ro_ops: clk_ops = clk_ops { init: Some(clk_regmap_init), enable: None, disable: None, is_enabled: Some(clk_regmap_gate_is_enabled), recalc_rate: None, determine_rate: None, set_rate: None, get_parent: None, set_parent: None };

unsafe extern "C" fn clk_regmap_div_recalc_rate(hw: *mut clk_hw, prate: usize) -> usize { let clk=to_clk_regmap(hw); let div=clk_get_regmap_div_data(clk); let mut val=0; if regmap_read((*clk).map,(*div).offset,&mut val)!=0{return 0;} val >>= (*div).shift; val &= clk_div_mask((*div).width); divider_recalc_rate(hw,prate,val,(*div).table,(*div).flags,(*div).width) }
unsafe extern "C" fn clk_regmap_div_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32 { let clk=to_clk_regmap(hw); let div=clk_get_regmap_div_data(clk); if (*div).flags&CLK_DIVIDER_READ_ONLY!=0 { let mut val=0; let ret=regmap_read((*clk).map,(*div).offset,&mut val); if ret!=0{return ret;} val>>=(*div).shift; val&=clk_div_mask((*div).width); return divider_ro_determine_rate(hw,req,(*div).table,(*div).width,(*div).flags,val); } divider_determine_rate(hw,req,(*div).table,(*div).width,(*div).flags) }
unsafe extern "C" fn clk_regmap_div_set_rate(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32 { let clk=to_clk_regmap(hw); let div=clk_get_regmap_div_data(clk); let ret=divider_get_val(rate,parent_rate,(*div).table,(*div).width,(*div).flags); if ret<0{return ret;} regmap_update_bits((*clk).map,(*div).offset,clk_div_mask((*div).width)<<(*div).shift,(ret as u32)<<(*div).shift) }
pub static clk_regmap_divider_ops: clk_ops = clk_ops { init:Some(clk_regmap_init), enable:None, disable:None, is_enabled:None, recalc_rate:Some(clk_regmap_div_recalc_rate), determine_rate:Some(clk_regmap_div_determine_rate), set_rate:Some(clk_regmap_div_set_rate), get_parent:None, set_parent:None };
pub static clk_regmap_divider_ro_ops: clk_ops = clk_ops { init:Some(clk_regmap_init), enable:None, disable:None, is_enabled:None, recalc_rate:Some(clk_regmap_div_recalc_rate), determine_rate:Some(clk_regmap_div_determine_rate), set_rate:None, get_parent:None, set_parent:None };

unsafe extern "C" fn clk_regmap_mux_get_parent(hw:*mut clk_hw)->u8 { let clk=to_clk_regmap(hw); let mux=clk_get_regmap_mux_data(clk); let mut val=0; let ret=regmap_read((*clk).map,(*mux).offset,&mut val); if ret!=0{return ret as u8;} val>>=(*mux).shift; val&=(*mux).mask; clk_mux_val_to_index(hw,(*mux).table,(*mux).flags,val) }
unsafe extern "C" fn clk_regmap_mux_set_parent(hw:*mut clk_hw,index:u8)->i32 { let clk=to_clk_regmap(hw); let mux=clk_get_regmap_mux_data(clk); let val=clk_mux_index_to_val((*mux).table,(*mux).flags,index); regmap_update_bits((*clk).map,(*mux).offset,(*mux).mask<<(*mux).shift,val<<(*mux).shift) }
unsafe extern "C" fn clk_regmap_mux_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32 { let clk=to_clk_regmap(hw); let mux=clk_get_regmap_mux_data(clk); clk_mux_determine_rate_flags(hw,req,(*mux).flags) }
pub static clk_regmap_mux_ops: clk_ops = clk_ops { init:Some(clk_regmap_init), enable:None, disable:None, is_enabled:None, recalc_rate:None, determine_rate:Some(clk_regmap_mux_determine_rate), set_rate:None, get_parent:Some(clk_regmap_mux_get_parent), set_parent:Some(clk_regmap_mux_set_parent) };
pub static clk_regmap_mux_ro_ops: clk_ops = clk_ops { init:Some(clk_regmap_init), enable:None, disable:None, is_enabled:None, recalc_rate:None, determine_rate:None, set_rate:None, get_parent:Some(clk_regmap_mux_get_parent), set_parent:None };

// MODULE_DESCRIPTION("Amlogic regmap backed clock driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
