// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// C dependencies:
//   <linux/clk-provider.h>
//   <linux/device.h>
//   "clk-uniphier.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk_fixed_rate {
    pub hw: clk_hw,
    pub fixed_rate: u64,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct uniphier_clk_fixed_rate_data {
    pub fixed_rate: u64,
}

extern "C" {
    pub static clk_fixed_rate_ops: clk_ops;

    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    pub fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    pub fn ERR_PTR(error: isize) -> *mut clk_hw;
}

const GFP_KERNEL: u32 = 0;

pub unsafe fn uniphier_clk_register_fixed_rate(
    dev: *mut device,
    name: *const c_char,
    data: *const uniphier_clk_fixed_rate_data,
) -> *mut clk_hw {
    let fixed: *mut clk_fixed_rate;
    let mut init: clk_init_data;
    let ret: c_int;

    /* allocate fixed-rate clock */
    fixed = devm_kzalloc(
        dev,
        core::mem::size_of::<clk_fixed_rate>(),
        GFP_KERNEL,
    ) as *mut clk_fixed_rate;
    if fixed.is_null() {
        return ERR_PTR(-12);
    }

    init = clk_init_data {
        name,
        ops: &clk_fixed_rate_ops,
        flags: 0,
        parent_names: core::ptr::null(),
        num_parents: 0,
    };

    (*fixed).fixed_rate = (*data).fixed_rate;
    (*fixed).hw.init = &mut init;

    ret = devm_clk_hw_register(dev, &mut (*fixed).hw);
    if ret != 0 {
        return ERR_PTR(ret as isize);
    }

    &mut (*fixed).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
