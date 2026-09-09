// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum composite clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependency declarations supplied by the Linux clock-provider and composite
// headers are intentionally left to the surrounding translation unit.

unsafe fn sprd_comp_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let cc: *mut sprd_comp = hw_to_sprd_comp(hw);

    divider_determine_rate(hw, req, core::ptr::null_mut(), (*cc).div.width, 0)
}

unsafe fn sprd_comp_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let cc: *mut sprd_comp = hw_to_sprd_comp(hw);

    sprd_div_helper_recalc_rate(&mut (*cc).common, &mut (*cc).div, parent_rate)
}

unsafe fn sprd_comp_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let cc: *mut sprd_comp = hw_to_sprd_comp(hw);

    sprd_div_helper_set_rate(&mut (*cc).common, &mut (*cc).div, rate, parent_rate)
}

unsafe fn sprd_comp_get_parent(hw: *mut clk_hw) -> u8 {
    let cc: *mut sprd_comp = hw_to_sprd_comp(hw);

    sprd_mux_helper_get_parent(&mut (*cc).common, &mut (*cc).mux)
}

unsafe fn sprd_comp_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let cc: *mut sprd_comp = hw_to_sprd_comp(hw);

    sprd_mux_helper_set_parent(&mut (*cc).common, &mut (*cc).mux, index)
}

#[no_mangle]
pub static sprd_comp_ops: clk_ops = clk_ops {
    get_parent: Some(sprd_comp_get_parent),
    set_parent: Some(sprd_comp_set_parent),

    determine_rate: Some(sprd_comp_determine_rate),
    recalc_rate: Some(sprd_comp_recalc_rate),
    set_rate: Some(sprd_comp_set_rate),
};

// EXPORT_SYMBOL_GPL(sprd_comp_ops)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
