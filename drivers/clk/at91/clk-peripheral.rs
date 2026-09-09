// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Linux dependencies are supplied by the surrounding translation unit.

const PERIPHERAL_ID_MIN: u32 = 2;
const PERIPHERAL_ID_MAX: u32 = 31;
const PERIPHERAL_MAX_SHIFT: u32 = 3;

#[inline]
const fn peripheral_mask(id: u32) -> u32 { 1u32 << (id & PERIPHERAL_ID_MAX) }

#[repr(C)]
struct ClkPeripheral {
    hw: ClkHw,
    regmap: *mut Regmap,
    id: u32,
}

#[repr(C)]
struct ClkSam9x5Peripheral {
    hw: ClkHw,
    regmap: *mut Regmap,
    range: ClkRange,
    lock: *mut SpinlockT,
    id: u32,
    div: u32,
    layout: *const ClkPcrLayout,
    pms: At91ClkPms,
    auto_div: bool,
    chg_pid: i32,
}

#[inline]
unsafe fn to_clk_peripheral(hw: *mut ClkHw) -> *mut ClkPeripheral {
    (hw as *mut u8).sub(core::mem::offset_of!(ClkPeripheral, hw)) as *mut ClkPeripheral
}

#[inline]
unsafe fn to_clk_sam9x5_peripheral(hw: *mut ClkHw) -> *mut ClkSam9x5Peripheral {
    (hw as *mut u8).sub(core::mem::offset_of!(ClkSam9x5Peripheral, hw)) as *mut ClkSam9x5Peripheral
}

unsafe extern "C" fn clk_peripheral_enable(hw: *mut ClkHw) -> i32 {
    let periph = to_clk_peripheral(hw);
    let mut offset = AT91_PMC_PCER;
    let id = (*periph).id;
    if id < PERIPHERAL_ID_MIN { return 0; }
    if id > PERIPHERAL_ID_MAX { offset = AT91_PMC_PCER1; }
    regmap_write((*periph).regmap, offset, peripheral_mask(id));
    0
}

unsafe extern "C" fn clk_peripheral_disable(hw: *mut ClkHw) {
    let periph = to_clk_peripheral(hw);
    let mut offset = AT91_PMC_PCDR;
    let id = (*periph).id;
    if id < PERIPHERAL_ID_MIN { return; }
    if id > PERIPHERAL_ID_MAX { offset = AT91_PMC_PCDR1; }
    regmap_write((*periph).regmap, offset, peripheral_mask(id));
}

unsafe extern "C" fn clk_peripheral_is_enabled(hw: *mut ClkHw) -> i32 {
    let periph = to_clk_peripheral(hw);
    let mut offset = AT91_PMC_PCSR;
    let mut status = 0u32;
    let id = (*periph).id;
    if id < PERIPHERAL_ID_MIN { return 1; }
    if id > PERIPHERAL_ID_MAX { offset = AT91_PMC_PCSR1; }
    regmap_read((*periph).regmap, offset, &mut status);
    if status & peripheral_mask(id) != 0 { 1 } else { 0 }
}

static PERIPHERAL_OPS: ClkOps = ClkOps {
    enable: Some(clk_peripheral_enable), disable: Some(clk_peripheral_disable),
    is_enabled: Some(clk_peripheral_is_enabled), ..ClkOps::zeroed()
};

unsafe fn clk_sam9x5_peripheral_autodiv(periph: *mut ClkSam9x5Peripheral) {
    if !(*periph).auto_div { return; }
    let mut shift = 0u32;
    if (*periph).range.max != 0 {
        let parent = clk_hw_get_parent_by_index(&mut (*periph).hw, 0);
        let parent_rate = clk_hw_get_rate(parent);
        if parent_rate == 0 { return; }
        while shift < PERIPHERAL_MAX_SHIFT {
            if parent_rate >> shift <= (*periph).range.max { break; }
            shift += 1;
        }
    }
    (*periph).auto_div = false;
    (*periph).div = shift;
}

