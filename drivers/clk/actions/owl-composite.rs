// SPDX-License-Identifier: GPL-2.0+
//
// OWL composite clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies supplied by the surrounding kernel clock-driver implementation.

unsafe fn owl_comp_get_parent(hw: *mut clk_hw) -> u8 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_mux_helper_get_parent(&mut (*comp).common, &mut (*comp).mux_hw)
}

unsafe fn owl_comp_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_mux_helper_set_parent(&mut (*comp).common, &mut (*comp).mux_hw, index)
}

unsafe fn owl_comp_disable(hw: *mut clk_hw) {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let common: *mut owl_clk_common = &mut (*comp).common;

    owl_gate_set(common, &mut (*comp).gate_hw, false);
}

unsafe fn owl_comp_enable(hw: *mut clk_hw) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let common: *mut owl_clk_common = &mut (*comp).common;

    owl_gate_set(common, &mut (*comp).gate_hw, true);

    0
}

unsafe fn owl_comp_is_enabled(hw: *mut clk_hw) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let common: *mut owl_clk_common = &mut (*comp).common;

    owl_gate_clk_is_enabled(common, &mut (*comp).gate_hw)
}

unsafe fn owl_comp_div_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let div: *mut owl_divider_hw = &mut (*comp).rate.div_hw;

    divider_determine_rate(
        &mut (*comp).common.hw,
        req,
        (*div).table,
        (*div).width,
        (*div).div_flags,
    )
}

unsafe fn owl_comp_div_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_divider_helper_recalc_rate(&mut (*comp).common, &mut (*comp).rate.div_hw, parent_rate)
}

unsafe fn owl_comp_div_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_divider_helper_set_rate(&mut (*comp).common, &mut (*comp).rate.div_hw, rate, parent_rate)
}

unsafe fn owl_comp_fact_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let rate: c_long;

    rate = owl_factor_helper_round_rate(
        &mut (*comp).common,
        &mut (*comp).rate.factor_hw,
        (*req).rate,
        &mut (*req).best_parent_rate,
    );
    if rate < 0 {
        return rate as i32;
    }

    (*req).rate = rate as c_ulong;
    0
}

unsafe fn owl_comp_fact_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_factor_helper_recalc_rate(&mut (*comp).common, &mut (*comp).rate.factor_hw, parent_rate)
}

unsafe fn owl_comp_fact_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);

    owl_factor_helper_set_rate(&mut (*comp).common, &mut (*comp).rate.factor_hw, rate, parent_rate)
}

unsafe fn owl_comp_fix_fact_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let fix_fact_hw: *mut clk_fixed_factor = &mut (*comp).rate.fix_fact_hw;

    ((*(*comp).fix_fact_ops).determine_rate)(&mut (*fix_fact_hw).hw, req)
}

unsafe fn owl_comp_fix_fact_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let comp: *mut owl_composite = hw_to_owl_comp(hw);
    let fix_fact_hw: *mut clk_fixed_factor = &mut (*comp).rate.fix_fact_hw;

    ((*(*comp).fix_fact_ops).recalc_rate)(&mut (*fix_fact_hw).hw, parent_rate)
}

unsafe fn owl_comp_fix_fact_set_rate(
    _hw: *mut clk_hw,
    _rate: c_ulong,
    _parent_rate: c_ulong,
) -> i32 {
    /*
     * We must report success but we can do so unconditionally because
     * owl_comp_fix_fact_round_rate returns values that ensure this call is
     * a nop.
     */

    0
}

pub static mut owl_comp_div_ops: clk_ops = clk_ops {
    get_parent: Some(owl_comp_get_parent),
    set_parent: Some(owl_comp_set_parent),
    disable: Some(owl_comp_disable),
    enable: Some(owl_comp_enable),
    is_enabled: Some(owl_comp_is_enabled),
    determine_rate: Some(owl_comp_div_determine_rate),
    recalc_rate: Some(owl_comp_div_recalc_rate),
    set_rate: Some(owl_comp_div_set_rate),
};

pub static mut owl_comp_fact_ops: clk_ops = clk_ops {
    get_parent: Some(owl_comp_get_parent),
    set_parent: Some(owl_comp_set_parent),
    disable: Some(owl_comp_disable),
    enable: Some(owl_comp_enable),
    is_enabled: Some(owl_comp_is_enabled),
    determine_rate: Some(owl_comp_fact_determine_rate),
    recalc_rate: Some(owl_comp_fact_recalc_rate),
    set_rate: Some(owl_comp_fact_set_rate),
};

pub static mut owl_comp_fix_fact_ops: clk_ops = clk_ops {
    disable: Some(owl_comp_disable),
    enable: Some(owl_comp_enable),
    is_enabled: Some(owl_comp_is_enabled),
    determine_rate: Some(owl_comp_fix_fact_determine_rate),
    recalc_rate: Some(owl_comp_fix_fact_recalc_rate),
    set_rate: Some(owl_comp_fix_fact_set_rate),
};

pub static mut owl_comp_pass_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(owl_comp_get_parent),
    set_parent: Some(owl_comp_set_parent),
    disable: Some(owl_comp_disable),
    enable: Some(owl_comp_enable),
    is_enabled: Some(owl_comp_is_enabled),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
