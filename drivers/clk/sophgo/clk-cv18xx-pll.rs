// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// Dependencies supplied by the Linux clock-provider and CV18xx clock headers
// are intentionally referenced here without local implementations.

unsafe fn hw_to_cv1800_clk_pll(hw: *mut clk_hw) -> *mut cv1800_clk_pll {
    let common = hw_to_cv1800_clk_common(hw);
    container_of!(common, cv1800_clk_pll, common)
}

unsafe fn ipll_calc_rate(parent_rate: c_ulong, pre_div_sel: c_ulong,
                         div_sel: c_ulong, post_div_sel: c_ulong) -> c_ulong {
    let mut rate: u64 = parent_rate as u64;
    rate = rate.wrapping_mul(div_sel as u64);
    rate /= (pre_div_sel.wrapping_mul(post_div_sel)) as u64;
    rate as c_ulong
}

unsafe fn ipll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pll = hw_to_cv1800_clk_pll(hw);
    let value = readl((*pll).common.base.add((*pll).pll_reg as usize));
    ipll_calc_rate(parent_rate, PLL_GET_PRE_DIV_SEL!(value),
                   PLL_GET_DIV_SEL!(value), PLL_GET_POST_DIV_SEL!(value))
}

unsafe fn ipll_find_rate(limit: *const cv1800_clk_pll_limit,
                         prate: c_ulong, rate: *mut c_ulong,
                         value: *mut u32) -> i32 {
    let mut best_rate: c_ulong = 0;
    let trate = *rate;
    let (mut pre_div_sel, mut div_sel, mut post_div_sel): (c_ulong, c_ulong, c_ulong) = (0, 0, 0);
    let (mut pre, mut div, mut post): (c_ulong, c_ulong, c_ulong) = (0, 0, 0);
    let mut detected = *value;
    let mut tmp: c_ulong;

    for_each_pll_limit_range!(pre, &(*limit).pre_div, {
        for_each_pll_limit_range!(div, &(*limit).div, {
            for_each_pll_limit_range!(post, &(*limit).post_div, {
                tmp = ipll_calc_rate(prate, pre, div, post);
                if tmp > trate { continue; }
                if trate - tmp < trate - best_rate {
                    best_rate = tmp;
                    pre_div_sel = pre;
                    div_sel = div;
                    post_div_sel = post;
                }
            });
        });
    });

    if best_rate != 0 {
        detected = PLL_SET_PRE_DIV_SEL!(detected, pre_div_sel);
        detected = PLL_SET_POST_DIV_SEL!(detected, post_div_sel);
        detected = PLL_SET_DIV_SEL!(detected, div_sel);
        *value = detected;
        *rate = best_rate;
        return 0;
    }
    -EINVAL
}

unsafe fn ipll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut val: u32 = 0;
    let pll = hw_to_cv1800_clk_pll(hw);
    ipll_find_rate((*pll).pll_limit, (*req).best_parent_rate, &mut (*req).rate, &mut val)
}

unsafe fn pll_get_mode_ctrl(div_sel: c_ulong,
    mode_ctrl_check: Option<unsafe extern "C" fn(c_ulong, c_ulong, c_ulong) -> bool>,
    limit: *const cv1800_clk_pll_limit, value: *mut u32) {
    let (mut ictrl, mut mode): (c_ulong, c_ulong) = (0, 0);
    let mut detected = *value;
    for_each_pll_limit_range!(mode, &(*limit).mode, {
        for_each_pll_limit_range!(ictrl, &(*limit).ictrl, {
            if mode_ctrl_check.unwrap()(div_sel, ictrl, mode) {
                detected = PLL_SET_SEL_MODE!(detected, mode);
                detected = PLL_SET_ICTRL!(detected, ictrl);
                *value = detected;
                return;
            }
        });
    });
}

unsafe extern "C" fn ipll_check_mode_ctrl_restrict(div_sel: c_ulong, ictrl: c_ulong, mode: c_ulong) -> bool {
    let left_rest = 20 * div_sel;
    let right_rest = 35 * div_sel;
    let test = 184 * (1 + mode) * (1 + ictrl) / 2;
    test > left_rest && test <= right_rest
}

