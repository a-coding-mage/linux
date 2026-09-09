// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 *
 * MIX clock type is the combination of mux, factor or divider, and gate
 */

// Dependencies supplied by the surrounding kernel/CCU implementation.

const MIX_FC_TIMEOUT_US: u32 = 10000;
const MIX_FC_DELAY_US: u32 = 5;

unsafe fn ccu_gate_disable(hw: *mut clk_hw) {
    let mix = hw_to_ccu_mix(hw);
    let gate = &mut (*mix).gate;
    let val: u32 = if gate.inverted { gate.mask } else { 0 };
    ccu_update(&mut (*mix).common, ctrl, gate.mask, val);
}

unsafe fn ccu_gate_enable(hw: *mut clk_hw) -> i32 {
    let mix = hw_to_ccu_mix(hw);
    let gate = &mut (*mix).gate;
    let val: u32 = if gate.inverted { 0 } else { gate.mask };
    ccu_update(&mut (*mix).common, ctrl, gate.mask, val);
    0
}

unsafe fn ccu_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let mix = hw_to_ccu_mix(hw);
    let gate = &mut (*mix).gate;
    let tmp = ccu_read(&(*mix).common, ctrl) & gate.mask;
    let val: u32 = if gate.inverted { 0 } else { gate.mask };
    (tmp == val) as i32
}

unsafe fn ccu_factor_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let mix = hw_to_ccu_mix(hw);
    parent_rate * (*mix).factor.mul / (*mix).factor.div
}

unsafe fn ccu_div_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let mix = hw_to_ccu_mix(hw);
    let div = &mut (*mix).div;
    let mut val = ccu_read(&(*mix).common, ctrl) >> div.shift;
    val &= (1u32 << div.width) - 1;
    divider_recalc_rate(hw, parent_rate, val, core::ptr::null(), 0, div.width)
}

unsafe fn ccu_mix_trigger_fc(hw: *mut clk_hw) -> i32 {
    let common = hw_to_ccu_common(hw);
    if (*common).reg_fc.is_null() {
        return 0;
    }
    ccu_update(common, fc, (*common).mask_fc, (*common).mask_fc);
    regmap_read_poll_timeout_atomic(
        (*common).regmap, (*common).reg_fc, core::ptr::null_mut(),
        MIX_FC_DELAY_US, MIX_FC_TIMEOUT_US,
    )
}

unsafe fn ccu_factor_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    (*req).rate = ccu_factor_recalc_rate(hw, (*req).best_parent_rate);
    0
}

unsafe fn ccu_factor_set_rate(_hw: *mut clk_hw, _rate: c_ulong, _parent_rate: c_ulong) -> i32 { 0 }

unsafe fn ccu_mix_calc_best_rate(
    hw: *mut clk_hw, rate: c_ulong, best_parent: *mut *mut clk_hw,
    best_parent_rate: *mut c_ulong, div_val: *mut u32,
) -> c_ulong {
    let mix = hw_to_ccu_mix(hw);
    let parent_num = clk_hw_get_num_parents(hw);
    let div = &mut (*mix).div;
    let div_max = 1u32 << div.width;
    let mut best_rate: c_ulong = 0;
    for i in 0..parent_num {
        let parent = clk_hw_get_parent_by_index(hw, i);
        if parent.is_null() { continue; }
        let parent_rate = clk_hw_get_rate(parent);
        for j in 1..=div_max {
            let tmp = div_round_closest_ull(parent_rate, j as c_ulong);
            if tmp.abs_diff(rate) < best_rate.abs_diff(rate) {
                best_rate = tmp;
                if !div_val.is_null() { *div_val = j - 1; }
                if !best_parent.is_null() {
                    *best_parent = parent;
                    *best_parent_rate = parent_rate;
                }
            }
        }
    }
    best_rate
}

unsafe fn ccu_mix_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    (*req).rate = ccu_mix_calc_best_rate(hw, (*req).rate, &mut (*req).best_parent_hw,
                                         &mut (*req).best_parent_rate, core::ptr::null_mut());
    0
}

