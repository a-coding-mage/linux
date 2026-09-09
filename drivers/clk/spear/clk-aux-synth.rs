// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * Auxiliary Synthesizer clock implementation
 */

// Dependency intent from Linux headers and "clk.h" is preserved through the
// external types, constants, and functions referenced below.

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: core::ffi::c_ulong,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, core::ffi::c_ulong) -> core::ffi::c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, core::ffi::c_ulong, core::ffi::c_ulong) -> i32>,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: core::ffi::c_ulong,
    pub best_parent_rate: core::ffi::c_ulong,
}

#[repr(C)]
pub struct aux_clk_masks {
    pub eq_sel_mask: u32,
    pub eq_sel_shift: u32,
    pub eq1_mask: u32,
    pub eq2_mask: u32,
    pub xscale_sel_mask: u32,
    pub xscale_sel_shift: u32,
    pub yscale_sel_mask: u32,
    pub yscale_sel_shift: u32,
    pub enable_bit: u32,
}

#[repr(C)]
pub struct aux_rate_tbl {
    pub eq: u32,
    pub xscale: u32,
    pub yscale: u32,
}

#[repr(C)]
pub struct clk_aux {
    pub hw: clk_hw,
    pub masks: *const aux_clk_masks,
    pub reg: *mut core::ffi::c_void,
    pub rtbl: *mut aux_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct spinlock_t {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    static AUX_EQ_SEL_MASK: u32;
    static AUX_EQ_SEL_SHIFT: u32;
    static AUX_EQ1_SEL: u32;
    static AUX_EQ2_SEL: u32;
    static AUX_XSCALE_MASK: u32;
    static AUX_XSCALE_SHIFT: u32;
    static AUX_YSCALE_MASK: u32;
    static AUX_YSCALE_SHIFT: u32;
    static AUX_SYNT_ENB: u32;

    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn clk_round_rate_index(
        hw: *mut clk_hw,
        rate: core::ffi::c_ulong,
        parent_rate: core::ffi::c_ulong,
        callback: unsafe extern "C" fn(*mut clk_hw, core::ffi::c_ulong, i32) -> core::ffi::c_ulong,
        count: u8,
        index: *mut i32,
    ) -> core::ffi::c_ulong;
    fn clk_register(parent: *mut clk, hw: *mut clk_hw) -> *mut clk;
    fn clk_register_gate(
        parent: *mut clk,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: core::ffi::c_ulong,
        reg: *mut core::ffi::c_void,
        bit_idx: u32,
        flags2: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

const CLK_SET_RATE_PARENT: core::ffi::c_ulong = 1 << 2;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

static DEFAULT_AUX_MASKS: aux_clk_masks = aux_clk_masks {
    eq_sel_mask: unsafe { AUX_EQ_SEL_MASK },
    eq_sel_shift: unsafe { AUX_EQ_SEL_SHIFT },
    eq1_mask: unsafe { AUX_EQ1_SEL },
    eq2_mask: unsafe { AUX_EQ2_SEL },
    xscale_sel_mask: unsafe { AUX_XSCALE_MASK },
    xscale_sel_shift: unsafe { AUX_XSCALE_SHIFT },
    yscale_sel_mask: unsafe { AUX_YSCALE_MASK },
    yscale_sel_shift: unsafe { AUX_YSCALE_SHIFT },
    enable_bit: unsafe { AUX_SYNT_ENB },
};

unsafe fn aux_calc_rate(hw: *mut clk_hw, prate: core::ffi::c_ulong, index: i32) -> core::ffi::c_ulong {
    let aux = &*(hw as *mut clk_aux);
    let rtbl = &*aux.rtbl.add(index as usize);
    let eq = if rtbl.eq != 0 { 1 } else { 2 };
    (((prate / 10000) * rtbl.xscale as core::ffi::c_ulong)
        / (rtbl.yscale as core::ffi::c_ulong * eq)) * 10000
}

unsafe fn clk_aux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let aux = &*(hw as *mut clk_aux);
    let mut unused = 0i32;
    (*req).rate = clk_round_rate_index(hw, (*req).rate, (*req).best_parent_rate,
        aux_calc_rate, aux.rtbl_cnt, &mut unused);
    0
}

unsafe fn clk_aux_recalc_rate(hw: *mut clk_hw, parent_rate: core::ffi::c_ulong) -> core::ffi::c_ulong {
    let aux = &*(hw as *mut clk_aux);
    let mut num = 1u32;
    let mut den = 1u32;
    let mut flags = 0usize;
    if !aux.lock.is_null() { spin_lock_irqsave(aux.lock, &mut flags); }
    let val = readl_relaxed(aux.reg);
    if !aux.lock.is_null() { spin_unlock_irqrestore(aux.lock, flags); }
    let masks = &*aux.masks;
    let eqn = (val >> masks.eq_sel_shift) & masks.eq_sel_mask;
    if eqn == masks.eq1_mask { den = 2; }
    num = (val >> masks.xscale_sel_shift) & masks.xscale_sel_mask;
    den *= (val >> masks.yscale_sel_shift) & masks.yscale_sel_mask;
    if den == 0 { return 0; }
    (((parent_rate / 10000) * num as core::ffi::c_ulong) / den as core::ffi::c_ulong) * 10000
}

unsafe fn clk_aux_set_rate(hw: *mut clk_hw, drate: core::ffi::c_ulong, prate: core::ffi::c_ulong) -> i32 {
    let aux = &*(hw as *mut clk_aux);
    let mut i = 0i32;
    clk_round_rate_index(hw, drate, prate, aux_calc_rate, aux.rtbl_cnt, &mut i);
    let rtbl = &*aux.rtbl.add(i as usize);
    let masks = &*aux.masks;
    let mut flags = 0usize;
    if !aux.lock.is_null() { spin_lock_irqsave(aux.lock, &mut flags); }
    let mut val = readl_relaxed(aux.reg) & !(masks.eq_sel_mask << masks.eq_sel_shift);
    val |= (rtbl.eq & masks.eq_sel_mask) << masks.eq_sel_shift;
    val &= !(masks.xscale_sel_mask << masks.xscale_sel_shift);
    val |= (rtbl.xscale & masks.xscale_sel_mask) << masks.xscale_sel_shift;
    val &= !(masks.yscale_sel_mask << masks.yscale_sel_shift);
    val |= (rtbl.yscale & masks.yscale_sel_mask) << masks.yscale_sel_shift;
    writel_relaxed(val, aux.reg);
    if !aux.lock.is_null() { spin_unlock_irqrestore(aux.lock, flags); }
    0
}

unsafe extern "C" fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
unsafe extern "C" fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);

static CLK_AUX_OPS: clk_ops = clk_ops {
    recalc_rate: Some(clk_aux_recalc_rate),
    determine_rate: Some(clk_aux_determine_rate),
    set_rate: Some(clk_aux_set_rate),
};

pub unsafe fn clk_register_aux(
    aux_name: *const core::ffi::c_char, gate_name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char, flags: core::ffi::c_ulong,
    reg: *mut core::ffi::c_void, masks: *const aux_clk_masks,
    rtbl: *mut aux_rate_tbl, rtbl_cnt: u8, lock: *mut spinlock_t,
    gate_clk: *mut *mut clk,
) -> *mut clk {
    if aux_name.is_null() || parent_name.is_null() || reg.is_null() || rtbl.is_null() || rtbl_cnt == 0 {
        return (-EINVAL as isize) as *mut clk;
    }
    let aux = kzalloc(core::mem::size_of::<clk_aux>()) as *mut clk_aux;
    if aux.is_null() { return (-ENOMEM as isize) as *mut clk; }
    (*aux).masks = if masks.is_null() { &DEFAULT_AUX_MASKS } else { masks };
    (*aux).reg = reg; (*aux).rtbl = rtbl; (*aux).rtbl_cnt = rtbl_cnt; (*aux).lock = lock;
    let mut init = clk_init_data { name: aux_name, ops: &CLK_AUX_OPS, flags,
        parent_names: &parent_name, num_parents: 1 };
    (*aux).hw.init = &mut init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*aux).hw);
    if clk.is_null() { kfree(aux as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    if !gate_name.is_null() {
        let gate = clk_register_gate(core::ptr::null_mut(), gate_name, aux_name, CLK_SET_RATE_PARENT,
            reg, (*(*aux).masks).enable_bit, 0, lock);
        if gate.is_null() { kfree(aux as *mut core::ffi::c_void); return core::ptr::null_mut(); }
        if !gate_clk.is_null() { *gate_clk = gate; }
    }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
