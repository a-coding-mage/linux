// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 NVIDIA CORPORATION.  All rights reserved.
 *
 * based on clk-mux.c
 *
 * Copyright (C) 2011 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
 * Copyright (C) 2011 Richard Zhao, Linaro <richard.zhao@linaro.org>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 */

// External declarations supplied by the kernel and clk support code.
extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn fence_udelay(usecs: u32, addr: *mut core::ffi::c_void);
    fn clk_hw_get_num_parents(hw: *mut clk_hw) -> i32;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> usize;
    fn clk_hw_get_parent_index(hw: *mut clk_hw) -> i32;
    fn clk_register(parent: *mut core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
    fn get_reg_bank(clk_num: u32) -> *const tegra_clk_periph_regs;
    fn div_frac_get(rate: usize, parent_rate: usize, width: u32, shift: u32,
                    flags: u8) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree<T>(ptr: *mut T);
    fn __clk_hw_set_clk(gate_hw: *mut clk_hw, hw: *mut clk_hw);
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
}

const DIV_MASK: u32 = 0xff;
const MUX_SHIFT: u32 = 29;
const MUX_MASK: u32 = 0x_e000_0000;
const SDMMC_MUL: usize = 2;
const TEGRA_DIVIDER_ROUND_UP: u8 = 1 << 0;
const TEGRA_PERIPH_ON_APB: u32 = 1 << 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

#[inline]
fn get_max_div(_d: u32) -> u32 { DIV_MASK }

#[inline]
fn get_div_field(val: u32) -> u32 { val & DIV_MASK }

#[inline]
fn get_mux_field(val: u32) -> u32 { (val & MUX_MASK) >> MUX_SHIFT }

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data, pub clk: *mut clk }
#[repr(C)] pub struct clk;
#[repr(C)] pub struct clk_init_data {
    pub ops: *const clk_ops, pub name: *const core::ffi::c_char,
    pub flags: usize, pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: usize,
}
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub min_rate: usize, pub max_rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub disable_unused: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub restore_context: Option<unsafe extern "C" fn(*mut clk_hw)>,
}
#[repr(C)] pub struct tegra_clk_periph_regs;
#[repr(C)] pub struct tegra_periph_clk {
    pub hw: clk_hw, pub clk_base: *mut core::ffi::c_void,
    pub regs: *const tegra_clk_periph_regs, pub enable_refcnt: *mut u32,
    pub clk_num: u32, pub flags: u32,
}
#[repr(C)] pub struct tegra_sdmmc_mux {
    pub hw: clk_hw, pub reg: *mut core::ffi::c_void, pub lock: *mut core::ffi::c_void,
    pub gate: tegra_periph_clk, pub div_flags: u8, pub gate_ops: *const clk_ops,
}

static MUX_SDMMC_PARENTS: [*const core::ffi::c_char; 5] = [
    b"pll_p\0".as_ptr() as _, b"pll_c4_out2\0".as_ptr() as _,
    b"pll_c4_out0\0".as_ptr() as _, b"pll_c4_out1\0".as_ptr() as _,
    b"clk_m\0".as_ptr() as _,
];
static MUX_LJ_IDX: [u8; 5] = [0, 1, 2, 5, 6];
static MUX_NON_LJ_IDX: [u8; 5] = [0, 3, 7, 4, 6];

#[inline]
unsafe fn to_clk_sdmmc_mux(hw: *mut clk_hw) -> *mut tegra_sdmmc_mux { hw as *mut tegra_sdmmc_mux }

unsafe extern "C" fn clk_sdmmc_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = &*to_clk_sdmmc_mux(hw); let val = readl_relaxed(mux.reg);
    let src = get_mux_field(val); let idx = if get_div_field(val) != 0 { &MUX_NON_LJ_IDX } else { &MUX_LJ_IDX };
    for i in 0..5 { if idx[i] as u32 == src { return i as u8; } }
    0
}

unsafe extern "C" fn clk_sdmmc_mux_set_parent(hw: *mut clk_hw, mut index: u8) -> i32 {
    let mux = &*to_clk_sdmmc_mux(hw); let mut val = readl_relaxed(mux.reg);
    index = if get_div_field(val) != 0 { MUX_NON_LJ_IDX[index as usize] } else { MUX_LJ_IDX[index as usize] };
    val = (val & !MUX_MASK) | ((index as u32) << MUX_SHIFT); writel(val, mux.reg); 0
}

unsafe extern "C" fn clk_sdmmc_mux_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let mux = &*to_clk_sdmmc_mux(hw); let div = get_div_field(readl_relaxed(mux.reg)) as usize + SDMMC_MUL;
    (parent_rate * SDMMC_MUL + div - 1) / div
}

