// SPDX-License-Identifier: GPL-2.0+
//
// OWL gate clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies supplied by the Linux clock-provider, regmap, and owl-gate
// interfaces are intentionally left as external Rust items.

pub unsafe fn owl_gate_set(
    common: *const owl_clk_common,
    gate_hw: *const owl_gate_hw,
    enable: bool,
) {
    let mut set: i32 = if ((*gate_hw).gate_flags & CLK_GATE_SET_TO_DISABLE) != 0 {
        1
    } else {
        0
    };
    let mut reg: u32 = 0;

    set ^= enable as i32;

    regmap_read((*common).regmap, (*gate_hw).reg, &mut reg);

    if set != 0 {
        reg |= 1u32.wrapping_shl((*gate_hw).bit_idx);
    } else {
        reg &= !(1u32.wrapping_shl((*gate_hw).bit_idx));
    }

    regmap_write((*common).regmap, (*gate_hw).reg, reg);
}

unsafe fn owl_gate_disable(hw: *mut clk_hw) {
    let gate: *mut owl_gate = hw_to_owl_gate(hw);
    let common: *mut owl_clk_common = &mut (*gate).common;

    owl_gate_set(common, &(*gate).gate_hw, false);
}

unsafe fn owl_gate_enable(hw: *mut clk_hw) -> i32 {
    let gate: *mut owl_gate = hw_to_owl_gate(hw);
    let common: *mut owl_clk_common = &mut (*gate).common;

    owl_gate_set(common, &(*gate).gate_hw, true);

    0
}

pub unsafe fn owl_gate_clk_is_enabled(
    common: *const owl_clk_common,
    gate_hw: *const owl_gate_hw,
) -> i32 {
    let mut reg: u32 = 0;

    regmap_read((*common).regmap, (*gate_hw).reg, &mut reg);

    if ((*gate_hw).gate_flags & CLK_GATE_SET_TO_DISABLE) != 0 {
        reg ^= 1u32.wrapping_shl((*gate_hw).bit_idx);
    }

    if (reg & 1u32.wrapping_shl((*gate_hw).bit_idx)) != 0 {
        1
    } else {
        0
    }
}

unsafe fn owl_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate: *mut owl_gate = hw_to_owl_gate(hw);
    let common: *mut owl_clk_common = &mut (*gate).common;

    owl_gate_clk_is_enabled(common, &(*gate).gate_hw)
}

pub static owl_gate_ops: clk_ops = clk_ops {
    disable: Some(owl_gate_disable),
    enable: Some(owl_gate_enable),
    is_enabled: Some(owl_gate_is_enabled),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
