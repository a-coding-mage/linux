// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const SUPER_STATE_IDLE: u32 = 0;
const SUPER_STATE_RUN: u32 = 1;
const SUPER_STATE_IRQ: u32 = 2;
const SUPER_STATE_FIQ: u32 = 3;

const SUPER_STATE_SHIFT: u32 = 28;
const SUPER_STATE_MASK: u32 =
    ((1u32 << SUPER_STATE_IDLE) | (1u32 << SUPER_STATE_RUN) |
     (1u32 << SUPER_STATE_IRQ) | (1u32 << SUPER_STATE_FIQ)) << SUPER_STATE_SHIFT;

const SUPER_LP_DIV2_BYPASS: u32 = 1 << 16;
const CCLK_SRC_PLLP_OUT0: u8 = 4;
const CCLK_SRC_PLLP_OUT4: u8 = 5;

#[inline]
fn super_state(s: u32) -> u32 { (1u32 << s) << SUPER_STATE_SHIFT }

#[inline]
unsafe fn super_state_to_src_shift(m: *const tegra_clk_super_mux, s: u8) -> u32 {
    ((*m).width as u32) * (s as u32)
}

#[inline]
unsafe fn super_state_to_src_mask(m: *const tegra_clk_super_mux) -> u32 {
    (1u32 << (*m).width) - 1
}

unsafe fn clk_super_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_clk_super_mux(hw);
    let val = readl_relaxed((*mux).reg);
    let state = val & SUPER_STATE_MASK;

    BUG_ON(state != super_state(SUPER_STATE_RUN) && state != super_state(SUPER_STATE_IDLE));
    let shift = if state == super_state(SUPER_STATE_IDLE) {
        super_state_to_src_shift(mux, SUPER_STATE_IDLE as u8)
    } else {
        super_state_to_src_shift(mux, SUPER_STATE_RUN as u8)
    };
    let mut source = ((val >> shift) & super_state_to_src_mask(mux)) as u8;

    // If LP_DIV2_BYPASS is not set and PLLX is current parent, PLLX/2 is the input source.
    if ((*mux).flags & TEGRA_DIVIDER_2) != 0 && (val & SUPER_LP_DIV2_BYPASS) == 0 &&
        source == (*mux).pllx_index {
        source = (*mux).div2_index;
    }
    source
}

unsafe fn clk_super_set_parent(hw: *mut clk_hw, mut index: u8) -> i32 {
    let mux = to_clk_super_mux(hw);
    let mut err: i32 = 0;
    let mut flags: c_ulong = 0;
    if !(*mux).lock.is_null() { spin_lock_irqsave((*mux).lock, &mut flags); }

    let mut val = readl_relaxed((*mux).reg);
    let state = val & SUPER_STATE_MASK;
    BUG_ON(state != super_state(SUPER_STATE_RUN) && state != super_state(SUPER_STATE_IDLE));
    let shift = if state == super_state(SUPER_STATE_IDLE) {
        super_state_to_src_shift(mux, SUPER_STATE_IDLE as u8)
    } else { super_state_to_src_shift(mux, SUPER_STATE_RUN as u8) };

    if ((*mux).flags & TEGRA_DIVIDER_2) != 0 &&
        (index == (*mux).div2_index || index == (*mux).pllx_index) {
        let parent_index = clk_super_get_parent(hw);
        if parent_index == (*mux).div2_index || parent_index == (*mux).pllx_index {
            err = -EINVAL;
            goto out;
        }
        val ^= SUPER_LP_DIV2_BYPASS;
        writel_relaxed(val, (*mux).reg);
        udelay(2);
        if index == (*mux).div2_index { index = (*mux).pllx_index; }
    }

    if ((*mux).flags & TEGRA210_CPU_CLK) != 0 &&
        (index == CCLK_SRC_PLLP_OUT0 || index == CCLK_SRC_PLLP_OUT4) {
        tegra_clk_set_pllp_out_cpu(true);
    }
    val &= !(super_state_to_src_mask(mux) << shift);
    val |= ((index as u32) & super_state_to_src_mask(mux)) << shift;
    writel_relaxed(val, (*mux).reg);
    udelay(2);
    if ((*mux).flags & TEGRA210_CPU_CLK) != 0 &&
        index != CCLK_SRC_PLLP_OUT0 && index != CCLK_SRC_PLLP_OUT4 {
        tegra_clk_set_pllp_out_cpu(false);
    }
out:
    if !(*mux).lock.is_null() { spin_unlock_irqrestore((*mux).lock, flags); }
    err
}

