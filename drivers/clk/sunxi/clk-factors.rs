// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Emilio López <emilio@elopez.com.ar>
 *
 * Adjustable factor-based clock implementation
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this file.

const FACTORS_MAX_PARENTS: usize = 5;

#[inline]
const fn setmask(len: u32, pos: u32) -> u32 {
    ((1u32 << len).wrapping_sub(1)) << pos
}

#[inline]
const fn clrmask(len: u32, pos: u32) -> u32 {
    !setmask(len, pos)
}

#[inline]
fn factor_get(bit: u32, len: u32, reg: u32) -> u32 {
    (reg & setmask(len, bit)) >> bit
}

#[inline]
fn factor_set(bit: u32, len: u32, reg: u32, val: u32) -> u32 {
    (reg & clrmask(len, bit)) | (val << bit)
}

unsafe fn clk_factors_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let mut n: u8 = 1;
    let mut k: u8 = 0;
    let mut p: u8 = 0;
    let mut m: u8 = 0;
    let factors = container_of_clk_factors(hw);
    let config = (*factors).config;
    let reg = readl((*factors).reg);

    if (*config).nwidth != SUNXI_FACTORS_NOT_APPLICABLE {
        n = factor_get((*config).nshift, (*config).nwidth, reg) as u8;
    }
    if (*config).kwidth != SUNXI_FACTORS_NOT_APPLICABLE {
        k = factor_get((*config).kshift, (*config).kwidth, reg) as u8;
    }
    if (*config).mwidth != SUNXI_FACTORS_NOT_APPLICABLE {
        m = factor_get((*config).mshift, (*config).mwidth, reg) as u8;
    }
    if (*config).pwidth != SUNXI_FACTORS_NOT_APPLICABLE {
        p = factor_get((*config).pshift, (*config).pwidth, reg) as u8;
    }

    if let Some(recalc) = (*factors).recalc {
        let mut factors_req = factors_request {
            parent_rate,
            n,
            k,
            m,
            p,
            ..core::mem::zeroed()
        };

        if !(*factors).mux.is_null() {
            factors_req.parent_index = (reg >> (*(*factors).mux).shift)
                & (*(*factors).mux).mask;
        }
        recalc(&mut factors_req);
        return factors_req.rate;
    }

    (parent_rate * (n as c_ulong + (*config).n_start as c_ulong)
        * (k as c_ulong + 1) >> p) / (m as c_ulong + 1)
}

unsafe fn clk_factors_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let factors = container_of_clk_factors(hw);
    let mut best_parent: *mut clk_hw = core::ptr::null_mut();
    let mut best: c_ulong = 0;
    let mut best_child_rate: c_ulong = 0;
    let num_parents = clk_hw_get_num_parents(hw);

    for i in 0..num_parents {
        let mut factors_req = factors_request {
            rate: (*req).rate,
            parent_index: i,
            ..core::mem::zeroed()
        };
        let parent = clk_hw_get_parent_by_index(hw, i);
        if parent.is_null() { continue; }
        let parent_rate = if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT != 0 {
            clk_hw_round_rate(parent, (*req).rate)
        } else { clk_hw_get_rate(parent) };
        factors_req.parent_rate = parent_rate;
        ((*factors).get_factors)(&mut factors_req);
        if factors_req.rate <= (*req).rate && factors_req.rate > best_child_rate {
            best_parent = parent;
            best = parent_rate;
            best_child_rate = factors_req.rate;
        }
    }
    if best_parent.is_null() { return -EINVAL; }
    (*req).best_parent_hw = best_parent;
    (*req).best_parent_rate = best;
    (*req).rate = best_child_rate;
    0
}

unsafe fn clk_factors_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let factors = container_of_clk_factors(hw);
    let config = (*factors).config;
    let mut req = factors_request { rate, parent_rate, ..core::mem::zeroed() };
    let mut flags: c_ulong = 0;
    ((*factors).get_factors)(&mut req);
    if !(*factors).lock.is_null() { spin_lock_irqsave((*factors).lock, &mut flags); }
    let mut reg = readl((*factors).reg);
    reg = factor_set((*config).nshift, (*config).nwidth, reg, req.n as u32);
    reg = factor_set((*config).kshift, (*config).kwidth, reg, req.k as u32);
    reg = factor_set((*config).mshift, (*config).mwidth, reg, req.m as u32);
    reg = factor_set((*config).pshift, (*config).pwidth, reg, req.p as u32);
    writel(reg, (*factors).reg);
    __delay((rate >> 20) * 500 / 2);
    if !(*factors).lock.is_null() { spin_unlock_irqrestore((*factors).lock, flags); }
    0
}

