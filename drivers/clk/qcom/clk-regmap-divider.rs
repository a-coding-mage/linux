// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel clock/regmap implementation.

#[inline]
unsafe fn to_clk_regmap_div(hw: *mut clk_hw) -> *mut clk_regmap_div {
    container_of(to_clk_regmap(hw), clk_regmap_div, clkr)
}

unsafe fn div_ro_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let divider: *mut clk_regmap_div = to_clk_regmap_div(hw);
    let clkr: *mut clk_regmap = &mut (*divider).clkr;
    let mut val: u32 = 0;

    regmap_read((*clkr).regmap, (*divider).reg, &mut val);
    val >>= (*divider).shift;
    val &= bit((*divider).width) - 1;

    divider_ro_determine_rate(
        hw,
        req,
        core::ptr::null_mut(),
        (*divider).width,
        CLK_DIVIDER_ROUND_CLOSEST,
        val,
    )
}

unsafe fn div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let divider: *mut clk_regmap_div = to_clk_regmap_div(hw);

    divider_determine_rate(
        hw,
        req,
        core::ptr::null_mut(),
        (*divider).width,
        CLK_DIVIDER_ROUND_CLOSEST,
    )
}

unsafe fn div_set_rate(
    hw: *mut clk_hw,
    rate: core::ffi::c_ulong,
    parent_rate: core::ffi::c_ulong,
) -> i32 {
    let divider: *mut clk_regmap_div = to_clk_regmap_div(hw);
    let clkr: *mut clk_regmap = &mut (*divider).clkr;
    let div: u32 = divider_get_val(
        rate,
        parent_rate,
        core::ptr::null_mut(),
        (*divider).width,
        CLK_DIVIDER_ROUND_CLOSEST,
    );

    regmap_update_bits(
        (*clkr).regmap,
        (*divider).reg,
        (bit((*divider).width) - 1) << (*divider).shift,
        div << (*divider).shift,
    )
}

unsafe fn div_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    let divider: *mut clk_regmap_div = to_clk_regmap_div(hw);
    let clkr: *mut clk_regmap = &mut (*divider).clkr;
    let mut div: u32 = 0;

    regmap_read((*clkr).regmap, (*divider).reg, &mut div);
    div >>= (*divider).shift;
    div &= bit((*divider).width) - 1;

    divider_recalc_rate(
        hw,
        parent_rate,
        div,
        core::ptr::null_mut(),
        CLK_DIVIDER_ROUND_CLOSEST,
        (*divider).width,
    )
}

pub static clk_regmap_div_ops: clk_ops = clk_ops {
    determine_rate: Some(div_determine_rate),
    set_rate: Some(div_set_rate),
    recalc_rate: Some(div_recalc_rate),
};

pub static clk_regmap_div_ro_ops: clk_ops = clk_ops {
    determine_rate: Some(div_ro_determine_rate),
    recalc_rate: Some(div_recalc_rate),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
