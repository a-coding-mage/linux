// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Dependencies corresponding to the original Linux kernel includes are
// supplied by the surrounding translation.

unsafe fn to_clk_regmap_mux(hw: *mut clk_hw) -> *mut clk_regmap_mux {
    container_of!(to_clk_regmap(hw), clk_regmap_mux, clkr)
}

unsafe fn mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_clk_regmap_mux(hw);
    let clkr = to_clk_regmap(hw);
    let mask: u32 = (1u32 << ((*mux).width as u32 - 1 + 1)) - 1;
    let mut val: u32 = 0;

    regmap_read((*clkr).regmap, (*mux).reg, &mut val);

    val >>= (*mux).shift;
    val &= mask;

    if !(*mux).parent_map.is_null() {
        return qcom_find_cfg_index(hw, (*mux).parent_map, val);
    }

    val as u8
}

unsafe fn mux_set_parent(hw: *mut clk_hw, mut index: u8) -> i32 {
    let mux = to_clk_regmap_mux(hw);
    let clkr = to_clk_regmap(hw);
    let mask: u32 = ((1u32 << ((*mux).width as u32 + (*mux).shift as u32)) - 1)
        & !((1u32 << (*mux).shift as u32) - 1);
    let mut val: u32;

    if !(*mux).parent_map.is_null() {
        index = (*(*mux).parent_map.add(index as usize)).cfg;
    }

    val = index as u32;
    val <<= (*mux).shift;

    regmap_update_bits((*clkr).regmap, (*mux).reg, mask, val)
}

pub static clk_regmap_mux_closest_ops: clk_ops = clk_ops {
    get_parent: Some(mux_get_parent),
    set_parent: Some(mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate_closest),
};

EXPORT_SYMBOL_GPL!(clk_regmap_mux_closest_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
