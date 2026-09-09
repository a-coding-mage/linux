// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Translated from ccu_gate.c. Declarations supplied by the Linux clock and
// Sunxi CCU headers are intentionally left as external dependencies.

use core::ffi::c_void;

extern "C" {
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn clk_hw_get_flags(hw: *mut clk_hw) -> u32;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
}

pub type c_ulong = usize;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ccu_common {
    pub lock: *mut c_void,
    pub base: *mut u8,
    pub reg: usize,
    pub features: u32,
    pub prediv: c_ulong,
}

#[repr(C)]
pub struct ccu_gate {
    pub common: ccu_common,
    pub enable: u32,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub determine_rate:
        Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}

pub type c_int = i32;

const CCU_FEATURE_UPDATE_BIT: u32 = 1 << 0;
const CCU_SUNXI_UPDATE_BIT: u32 = 1 << 31;
const CLK_SET_RATE_PARENT: u32 = 1 << 2;

#[inline]
unsafe fn hw_to_ccu_gate(hw: *mut clk_hw) -> *mut ccu_gate {
    hw as *mut ccu_gate
}

pub unsafe extern "C" fn ccu_gate_helper_disable(common: *mut ccu_common, gate: u32) {
    let mut flags: c_ulong = 0;
    let mut reg: u32;

    if gate == 0 {
        return;
    }

    spin_lock_irqsave((*common).lock, &mut flags);

    reg = readl((*common).base.add((*common).reg) as *const c_void);
    if (*common).features & CCU_FEATURE_UPDATE_BIT != 0 {
        reg |= CCU_SUNXI_UPDATE_BIT;
    }
    writel(reg & !gate, (*common).base.add((*common).reg) as *mut c_void);

    spin_unlock_irqrestore((*common).lock, flags);
}

pub unsafe extern "C" fn ccu_gate_disable(hw: *mut clk_hw) {
    let cg = hw_to_ccu_gate(hw);
    ccu_gate_helper_disable(&mut (*cg).common, (*cg).enable);
}

pub unsafe extern "C" fn ccu_gate_helper_enable(common: *mut ccu_common, gate: u32) -> c_int {
    let mut flags: c_ulong = 0;
    let mut reg: u32;

    if gate == 0 {
        return 0;
    }

    spin_lock_irqsave((*common).lock, &mut flags);

    reg = readl((*common).base.add((*common).reg) as *const c_void);
    if (*common).features & CCU_FEATURE_UPDATE_BIT != 0 {
        reg |= CCU_SUNXI_UPDATE_BIT;
    }
    writel(reg | gate, (*common).base.add((*common).reg) as *mut c_void);

    spin_unlock_irqrestore((*common).lock, flags);

    0
}

pub unsafe extern "C" fn ccu_gate_enable(hw: *mut clk_hw) -> c_int {
    let cg = hw_to_ccu_gate(hw);
    ccu_gate_helper_enable(&mut (*cg).common, (*cg).enable)
}

pub unsafe extern "C" fn ccu_gate_helper_is_enabled(common: *mut ccu_common, gate: u32) -> c_int {
    if gate == 0 {
        return 1;
    }

    (readl((*common).base.add((*common).reg) as *const c_void) & gate) as c_int
}

pub unsafe extern "C" fn ccu_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let cg = hw_to_ccu_gate(hw);
    ccu_gate_helper_is_enabled(&mut (*cg).common, (*cg).enable)
}

pub unsafe extern "C" fn ccu_gate_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let cg = hw_to_ccu_gate(hw);
    let mut rate = parent_rate;

    if (*cg).common.features & CCU_FEATURE_ALL_PREDIV != 0 {
        rate /= (*cg).common.prediv;
    }

    rate
}

pub unsafe extern "C" fn ccu_gate_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let cg = hw_to_ccu_gate(hw);
    let mut div: c_ulong = 1;

    if (*cg).common.features & CCU_FEATURE_ALL_PREDIV != 0 {
        div = (*cg).common.prediv;
    }

    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT != 0 {
        let mut best_parent = (*req).rate;

        if (*cg).common.features & CCU_FEATURE_ALL_PREDIV != 0 {
            best_parent *= div;
        }
        (*req).best_parent_rate =
            clk_hw_round_rate(clk_hw_get_parent(hw), best_parent);
    }

    (*req).rate = (*req).best_parent_rate / div;

    0
}

pub unsafe extern "C" fn ccu_gate_set_rate(
    _hw: *mut clk_hw,
    _rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    /*
     * We must report success but we can do so unconditionally because
     * clk_factor_round_rate returns values that ensure this call is a
     * nop.
     */

    0
}

pub const ccu_gate_ops: clk_ops = clk_ops {
    disable: Some(ccu_gate_disable),
    enable: Some(ccu_gate_enable),
    is_enabled: Some(ccu_gate_is_enabled),
    determine_rate: Some(ccu_gate_determine_rate),
    set_rate: Some(ccu_gate_set_rate),
    recalc_rate: Some(ccu_gate_recalc_rate),
};

// CCU_FEATURE_ALL_PREDIV is supplied by the Sunxi CCU headers.
extern "C" {
    static CCU_FEATURE_ALL_PREDIV: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
