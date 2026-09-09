// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 *
 * Sample clock generator divider:
 * This HW divider gates with value 0 but is otherwise a zero based divider:
 *
 * val >= 1
 * divider = val + 1
 *
 * The duty cycle may also be set for the LR clock variant. The duty cycle
 * ratio is:
 *
 * hi = [0 - val]
 * duty_cycle = (1 + hi) / (1 + val)
 */

use core::ffi::{c_int, c_ulong, c_uint};

unsafe extern "C" {
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_get_flags(hw: *mut clk_hw) -> c_ulong;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
    fn clk_regmap_init(hw: *mut clk_hw) -> c_int;
    fn clk_hw_is_enabled(hw: *mut clk_hw) -> bool;
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub data: *mut core::ffi::c_void,
    pub map: *mut regmap,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct meson_sclk_div_data {
    pub div: meson_parm,
    pub hi: meson_parm,
    pub cached_div: c_ulong,
    pub cached_duty: clk_duty,
}

#[repr(C)]
pub struct meson_parm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_duty {
    pub num: c_uint,
    pub den: c_uint,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub get_duty_cycle: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_duty) -> c_int>,
    pub set_duty_cycle: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_duty) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
}

unsafe extern "C" {
    fn meson_parm_read(map: *mut regmap, parm: *const meson_parm) -> c_uint;
    fn meson_parm_write(map: *mut regmap, parm: *const meson_parm, value: c_uint);
}

const CLK_SET_RATE_PARENT: c_ulong = 1 << 5;

#[inline]
unsafe fn meson_sclk_div_data(clk: *mut clk_regmap) -> *mut meson_sclk_div_data {
    (*clk).data as *mut meson_sclk_div_data
}

unsafe fn sclk_div_maxval(sclk: *mut meson_sclk_div_data) -> c_int {
    (1 << (*sclk).div.width) - 1
}

unsafe fn sclk_div_maxdiv(sclk: *mut meson_sclk_div_data) -> c_int {
    sclk_div_maxval(sclk) + 1
}

unsafe fn sclk_div_getdiv(_hw: *mut clk_hw, rate: c_ulong, prate: c_ulong, maxdiv: c_int) -> c_int {
    let div = ((prate as u64 + (rate as u64 / 2)) / rate as u64) as c_int;
    div.clamp(2, maxdiv)
}

unsafe fn sclk_div_bestdiv(hw: *mut clk_hw, mut rate: c_ulong, prate: *mut c_ulong, sclk: *mut meson_sclk_div_data) -> c_int {
    let parent = clk_hw_get_parent(hw);
    let mut bestdiv = 0;
    let mut best: c_ulong = 0;
    let mut best_parent: c_ulong = 0;
    let mut maxdiv = sclk_div_maxdiv(sclk) as c_ulong;

    if rate == 0 { rate = 1; }
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        return sclk_div_getdiv(hw, rate, *prate, maxdiv as c_int);
    }
    maxdiv = (c_ulong::MAX / rate).min(maxdiv);
    for i in 2..=maxdiv {
        if rate * i == *prate { return i as c_int; }
        let parent_now = clk_hw_round_rate(parent, rate * i);
        let now = (parent_now as u64 + i as u64 - 1) / i as u64;
        if (rate as i128 - now as i128).abs() < (rate as i128 - best as i128).abs() {
            bestdiv = i as c_int;
            best = now as c_ulong;
            best_parent = parent_now;
        }
    }
    if bestdiv == 0 { sclk_div_maxdiv(sclk) } else { *prate = best_parent; bestdiv }
}

unsafe fn sclk_div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = hw as *mut clk_regmap;
    let sclk = meson_sclk_div_data(clk);
    let div = sclk_div_bestdiv(hw, (*req).rate, &mut (*req).best_parent_rate, sclk);
    (*req).rate = ((*req).best_parent_rate as u64 + div as u64 - 1) / div as u64;
    0
}

unsafe fn sclk_apply_ratio(clk: *mut clk_regmap, sclk: *mut meson_sclk_div_data) {
    let mut hi = (((*sclk).cached_div * (*sclk).cached_duty.num as c_ulong) / (*sclk).cached_duty.den as c_ulong) as c_uint;
    if hi != 0 { hi -= 1; }
    meson_parm_write((*clk).map, &(*sclk).hi, hi);
}