unsafe fn ipll_set_rate(hw: *mut clk_hw, mut rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let mut regval: u32;
    let mut detected: u32 = 0;
    let mut flags: c_ulong = 0;
    let pll = hw_to_cv1800_clk_pll(hw);
    ipll_find_rate((*pll).pll_limit, parent_rate, &mut rate, &mut detected);
    pll_get_mode_ctrl(PLL_GET_DIV_SEL!(detected), Some(ipll_check_mode_ctrl_restrict), (*pll).pll_limit, &mut detected);
    spin_lock_irqsave((*pll).common.lock, &mut flags);
    regval = readl((*pll).common.base.add((*pll).pll_reg as usize));
    regval = PLL_COPY_REG!(regval, detected);
    writel(regval, (*pll).common.base.add((*pll).pll_reg as usize));
    spin_unlock_irqrestore((*pll).common.lock, flags);
    cv1800_clk_wait_for_lock(&mut (*pll).common, (*pll).pll_status.reg, BIT!((*pll).pll_status.shift));
    0
}

unsafe fn pll_enable(hw: *mut clk_hw) -> i32 {
    let pll = hw_to_cv1800_clk_pll(hw);
    cv1800_clk_clearbit(&mut (*pll).common, &mut (*pll).pll_pwd)
}
unsafe fn pll_disable(hw: *mut clk_hw) {
    let pll = hw_to_cv1800_clk_pll(hw);
    cv1800_clk_setbit(&mut (*pll).common, &mut (*pll).pll_pwd);
}
unsafe fn pll_is_enable(hw: *mut clk_hw) -> i32 {
    let pll = hw_to_cv1800_clk_pll(hw);
    (cv1800_clk_checkbit(&mut (*pll).common, &mut (*pll).pll_pwd) == 0) as i32
}

static cv1800_clk_ipll_ops: clk_ops = clk_ops {
    disable: Some(pll_disable), enable: Some(pll_enable), is_enabled: Some(pll_is_enable),
    recalc_rate: Some(ipll_recalc_rate), determine_rate: Some(ipll_determine_rate), set_rate: Some(ipll_set_rate),
};

const PLL_SYN_FACTOR_DOT_POS: u32 = 26;
const PLL_SYN_FACTOR_MINIMUM: u32 = (4 << PLL_SYN_FACTOR_DOT_POS) + 1;

unsafe fn fpll_is_factional_mode(pll: *mut cv1800_clk_pll) -> bool {
    cv1800_clk_checkbit(&mut (*pll).common, &mut (*pll).pll_syn.en)
}

unsafe fn fpll_calc_rate(parent_rate: c_ulong, pre_div_sel: c_ulong, div_sel: c_ulong,
    post_div_sel: c_ulong, ssc_syn_set: c_ulong, is_full_parent: bool) -> c_ulong {
    let mut dividend = (parent_rate as u64).wrapping_mul(div_sel as u64);
    let factor = (ssc_syn_set as u64).wrapping_mul(pre_div_sel as u64).wrapping_mul(post_div_sel as u64);
    dividend <<= PLL_SYN_FACTOR_DOT_POS - 1;
    let mut rate = dividend / factor;
    dividend %= factor;
    if is_full_parent { dividend <<= 1; rate <<= 1; }
    rate += (dividend + factor / 2) / factor;
    rate as c_ulong
}

unsafe fn fpll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pll = hw_to_cv1800_clk_pll(hw);
    if !fpll_is_factional_mode(pll) { return ipll_recalc_rate(hw, parent_rate); }
    let syn_set = readl((*pll).common.base.add((*pll).pll_syn.set as usize));
    if syn_set == 0 { return 0; }
    let clk_full = cv1800_clk_checkbit(&mut (*pll).common, &mut (*pll).pll_syn.clk_half);
    let value = readl((*pll).common.base.add((*pll).pll_reg as usize));
    fpll_calc_rate(parent_rate, PLL_GET_PRE_DIV_SEL!(value), PLL_GET_DIV_SEL!(value),
                   PLL_GET_POST_DIV_SEL!(value), syn_set as c_ulong, clk_full)
}

unsafe fn fpll_find_synthesizer(parent: c_ulong, rate: c_ulong, pre_div: c_ulong,
    div: c_ulong, post_div: c_ulong, is_full_parent: bool, ssc_syn_set: *mut u32) -> c_ulong {
    let mut test_max: u32 = U32_MAX;
    let mut test_min: u32 = PLL_SYN_FACTOR_MINIMUM;
    let mut trate: c_ulong = 0;
    while test_min < test_max {
        let tssc = (test_max + test_min) / 2;
        trate = fpll_calc_rate(parent, pre_div, div, post_div, tssc as c_ulong, is_full_parent);
        if trate == rate { test_min = tssc; break; }
        if trate > rate { test_min = tssc + 1; } else { test_max = tssc - 1; }
    }
    if trate != 0 { *ssc_syn_set = test_min; }
    trate
}

