// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Dependencies supplied by the Linux clock-provider, device, and Uniphier
// headers are referenced here as external Rust items.

pub unsafe fn uniphier_clk_register_fixed_factor(
    dev: *mut device,
    name: *const core::ffi::c_char,
    data: *const uniphier_clk_fixed_factor_data,
) -> *mut clk_hw {
    let mut fix: *mut clk_fixed_factor;
    let mut init: clk_init_data;
    let mut ret: i32;

    fix = devm_kzalloc(dev, core::mem::size_of::<clk_fixed_factor>(), GFP_KERNEL);
    if fix.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*(&mut init as *mut clk_init_data)).name = name;
    (*(&mut init as *mut clk_init_data)).ops = &clk_fixed_factor_ops;
    (*(&mut init as *mut clk_init_data)).flags = if !(*data).parent_name.is_null() {
        CLK_SET_RATE_PARENT
    } else {
        0
    };
    (*(&mut init as *mut clk_init_data)).parent_names = if !(*data).parent_name.is_null() {
        &(*data).parent_name
    } else {
        core::ptr::null()
    };
    (*(&mut init as *mut clk_init_data)).num_parents = if !(*data).parent_name.is_null() {
        1
    } else {
        0
    };

    (*fix).mult = (*data).mult;
    (*fix).div = (*data).div;
    (*fix).hw.init = &init;

    ret = devm_clk_hw_register(dev, &mut (*fix).hw);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    &mut (*fix).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
