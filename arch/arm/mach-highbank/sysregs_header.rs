/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2011 Calxeda, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

unsafe extern "C" {
    static mut sregs_base: *mut c_void;
    static mut scu_base_addr: *mut c_void;

    fn cpu_logical_map(cpu: i32) -> u32;
    fn smp_processor_id() -> i32;
    fn MPIDR_AFFINITY_LEVEL(mpidr: u32, level: i32) -> i32;
    fn scu_power_mode(base: *mut c_void, mode: u32);
    fn writel_relaxed(value: u32, address: *mut c_void);
    fn writel(value: u32, address: *mut c_void);
}

pub const HB_SREG_A9_PWR_REQ: usize = 0xf00;
pub const HB_SREG_A9_BOOT_STAT: usize = 0xf04;
pub const HB_SREG_A9_BOOT_DATA: usize = 0xf08;

pub const HB_PWR_SUSPEND: u32 = 0;
pub const HB_PWR_SOFT_RESET: u32 = 1;
pub const HB_PWR_HARD_RESET: u32 = 2;
pub const HB_PWR_SHUTDOWN: u32 = 3;

#[inline]
pub const fn SREG_CPU_PWR_CTRL(c: i32) -> usize {
    0x200 + ((c as usize) * 4)
}

#[inline]
pub unsafe fn highbank_set_core_pwr() {
    let cpu = MPIDR_AFFINITY_LEVEL(cpu_logical_map(smp_processor_id()), 0);
    if !scu_base_addr.is_null() {
        scu_power_mode(scu_base_addr, 1 /* SCU_PM_POWEROFF */);
    } else {
        writel_relaxed(
            1,
            sregs_base.add(SREG_CPU_PWR_CTRL(cpu)),
        );
    }
}

#[inline]
pub unsafe fn highbank_clear_core_pwr() {
    let cpu = MPIDR_AFFINITY_LEVEL(cpu_logical_map(smp_processor_id()), 0);
    if !scu_base_addr.is_null() {
        scu_power_mode(scu_base_addr, 0 /* SCU_PM_NORMAL */);
    } else {
        writel_relaxed(
            0,
            sregs_base.add(SREG_CPU_PWR_CTRL(cpu)),
        );
    }
}

#[inline]
pub unsafe fn highbank_set_pwr_suspend() {
    writel(HB_PWR_SUSPEND, sregs_base.add(HB_SREG_A9_PWR_REQ));
    highbank_set_core_pwr();
}

#[inline]
pub unsafe fn highbank_set_pwr_shutdown() {
    writel(HB_PWR_SHUTDOWN, sregs_base.add(HB_SREG_A9_PWR_REQ));
    highbank_set_core_pwr();
}

#[inline]
pub unsafe fn highbank_set_pwr_soft_reset() {
    writel(HB_PWR_SOFT_RESET, sregs_base.add(HB_SREG_A9_PWR_REQ));
    highbank_set_core_pwr();
}

#[inline]
pub unsafe fn highbank_set_pwr_hard_reset() {
    writel(HB_PWR_HARD_RESET, sregs_base.add(HB_SREG_A9_PWR_REQ));
    highbank_set_core_pwr();
}

#[inline]
pub unsafe fn highbank_clear_pwr_request() {
    writel(u32::MAX, sregs_base.add(HB_SREG_A9_PWR_REQ));
    highbank_clear_core_pwr();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
