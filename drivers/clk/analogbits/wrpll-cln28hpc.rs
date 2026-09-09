// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018-2019 SiFive, Inc.
 * Wesley Terpstra
 * Paul Walmsley
 *
 * This library supports configuration parsing and reprogramming of
 * the CLN28HPC variant of the Analog Bits Wide Range PLL.
 */

// Linux header dependencies are supplied by the surrounding translation.

const MIN_INPUT_FREQ: u64 = 7000000;
const MAX_INPUT_FREQ: u64 = 600000000;
const MIN_POST_DIVR_FREQ: u64 = 7000000;
const MAX_POST_DIVR_FREQ: u64 = 200000000;
const MIN_VCO_FREQ: u64 = 2400000000;
const MAX_VCO_FREQ: u64 = 4800000000;
const MAX_DIVQ_DIVISOR: u64 = 64;
const MAX_DIVR_DIVISOR: u64 = 64;
const MAX_LOCK_US: u32 = 70;
const ROUND_SHIFT: u32 = 20;

// Defined by the external analogbits-wrpll-cln28hpc interface.
#[repr(C)]
pub struct wrpll_cfg {
    pub flags: u32,
    pub parent_rate: u64,
    pub max_r: u8,
    pub init_r: u8,
    pub divq: u8,
    pub divr: u8,
    pub divf: u32,
    pub range: i32,
}

extern "C" {
    static WRPLL_FLAGS_INT_FEEDBACK_MASK: u32;
    static WRPLL_FLAGS_EXT_FEEDBACK_MASK: u32;
    static WRPLL_FLAGS_RESET_MASK: u32;
    static WRPLL_FLAGS_BYPASS_MASK: u32;
}

unsafe fn __wrpll_calc_filter_range(post_divr_freq: u64) -> i32 {
    if post_divr_freq < MIN_POST_DIVR_FREQ || post_divr_freq > MAX_POST_DIVR_FREQ {
        return -(34i32); // -ERANGE
    }
    match post_divr_freq {
        0..=10999999 => 1,
        11000000..=17999999 => 2,
        18000000..=29999999 => 3,
        30000000..=49999999 => 4,
        50000000..=79999999 => 5,
        80000000..=129999999 => 6,
        _ => 7,
    }
}

unsafe fn __wrpll_calc_fbdiv(c: *const wrpll_cfg) -> u8 {
    if (*c).flags & WRPLL_FLAGS_INT_FEEDBACK_MASK != 0 { 2 } else { 1 }
}

unsafe fn __wrpll_calc_divq(target_rate: u32, vco_rate: *mut u64) -> u8 {
    if vco_rate.is_null() { return 0; }
    let s = MAX_VCO_FREQ / target_rate as u64;
    if s <= 1 {
        *vco_rate = MAX_VCO_FREQ;
        1
    } else if s > MAX_DIVQ_DIVISOR {
        *vco_rate = MIN_VCO_FREQ;
        6
    } else {
        let divq = 63 - s.leading_zeros() as u64;
        *vco_rate = (target_rate as u64) << divq;
        divq as u8
    }
}

unsafe fn __wrpll_update_parent_rate(c: *mut wrpll_cfg, parent_rate: u64) -> i32 {
    if parent_rate > MAX_INPUT_FREQ || parent_rate < MIN_POST_DIVR_FREQ { return -34; }
    (*c).parent_rate = parent_rate;
    let max_r_for_parent = parent_rate / MIN_POST_DIVR_FREQ;
    (*c).max_r = core::cmp::min(MAX_DIVR_DIVISOR as u8, max_r_for_parent as u8);
    (*c).init_r = ((parent_rate + MAX_POST_DIVR_FREQ - 1) / MAX_POST_DIVR_FREQ) as u8;
    0
}

pub unsafe fn wrpll_configure_for_rate(c: *mut wrpll_cfg, target_rate: u32, parent_rate: u64) -> i32 {
    if (*c).flags == 0 { return -22; }
    if parent_rate != (*c).parent_rate && __wrpll_update_parent_rate(c, parent_rate) != 0 { return -34; }
    (*c).flags &= !WRPLL_FLAGS_RESET_MASK;
    if target_rate as u64 == parent_rate {
        (*c).flags |= WRPLL_FLAGS_BYPASS_MASK;
        return 0;
    }
    (*c).flags &= !WRPLL_FLAGS_BYPASS_MASK;
    let mut target_vco_rate = 0u64;
    let divq = __wrpll_calc_divq(target_rate, &mut target_vco_rate);
    if divq == 0 { return -1; }
    (*c).divq = divq;
    let ratio = (target_vco_rate << ROUND_SHIFT) / parent_rate;
    let fbdiv = __wrpll_calc_fbdiv(c);
    let mut best_r = 0u8;
    let mut best_f = 0u32;
    let mut best_delta = MAX_VCO_FREQ;
    let mut r = (*c).init_r;
    while r <= (*c).max_r {
        let f_pre_div = ratio * r as u64;
        let mut f = ((f_pre_div + (1u64 << ROUND_SHIFT)) >> ROUND_SHIFT) as u32;
        f >>= fbdiv - 1;
        let post_divr_freq = parent_rate / r as u64;
        let vco_pre = fbdiv as u64 * post_divr_freq;
        let mut vco = vco_pre * f as u64;
        if vco > target_vco_rate { f -= 1; vco = vco_pre * f as u64; }
        else if vco < MIN_VCO_FREQ { f += 1; vco = vco_pre * f as u64; }
        let delta = target_vco_rate.abs_diff(vco);
        if delta < best_delta { best_delta = delta; best_r = r; best_f = f; }
        if r == u8::MAX { break; }
        r += 1;
    }
    (*c).divr = best_r - 1;
    (*c).divf = best_f - 1;
    let range = __wrpll_calc_filter_range(parent_rate / best_r as u64);
    if range < 0 { return range; }
    (*c).range = range;
    0
}

pub unsafe fn wrpll_calc_output_rate(c: *const wrpll_cfg, parent_rate: u64) -> u64 {
    if (*c).flags & WRPLL_FLAGS_EXT_FEEDBACK_MASK != 0 { return u64::MAX; }
    let fbdiv = __wrpll_calc_fbdiv(c);
    let mut n = parent_rate * fbdiv as u64 * ((*c).divf as u64 + 1);
    n /= (*c).divr as u64 + 1;
    n >> (*c).divq
}

pub unsafe fn wrpll_calc_max_lock_us(_c: *const wrpll_cfg) -> u32 { MAX_LOCK_US }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
