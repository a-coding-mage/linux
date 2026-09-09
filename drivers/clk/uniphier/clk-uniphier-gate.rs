// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Dependencies supplied by the surrounding kernel/uniphier code.

#[repr(C)]
pub struct uniphier_clk_gate {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: ::core::ffi::c_uint,
    pub bit: ::core::ffi::c_uint,
}

#[inline]
unsafe fn to_uniphier_clk_gate(hw: *mut clk_hw) -> *mut uniphier_clk_gate {
    // `hw` is the first field of uniphier_clk_gate, matching container_of.
    hw as *mut uniphier_clk_gate
}

unsafe fn uniphier_clk_gate_endisable(hw: *mut clk_hw, enable: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let gate = &mut *to_uniphier_clk_gate(hw);

    regmap_write_bits(
        gate.regmap,
        gate.reg,
        1u32.wrapping_shl(gate.bit),
        if enable != 0 { 1u32.wrapping_shl(gate.bit) } else { 0 },
    )
}

unsafe fn uniphier_clk_gate_enable(hw: *mut clk_hw) -> ::core::ffi::c_int {
    uniphier_clk_gate_endisable(hw, 1)
}

unsafe fn uniphier_clk_gate_disable(hw: *mut clk_hw) {
    if uniphier_clk_gate_endisable(hw, 0) < 0 {
        pr_warn!("failed to disable clk\n");
    }
}

unsafe fn uniphier_clk_gate_is_enabled(hw: *mut clk_hw) -> ::core::ffi::c_int {
    let gate = &mut *to_uniphier_clk_gate(hw);
    let mut val: ::core::ffi::c_uint = 0;

    if regmap_read(gate.regmap, gate.reg, &mut val) < 0 {
        pr_warn!("is_enabled() may return wrong result\n");
    }

    if (val & 1u32.wrapping_shl(gate.bit)) != 0 { 1 } else { 0 }
}

static uniphier_clk_gate_ops: clk_ops = clk_ops {
    enable: Some(uniphier_clk_gate_enable),
    disable: Some(uniphier_clk_gate_disable),
    is_enabled: Some(uniphier_clk_gate_is_enabled),
};

pub unsafe fn uniphier_clk_register_gate(
    dev: *mut device,
    regmap: *mut regmap,
    name: *const ::core::ffi::c_char,
    data: *const uniphier_clk_gate_data,
) -> *mut clk_hw {
    let gate: *mut uniphier_clk_gate;
    let mut init: clk_init_data;
    let ret: ::core::ffi::c_int;

    gate = devm_kzalloc(dev, ::core::mem::size_of::<uniphier_clk_gate>(), GFP_KERNEL) as *mut uniphier_clk_gate;
    if gate.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*gate).hw.init = &mut init;
    init.name = name;
    init.ops = &uniphier_clk_gate_ops;
    init.flags = if !(*data).parent_name.is_null() { CLK_SET_RATE_PARENT } else { 0 };
    init.parent_names = if !(*data).parent_name.is_null() { &(*data).parent_name } else { ::core::ptr::null() };
    init.num_parents = if !(*data).parent_name.is_null() { 1 } else { 0 };

    (*gate).regmap = regmap;
    (*gate).reg = (*data).reg;
    (*gate).bit = (*data).bit;

    ret = devm_clk_hw_register(dev, &mut (*gate).hw);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    &mut (*gate).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
