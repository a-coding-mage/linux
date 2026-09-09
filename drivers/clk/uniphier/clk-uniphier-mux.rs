// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Dependencies supplied by the corresponding Linux clock, device, regmap, and
// UniPhier headers are intentionally left as external Rust items.

#[repr(C)]
struct uniphier_clk_mux {
    hw: clk_hw,
    regmap: *mut regmap,
    reg: ::core::ffi::c_uint,
    masks: *const ::core::ffi::c_uint,
    vals: *const ::core::ffi::c_uint,
}

// Equivalent to container_of(_hw, struct uniphier_clk_mux, hw).
unsafe fn to_uniphier_clk_mux(hw: *mut clk_hw) -> *mut uniphier_clk_mux {
    hw as *mut uniphier_clk_mux
}

unsafe fn uniphier_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> ::core::ffi::c_int {
    let mux = &mut *to_uniphier_clk_mux(hw);

    regmap_write_bits(
        mux.regmap,
        mux.reg,
        *mux.masks.add(index as usize),
        *mux.vals.add(index as usize),
    )
}

unsafe fn uniphier_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = &mut *to_uniphier_clk_mux(hw);
    let num_parents: ::core::ffi::c_uint = clk_hw_get_num_parents(hw);
    let mut val: ::core::ffi::c_uint = 0;

    let ret = regmap_read(mux.regmap, mux.reg, &mut val);
    if ret != 0 {
        return ret as u8;
    }

    let mut i: ::core::ffi::c_uint = 0;
    while i < num_parents {
        if (*mux.masks.add(i as usize) & val) == *mux.vals.add(i as usize) {
            return i as u8;
        }
        i = i.wrapping_add(1);
    }

    (-EINVAL) as u8
}

static uniphier_clk_mux_ops: clk_ops = clk_ops {
    determine_rate: __clk_mux_determine_rate,
    set_parent: uniphier_clk_mux_set_parent,
    get_parent: uniphier_clk_mux_get_parent,
};

unsafe fn uniphier_clk_register_mux(
    dev: *mut device,
    regmap: *mut regmap,
    name: *const ::core::ffi::c_char,
    data: *const uniphier_clk_mux_data,
) -> *mut clk_hw {
    let mux = devm_kzalloc(
        dev,
        ::core::mem::size_of::<uniphier_clk_mux>(),
        GFP_KERNEL,
    ) as *mut uniphier_clk_mux;
    if mux.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let mut init: clk_init_data = ::core::mem::zeroed();
    init.name = name;
    init.ops = &uniphier_clk_mux_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = (*data).parent_names;
    init.num_parents = (*data).num_parents;

    (*mux).regmap = regmap;
    (*mux).reg = (*data).reg;
    (*mux).masks = (*data).masks;
    (*mux).vals = (*data).vals;
    (*mux).hw.init = &init;

    let ret = devm_clk_hw_register(dev, &mut (*mux).hw);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    &mut (*mux).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