unsafe fn ccu_mix_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> i32 {
    let mix = hw_to_ccu_mix(hw);
    let common = &mut (*mix).common;
    let div = &mut (*mix).div;
    let mut target_div = 0;
    ccu_mix_calc_best_rate(hw, rate, core::ptr::null_mut(), core::ptr::null_mut(), &mut target_div);
    let mut current_div = ccu_read(common, ctrl) >> div.shift;
    current_div &= (1u32 << div.width) - 1;
    if current_div == target_div { return 0; }
    let mask = genmask(div.width + div.shift - 1, div.shift);
    ccu_update(common, ctrl, mask, target_div << div.shift);
    ccu_mix_trigger_fc(hw)
}

unsafe fn ccu_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mix = hw_to_ccu_mix(hw);
    let mux = &mut (*mix).mux;
    ((ccu_read(&(*mix).common, ctrl) >> mux.shift) & ((1u32 << mux.width) - 1)) as u8
}

unsafe fn ccu_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mix = hw_to_ccu_mix(hw);
    let mux = &mut (*mix).mux;
    let mask = genmask(mux.width + mux.shift - 1, mux.shift);
    ccu_update(&mut (*mix).common, ctrl, mask, (index as u32) << mux.shift);
    ccu_mix_trigger_fc(hw)
}

// Operation tables corresponding to the C clk_ops instances.
const spacemit_ccu_gate_ops: clk_ops = clk_ops {
    disable: Some(ccu_gate_disable), enable: Some(ccu_gate_enable), is_enabled: Some(ccu_gate_is_enabled), ..clk_ops::EMPTY
};
const spacemit_ccu_factor_ops: clk_ops = clk_ops { determine_rate: Some(ccu_factor_determine_rate), recalc_rate: Some(ccu_factor_recalc_rate), set_rate: Some(ccu_factor_set_rate), ..clk_ops::EMPTY };
const spacemit_ccu_mux_ops: clk_ops = clk_ops { determine_rate: Some(ccu_mix_determine_rate), get_parent: Some(ccu_mux_get_parent), set_parent: Some(ccu_mux_set_parent), ..clk_ops::EMPTY };
const spacemit_ccu_div_ops: clk_ops = clk_ops { determine_rate: Some(ccu_mix_determine_rate), recalc_rate: Some(ccu_div_recalc_rate), set_rate: Some(ccu_mix_set_rate), ..clk_ops::EMPTY };
const spacemit_ccu_factor_gate_ops: clk_ops = clk_ops { disable: Some(ccu_gate_disable), enable: Some(ccu_gate_enable), is_enabled: Some(ccu_gate_is_enabled), determine_rate: Some(ccu_factor_determine_rate), recalc_rate: Some(ccu_factor_recalc_rate), set_rate: Some(ccu_factor_set_rate), ..clk_ops::EMPTY };
const spacemit_ccu_mux_gate_ops: clk_ops = clk_ops { disable: Some(ccu_gate_disable), enable: Some(ccu_gate_enable), is_enabled: Some(ccu_gate_is_enabled), determine_rate: Some(ccu_mix_determine_rate), get_parent: Some(ccu_mux_get_parent), set_parent: Some(ccu_mux_set_parent), ..clk_ops::EMPTY };
const spacemit_ccu_div_gate_ops: clk_ops = clk_ops { disable: Some(ccu_gate_disable), enable: Some(ccu_gate_enable), is_enabled: Some(ccu_gate_is_enabled), determine_rate: Some(ccu_mix_determine_rate), recalc_rate: Some(ccu_div_recalc_rate), set_rate: Some(ccu_mix_set_rate), ..clk_ops::EMPTY };
const spacemit_ccu_mux_div_gate_ops: clk_ops = clk_ops { disable: Some(ccu_gate_disable), enable: Some(ccu_gate_enable), is_enabled: Some(ccu_gate_is_enabled), get_parent: Some(ccu_mux_get_parent), set_parent: Some(ccu_mux_set_parent), determine_rate: Some(ccu_mix_determine_rate), recalc_rate: Some(ccu_div_recalc_rate), set_rate: Some(ccu_mix_set_rate), ..clk_ops::EMPTY };
const spacemit_ccu_mux_div_ops: clk_ops = clk_ops { get_parent: Some(ccu_mux_get_parent), set_parent: Some(ccu_mux_set_parent), determine_rate: Some(ccu_mix_determine_rate), recalc_rate: Some(ccu_div_recalc_rate), set_rate: Some(ccu_mix_set_rate), ..clk_ops::EMPTY };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
