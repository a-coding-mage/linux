// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 */

// Dependencies are supplied by the surrounding clock-provider implementation.

const PLL_TIMEOUT_US: u32 = 3000;
const PLL_DELAY_US: u32 = 5;

const PLL_SWCR3_EN: u32 = 1u32 << 31;
const PLL_SWCR3_MASK: u32 = 0x7fff_ffff;

const PLLA_SWCR2_EN: u32 = 1u32 << 16;
const PLLA_SWCR2_MASK: u32 = 0x0000_ff00;

unsafe fn ccu_pll_lookup_best_rate(
    pll: *mut ccu_pll,
    rate: c_ulong,
) -> *const ccu_pll_rate_tbl {
    let config = unsafe { &mut (*pll).config };
    let mut best_entry: *const ccu_pll_rate_tbl = core::ptr::null();
    let mut best_delta = c_ulong::MAX;
    let mut i = 0;

    while i < config.tbl_num {
        let entry = unsafe { &config.rate_tbl.add(i as usize) };
        let delta = if (*entry).rate > rate {
            (*entry).rate - rate
        } else {
            rate - (*entry).rate
        };

        if delta < best_delta {
            best_delta = delta;
            best_entry = entry;
        }
        i += 1;
    }

    best_entry
}

unsafe fn ccu_pll_lookup_matched_entry(pll: *mut ccu_pll) -> *const ccu_pll_rate_tbl {
    let config = unsafe { &mut (*pll).config };
    let swcr1 = unsafe { ccu_read(&(*pll).common, swcr1) };
    let mut swcr3 = unsafe { ccu_read(&(*pll).common, swcr3) };
    swcr3 &= PLL_SWCR3_MASK;

    let mut i = 0;
    while i < config.tbl_num {
        let entry = unsafe { &config.rate_tbl.add(i as usize) };
        if swcr1 == (*entry).swcr1 && swcr3 == (*entry).swcr3 {
            return entry;
        }
        i += 1;
    }
    core::ptr::null()
}

unsafe fn ccu_pll_update_param(pll: *mut ccu_pll, entry: *const ccu_pll_rate_tbl) {
    let common = unsafe { &mut (*pll).common };
    unsafe { regmap_write(common.regmap, common.reg_swcr1, (*entry).swcr1) };
    unsafe { ccu_update(common, swcr3, PLL_SWCR3_MASK, (*entry).swcr3) };
}

unsafe fn ccu_pll_is_enabled(hw: *mut clk_hw) -> i32 {
    let common = unsafe { hw_to_ccu_common(hw) };
    unsafe { ccu_read(common, swcr3) & PLL_SWCR3_EN } as i32
}

unsafe fn ccu_pll_enable(hw: *mut clk_hw) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    let common = unsafe { &mut (*pll).common };
    let mut tmp: u32 = 0;
    unsafe { ccu_update(common, swcr3, PLL_SWCR3_EN, PLL_SWCR3_EN) };
    unsafe {
        regmap_read_poll_timeout_atomic(
            common.lock_regmap,
            (*pll).config.reg_lock,
            &mut tmp,
            tmp & (*pll).config.mask_lock,
            PLL_DELAY_US,
            PLL_TIMEOUT_US,
        )
    }
}

unsafe fn ccu_pll_disable(hw: *mut clk_hw) {
    let common = unsafe { hw_to_ccu_common(hw) };
    unsafe { ccu_update(common, swcr3, PLL_SWCR3_EN, 0) };
}

/*
 * PLLs must be gated before changing rate, which is ensured by
 * flag CLK_SET_RATE_GATE.
 */
unsafe fn ccu_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    let entry = unsafe { ccu_pll_lookup_best_rate(pll, rate) };
    unsafe { ccu_pll_update_param(pll, entry) };
    0
}

unsafe fn ccu_pll_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    let entry = unsafe { ccu_pll_lookup_matched_entry(pll) };
    unsafe { WARN_ON_ONCE(entry.is_null()) };
    if entry.is_null() { 0 } else { unsafe { (*entry).rate } }
}

unsafe fn ccu_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    unsafe { (*req).rate = (*ccu_pll_lookup_best_rate(pll, (*req).rate)).rate };
    0
}

unsafe fn ccu_pll_init(hw: *mut clk_hw) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    if !unsafe { ccu_pll_lookup_matched_entry(pll) }.is_null() { return 0; }
    unsafe { ccu_pll_disable(hw) };
    unsafe { ccu_pll_update_param(pll, (*pll).config.rate_tbl) };
    0
}