static clk_factors_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_factors_determine_rate),
    recalc_rate: Some(clk_factors_recalc_rate),
    set_rate: Some(clk_factors_set_rate),
};

unsafe fn __sunxi_factors_register(
    node: *mut device_node, data: *const factors_data, lock: *mut spinlock_t,
    reg: *mut core::ffi::c_void, flags: c_ulong,
) -> *mut clk {
    let mut i: c_int;
    let mut factors: *mut clk_factors;
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let mut gate_hw: *mut clk_hw = core::ptr::null_mut();
    let mut mux_hw: *mut clk_hw = core::ptr::null_mut();
    let mut clk_name = (*node).name;
    let mut parents: [*const c_char; FACTORS_MAX_PARENTS] = [core::ptr::null(); FACTORS_MAX_PARENTS];
    i = of_clk_parent_fill(node, parents.as_mut_ptr(), FACTORS_MAX_PARENTS as c_int);
    if !(*data).name.is_null() { clk_name = (*data).name; }
    else { of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const c_char, &mut clk_name); }
    factors = kzalloc_clk_factors();
    if factors.is_null() { return core::ptr::null_mut(); }
    (*factors).reg = reg; (*factors).config = (*data).table; (*factors).get_factors = (*data).getter;
    (*factors).recalc = (*data).recalc; (*factors).lock = lock;
    if (*data).enable != 0 {
        gate = kzalloc_clk_gate(); if gate.is_null() { kfree(factors as *mut _); return core::ptr::null_mut(); }
        (*factors).gate = gate; (*gate).reg = reg; (*gate).bit_idx = (*data).enable;
        (*gate).lock = (*factors).lock; gate_hw = &mut (*gate).hw;
    }
    if (*data).mux != 0 {
        mux = kzalloc_clk_mux(); if mux.is_null() { kfree(gate as *mut _); kfree(factors as *mut _); return core::ptr::null_mut(); }
        (*factors).mux = mux; (*mux).reg = reg; (*mux).shift = (*data).mux;
        (*mux).mask = (*data).muxmask; (*mux).lock = (*factors).lock; mux_hw = &mut (*mux).hw;
    }
    let clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents.as_ptr(), i,
        mux_hw, &clk_mux_ops, &mut clk_factors_ops, gate_hw, &clk_gate_ops, CLK_IS_CRITICAL | flags);
    if IS_ERR(clk) { kfree(mux as *mut _); kfree(gate as *mut _); kfree(factors as *mut _); return core::ptr::null_mut(); }
    if of_clk_add_provider(node, of_clk_src_simple_get, clk) != 0 { clk_unregister(clk); kfree(mux as *mut _); kfree(gate as *mut _); kfree(factors as *mut _); return core::ptr::null_mut(); }
    clk
}

pub unsafe fn sunxi_factors_register(node: *mut device_node, data: *const factors_data, lock: *mut spinlock_t, reg: *mut core::ffi::c_void) -> *mut clk {
    __sunxi_factors_register(node, data, lock, reg, 0)
}

pub unsafe fn sunxi_factors_register_critical(node: *mut device_node, data: *const factors_data, lock: *mut spinlock_t, reg: *mut core::ffi::c_void) -> *mut clk {
    __sunxi_factors_register(node, data, lock, reg, CLK_IS_CRITICAL)
}

pub unsafe fn sunxi_factors_unregister(node: *mut device_node, clk: *mut clk) {
    let hw = __clk_get_hw(clk); if hw.is_null() { return; }
    let factors = container_of_clk_factors(hw);
    of_clk_del_provider(node); clk_unregister(clk);
    kfree((*factors).mux as *mut _); kfree((*factors).gate as *mut _); kfree(factors as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
