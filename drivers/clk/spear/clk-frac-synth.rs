// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * Fractional Synthesizer clock implementation
 */

// #define pr_fmt(fmt) "clk-frac-synth: " fmt
// Linux kernel dependencies supplied by other translation units.

const DIV_FACTOR_MASK: u32 = 0x1FFFF;

/*
 * DOC: Fractional Synthesizer clock
 *
 * Fout from synthesizer can be given from below equation:
 *
 * Fout= Fin/2*div (division factor)
 * div is 17 bits:-
 *	0-13 (fractional part)
 *	14-16 (integer part)
 * div is (16-14 bits).(13-0 bits) (in binary)
 *
 * Fout = Fin/(2 * div)
 * Fout = ((Fin / 10000)/(2 * div)) * 10000
 * Fout = (2^14 * (Fin / 10000)/(2^14 * (2 * div))) * 10000
 * Fout = (((Fin / 10000) << 14)/(2 * (div << 14))) * 10000
 *
 * div << 14 simply 17 bit value written at register.
 * Max error due to scaling down by 10000 is 10 KHz
 */

#[repr(C)]
pub struct ClkHw {
    pub init: *mut ClkInitData,
}

#[repr(C)]
pub struct ClkInitData {
    pub name: *const ::kernel::ffi::c_char,
    pub ops: *const ClkOps,
    pub flags: ::kernel::ffi::c_ulong,
    pub parent_names: *const *const ::kernel::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct FracRateTbl {
    pub div: u32,
}

#[repr(C)]
pub struct Spinlock;

#[repr(C)]
pub struct ClkFrac {
    pub hw: ClkHw,
    pub reg: *mut u8,
    pub rtbl: *mut FracRateTbl,
    pub rtbl_cnt: u8,
    pub lock: *mut Spinlock,
}

#[repr(C)]
pub struct ClkRateRequest {
    pub rate: ::kernel::ffi::c_ulong,
    pub best_parent_rate: ::kernel::ffi::c_ulong,
}

pub type ClkRoundRateFn = unsafe extern "C" fn(
    hw: *mut ClkHw,
    prate: ::kernel::ffi::c_ulong,
    index: ::kernel::ffi::c_int,
) -> ::kernel::ffi::c_ulong;

#[repr(C)]
pub struct ClkOps {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> ::kernel::ffi::c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw, ::kernel::ffi::c_ulong, ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int>,
}

#[repr(C)]
pub struct Clk;

unsafe extern "C" {
    fn clk_round_rate_index(hw: *mut ClkHw, rate: ::kernel::ffi::c_ulong,
        best_parent_rate: ::kernel::ffi::c_ulong, calc_rate: ClkRoundRateFn,
        count: u8, index: *mut ::kernel::ffi::c_int) -> ::kernel::ffi::c_ulong;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: ::kernel::ffi::c_ulong, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut ::kernel::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: ::kernel::ffi::c_ulong);
    fn clk_register(parent: *mut Clk, hw: *mut ClkHw) -> *mut Clk;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut ClkFrac);
}

unsafe extern "C" fn frac_calc_rate(hw: *mut ClkHw, mut prate: ::kernel::ffi::c_ulong,
    index: ::kernel::ffi::c_int) -> ::kernel::ffi::c_ulong {
    let frac = &*(hw as *mut ClkFrac);
    let rtbl = frac.rtbl;
    prate /= 10000;
    prate <<= 14;
    prate /= 2 * (*rtbl.add(index as usize)).div as ::kernel::ffi::c_ulong;
    prate *= 10000;
    prate
}

unsafe extern "C" fn clk_frac_determine_rate(hw: *mut ClkHw,
    req: *mut ClkRateRequest) -> ::kernel::ffi::c_int {
    let frac = &*(hw as *mut ClkFrac);
    let mut unused = 0;
    (*req).rate = clk_round_rate_index(hw, (*req).rate, (*req).best_parent_rate,
        frac_calc_rate, frac.rtbl_cnt, &mut unused);
    0
}

unsafe extern "C" fn clk_frac_recalc_rate(hw: *mut ClkHw,
    mut parent_rate: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong {
    let frac = &*(hw as *mut ClkFrac);
    let mut flags = 0;
    let val;
    if !frac.lock.is_null() { spin_lock_irqsave(frac.lock, &mut flags); }
    val = readl_relaxed(frac.reg);
    if !frac.lock.is_null() { spin_unlock_irqrestore(frac.lock, flags); }
    let div = val & DIV_FACTOR_MASK;
    if div == 0 { return 0; }
    parent_rate /= 10000;
    parent_rate = (parent_rate << 14) / (2 * div as ::kernel::ffi::c_ulong);
    parent_rate * 10000
}

/* Configures new clock rate of frac */
unsafe extern "C" fn clk_frac_set_rate(hw: *mut ClkHw,
    drate: ::kernel::ffi::c_ulong, prate: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int {
    let frac = &*(hw as *mut ClkFrac);
    let mut flags = 0;
    let mut i = 0;
    clk_round_rate_index(hw, drate, prate, frac_calc_rate, frac.rtbl_cnt, &mut i);
    if !frac.lock.is_null() { spin_lock_irqsave(frac.lock, &mut flags); }
    let mut val = (readl_relaxed(frac.reg) as ::kernel::ffi::c_ulong) & !(DIV_FACTOR_MASK as ::kernel::ffi::c_ulong);
    val |= ((*frac.rtbl.add(i as usize)).div & DIV_FACTOR_MASK) as ::kernel::ffi::c_ulong;
    writel_relaxed(val, frac.reg);
    if !frac.lock.is_null() { spin_unlock_irqrestore(frac.lock, flags); }
    0
}

static CLK_FRAC_OPS: ClkOps = ClkOps {
    recalc_rate: Some(clk_frac_recalc_rate),
    determine_rate: Some(clk_frac_determine_rate),
    set_rate: Some(clk_frac_set_rate),
};

pub unsafe extern "C" fn clk_register_frac(name: *const ::kernel::ffi::c_char,
    parent_name: *const ::kernel::ffi::c_char, flags: ::kernel::ffi::c_ulong,
    reg: *mut u8, rtbl: *mut FracRateTbl, rtbl_cnt: u8,
    lock: *mut Spinlock) -> *mut Clk {
    if name.is_null() || parent_name.is_null() || reg.is_null() || rtbl.is_null() || rtbl_cnt == 0 {
        return (-22isize) as *mut Clk;
    }
    let frac = kzalloc_obj::<ClkFrac>();
    if frac.is_null() { return (-12isize) as *mut Clk; }
    (*frac).reg = reg;
    (*frac).rtbl = rtbl;
    (*frac).rtbl_cnt = rtbl_cnt;
    (*frac).lock = lock;
    let mut init = ClkInitData { name, ops: &CLK_FRAC_OPS, flags,
        parent_names: &parent_name, num_parents: 1 };
    (*frac).hw.init = &mut init;
    let clk = clk_register(core::ptr::null_mut(), &mut (*frac).hw);
    if !clk.is_null() { return clk; }
    kfree(frac);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