unsafe fn clk_sam9x5_peripheral_set(periph: *mut ClkSam9x5Peripheral, status: u32) -> i32 {
    if (*periph).id < PERIPHERAL_ID_MIN { return 0; }
    let mut flags = 0ul;
    let enable = if status != 0 { AT91_PMC_PCR_EN } else { 0 };
    spin_lock_irqsave((*periph).lock, &mut flags);
    regmap_write((*periph).regmap, (*(*periph).layout).offset, (*periph).id & (*(*periph).layout).pid_mask);
    regmap_update_bits((*periph).regmap, (*(*periph).layout).offset,
        (*(*periph).layout).div_mask | (*(*periph).layout).cmd | enable,
        field_prep((*(*periph).layout).div_mask, (*periph).div) | (*(*periph).layout).cmd | enable);
    spin_unlock_irqrestore((*periph).lock, flags);
    0
}

unsafe extern "C" fn clk_sam9x5_peripheral_enable(hw: *mut ClkHw) -> i32 {
    clk_sam9x5_peripheral_set(to_clk_sam9x5_peripheral(hw), 1)
}

unsafe extern "C" fn clk_sam9x5_peripheral_disable(hw: *mut ClkHw) {
    let periph = to_clk_sam9x5_peripheral(hw);
    if (*periph).id < PERIPHERAL_ID_MIN { return; }
    let mut flags = 0ul;
    spin_lock_irqsave((*periph).lock, &mut flags);
    regmap_write((*periph).regmap, (*(*periph).layout).offset, (*periph).id & (*(*periph).layout).pid_mask);
    regmap_update_bits((*periph).regmap, (*(*periph).layout).offset,
        AT91_PMC_PCR_EN | (*(*periph).layout).cmd, (*(*periph).layout).cmd);
    spin_unlock_irqrestore((*periph).lock, flags);
}

unsafe extern "C" fn clk_sam9x5_peripheral_is_enabled(hw: *mut ClkHw) -> i32 {
    let periph = to_clk_sam9x5_peripheral(hw);
    if (*periph).id < PERIPHERAL_ID_MIN { return 1; }
    let mut flags = 0ul; let mut status = 0u32;
    spin_lock_irqsave((*periph).lock, &mut flags);
    regmap_write((*periph).regmap, (*(*periph).layout).offset, (*periph).id & (*(*periph).layout).pid_mask);
    regmap_read((*periph).regmap, (*(*periph).layout).offset, &mut status);
    spin_unlock_irqrestore((*periph).lock, flags);
    if status & AT91_PMC_PCR_EN != 0 { 1 } else { 0 }
}

unsafe extern "C" fn clk_sam9x5_peripheral_recalc_rate(hw: *mut ClkHw, parent_rate: u64) -> u64 {
    let periph = to_clk_sam9x5_peripheral(hw);
    if (*periph).id < PERIPHERAL_ID_MIN { return parent_rate; }
    let mut flags = 0ul; let mut status = 0u32;
    spin_lock_irqsave((*periph).lock, &mut flags);
    regmap_write((*periph).regmap, (*(*periph).layout).offset, (*periph).id & (*(*periph).layout).pid_mask);
    regmap_read((*periph).regmap, (*(*periph).layout).offset, &mut status);
    spin_unlock_irqrestore((*periph).lock, flags);
    if status & AT91_PMC_PCR_EN != 0 {
        (*periph).div = field_get((*(*periph).layout).div_mask, status);
        (*periph).auto_div = false;
    } else { clk_sam9x5_peripheral_autodiv(periph); }
    parent_rate >> (*periph).div
}

unsafe fn clk_sam9x5_peripheral_best_diff(req: *mut ClkRateRequest, parent: *mut ClkHw,
    parent_rate: u64, shift: u32, best_diff: *mut i64, best_rate: *mut i64) {
    let tmp_rate = parent_rate >> shift;
    let tmp_diff = ((*req).rate as i64 - tmp_rate as i64).abs();
    if *best_diff < 0 || *best_diff >= tmp_diff {
        *best_rate = tmp_rate as i64; *best_diff = tmp_diff;
        (*req).best_parent_rate = parent_rate; (*req).best_parent_hw = parent;
    }
}