unsafe fn sclk_div_set_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> c_int {
    let clk = hw as *mut clk_regmap;
    let sclk = meson_sclk_div_data(clk);
    if meson_parm_applicable(&(*sclk).hi) {
        (*sclk).cached_duty = *duty;
        sclk_apply_ratio(clk, sclk);
    }
    0
}

unsafe fn sclk_div_get_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> c_int {
    let clk = hw as *mut clk_regmap;
    let sclk = meson_sclk_div_data(clk);
    if !meson_parm_applicable(&(*sclk).hi) { (*duty).num = 1; (*duty).den = 2; return 0; }
    let hi = meson_parm_read((*clk).map, &(*sclk).hi);
    (*duty).num = hi + 1;
    (*duty).den = (*sclk).cached_div as c_uint;
    0
}

unsafe fn sclk_apply_divider(clk: *mut clk_regmap, sclk: *mut meson_sclk_div_data) {
    if meson_parm_applicable(&(*sclk).hi) { sclk_apply_ratio(clk, sclk); }
    meson_parm_write((*clk).map, &(*sclk).div, (*sclk).cached_div as c_uint - 1);
}

unsafe fn sclk_div_set_rate(hw: *mut clk_hw, rate: c_ulong, prate: c_ulong) -> c_int {
    let clk = hw as *mut clk_regmap;
    let sclk = meson_sclk_div_data(clk);
    (*sclk).cached_div = sclk_div_getdiv(hw, rate, prate, sclk_div_maxdiv(sclk)) as c_ulong;
    if clk_hw_is_enabled(hw) { sclk_apply_divider(clk, sclk); }
    0
}

unsafe fn sclk_div_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    let sclk = meson_sclk_div_data(hw as *mut clk_regmap);
    (prate as u64 + (*sclk).cached_div - 1) / (*sclk).cached_div
}

unsafe fn sclk_div_enable(hw: *mut clk_hw) -> c_int { sclk_apply_divider(hw as *mut clk_regmap, meson_sclk_div_data(hw as *mut clk_regmap)); 0 }
unsafe fn sclk_div_disable(hw: *mut clk_hw) { let clk = hw as *mut clk_regmap; let sclk = meson_sclk_div_data(clk); meson_parm_write((*clk).map, &(*sclk).div, 0); }
unsafe fn sclk_div_is_enabled(hw: *mut clk_hw) -> c_int { let clk = hw as *mut clk_regmap; let sclk = meson_sclk_div_data(clk); if meson_parm_read((*clk).map, &(*sclk).div) != 0 { 1 } else { 0 } }

unsafe fn sclk_div_init(hw: *mut clk_hw) -> c_int {
    let clk = hw as *mut clk_regmap;
    let sclk = meson_sclk_div_data(clk);
    let ret = clk_regmap_init(hw);
    if ret != 0 { return ret; }
    let val = meson_parm_read((*clk).map, &(*sclk).div);
    (*sclk).cached_div = if val == 0 { sclk_div_maxdiv(sclk) as c_ulong } else { (val + 1) as c_ulong };
    sclk_div_get_duty_cycle(hw, &mut (*sclk).cached_duty);
    0
}

unsafe extern "C" { fn meson_parm_applicable(parm: *const meson_parm) -> bool; }

#[no_mangle]
pub static meson_sclk_div_ops: clk_ops = clk_ops {
    recalc_rate: Some(sclk_div_recalc_rate), determine_rate: Some(sclk_div_determine_rate),
    set_rate: Some(sclk_div_set_rate), enable: Some(sclk_div_enable), disable: Some(sclk_div_disable),
    is_enabled: Some(sclk_div_is_enabled), get_duty_cycle: Some(sclk_div_get_duty_cycle),
    set_duty_cycle: Some(sclk_div_set_duty_cycle), init: Some(sclk_div_init),
};

// EXPORT_SYMBOL_NS_GPL(meson_sclk_div_ops, "CLK_MESON");
// MODULE_DESCRIPTION("Amlogic Sample divider driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