unsafe fn ccu_plla_lookup_matched_entry(pll: *mut ccu_pll) -> *const ccu_pll_rate_tbl {
    let config = unsafe { &mut (*pll).config };
    let swcr1 = unsafe { ccu_read(&(*pll).common, swcr1) };
    let mut swcr2 = unsafe { ccu_read(&(*pll).common, swcr2) };
    swcr2 &= PLLA_SWCR2_MASK;
    let swcr3 = unsafe { ccu_read(&(*pll).common, swcr3) };
    let mut i = 0;
    while i < config.tbl_num {
        let entry = unsafe { &config.rate_tbl.add(i as usize) };
        if swcr1 == (*entry).swcr1 && swcr2 == (*entry).swcr2 && swcr3 == (*entry).swcr3 { return entry; }
        i += 1;
    }
    core::ptr::null()
}

unsafe fn ccu_plla_update_param(pll: *mut ccu_pll, entry: *const ccu_pll_rate_tbl) {
    let common = unsafe { &mut (*pll).common };
    unsafe { regmap_write(common.regmap, common.reg_swcr1, (*entry).swcr1) };
    unsafe { regmap_write(common.regmap, common.reg_swcr3, (*entry).swcr3) };
    unsafe { ccu_update(common, swcr2, PLLA_SWCR2_MASK, (*entry).swcr2) };
}

unsafe fn ccu_plla_is_enabled(hw: *mut clk_hw) -> i32 {
    let common = unsafe { hw_to_ccu_common(hw) };
    unsafe { ccu_read(common, swcr2) & PLLA_SWCR2_EN } as i32
}

unsafe fn ccu_plla_enable(hw: *mut clk_hw) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    let common = unsafe { &mut (*pll).common };
    let mut tmp: u32 = 0;
    unsafe { ccu_update(common, swcr2, PLLA_SWCR2_EN, PLLA_SWCR2_EN) };
    unsafe { regmap_read_poll_timeout_atomic(common.lock_regmap, (*pll).config.reg_lock, &mut tmp, tmp & (*pll).config.mask_lock, PLL_DELAY_US, PLL_TIMEOUT_US) }
}

unsafe fn ccu_plla_disable(hw: *mut clk_hw) {
    let common = unsafe { hw_to_ccu_common(hw) };
    unsafe { ccu_update(common, swcr2, PLLA_SWCR2_EN, 0) };
}

/*
 * PLLAs must be gated before changing rate, which is ensured by
 * flag CLK_SET_RATE_GATE.
 */
unsafe fn ccu_plla_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    unsafe { ccu_plla_update_param(pll, ccu_pll_lookup_best_rate(pll, rate)) };
    0
}

unsafe fn ccu_plla_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    let entry = unsafe { ccu_plla_lookup_matched_entry(pll) };
    unsafe { WARN_ON_ONCE(entry.is_null()) };
    if entry.is_null() { 0 } else { unsafe { (*entry).rate } }
}

unsafe fn ccu_plla_init(hw: *mut clk_hw) -> i32 {
    let pll = unsafe { hw_to_ccu_pll(hw) };
    if !unsafe { ccu_plla_lookup_matched_entry(pll) }.is_null() { return 0; }
    unsafe { ccu_plla_disable(hw); ccu_plla_update_param(pll, (*pll).config.rate_tbl) };
    0
}

pub static spacemit_ccu_pll_ops: clk_ops = clk_ops {
    init: Some(ccu_pll_init), enable: Some(ccu_pll_enable), disable: Some(ccu_pll_disable),
    set_rate: Some(ccu_pll_set_rate), recalc_rate: Some(ccu_pll_recalc_rate),
    determine_rate: Some(ccu_pll_determine_rate), is_enabled: Some(ccu_pll_is_enabled),
};

pub static spacemit_ccu_plla_ops: clk_ops = clk_ops {
    init: Some(ccu_plla_init), enable: Some(ccu_plla_enable), disable: Some(ccu_plla_disable),
    set_rate: Some(ccu_plla_set_rate), recalc_rate: Some(ccu_plla_recalc_rate),
    determine_rate: Some(ccu_pll_determine_rate), is_enabled: Some(ccu_plla_is_enabled),
};

// EXPORT_SYMBOL_NS_GPL(spacemit_ccu_pll_ops, "CLK_SPACEMIT");
// EXPORT_SYMBOL_NS_GPL(spacemit_ccu_plla_ops, "CLK_SPACEMIT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
