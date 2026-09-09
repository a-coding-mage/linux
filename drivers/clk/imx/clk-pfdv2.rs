// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 * Author: Dong Aisheng <aisheng.dong@nxp.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this file.

#[repr(C)]
pub struct clk_pfdv2 {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub gate_bit: u8,
    pub vld_bit: u8,
    pub frac_off: u8,
}

pub const CLK_PFDV2_FRAC_MASK: u32 = 0x3f;
pub const LOCK_TIMEOUT_US: u32 = USEC_PER_MSEC;

static mut pfd_lock: spinlock_t = spinlock_t::new();

unsafe fn clk_pfdv2_wait(pfd: *mut clk_pfdv2) -> i32 {
    let mut val: u32 = 0;
    readl_poll_timeout(
        (*pfd).reg,
        &mut val,
        val & (1u32 << (*pfd).vld_bit) != 0,
        0,
        LOCK_TIMEOUT_US,
    )
}

unsafe fn clk_pfdv2_enable(hw: *mut clk_hw) -> i32 {
    let pfd = container_of_clk_pfdv2(hw);
    let mut flags: unsigned_long = 0;
    let mut val: u32;

    spin_lock_irqsave(&mut pfd_lock, &mut flags);
    val = readl_relaxed((*pfd).reg);
    val &= !(1u32 << (*pfd).gate_bit);
    writel_relaxed(val, (*pfd).reg);
    spin_unlock_irqrestore(&mut pfd_lock, flags);

    clk_pfdv2_wait(pfd)
}

unsafe fn clk_pfdv2_disable(hw: *mut clk_hw) {
    let pfd = container_of_clk_pfdv2(hw);
    let mut flags: unsigned_long = 0;
    let mut val: u32;

    spin_lock_irqsave(&mut pfd_lock, &mut flags);
    val = readl_relaxed((*pfd).reg);
    val |= 1u32 << (*pfd).gate_bit;
    writel_relaxed(val, (*pfd).reg);
    spin_unlock_irqrestore(&mut pfd_lock, flags);
}

unsafe fn clk_pfdv2_recalc_rate(hw: *mut clk_hw, parent_rate: unsigned_long) -> unsigned_long {
    let pfd = container_of_clk_pfdv2(hw);
    let mut tmp: u64 = parent_rate as u64;
    let frac = ((readl_relaxed((*pfd).reg) >> (*pfd).frac_off) & CLK_PFDV2_FRAC_MASK) as u8;

    if frac == 0 {
        pr_debug("clk_pfdv2: %s invalid pfd frac value 0\n", clk_hw_get_name(hw));
        return 0;
    }

    tmp *= 18;
    tmp /= frac as u64;
    tmp as unsigned_long
}

unsafe fn clk_pfdv2_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let parent_rates: [unsigned_long; 3] = [480000000, 528000000, (*req).best_parent_rate];
    let mut best_rate: unsigned_long = !0;
    let rate = (*req).rate;
    let mut best_parent_rate = (*req).best_parent_rate;

    for parent_rate in parent_rates {
        let mut tmp = parent_rate as u64;
        tmp = tmp * 18 + (rate / 2) as u64;
        tmp /= rate as u64;
        let mut frac = tmp as u8;

        if frac < 12 { frac = 12; }
        else if frac > 35 { frac = 35; }

        tmp = parent_rate as u64;
        tmp *= 18;
        tmp /= frac as u64;
        let candidate = tmp as unsigned_long;

        if abs_diff(candidate, rate) < abs_diff(best_rate, rate) {
            best_rate = candidate;
            best_parent_rate = parent_rate;
        }
    }

    (*req).best_parent_rate = best_parent_rate;
    (*req).rate = best_rate;
    0
}

unsafe fn clk_pfdv2_is_enabled(hw: *mut clk_hw) -> i32 {
    let pfd = container_of_clk_pfdv2(hw);
    if readl_relaxed((*pfd).reg) & (1u32 << (*pfd).gate_bit) != 0 { 0 } else { 1 }
}

unsafe fn clk_pfdv2_set_rate(hw: *mut clk_hw, rate: unsigned_long, parent_rate: unsigned_long) -> i32 {
    let pfd = container_of_clk_pfdv2(hw);
    let mut flags: unsigned_long = 0;
    let mut tmp = parent_rate as u64;
    let mut val: u32;

    if rate == 0 { return -EINVAL; }
    if clk_pfdv2_is_enabled(hw) != 0 { clk_pfdv2_disable(hw); }

    tmp = tmp * 18 + (rate / 2) as u64;
    tmp /= rate as u64;
    let mut frac = tmp as u8;
    if frac < 12 { frac = 12; }
    else if frac > 35 { frac = 35; }

    spin_lock_irqsave(&mut pfd_lock, &mut flags);
    val = readl_relaxed((*pfd).reg);
    val &= !(CLK_PFDV2_FRAC_MASK << (*pfd).frac_off);
    val |= (frac as u32) << (*pfd).frac_off;
    writel_relaxed(val, (*pfd).reg);
    spin_unlock_irqrestore(&mut pfd_lock, flags);
    0
}

static clk_pfdv2_ops: clk_ops = clk_ops {
    enable: Some(clk_pfdv2_enable),
    disable: Some(clk_pfdv2_disable),
    recalc_rate: Some(clk_pfdv2_recalc_rate),
    determine_rate: Some(clk_pfdv2_determine_rate),
    set_rate: Some(clk_pfdv2_set_rate),
    is_enabled: Some(clk_pfdv2_is_enabled),
};

pub unsafe fn imx_clk_hw_pfdv2(
    type_: imx_pfdv2_type,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
    idx: u8,
) -> *mut clk_hw {
    WARN_ON(idx > 3);
    let pfd = kzalloc_obj::<clk_pfdv2>();
    if pfd.is_null() { return ERR_PTR(-ENOMEM); }

    (*pfd).reg = reg;
    (*pfd).gate_bit = (idx + 1) * 8 - 1;
    (*pfd).vld_bit = (*pfd).gate_bit - 1;
    (*pfd).frac_off = idx * 8;

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &clk_pfdv2_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = if type_ == IMX_PFDV2_IMX7ULP {
        CLK_SET_RATE_GATE | CLK_SET_RATE_PARENT
    } else { CLK_SET_RATE_GATE };
    (*pfd).hw.init = &init;

    let hw = &mut (*pfd).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(pfd as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }
    hw
}

// EXPORT_SYMBOL_GPL(imx_clk_hw_pfdv2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
