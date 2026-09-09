// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2013 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// C includes and pr_fmt("psci: " fmt) are supplied by the surrounding kernel.

use core::ffi::c_void;

type PhysAddr = usize;

#[repr(C)]
pub struct PsciOperations {
    pub cpu_on: Option<unsafe extern "C" fn(usize, PhysAddr) -> i32>,
    pub cpu_off: Option<unsafe extern "C" fn(u32)>,
    pub affinity_info: Option<unsafe extern "C" fn(usize, u32) -> i32>,
}

#[repr(C)]
pub struct CpuOperations {
    pub name: *const u8,
    pub cpu_init: Option<unsafe extern "C" fn(u32) -> i32>,
    pub cpu_prepare: Option<unsafe extern "C" fn(u32) -> i32>,
    pub cpu_boot: Option<unsafe extern "C" fn(u32) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_can_disable: Option<unsafe extern "C" fn(u32) -> bool>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_disable: Option<unsafe extern "C" fn(u32) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

extern "C" {
    static mut psci_ops: PsciOperations;
    static secondary_entry: c_void;
    fn cpu_logical_map(cpu: u32) -> usize;
    fn __pa_symbol(symbol: *const c_void) -> PhysAddr;
    fn psci_tos_resident_on(cpu: u32) -> bool;
    fn usleep_range(min: u32, max: u32);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    static mut jiffies: u64;
    fn msecs_to_jiffies(ms: u32) -> u64;
    fn jiffies_to_msecs(j: u64) -> u32;
}

const ENODEV: i32 = 19;
const EOPNOTSUPP: i32 = 95;
const EPERM: i32 = 1;
const ETIMEDOUT: i32 = 110;
const PSCI_POWER_STATE_TYPE_POWER_DOWN: u32 = 1;
const PSCI_0_2_POWER_STATE_TYPE_SHIFT: u32 = 16;
const PSCI_0_2_AFFINITY_LEVEL_OFF: i32 = 0;

unsafe extern "C" fn cpu_psci_cpu_init(_cpu: u32) -> i32 {
    0
}

unsafe extern "C" fn cpu_psci_cpu_prepare(cpu: u32) -> i32 {
    if (*core::ptr::addr_of!(psci_ops)).cpu_on.is_none() {
        pr_err(b"no cpu_on method, not booting CPU%d\n\0".as_ptr(), cpu);
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn cpu_psci_cpu_boot(cpu: u32) -> i32 {
    let pa_secondary_entry = __pa_symbol(core::ptr::addr_of!(secondary_entry));
    let err = (*core::ptr::addr_of!(psci_ops)).cpu_on.unwrap()(cpu_logical_map(cpu), pa_secondary_entry);
    if err != 0 && err != -EPERM {
        pr_err(b"failed to boot CPU%d (%d)\n\0".as_ptr(), cpu, err);
    }

    err
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn cpu_psci_cpu_can_disable(cpu: u32) -> bool {
    !psci_tos_resident_on(cpu)
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn cpu_psci_cpu_disable(cpu: u32) -> i32 {
    if (*core::ptr::addr_of!(psci_ops)).cpu_off.is_none() {
        return -EOPNOTSUPP;
    }
    if psci_tos_resident_on(cpu) {
        return -EPERM;
    }
    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn cpu_psci_cpu_die(_cpu: u32) {
    let state = PSCI_POWER_STATE_TYPE_POWER_DOWN << PSCI_0_2_POWER_STATE_TYPE_SHIFT;
    (*core::ptr::addr_of!(psci_ops)).cpu_off.unwrap()(state);
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn cpu_psci_cpu_kill(cpu: u32) -> i32 {
    let mut err: i32 = 0;
    if (*core::ptr::addr_of!(psci_ops)).affinity_info.is_none() {
        return 0;
    }

    let start = jiffies;
    let end = start.wrapping_add(msecs_to_jiffies(100));
    loop {
        err = (*core::ptr::addr_of!(psci_ops)).affinity_info.unwrap()(cpu_logical_map(cpu), 0);
        if err == PSCI_0_2_AFFINITY_LEVEL_OFF {
            pr_info(b"CPU%d killed (polled %d ms)\n\0".as_ptr(), cpu, jiffies_to_msecs(jiffies.wrapping_sub(start)));
            return 0;
        }
        usleep_range(100, 1000);
        if !jiffies.wrapping_sub(end) as i64 < 0 {
            break;
        }
    }

    pr_warn(b"CPU%d may not have shut down cleanly (AFFINITY_INFO reports %d)\n\0".as_ptr(), cpu, err);
    -ETIMEDOUT
}

#[no_mangle]
pub static cpu_psci_ops: CpuOperations = CpuOperations {
    name: b"psci\0".as_ptr(),
    cpu_init: Some(cpu_psci_cpu_init),
    cpu_prepare: Some(cpu_psci_cpu_prepare),
    cpu_boot: Some(cpu_psci_cpu_boot),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_can_disable: Some(cpu_psci_cpu_can_disable),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_disable: Some(cpu_psci_cpu_disable),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: Some(cpu_psci_cpu_die),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_kill: Some(cpu_psci_cpu_kill),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
