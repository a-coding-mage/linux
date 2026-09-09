// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// Dependencies supplied by the corresponding kernel/Rust bindings.

use core::ffi::c_ulong;

const PLL_LOCK_TIMEOUT_US: u32 = 200 * 1000;

extern "C" {
    fn readl(addr: *const u8) -> u32;
    fn readl_relaxed_poll_timeout(
        addr: *const u8,
        val: *mut u32,
        cond: u32,
        delay_us: u32,
        timeout_us: u32,
    ) -> i32;
    fn writel(value: u32, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: c_ulong);
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
pub struct cv1800_clk_regbit {
    pub reg: usize,
    pub shift: u32,
}

#[repr(C)]
pub struct cv1800_clk_common {
    pub base: *mut u8,
    pub lock: *mut core::ffi::c_void,
}

pub unsafe fn cv1800_clk_setbit(
    common: *mut cv1800_clk_common,
    field: *mut cv1800_clk_regbit,
) -> i32 {
    let mask: u32 = 1u32 << (*field).shift;
    let value: u32;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave((*common).lock, &mut flags);

    value = readl((*common).base.add((*field).reg));
    writel(value | mask, (*common).base.add((*field).reg));

    spin_unlock_irqrestore((*common).lock, flags);

    0
}

pub unsafe fn cv1800_clk_clearbit(
    common: *mut cv1800_clk_common,
    field: *mut cv1800_clk_regbit,
) -> i32 {
    let mask: u32 = 1u32 << (*field).shift;
    let value: u32;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave((*common).lock, &mut flags);

    value = readl((*common).base.add((*field).reg));
    writel(value & !mask, (*common).base.add((*field).reg));

    spin_unlock_irqrestore((*common).lock, flags);

    0
}

pub unsafe fn cv1800_clk_checkbit(
    common: *mut cv1800_clk_common,
    field: *mut cv1800_clk_regbit,
) -> u32 {
    readl((*common).base.add((*field).reg)) & (1u32 << (*field).shift)
}

pub unsafe fn cv1800_clk_wait_for_lock(
    common: *mut cv1800_clk_common,
    reg: u32,
    lock: u32,
) {
    let addr = (*common).base.add(reg as usize);
    let mut regval: u32 = 0;

    if lock == 0 {
        return;
    }

    let result = readl_relaxed_poll_timeout(
        addr,
        &mut regval,
        regval & lock,
        100,
        PLL_LOCK_TIMEOUT_US,
    );
    let _ = warn_on(result != 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
