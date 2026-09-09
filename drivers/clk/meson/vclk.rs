// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024 Neil Armstrong <neil.armstrong@linaro.org>
 */

use core::ffi::c_void;

// Declarations supplied by the surrounding kernel clock framework and vclk.h.
#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_rate_request {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct parm {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
}
#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub data: *mut c_void,
}
#[repr(C)]
pub struct meson_vclk_gate_data {
    pub enable: parm,
    pub reset: parm,
}
#[repr(C)]
pub struct meson_vclk_div_data {
    pub div: parm,
    pub reset: parm,
    pub enable: parm,
    pub table: *const c_void,
    pub flags: u32,
}
#[repr(C)]
pub struct clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64, u64) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

extern "C" {
    fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap;
    fn clk_regmap_init(hw: *mut clk_hw) -> i32;
    fn meson_parm_write(map: *mut regmap, parm: *const parm, val: i32);
    fn meson_parm_read(map: *mut regmap, parm: *const parm) -> i32;
    fn divider_recalc_rate(hw: *mut clk_hw, prate: u64, val: i32,
                           table: *const c_void, flags: u32, width: u8) -> u64;
    fn divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request,
                              table: *const c_void, width: u8, flags: u32) -> i32;
    fn divider_get_val(rate: u64, parent_rate: u64, table: *const c_void,
                       width: u8, flags: u32) -> i32;
}

#[inline]
unsafe fn clk_get_meson_vclk_gate_data(clk: *mut clk_regmap) -> *mut meson_vclk_gate_data {
    (*clk).data as *mut meson_vclk_gate_data
}

unsafe extern "C" fn meson_vclk_gate_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let vclk = clk_get_meson_vclk_gate_data(clk);
    meson_parm_write((*clk).map, &(*vclk).enable, 1);
    // Do a reset pulse
    meson_parm_write((*clk).map, &(*vclk).reset, 1);
    meson_parm_write((*clk).map, &(*vclk).reset, 0);
    0
}

unsafe extern "C" fn meson_vclk_gate_disable(hw: *mut clk_hw) {
    let clk = to_clk_regmap(hw);
    let vclk = clk_get_meson_vclk_gate_data(clk);
    meson_parm_write((*clk).map, &(*vclk).enable, 0);
}

unsafe extern "C" fn meson_vclk_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let vclk = clk_get_meson_vclk_gate_data(clk);
    meson_parm_read((*clk).map, &(*vclk).enable)
}

#[no_mangle]
pub static meson_vclk_gate_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init), enable: Some(meson_vclk_gate_enable),
    disable: Some(meson_vclk_gate_disable), is_enabled: Some(meson_vclk_gate_is_enabled),
    recalc_rate: None, determine_rate: None, set_rate: None,
};

#[inline]
unsafe fn clk_get_meson_vclk_div_data(clk: *mut clk_regmap) -> *mut meson_vclk_div_data {
    (*clk).data as *mut meson_vclk_div_data
}

unsafe extern "C" fn meson_vclk_div_recalc_rate(hw: *mut clk_hw, prate: u64) -> u64 {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk);
    divider_recalc_rate(hw, prate, meson_parm_read((*clk).map, &(*vclk).div),
                        (*vclk).table, (*vclk).flags, (*vclk).div.width)
}
unsafe extern "C" fn meson_vclk_div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk);
    divider_determine_rate(hw, req, (*vclk).table, (*vclk).div.width, (*vclk).flags)
}
unsafe extern "C" fn meson_vclk_div_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk);
    let ret = divider_get_val(rate, parent_rate, (*vclk).table, (*vclk).div.width, (*vclk).flags);
    if ret < 0 { return ret; }
    meson_parm_write((*clk).map, &(*vclk).div, ret); 0
}
unsafe extern "C" fn meson_vclk_div_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk);
    // Unreset the divider when ungating
    meson_parm_write((*clk).map, &(*vclk).reset, 0); meson_parm_write((*clk).map, &(*vclk).enable, 1); 0
}
unsafe extern "C" fn meson_vclk_div_disable(hw: *mut clk_hw) {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk);
    // Reset the divider when gating
    meson_parm_write((*clk).map, &(*vclk).enable, 0); meson_parm_write((*clk).map, &(*vclk).reset, 1);
}
unsafe extern "C" fn meson_vclk_div_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw); let vclk = clk_get_meson_vclk_div_data(clk); meson_parm_read((*clk).map, &(*vclk).enable)
}

#[no_mangle]
pub static meson_vclk_div_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init), recalc_rate: Some(meson_vclk_div_recalc_rate),
    determine_rate: Some(meson_vclk_div_determine_rate), set_rate: Some(meson_vclk_div_set_rate),
    enable: Some(meson_vclk_div_enable), disable: Some(meson_vclk_div_disable),
    is_enabled: Some(meson_vclk_div_is_enabled),
};

// MODULE_DESCRIPTION("Amlogic vclk clock driver");
// MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
