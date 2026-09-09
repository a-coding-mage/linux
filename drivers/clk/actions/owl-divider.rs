// SPDX-License-Identifier: GPL-2.0+
//
// OWL divider clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies supplied by the surrounding clock-provider, regmap, and
// owl-divider interfaces are intentionally left external to this translation.

unsafe fn owl_divider_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let div: *mut owl_divider = hw_to_owl_divider(hw);

    divider_determine_rate(
        hw,
        req,
        (*div).div_hw.table,
        (*div).div_hw.width,
        (*div).div_hw.div_flags,
    )
}

pub unsafe fn owl_divider_helper_recalc_rate(
    common: *mut owl_clk_common,
    div_hw: *const owl_divider_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let mut val: libc::c_ulong;
    let mut reg: libc::c_uint = 0;

    regmap_read((*common).regmap, (*div_hw).reg, &mut reg);
    val = (reg >> (*div_hw).shift) as libc::c_ulong;
    val &= ((1u64 << (*div_hw).width) - 1) as libc::c_ulong;

    divider_recalc_rate(
        &(*common).hw,
        parent_rate,
        val,
        (*div_hw).table,
        (*div_hw).div_flags,
        (*div_hw).width,
    )
}

unsafe fn owl_divider_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let div: *mut owl_divider = hw_to_owl_divider(hw);

    owl_divider_helper_recalc_rate(&mut (*div).common, &(*div).div_hw, parent_rate)
}

pub unsafe fn owl_divider_helper_set_rate(
    common: *const owl_clk_common,
    div_hw: *const owl_divider_hw,
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
) -> i32 {
    let val: libc::c_ulong;
    let mut reg: libc::c_uint = 0;

    val = divider_get_val(
        rate,
        parent_rate,
        (*div_hw).table,
        (*div_hw).width,
        0,
    );

    regmap_read((*common).regmap, (*div_hw).reg, &mut reg);
    reg &= !genmask((*div_hw).width + (*div_hw).shift - 1, (*div_hw).shift);

    regmap_write(
        (*common).regmap,
        (*div_hw).reg,
        reg | ((val << (*div_hw).shift) as libc::c_uint),
    );

    0
}

unsafe fn owl_divider_set_rate(
    hw: *mut clk_hw,
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
) -> i32 {
    let div: *mut owl_divider = hw_to_owl_divider(hw);

    owl_divider_helper_set_rate(&(*div).common, &(*div).div_hw, rate, parent_rate)
}

pub static owl_divider_ops: clk_ops = clk_ops {
    recalc_rate: Some(owl_divider_recalc_rate),
    determine_rate: Some(owl_divider_determine_rate),
    set_rate: Some(owl_divider_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