unsafe extern "C" fn clk_sdmmc_mux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mux = &*to_clk_sdmmc_mux(hw); let r = &mut *req; let output = r.best_parent_rate;
    r.rate = core::cmp::max(r.rate, r.min_rate); r.rate = core::cmp::min(r.rate, r.max_rate);
    if r.rate == 0 { return output as i32; }
    let mut div = div_frac_get(r.rate, output, 8, 1, mux.div_flags); if div < 0 { div = 0; }
    let denom = div as usize + SDMMC_MUL;
    r.rate = if mux.div_flags & TEGRA_DIVIDER_ROUND_UP != 0 { (output * SDMMC_MUL + denom - 1) / denom } else { output * SDMMC_MUL / denom }; 0
}

unsafe extern "C" fn clk_sdmmc_mux_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let mux = &*to_clk_sdmmc_mux(hw); let div = div_frac_get(rate, parent_rate, 8, 1, mux.div_flags); if div < 0 { return div; }
    let mut flags = 0usize; if !mux.lock.is_null() { spin_lock_irqsave(mux.lock, &mut flags); }
    let mut src = clk_sdmmc_mux_get_parent(hw); src = if div != 0 { MUX_NON_LJ_IDX[src as usize] } else { MUX_LJ_IDX[src as usize] };
    writel(((src as u32) << MUX_SHIFT) | div as u32, mux.reg); fence_udelay(2, mux.reg);
    if !mux.lock.is_null() { spin_unlock_irqrestore(mux.lock, flags); } 0
}

unsafe extern "C" fn clk_sdmmc_mux_is_enabled(hw: *mut clk_hw) -> i32 { let m=&*to_clk_sdmmc_mux(hw); let g=&m.gate.hw as *const _ as *mut _; __clk_hw_set_clk(g,hw); ((*m.gate_ops).is_enabled.unwrap())(g) }
unsafe extern "C" fn clk_sdmmc_mux_enable(hw: *mut clk_hw) -> i32 { let m=&*to_clk_sdmmc_mux(hw); let g=&m.gate.hw as *const _ as *mut _; __clk_hw_set_clk(g,hw); ((*m.gate_ops).enable.unwrap())(g) }
unsafe extern "C" fn clk_sdmmc_mux_disable(hw: *mut clk_hw) { let m=&*to_clk_sdmmc_mux(hw); ((*m.gate_ops).disable.unwrap())(&m.gate.hw as *const _ as *mut _); }
unsafe extern "C" fn clk_sdmmc_mux_disable_unused(hw: *mut clk_hw) { let m=&*to_clk_sdmmc_mux(hw); ((*m.gate_ops).disable_unused.unwrap())(&m.gate.hw as *const _ as *mut _); }
unsafe extern "C" fn clk_sdmmc_mux_restore_context(hw: *mut clk_hw) { let p=clk_hw_get_parent(hw); let pr=clk_hw_get_rate(p); let r=clk_hw_get_rate(hw); let id=clk_hw_get_parent_index(hw); if id < 0 { return; } clk_sdmmc_mux_set_parent(hw,id as u8); clk_sdmmc_mux_set_rate(hw,r,pr); }

static TEGRA_CLK_SDMMC_MUX_OPS: clk_ops = clk_ops { get_parent:Some(clk_sdmmc_mux_get_parent), set_parent:Some(clk_sdmmc_mux_set_parent), determine_rate:Some(clk_sdmmc_mux_determine_rate), recalc_rate:Some(clk_sdmmc_mux_recalc_rate), set_rate:Some(clk_sdmmc_mux_set_rate), is_enabled:Some(clk_sdmmc_mux_is_enabled), enable:Some(clk_sdmmc_mux_enable), disable:Some(clk_sdmmc_mux_disable), disable_unused:Some(clk_sdmmc_mux_disable_unused), restore_context:Some(clk_sdmmc_mux_restore_context) };

pub unsafe fn tegra_clk_register_sdmmc_mux_div(name: *const core::ffi::c_char, clk_base: *mut core::ffi::c_void, offset: u32, clk_num: u32, div_flags: u8, flags: usize, lock: *mut core::ffi::c_void) -> *mut clk {
    let bank = get_reg_bank(clk_num); if bank.is_null() { return (-EINVAL as isize) as *mut clk; }
    let mux = kzalloc_obj::<tegra_sdmmc_mux>(); if mux.is_null() { return (-ENOMEM as isize) as *mut clk; }
    let init = clk_init_data { ops:&TEGRA_CLK_SDMMC_MUX_OPS, name, flags, parent_names:MUX_SDMMC_PARENTS.as_ptr(), num_parents:5 };
    (*mux).hw.init=&init; (*mux).reg=clk_base.add(offset as usize); (*mux).lock=lock; (*mux).gate.clk_base=clk_base; (*mux).gate.regs=bank; (*mux).gate.clk_num=clk_num; (*mux).gate.flags=TEGRA_PERIPH_ON_APB; (*mux).div_flags=div_flags;
    let clk=clk_register(core::ptr::null_mut(), &mut (*mux).hw); if clk.is_null() { kfree(mux); return clk; } (*mux).gate.hw.clk=clk; clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
