// SPDX-License-Identifier: GPL-2.0+
//
// OWL factor clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

use core::ffi::c_int;

// Declarations supplied by the Linux clock and regmap dependencies.
#[repr(C)]
pub struct clk_factor_table {
    pub val: u32,
    pub mul: u32,
    pub div: u32,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

pub type c_ulong = usize;
pub type u32_t = u32;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct owl_clk_common {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct owl_factor_hw {
    pub table: *const clk_factor_table,
    pub reg: u32,
    pub shift: u32,
    pub fct_flags: u32,
}

#[repr(C)]
pub struct owl_factor {
    pub common: owl_clk_common,
    pub factor_hw: owl_factor_hw,
}

#[repr(C)]
pub struct clk_ops {
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}

extern "C" {
    fn clk_hw_get_flags(hw: *mut clk_hw) -> u32;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn __clk_get_name(clk: *mut core::ffi::c_void) -> *const i8;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const CLK_DIVIDER_ALLOW_ZERO: u32 = 1 << 4;

unsafe fn div_mask(factor_hw: *const owl_factor_hw) -> u32 {
    // Supplied by owl-factor.h in the original translation unit.
    (*(factor_hw)).fct_flags
}

unsafe fn hw_to_owl_factor(hw: *mut clk_hw) -> *mut owl_factor {
    hw as *mut owl_factor
}

unsafe fn _get_table_maxval(table: *const clk_factor_table) -> u32 {
    let mut maxval = 0;
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).val > maxval {
            maxval = (*clkt).val;
        }
        clkt = clkt.add(1);
    }
    maxval
}

unsafe fn _get_table_div_mul(table: *const clk_factor_table, val: u32, mul: *mut u32, div: *mut u32) -> c_int {
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).val == val {
            *mul = (*clkt).mul;
            *div = (*clkt).div;
            return 1;
        }
        clkt = clkt.add(1);
    }
    0
}

unsafe fn _get_table_val(table: *const clk_factor_table, rate: c_ulong, parent_rate: c_ulong) -> u32 {
    let mut clkt = table;
    let mut val: i32 = -1;
    while (*clkt).div != 0 {
        let calc_rate = parent_rate.wrapping_mul((*clkt).mul as usize) / (*clkt).div as usize;
        if calc_rate <= rate {
            val = (*clkt).val as i32;
            break;
        }
        clkt = clkt.add(1);
    }
    if val == -1 { _get_table_maxval(table) } else { val as u32 }
}

unsafe fn owl_clk_val_best(factor_hw: *const owl_factor_hw, hw: *mut clk_hw, mut rate: c_ulong, best_parent_rate: *mut c_ulong) -> c_int {
    let table = (*factor_hw).table;
    let parent_saved = *best_parent_rate;
    if rate == 0 { rate = 1; }
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        return _get_table_val(table, rate, parent_saved) as c_int;
    }
    let mut clkt = table;
    let mut best = 0;
    let mut bestval = 0;
    while (*clkt).div != 0 {
        let try_parent_rate = rate * (*clkt).div as usize / (*clkt).mul as usize;
        if try_parent_rate == parent_saved {
            *best_parent_rate = parent_saved;
            return (*clkt).val as c_int;
        }
        let parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), try_parent_rate);
        let cur_rate = (parent_rate + (*clkt).div as usize - 1) / (*clkt).div as usize * (*clkt).mul as usize;
        if cur_rate <= rate && cur_rate > best {
            bestval = (*clkt).val;
            best = cur_rate;
            *best_parent_rate = parent_rate;
        }
        clkt = clkt.add(1);
    }
    if bestval == 0 {
        bestval = _get_table_maxval(clkt);
        *best_parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), 1);
    }
    bestval as c_int
}

pub unsafe extern "C" fn owl_factor_helper_round_rate(common: *mut owl_clk_common, factor_hw: *const owl_factor_hw, rate: c_ulong, parent_rate: *mut c_ulong) -> c_ulong {
    let mut mul = 0;
    let mut div = 1;
    let val = owl_clk_val_best(factor_hw, &mut (*common).hw, rate, parent_rate);
    _get_table_div_mul((*factor_hw).table, val as u32, &mut mul, &mut div);
    *parent_rate * mul as usize / div as usize
}

unsafe extern "C" fn owl_factor_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let factor = hw_to_owl_factor(hw);
    (*req).rate = owl_factor_helper_round_rate(&mut (*factor).common, &(*factor).factor_hw, (*req).rate, &mut (*req).best_parent_rate);
    0
}

pub unsafe extern "C" fn owl_factor_helper_recalc_rate(common: *mut owl_clk_common, factor_hw: *const owl_factor_hw, parent_rate: c_ulong) -> c_ulong {
    let mut reg = 0;
    let mut mul = 0;
    let mut div = 0;
    regmap_read((*common).regmap, (*factor_hw).reg, &mut reg);
    let val = (reg >> (*factor_hw).shift) & div_mask(factor_hw);
    _get_table_div_mul((*factor_hw).table, val, &mut mul, &mut div);
    if div == 0 { return parent_rate; }
    parent_rate * mul as usize / div as usize
}

unsafe extern "C" fn owl_factor_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let factor = hw_to_owl_factor(hw);
    owl_factor_helper_recalc_rate(&mut (*factor).common, &(*factor).factor_hw, parent_rate)
}

pub unsafe extern "C" fn owl_factor_helper_set_rate(common: *const owl_clk_common, factor_hw: *const owl_factor_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let mask = div_mask(factor_hw);
    let mut val = _get_table_val((*factor_hw).table, rate, parent_rate);
    if val > mask { val = mask; }
    let mut reg = 0;
    regmap_read((*common).regmap, (*factor_hw).reg, &mut reg);
    reg &= !(mask << (*factor_hw).shift);
    reg |= val << (*factor_hw).shift;
    regmap_write((*common).regmap, (*factor_hw).reg, reg);
    0
}

unsafe extern "C" fn owl_factor_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let factor = hw_to_owl_factor(hw);
    owl_factor_helper_set_rate(&(*factor).common, &(*factor).factor_hw, rate, parent_rate)
}

#[no_mangle]
pub static owl_factor_ops: clk_ops = clk_ops {
    determine_rate: Some(owl_factor_determine_rate),
    recalc_rate: Some(owl_factor_recalc_rate),
    set_rate: Some(owl_factor_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
