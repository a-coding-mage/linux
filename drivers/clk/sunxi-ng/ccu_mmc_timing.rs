// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Chen-Yu Tsai. All rights reserved.
 */

use core::ffi::c_int;
use core::ptr;

// Dependencies supplied by the surrounding kernel/CCU implementation.
#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ClkHw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CcuCommon {
    pub base: *mut u8,
    pub reg: usize,
    pub lock: *mut Spinlock,
    pub features: u32,
}

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

pub const CCU_FEATURE_MMC_TIMING_SWITCH: u32 = 1 << 0;
pub const CCU_MMC_NEW_TIMING_MODE: u32 = 1 << 30;
pub const ENOTSUPP: c_int = 524;

extern "C" {
    fn __clk_get_hw(clk: *mut Clk) -> *mut ClkHw;
    fn hw_to_ccu_common(hw: *mut ClkHw) -> *mut CcuCommon;
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
}

/// sunxi_ccu_set_mmc_timing_mode - Configure the MMC clock timing mode
/// @clk: clock to be configured
/// @new_mode: true for new timing mode introduced in A83T and later
///
/// Return: %0 on success, %-ENOTSUPP if the clock does not support
/// switching modes.
#[no_mangle]
pub unsafe extern "C" fn sunxi_ccu_set_mmc_timing_mode(
    clk: *mut Clk,
    new_mode: bool,
) -> c_int {
    let hw: *mut ClkHw = __clk_get_hw(clk);
    let cm: *mut CcuCommon = hw_to_ccu_common(hw);
    let mut flags: usize = 0;
    let mut val: u32;

    if ((*cm).features & CCU_FEATURE_MMC_TIMING_SWITCH) == 0 {
        return -ENOTSUPP;
    }

    spin_lock_irqsave((*cm).lock, &mut flags);

    let addr = (*cm).base.add((*cm).reg);
    val = readl(addr as *const u8);
    if new_mode {
        val |= CCU_MMC_NEW_TIMING_MODE;
    } else {
        val &= !CCU_MMC_NEW_TIMING_MODE;
    }
    writel(val, addr);

    spin_unlock_irqrestore((*cm).lock, flags);

    0
}

/// sunxi_ccu_get_mmc_timing_mode: Get the current MMC clock timing mode
/// @clk: clock to query
///
/// Return: %0 if the clock is in old timing mode, > %0 if it is in
/// new timing mode, and %-ENOTSUPP if the clock does not support
/// this function.
#[no_mangle]
pub unsafe extern "C" fn sunxi_ccu_get_mmc_timing_mode(clk: *mut Clk) -> c_int {
    let hw: *mut ClkHw = __clk_get_hw(clk);
    let cm: *mut CcuCommon = hw_to_ccu_common(hw);

    if ((*cm).features & CCU_FEATURE_MMC_TIMING_SWITCH) == 0 {
        return -ENOTSUPP;
    }

    let addr = (*cm).base.add((*cm).reg);
    ((readl(addr as *const u8) & CCU_MMC_NEW_TIMING_MODE) != 0) as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