unsafe fn clk_super_mux_restore_context(hw: *mut clk_hw) {
    let parent_id = clk_hw_get_parent_index(hw);
    if WARN_ON(parent_id < 0) { return; }
    clk_super_set_parent(hw, parent_id as u8);
}

static tegra_clk_super_mux_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(clk_super_get_parent), set_parent: Some(clk_super_set_parent),
    restore_context: Some(clk_super_mux_restore_context),
};

unsafe fn clk_super_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let super_ = to_clk_super_mux(hw); let div_hw = &mut (*super_).frac_div.hw;
    __clk_hw_set_clk(div_hw, hw); ((*super_).div_ops).determine_rate(div_hw, req)
}
unsafe fn clk_super_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let super_ = to_clk_super_mux(hw); let div_hw = &mut (*super_).frac_div.hw;
    __clk_hw_set_clk(div_hw, hw); ((*super_).div_ops).recalc_rate(div_hw, parent_rate)
}
unsafe fn clk_super_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let super_ = to_clk_super_mux(hw); let div_hw = &mut (*super_).frac_div.hw;
    __clk_hw_set_clk(div_hw, hw); ((*super_).div_ops).set_rate(div_hw, rate, parent_rate)
}
unsafe fn clk_super_restore_context(hw: *mut clk_hw) {
    let super_ = to_clk_super_mux(hw); let div_hw = &mut (*super_).frac_div.hw;
    let parent_id = clk_hw_get_parent_index(hw); if WARN_ON(parent_id < 0) { return; }
    ((*super_).div_ops).restore_context(div_hw); clk_super_set_parent(hw, parent_id as u8);
}

static tegra_clk_super_ops: clk_ops = clk_ops {
    get_parent: Some(clk_super_get_parent), set_parent: Some(clk_super_set_parent),
    set_rate: Some(clk_super_set_rate), determine_rate: Some(clk_super_determine_rate),
    recalc_rate: Some(clk_super_recalc_rate), restore_context: Some(clk_super_restore_context),
};

unsafe fn tegra_clk_register_super_mux(name: *const c_char, parent_names: *const *const c_char,
    num_parents: u8, flags: c_ulong, reg: *mut c_void, clk_super_flags: u8, width: u8,
    pllx_index: u8, div2_index: u8, lock: *mut spinlock_t) -> *mut clk {
    let super_ = kzalloc_obj::<tegra_clk_super_mux>(); if super_.is_null() { return ERR_PTR(-ENOMEM); }
    let init = clk_init_data { name, ops: &tegra_clk_super_mux_ops, flags, parent_names, num_parents };
    (*super_).reg = reg; (*super_).pllx_index = pllx_index; (*super_).div2_index = div2_index;
    (*super_).lock = lock; (*super_).width = width; (*super_).flags = clk_super_flags;
    (*super_).hw.init = &init;
    let clk = tegra_clk_dev_register(&mut (*super_).hw); if IS_ERR(clk) { kfree(super_); } clk
}

unsafe fn tegra_clk_register_super_clk(name: *const c_char, parent_names: *const *const c_char,
    num_parents: u8, flags: c_ulong, reg: *mut c_void, clk_super_flags: u8,
    lock: *mut spinlock_t) -> *mut clk {
    let super_ = kzalloc_obj::<tegra_clk_super_mux>(); if super_.is_null() { return ERR_PTR(-ENOMEM); }
    let init = clk_init_data { name, ops: &tegra_clk_super_ops, flags, parent_names, num_parents };
    (*super_).reg = reg; (*super_).lock = lock; (*super_).width = 4; (*super_).flags = clk_super_flags;
    (*super_).frac_div.reg = reg.add(4); (*super_).frac_div.shift = 16; (*super_).frac_div.width = 8;
    (*super_).frac_div.frac_width = 1; (*super_).frac_div.lock = lock;
    (*super_).div_ops = &tegra_clk_frac_div_ops; (*super_).hw.init = &init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*super_).hw); if IS_ERR(clk) { kfree(super_); } clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