unsafe extern "C" fn clk_sam9x5_peripheral_no_parent_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let periph = to_clk_sam9x5_peripheral(hw);
    if (*periph).id < PERIPHERAL_ID_MIN || (*periph).range.max == 0 { (*req).rate = (*req).best_parent_rate; return 0; }
    let mut shift = 0u32; let mut cur_rate = (*req).best_parent_rate;
    while shift <= PERIPHERAL_MAX_SHIFT { cur_rate = (*req).best_parent_rate >> shift; if cur_rate <= (*periph).range.max { break; } shift += 1; }
    if (*req).rate >= cur_rate { (*req).rate = cur_rate; return 0; }
    let mut best_rate = cur_rate; let mut best_diff = cur_rate - (*req).rate;
    while shift <= PERIPHERAL_MAX_SHIFT {
        cur_rate = (*req).best_parent_rate >> shift;
        let cur_diff = if cur_rate < (*req).rate { (*req).rate - cur_rate } else { cur_rate - (*req).rate };
        if cur_diff < best_diff { best_diff = cur_diff; best_rate = cur_rate; }
        if best_diff == 0 || cur_rate < (*req).rate { break; } shift += 1;
    }
    (*req).rate = best_rate; 0
}

unsafe extern "C" fn clk_sam9x5_peripheral_set_rate(hw: *mut ClkHw, rate: u64, parent_rate: u64) -> i32 {
    let periph = to_clk_sam9x5_peripheral(hw);
    if (*periph).id < PERIPHERAL_ID_MIN || (*periph).range.max == 0 { return if parent_rate == rate { 0 } else { -EINVAL }; }
    if rate > (*periph).range.max { return -EINVAL; }
    for shift in 0..=PERIPHERAL_MAX_SHIFT { if parent_rate >> shift == rate { (*periph).auto_div = false; (*periph).div = shift; return 0; } }
    -EINVAL
}

unsafe extern "C" fn clk_sam9x5_peripheral_save_context(hw: *mut ClkHw) -> i32 {
    let p = to_clk_sam9x5_peripheral(hw); (*p).pms.status = clk_sam9x5_peripheral_is_enabled(hw); 0
}
unsafe extern "C" fn clk_sam9x5_peripheral_restore_context(hw: *mut ClkHw) {
    let p = to_clk_sam9x5_peripheral(hw); if (*p).pms.status != 0 { clk_sam9x5_peripheral_set(p, (*p).pms.status as u32); }
}

static SAM9X5_PERIPHERAL_OPS: ClkOps = ClkOps { enable: Some(clk_sam9x5_peripheral_enable), disable: Some(clk_sam9x5_peripheral_disable), is_enabled: Some(clk_sam9x5_peripheral_is_enabled), recalc_rate: Some(clk_sam9x5_peripheral_recalc_rate), determine_rate: Some(clk_sam9x5_peripheral_no_parent_determine_rate), set_rate: Some(clk_sam9x5_peripheral_set_rate), save_context: Some(clk_sam9x5_peripheral_save_context), restore_context: Some(clk_sam9x5_peripheral_restore_context), ..ClkOps::zeroed() };

// Registration declarations and external kernel types are intentionally left
// in their source-level form for resolution by the surrounding translation.
extern "C" {
    fn at91_clk_register_peripheral(regmap: *mut Regmap, name: *const i8, parent_name: *const i8, parent_hw: *mut ClkHw, id: u32) -> *mut ClkHw;
    fn at91_clk_register_sam9x5_peripheral(regmap: *mut Regmap, lock: *mut SpinlockT, layout: *const ClkPcrLayout, name: *const i8, parent_name: *const i8, parent_hw: *mut ClkHw, id: u32, range: *const ClkRange, chg_pid: i32, flags: u64) -> *mut ClkHw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
