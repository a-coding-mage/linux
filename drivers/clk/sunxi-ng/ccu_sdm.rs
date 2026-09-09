// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Chen-Yu Tsai <wens@csie.org>
 */

use core::ffi::c_void;

// Declarations supplied by the Linux clock-provider and CCU headers.
pub type U32 = u32;
pub type CUInt = u32;
pub type CULong = usize;

pub const CCU_FEATURE_SIGMA_DELTA_MOD: u32 = 1 << 0;
pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct CcuSdmTable {
    pub rate: CULong,
    pub pattern: U32,
    pub m: U32,
    pub n: U32,
}

#[repr(C)]
pub struct CcuCommon {
    pub hw: CClkHw,
    pub base: *mut u8,
    pub reg: usize,
    pub features: u32,
    pub lock: *mut CSpinlock,
}

#[repr(C)]
pub struct CcuSdmInternal {
    pub enable: U32,
    pub tuning_reg: usize,
    pub tuning_enable: U32,
    pub table_size: CUInt,
    pub table: *const CcuSdmTable,
}

#[repr(C)]
pub struct CClkHw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CSpinlock {
    _private: [u8; 0],
}

extern "C" {
    fn readl(addr: *mut u8) -> U32;
    fn writel(value: U32, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut CSpinlock, flags: *mut CULong);
    fn spin_unlock_irqrestore(lock: *mut CSpinlock, flags: CULong);
    fn clk_hw_get_name(hw: *const CClkHw) -> *const i8;
    fn pr_debug(format: *const i8, ...);
}

#[inline]
unsafe fn table_at(sdm: *const CcuSdmInternal, index: CUInt) -> *const CcuSdmTable {
    (*sdm).table.add(index as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_is_enabled(
    common: *mut CcuCommon,
    sdm: *mut CcuSdmInternal,
) -> bool {
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 {
        return false;
    }
    if (*sdm).enable != 0
        && readl((*common).base.add((*common).reg)) & (*sdm).enable == 0
    {
        return false;
    }
    (readl((*common).base.add((*sdm).tuning_reg)) & (*sdm).tuning_enable) != 0
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_enable(
    common: *mut CcuCommon,
    sdm: *mut CcuSdmInternal,
    rate: CULong,
) {
    let mut flags: CULong = 0;
    let mut reg: U32;
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 { return; }

    for i in 0..(*sdm).table_size {
        let entry = &*table_at(sdm, i);
        if entry.rate == rate {
            writel(entry.pattern, (*common).base.add((*sdm).tuning_reg));
        }
    }

    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*sdm).tuning_reg));
    writel(reg | (*sdm).tuning_enable, (*common).base.add((*sdm).tuning_reg));
    spin_unlock_irqrestore((*common).lock, flags);

    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*common).reg));
    writel(reg | (*sdm).enable, (*common).base.add((*common).reg));
    spin_unlock_irqrestore((*common).lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_disable(
    common: *mut CcuCommon,
    sdm: *mut CcuSdmInternal,
) {
    let mut flags: CULong = 0;
    let reg: U32;
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 { return; }
    spin_lock_irqsave((*common).lock, &mut flags);
    reg = readl((*common).base.add((*common).reg));
    writel(reg & !(*sdm).enable, (*common).base.add((*common).reg));
    spin_unlock_irqrestore((*common).lock, flags);
    spin_lock_irqsave((*common).lock, &mut flags);
    let reg = readl((*common).base.add((*sdm).tuning_reg));
    writel(reg & !(*sdm).tuning_enable, (*common).base.add((*sdm).tuning_reg));
    spin_unlock_irqrestore((*common).lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_has_rate(
    common: *mut CcuCommon, sdm: *mut CcuSdmInternal, rate: CULong,
) -> bool {
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 { return false; }
    for i in 0..(*sdm).table_size {
        if (*table_at(sdm, i)).rate == rate { return true; }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_read_rate(
    common: *mut CcuCommon, sdm: *mut CcuSdmInternal, m: U32, n: U32,
) -> CULong {
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 { return 0; }
    let reg = readl((*common).base.add((*sdm).tuning_reg));
    for i in 0..(*sdm).table_size {
        let entry = &*table_at(sdm, i);
        if entry.pattern == reg && entry.m == m && entry.n == n { return entry.rate; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ccu_sdm_helper_get_factors(
    common: *mut CcuCommon, sdm: *mut CcuSdmInternal, rate: CULong,
    m: *mut CULong, n: *mut CULong,
) -> i32 {
    if (*common).features & CCU_FEATURE_SIGMA_DELTA_MOD == 0 { return -EINVAL; }
    for i in 0..(*sdm).table_size {
        let entry = &*table_at(sdm, i);
        if entry.rate == rate {
            *m = entry.m as CULong;
            *n = entry.n as CULong;
            return 0;
        }
    }
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
