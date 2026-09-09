// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017, Linaro Limited
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const CMD_RCGR: u32 = 0x0;
const CMD_RCGR_UPDATE: u32 = 1 << 0;
const CMD_RCGR_DIRTY_CFG: u32 = 1 << 4;
const CMD_RCGR_ROOT_OFF: u32 = 1 << 31;
const CFG_RCGR: u32 = 0x4;

// Equivalent of container_of(to_clk_regmap(_hw), struct clk_regmap_mux_div, clkr).
unsafe fn to_clk_regmap_mux_div(_hw: *mut clk_hw) -> *mut clk_regmap_mux_div {
    unimplemented!()
}

unsafe fn mux_div_set_src_div(md: *mut clk_regmap_mux_div, src: u32, div: u32) -> i32 {
    let mut ret: i32;
    let mut count: i32;
    let mut val: u32;
    let mask: u32;
    let name: *const core::ffi::c_char = clk_hw_get_name(&(*(*md).clkr).hw);

    val = (div << (*md).hid_shift) | (src << (*md).src_shift);
    mask = (((1u32 << (*md).hid_width) - 1) << (*md).hid_shift)
        | (((1u32 << (*md).src_width) - 1) << (*md).src_shift);

    ret = regmap_update_bits(
        (*(*md).clkr).regmap,
        CFG_RCGR + (*md).reg_offset,
        mask,
        val,
    );
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*(*md).clkr).regmap,
        CMD_RCGR + (*md).reg_offset,
        CMD_RCGR_UPDATE,
        CMD_RCGR_UPDATE,
    );
    if ret != 0 {
        return ret;
    }

    // Wait for update to take effect
    count = 500;
    while count > 0 {
        ret = regmap_read(
            (*(*md).clkr).regmap,
            CMD_RCGR + (*md).reg_offset,
            &mut val,
        );
        if ret != 0 {
            return ret;
        }
        if (val & CMD_RCGR_UPDATE) == 0 {
            return 0;
        }
        udelay(1);
        count -= 1;
    }

    pr_err(name, " RCG did not update its configuration");
    -EBUSY
}

unsafe fn mux_div_get_src_div(md: *mut clk_regmap_mux_div, src: *mut u32, div: *mut u32) {
    let mut val: u32 = 0;
    let mut d: u32;
    let mut s: u32;
    let name: *const core::ffi::c_char = clk_hw_get_name(&(*(*md).clkr).hw);

    regmap_read((*(*md).clkr).regmap, CMD_RCGR + (*md).reg_offset, &mut val);

    if (val & CMD_RCGR_DIRTY_CFG) != 0 {
        pr_err(name, " RCG configuration is pending\n");
        return;
    }

    regmap_read((*(*md).clkr).regmap, CFG_RCGR + (*md).reg_offset, &mut val);
    s = val >> (*md).src_shift;
    s &= (1u32 << (*md).src_width) - 1;
    *src = s;

    d = val >> (*md).hid_shift;
    d &= (1u32 << (*md).hid_width) - 1;
    *div = d;
}

#[inline]
fn is_better_rate(req: u64, best: u64, new: u64) -> bool {
    (req <= new && new < best) || (best < req && best < new)
}

unsafe fn mux_div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let md = to_clk_regmap_mux_div(hw);
    let mut best_rate: u64 = 0;
    let req_rate = (*req).rate;

    let mut i: u32 = 0;
    while i < clk_hw_get_num_parents(hw) {
        let parent = clk_hw_get_parent_by_index(hw, i);
        let mut parent_rate = clk_hw_get_rate(parent);
        let max_div = (1u32 << (*md).hid_width) - 1;
        let mut div: u32 = 1;
        while div < max_div {
            parent_rate = mult_frac(req_rate, div as u64, 2);
            parent_rate = clk_hw_round_rate(parent, parent_rate);
            let actual_rate = mult_frac(parent_rate, 2, div as u64);

            if is_better_rate(req_rate, best_rate, actual_rate) {
                best_rate = actual_rate;
                (*req).rate = best_rate;
                (*req).best_parent_rate = parent_rate;
                (*req).best_parent_hw = parent;
            }
            if actual_rate < req_rate || best_rate <= req_rate {
                break;
            }
            div += 1;
        }
        i += 1;
    }
    if best_rate == 0 { -EINVAL } else { 0 }
}

