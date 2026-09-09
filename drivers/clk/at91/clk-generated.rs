// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Atmel Corporation,
 *                    Nicolas Ferre <nicolas.ferre@atmel.com>
 *
 * Based on clk-programmable & clk-peripheral drivers by Boris BREZILLON.
 */

// Linux headers and "pmc.h" provide the external types, constants, and
// functions referenced below.

const GENERATED_MAX_DIV: u32 = 255;

#[repr(C)]
struct ClkGenerated {
    hw: ClkHw,
    regmap: *mut Regmap,
    range: ClkRange,
    lock: *mut Spinlock,
    mux_table: *mut u32,
    id: u32,
    gckdiv: u32,
    layout: *const ClkPcrLayout,
    pms: At91ClkPms,
    parent_id: u8,
    chg_pid: i32,
}

unsafe fn clk_generated_set(gck: *mut ClkGenerated, status: i32) -> i32 {
    let enable: u32 = if status != 0 { AT91_PMC_PCR_GCKEN } else { 0 };
    let mut flags: CULong = 0;
    spin_lock_irqsave((*gck).lock, &mut flags);
    regmap_write((*gck).regmap, (*(*gck).layout).offset,
                 (*gck).id & (*(*gck).layout).pid_mask);
    regmap_update_bits(
        (*gck).regmap,
        (*(*gck).layout).offset,
        AT91_PMC_PCR_GCKDIV_MASK | (*(*gck).layout).gckcss_mask |
            (*(*gck).layout).cmd | enable,
        field_prep((*(*gck).layout).gckcss_mask, (*gck).parent_id as u32) |
            (*(*gck).layout).cmd |
            field_prep(AT91_PMC_PCR_GCKDIV_MASK, (*gck).gckdiv) | enable,
    );
    spin_unlock_irqrestore((*gck).lock, flags);
    0
}

unsafe fn clk_generated_enable(hw: *mut ClkHw) -> i32 {
    let gck = container_of_clk_generated(hw);
    pr_debug!("GCLK: {}, gckdiv = {}, parent id = {}\n", "clk_generated_enable", (*gck).gckdiv, (*gck).parent_id);
    clk_generated_set(gck, 1);
    0
}

unsafe fn clk_generated_disable(hw: *mut ClkHw) {
    let gck = container_of_clk_generated(hw);
    let mut flags: CULong = 0;
    spin_lock_irqsave((*gck).lock, &mut flags);
    regmap_write((*gck).regmap, (*(*gck).layout).offset,
                 (*gck).id & (*(*gck).layout).pid_mask);
    regmap_update_bits((*gck).regmap, (*(*gck).layout).offset,
                       (*(*gck).layout).cmd | AT91_PMC_PCR_GCKEN,
                       (*(*gck).layout).cmd);
    spin_unlock_irqrestore((*gck).lock, flags);
}

unsafe fn clk_generated_is_enabled(hw: *mut ClkHw) -> i32 {
    let gck = container_of_clk_generated(hw);
    let mut flags: CULong = 0;
    let mut status: u32 = 0;
    spin_lock_irqsave((*gck).lock, &mut flags);
    regmap_write((*gck).regmap, (*(*gck).layout).offset,
                 (*gck).id & (*(*gck).layout).pid_mask);
    regmap_read((*gck).regmap, (*(*gck).layout).offset, &mut status);
    spin_unlock_irqrestore((*gck).lock, flags);
    if status & AT91_PMC_PCR_GCKEN != 0 { 1 } else { 0 }
}

unsafe fn clk_generated_recalc_rate(hw: *mut ClkHw, parent_rate: CULong) -> CULong {
    let gck = container_of_clk_generated(hw);
    div_round_closest(parent_rate, (*gck).gckdiv as CULong + 1)
}

unsafe fn clk_generated_best_diff(req: *mut ClkRateRequest, parent: *mut ClkHw,
                                  parent_rate: CULong, div: u32,
                                  best_diff: *mut i32, best_rate: *mut CLong) {
    let tmp_rate = if div == 0 { parent_rate } else { parent_rate / div as CULong };
    if tmp_rate < (*req).min_rate || tmp_rate > (*req).max_rate { return; }
    let tmp_diff = ((*req).rate as CLong - tmp_rate as CLong).abs() as i32;
    if *best_diff < 0 || *best_diff >= tmp_diff {
        *best_rate = tmp_rate as CLong;
        *best_diff = tmp_diff;
        (*req).best_parent_rate = parent_rate;
        (*req).best_parent_hw = parent;
    }
}