unsafe fn fpll_find_rate(pll: *mut cv1800_clk_pll, limit: *const cv1800_clk_pll_limit,
    prate: c_ulong, rate: *mut c_ulong, value: *mut u32, ssc_syn_set: *mut u32) -> i32 {
    let mut best_rate: c_ulong = 0;
    let (mut pre_div_sel, mut div_sel, mut post_div_sel): (c_ulong, c_ulong, c_ulong) = (0, 0, 0);
    let (mut pre, mut div, mut post): (c_ulong, c_ulong, c_ulong) = (0, 0, 0);
    let trate = *rate;
    let mut detected = *value;
    let clk_full = cv1800_clk_checkbit(&mut (*pll).common, &mut (*pll).pll_syn.clk_half);
    for_each_pll_limit_range!(pre, &(*limit).pre_div, {
        for_each_pll_limit_range!(post, &(*limit).post_div, {
            for_each_pll_limit_range!(div, &(*limit).div, {
                let tmp = fpll_find_synthesizer(prate, trate, pre, div, post, clk_full, ssc_syn_set);
                if trate - tmp < trate - best_rate { best_rate = tmp; pre_div_sel = pre; div_sel = div; post_div_sel = post; }
            });
        });
    });
    if best_rate != 0 {
        detected = PLL_SET_PRE_DIV_SEL!(detected, pre_div_sel);
        detected = PLL_SET_POST_DIV_SEL!(detected, post_div_sel);
        detected = PLL_SET_DIV_SEL!(detected, div_sel);
        *value = detected; *rate = best_rate; return 0;
    }
    -EINVAL
}

unsafe fn fpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll = hw_to_cv1800_clk_pll(hw);
    let (mut val, mut ssc_syn_set): (u32, u32) = (0, 0);
    if !fpll_is_factional_mode(pll) { return ipll_determine_rate(hw, req); }
    fpll_find_rate(pll, (*pll).pll_limit.add(2), (*req).best_parent_rate, &mut (*req).rate, &mut val, &mut ssc_syn_set);
    0
}

unsafe extern "C" fn fpll_check_mode_ctrl_restrict(div_sel: c_ulong, ictrl: c_ulong, mode: c_ulong) -> bool {
    let left_rest = 10 * div_sel; let right_rest = 24 * div_sel;
    let test = 184 * (1 + mode) * (1 + ictrl) / 2;
    test > left_rest && test <= right_rest
}

unsafe fn fpll_set_rate(hw: *mut clk_hw, mut rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let pll = hw_to_cv1800_clk_pll(hw);
    if !fpll_is_factional_mode(pll) { return ipll_set_rate(hw, rate, parent_rate); }
    let (mut detected, mut detected_ssc): (u32, u32) = (0, 0);
    fpll_find_rate(pll, (*pll).pll_limit.add(2), parent_rate, &mut rate, &mut detected, &mut detected_ssc);
    pll_get_mode_ctrl(PLL_GET_DIV_SEL!(detected), Some(fpll_check_mode_ctrl_restrict), (*pll).pll_limit, &mut detected);
    let mut flags = 0; spin_lock_irqsave((*pll).common.lock, &mut flags);
    writel(detected_ssc, (*pll).common.base.add((*pll).pll_syn.set as usize));
    let mut regval = readl((*pll).common.base.add((*pll).pll_reg as usize));
    regval = PLL_COPY_REG!(regval, detected); writel(regval, (*pll).common.base.add((*pll).pll_reg as usize));
    spin_unlock_irqrestore((*pll).common.lock, flags);
    cv1800_clk_wait_for_lock(&mut (*pll).common, (*pll).pll_status.reg, BIT!((*pll).pll_status.shift)); 0
}

unsafe fn fpll_get_parent(hw: *mut clk_hw) -> u8 { if fpll_is_factional_mode(hw_to_cv1800_clk_pll(hw)) { 1 } else { 0 } }
unsafe fn fpll_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let pll = hw_to_cv1800_clk_pll(hw);
    if index != 0 { cv1800_clk_setbit(&mut (*pll).common, &mut (*pll).pll_syn.en); }
    else { cv1800_clk_clearbit(&mut (*pll).common, &mut (*pll).pll_syn.en); } 0
}

static cv1800_clk_fpll_ops: clk_ops = clk_ops {
    disable: Some(pll_disable), enable: Some(pll_enable), is_enabled: Some(pll_is_enable),
    recalc_rate: Some(fpll_recalc_rate), determine_rate: Some(fpll_determine_rate), set_rate: Some(fpll_set_rate),
    set_parent: Some(fpll_set_parent), get_parent: Some(fpll_get_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