unsafe fn __mux_div_set_rate_and_parent(hw: *mut clk_hw, rate: u64, _prate: u64, _src: u32) -> i32 {
    let md = to_clk_regmap_mux_div(hw);
    let mut best_src: u32 = 0;
    let mut best_div: u32 = 0;
    let mut best_rate: u64 = 0;
    let mut i: u32 = 0;
    while i < clk_hw_get_num_parents(hw) {
        let parent = clk_hw_get_parent_by_index(hw, i);
        let mut parent_rate = clk_hw_get_rate(parent);
        let max_div = (1u32 << (*md).hid_width) - 1;
        let mut div: u32 = 1;
        while div < max_div {
            parent_rate = mult_frac(rate, div as u64, 2);
            parent_rate = clk_hw_round_rate(parent, parent_rate);
            let actual_rate = mult_frac(parent_rate, 2, div as u64);
            if is_better_rate(rate, best_rate, actual_rate) {
                best_rate = actual_rate;
                best_src = (*md).parent_map[i as usize];
                best_div = div - 1;
            }
            if actual_rate < rate || best_rate <= rate { break; }
            div += 1;
        }
        i += 1;
    }
    let ret = mux_div_set_src_div(md, best_src, best_div);
    if ret == 0 { (*md).div = best_div; (*md).src = best_src; }
    ret
}

unsafe fn mux_div_get_parent(hw: *mut clk_hw) -> u8 {
    let md = to_clk_regmap_mux_div(hw);
    let name = clk_hw_get_name(hw);
    let mut src: u32 = 0;
    let mut div: u32 = 0;
    mux_div_get_src_div(md, &mut src, &mut div);
    let mut i: u32 = 0;
    while i < clk_hw_get_num_parents(hw) {
        if src == (*md).parent_map[i as usize] { return i as u8; }
        i += 1;
    }
    pr_err(name, " Can't find parent with src %d\n", src);
    0
}

unsafe fn mux_div_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let md = to_clk_regmap_mux_div(hw);
    mux_div_set_src_div(md, (*md).parent_map[index as usize], (*md).div)
}

unsafe fn mux_div_set_rate(hw: *mut clk_hw, rate: u64, prate: u64) -> i32 {
    let md = to_clk_regmap_mux_div(hw);
    __mux_div_set_rate_and_parent(hw, rate, prate, (*md).src)
}

unsafe fn mux_div_set_rate_and_parent(hw: *mut clk_hw, rate: u64, prate: u64, index: u8) -> i32 {
    let md = to_clk_regmap_mux_div(hw);
    __mux_div_set_rate_and_parent(hw, rate, prate, (*md).parent_map[index as usize])
}

unsafe fn mux_div_recalc_rate(hw: *mut clk_hw, _prate: u64) -> u64 {
    let md = to_clk_regmap_mux_div(hw);
    let mut div: u32 = 0;
    let mut src: u32 = 0;
    mux_div_get_src_div(md, &mut src, &mut div);
    let mut i: u32 = 0;
    while i < clk_hw_get_num_parents(hw) {
        if src == (*md).parent_map[i as usize] {
            let p = clk_hw_get_parent_by_index(hw, i);
            return mult_frac(clk_hw_get_rate(p), 2, (div + 1) as u64);
        }
        i += 1;
    }
    pr_err(clk_hw_get_name(hw), " Can't find parent %d\n", src);
    0
}

const clk_regmap_mux_div_ops: clk_ops = clk_ops {
    get_parent: Some(mux_div_get_parent), set_parent: Some(mux_div_set_parent),
    set_rate: Some(mux_div_set_rate), set_rate_and_parent: Some(mux_div_set_rate_and_parent),
    determine_rate: Some(mux_div_determine_rate), recalc_rate: Some(mux_div_recalc_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