unsafe fn clk_generated_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let gck = container_of_clk_generated(hw);
    let mut parent: *mut ClkHw = core::ptr::null_mut();
    let mut best_rate: CLong = -EINVAL as CLong;
    let mut best_diff: i32 = -1;
    let mut i: u32 = 0;
    if (*gck).range.max != 0 && (*req).rate > (*gck).range.max { (*req).rate = (*gck).range.max; }
    if (*gck).range.min != 0 && (*req).rate < (*gck).range.min { (*req).rate = (*gck).range.min; }
    while i < clk_hw_get_num_parents(hw) {
        if (*gck).chg_pid != i as i32 {
            parent = clk_hw_get_parent_by_index(hw, i);
            if !parent.is_null() {
                let parent_rate = clk_hw_get_rate(parent);
                let min_rate = div_round_closest(parent_rate, GENERATED_MAX_DIV as CULong + 1);
                if parent_rate != 0 && ((*gck).range.max == 0 || min_rate <= (*gck).range.max) {
                    let mut div = div_round_closest(parent_rate, (*req).rate) as u32;
                    if div > GENERATED_MAX_DIV + 1 { div = GENERATED_MAX_DIV + 1; }
                    clk_generated_best_diff(req, parent, parent_rate, div, &mut best_diff, &mut best_rate);
                    if best_diff == 0 { break; }
                }
            }
        }
        i += 1;
    }
    if (*gck).chg_pid >= 0 {
        parent = clk_hw_get_parent_by_index(hw, (*gck).chg_pid as u32);
        if !parent.is_null() {
            let mut div = 1;
            while div < GENERATED_MAX_DIV + 2 {
                let mut req_parent = core::mem::zeroed();
                clk_hw_forward_rate_request(hw, req, parent, &mut req_parent, (*req).rate * div as CULong);
                if __clk_determine_rate(parent, &mut req_parent) == 0 {
                    clk_generated_best_diff(req, parent, req_parent.rate, div, &mut best_diff, &mut best_rate);
                    if best_diff == 0 { break; }
                }
                div += 1;
            }
        }
    }
    pr_debug!("GCLK: {}, best_rate = {}, parent clk: {} @ {}\n", "clk_generated_determine_rate", best_rate, __clk_get_name((*req).best_parent_hw), (*req).best_parent_rate);
    if best_rate < 0 || ((*gck).range.max != 0 && best_rate as CULong > (*gck).range.max) { return -EINVAL; }
    (*req).rate = best_rate as CULong;
    0
}

unsafe fn clk_generated_set_parent(hw: *mut ClkHw, index: u8) -> i32 {
    let gck = container_of_clk_generated(hw);
    if index as u32 >= clk_hw_get_num_parents(hw) { return -EINVAL; }
    (*gck).parent_id = if !(*gck).mux_table.is_null() { clk_mux_index_to_val((*gck).mux_table, 0, index) as u8 } else { index };
    0
}

unsafe fn clk_generated_get_parent(hw: *mut ClkHw) -> u8 { (*container_of_clk_generated(hw)).parent_id }

unsafe fn clk_generated_set_rate(hw: *mut ClkHw, rate: CULong, parent_rate: CULong) -> i32 {
    let gck = container_of_clk_generated(hw);
    if rate == 0 || ((*gck).range.max != 0 && rate > (*gck).range.max) { return -EINVAL; }
    let div = div_round_closest(parent_rate, rate) as u32;
    if div > GENERATED_MAX_DIV + 1 || div == 0 { return -EINVAL; }
    (*gck).gckdiv = div - 1;
    0
}

unsafe fn clk_generated_save_context(hw: *mut ClkHw) -> i32 {
    let gck = container_of_clk_generated(hw);
    (*gck).pms.status = clk_generated_is_enabled(&mut (*gck).hw) != 0;
    0
}

unsafe fn clk_generated_restore_context(hw: *mut ClkHw) {
    let gck = container_of_clk_generated(hw);
    if (*gck).pms.status { clk_generated_set(gck, 1); }
}

static GENERATED_OPS: ClkOps = ClkOps {
    enable: Some(clk_generated_enable), disable: Some(clk_generated_disable),
    is_enabled: Some(clk_generated_is_enabled), recalc_rate: Some(clk_generated_recalc_rate),
    determine_rate: Some(clk_generated_determine_rate), get_parent: Some(clk_generated_get_parent),
    set_parent: Some(clk_generated_set_parent), set_rate: Some(clk_generated_set_rate),
    save_context: Some(clk_generated_save_context), restore_context: Some(clk_generated_restore_context),
};

unsafe fn clk_generated_startup(gck: *mut ClkGenerated) {
    let mut tmp = 0u32; let mut flags: CULong = 0;
    spin_lock_irqsave((*gck).lock, &mut flags);
    regmap_write((*gck).regmap, (*(*gck).layout).offset, (*gck).id & (*(*gck).layout).pid_mask);
    regmap_read((*gck).regmap, (*(*gck).layout).offset, &mut tmp);
    spin_unlock_irqrestore((*gck).lock, flags);
    (*gck).parent_id = field_get((*(*gck).layout).gckcss_mask, tmp) as u8;
    (*gck).gckdiv = field_get(AT91_PMC_PCR_GCKDIV_MASK, tmp);
}

unsafe fn at91_clk_register_generated(regmap: *mut Regmap, lock: *mut Spinlock,
                                      layout: *const ClkPcrLayout, name: *const i8,
                                      parent_names: *const *const i8,
                                      parent_hws: *mut *mut ClkHw,
                                      mux_table: *mut u32, num_parents: u8, id: u8,
                                      range: *const ClkRange, chg_pid: i32) -> *mut ClkHw {
    if parent_names.is_null() && parent_hws.is_null() { return err_ptr(-ENOMEM); }
    let gck = kzalloc_clk_generated();
    if gck.is_null() { return err_ptr(-ENOMEM); }
    let mut init: ClkInitData = core::mem::zeroed();
    init.name = name;
    init.ops = &GENERATED_OPS;
    if !parent_hws.is_null() { init.parent_hws = parent_hws as *const *const ClkHw; }
    else { init.parent_names = parent_names; }
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
    if chg_pid >= 0 { init.flags |= CLK_SET_RATE_PARENT; }
    (*gck).id = id as u32;
    (*gck).hw.init = &init;
    (*gck).regmap = regmap;
    (*gck).lock = lock;
    (*gck).range = *range;
    (*gck).chg_pid = chg_pid;
    (*gck).layout = layout;
    (*gck).mux_table = mux_table;
    clk_generated_startup(gck);
    let hw = &mut (*gck).hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree_clk_generated(gck); return err_ptr(ret); }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
