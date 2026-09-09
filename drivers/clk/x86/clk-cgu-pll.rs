// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 MaxLinear, Inc.
 * Copyright (C) 2020 Intel Corporation.
 * Zhu Yixin <yzhu@maxlinear.com>
 * Rahul Tanwar <rtanwar@maxlinear.com>
 */

// Dependencies are supplied by the surrounding kernel translation.
use core::ffi::c_void;

const TYPE_LJPLL: u32 = 0;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub ops: *const clk_ops,
    pub name: *const i8,
    pub flags: u32,
    pub parent_data: *const c_void,
    pub num_parents: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
}

#[repr(C)]
pub struct lgm_clk_pll {
    pub hw: clk_hw,
    pub membase: *mut c_void,
    pub reg: u32,
    pub flags: u32,
    pub r#type: u32,
}

#[repr(C)]
pub struct lgm_clk_provider {
    pub dev: *mut c_void,
    pub membase: *mut c_void,
    pub clk_data: lgm_clk_data,
}

#[repr(C)]
pub struct lgm_clk_data {
    pub hws: *mut *mut clk_hw,
}

#[repr(C)]
pub struct lgm_pll_clk_data {
    pub name: *const i8,
    pub flags: u32,
    pub parent_data: *const c_void,
    pub num_parents: u32,
    pub reg: u32,
    pub r#type: u32,
    pub id: usize,
}

extern "C" {
    fn lgm_get_clk_val(base: *mut c_void, reg: u32, shift: u32, width: u32) -> u32;
    fn lgm_set_clk_val(base: *mut c_void, reg: u32, shift: u32, width: u32, val: u32);
    fn regmap_read_poll_timeout_atomic(
        base: *mut c_void, reg: u32, val: *mut u32, condition: u32, delay: u32,
        timeout: u32,
    ) -> i32;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: u32) -> *mut c_void;
    fn devm_clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> i32;
    fn dev_err(dev: *mut c_void, fmt: *const i8, ...);
}

const fn pll_ref_div(x: u32) -> u32 { x + 0x08 }

unsafe fn lgm_pll_calc_rate(
    prate: usize, mult: u32, div: u32, frac: u32, frac_div: u32,
) -> usize {
    let mut crate_: u128 = prate as u128 * mult as u128;
    let frate: u128 = (prate as u128 * frac as u128) / frac_div as u128;
    crate_ += frate;
    (crate_ / div as u128) as usize
}

unsafe extern "C" fn lgm_pll_recalc_rate(hw: *mut clk_hw, prate: usize) -> usize {
    let pll = hw as *mut lgm_clk_pll;
    let mult = lgm_get_clk_val((*pll).membase, pll_ref_div((*pll).reg), 0, 12);
    let mut div = lgm_get_clk_val((*pll).membase, pll_ref_div((*pll).reg), 18, 6);
    let frac = lgm_get_clk_val((*pll).membase, (*pll).reg, 2, 24);
    if (*pll).r#type == TYPE_LJPLL { div *= 4; }
    lgm_pll_calc_rate(prate, mult, div, frac, 1u32 << 24)
}

unsafe extern "C" fn lgm_pll_is_enabled(hw: *mut clk_hw) -> i32 {
    let pll = hw as *mut lgm_clk_pll;
    lgm_get_clk_val((*pll).membase, (*pll).reg, 0, 1) as i32
}

unsafe extern "C" fn lgm_pll_enable(hw: *mut clk_hw) -> i32 {
    let pll = hw as *mut lgm_clk_pll;
    let mut val = 0u32;
    lgm_set_clk_val((*pll).membase, (*pll).reg, 0, 1, 1);
    regmap_read_poll_timeout_atomic((*pll).membase, (*pll).reg, &mut val, val & 0x1, 1, 100)
}

unsafe extern "C" fn lgm_pll_disable(hw: *mut clk_hw) {
    let pll = hw as *mut lgm_clk_pll;
    lgm_set_clk_val((*pll).membase, (*pll).reg, 0, 1, 0);
}

static LGM_PLL_OPS: clk_ops = clk_ops {
    recalc_rate: Some(lgm_pll_recalc_rate),
    is_enabled: Some(lgm_pll_is_enabled),
    enable: Some(lgm_pll_enable),
    disable: Some(lgm_pll_disable),
};

unsafe fn lgm_clk_register_pll(
    ctx: *mut lgm_clk_provider, list: *const lgm_pll_clk_data,
) -> *mut clk_hw {
    let mut init = clk_init_data {
        ops: &LGM_PLL_OPS,
        name: (*list).name,
        flags: (*list).flags,
        parent_data: (*list).parent_data,
        num_parents: (*list).num_parents,
    };
    let dev = (*ctx).dev;
    let pll = devm_kzalloc(dev, core::mem::size_of::<lgm_clk_pll>(), GFP_KERNEL) as *mut lgm_clk_pll;
    if pll.is_null() { return (-12isize) as *mut clk_hw; }
    (*pll).membase = (*ctx).membase;
    (*pll).reg = (*list).reg;
    (*pll).flags = (*list).flags;
    (*pll).r#type = (*list).r#type;
    (*pll).hw.init = &mut init;
    let hw = &mut (*pll).hw;
    let ret = devm_clk_hw_register(dev, hw);
    if ret != 0 { return ret as isize as *mut clk_hw; }
    hw
}

pub unsafe extern "C" fn lgm_clk_register_plls(
    ctx: *mut lgm_clk_provider, mut list: *const lgm_pll_clk_data, nr_clk: u32,
) -> i32 {
    for _ in 0..nr_clk {
        let hw = lgm_clk_register_pll(ctx, list);
        if hw as isize == -12 {
            dev_err((*ctx).dev, b"failed to register pll: %s\0".as_ptr() as *const i8, (*list).name);
            return -12;
        }
        *(*ctx).clk_data.hws.add((*list).id) = hw;
        list = list.add(1);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
