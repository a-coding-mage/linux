// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Daniel Palmer <daniel@thingy.jp>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const REG_LPF_LOW_L: u32 = 0x140;
const REG_LPF_LOW_H: u32 = 0x144;
const REG_LPF_HIGH_BOTTOM: u32 = 0x148;
const REG_LPF_HIGH_TOP: u32 = 0x14c;
const REG_LPF_TOGGLE: u32 = 0x150;
const REG_LPF_MYSTERYTWO: u32 = 0x154;
const REG_LPF_UPDATE_COUNT: u32 = 0x15c;
const REG_LPF_MYSTERYONE: u32 = 0x160;
const REG_LPF_TRANSITIONCTRL: u32 = 0x164;
const REG_LPF_LOCK: u32 = 0x174;
const REG_CURRENT: u32 = 0x180;

const LPF_LOCK_TIMEOUT: i64 = 100000000;
const MULTIPLIER_1: u64 = 16;
const MULTIPLIER_2: u64 = 524288;
const MULTIPLIER: u64 = MULTIPLIER_1 * MULTIPLIER_2;

#[repr(C)]
struct Msc313Cpupll {
    base: *mut core::ffi::c_void,
    clk_hw: ClkHw,
}

#[repr(C)]
struct ClkHw {
    init: *const ClkInitData,
}

#[repr(C)]
struct ClkInitData {
    name: *const core::ffi::c_char,
    ops: *const ClkOps,
    parent_data: *const ClkParentData,
    num_parents: u32,
}

#[repr(C)]
struct ClkParentData {
    index: u32,
}

#[repr(C)]
struct ClkRateRequest {
    rate: usize,
    best_parent_rate: usize,
}

#[repr(C)]
struct ClkOps {
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize, usize) -> i32>,
}

extern "C" {
    fn ioread16(addr: *mut core::ffi::c_void) -> u16;
    fn iowrite16(value: u16, addr: *mut core::ffi::c_void);
    fn ktime_get() -> i64;
    fn ktime_add_ns(time: i64, nsec: i64) -> i64;
    fn ktime_after(lhs: i64, rhs: i64) -> bool;
    fn cpu_relax();
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

unsafe fn msc313_cpupll_reg_read32(cpupll: *mut Msc313Cpupll, reg: u32) -> u32 {
    let base = (*cpupll).base;
    let mut value = (ioread16(base.add((reg + 4) as usize)) as u32) << 16;
    value |= ioread16(base.add(reg as usize)) as u32;
    value
}

unsafe fn msc313_cpupll_reg_write32(cpupll: *mut Msc313Cpupll, reg: u32, value: u32) {
    let base = (*cpupll).base;
    let l = (value & 0xffff) as u16;
    let h = ((value >> 16) & 0xffff) as u16;
    iowrite16(l, base.add(reg as usize));
    iowrite16(h, base.add((reg + 4) as usize));
}

unsafe fn msc313_cpupll_setfreq(cpupll: *mut Msc313Cpupll, regvalue: u32) {
    msc313_cpupll_reg_write32(cpupll, REG_LPF_HIGH_BOTTOM, regvalue);
    let base = (*cpupll).base;
    iowrite16(0x1, base.add(REG_LPF_MYSTERYONE as usize));
    iowrite16(0x6, base.add(REG_LPF_MYSTERYTWO as usize));
    iowrite16(0x8, base.add(REG_LPF_UPDATE_COUNT as usize));
    iowrite16(1u16 << 12, base.add(REG_LPF_TRANSITIONCTRL as usize));
    iowrite16(0, base.add(REG_LPF_TOGGLE as usize));
    iowrite16(1, base.add(REG_LPF_TOGGLE as usize));
    let timeout = ktime_add_ns(ktime_get(), LPF_LOCK_TIMEOUT);
    while ioread16(base.add(REG_LPF_LOCK as usize)) == 0 {
        if ktime_after(ktime_get(), timeout) {
            pr_err(b"timeout waiting for LPF_LOCK\n\0".as_ptr() as *const _);
            return;
        }
        cpu_relax();
    }
    iowrite16(0, base.add(REG_LPF_TOGGLE as usize));
    msc313_cpupll_reg_write32(cpupll, REG_LPF_LOW_L, regvalue);
}

unsafe fn msc313_cpupll_frequencyforreg(reg: u32, parent_rate: usize) -> usize {
    let prescaled = (parent_rate as u64).wrapping_mul(MULTIPLIER);
    if prescaled == 0 || reg == 0 { return 0; }
    (prescaled / reg as u64) as usize
}

unsafe fn msc313_cpupll_regforfrequecy(rate: usize, parent_rate: usize) -> u32 {
    let prescaled = (parent_rate as u64).wrapping_mul(MULTIPLIER);
    if prescaled == 0 || rate == 0 { return 0; }
    ((prescaled + rate as u64 - 1) / rate as u64) as u32
}

unsafe extern "C" fn msc313_cpupll_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let cpupll = (hw as *mut u8).sub(core::mem::offset_of!(Msc313Cpupll, clk_hw)) as *mut Msc313Cpupll;
    msc313_cpupll_frequencyforreg(msc313_cpupll_reg_read32(cpupll, REG_LPF_LOW_L), parent_rate)
}

unsafe extern "C" fn msc313_cpupll_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let _ = hw;
    let mut reg = msc313_cpupll_regforfrequecy((*req).rate, (*req).best_parent_rate);
    let mut rounded = msc313_cpupll_frequencyforreg(reg, (*req).best_parent_rate);
    while rounded >= (*req).rate && reg > 0 {
        reg -= 1;
        rounded = msc313_cpupll_frequencyforreg(reg, (*req).best_parent_rate);
    }
    (*req).rate = rounded;
    0
}

unsafe extern "C" fn msc313_cpupll_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32 {
    let cpupll = (hw as *mut u8).sub(core::mem::offset_of!(Msc313Cpupll, clk_hw)) as *mut Msc313Cpupll;
    msc313_cpupll_setfreq(cpupll, msc313_cpupll_regforfrequecy(rate, parent_rate));
    0
}

static MSC313_CPUPLL_OPS: ClkOps = ClkOps {
    recalc_rate: Some(msc313_cpupll_recalc_rate),
    determine_rate: Some(msc313_cpupll_determine_rate),
    set_rate: Some(msc313_cpupll_set_rate),
};

// Device-tree match table and platform-driver registration are provided by the kernel translation layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
