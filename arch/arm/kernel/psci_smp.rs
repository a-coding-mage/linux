// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn secondary_startup();
    static psci_ops: PsciOperations;
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PsciOperations {
    pub cpu_on: Option<unsafe extern "C" fn(u32, usize) -> i32>,
    pub cpu_off: Option<unsafe extern "C" fn(u32)>,
    pub affinity_info: Option<unsafe extern "C" fn(u32, u32) -> i32>,
}

extern "C" {
    fn cpu_logical_map(cpu: u32) -> u32;
    fn virt_to_idmap(address: *const ()) -> usize;
    fn psci_tos_resident_on(cpu: u32) -> bool;
    fn panic(message: *const u8) -> !;
    fn msleep(milliseconds: u32);
    fn pr_info(message: *const u8, ...);
    fn pr_warn(message: *const u8, ...);
}

// The following configuration-dependent constants and operations are provided
// by the kernel build environment.
extern "C" {
    static XIP_VIRT_ADDR: unsafe extern "C" fn(usize) -> usize;
    static CONFIG_XIP_PHYS_ADDR: usize;
}

const ENODEV: i32 = -19;
const EOPNOTSUPP: i32 = -95;
const EPERM: i32 = -1;
const PSCI_POWER_STATE_TYPE_POWER_DOWN: u32 = 1;
const PSCI_0_2_POWER_STATE_TYPE_SHIFT: u32 = 16;
const PSCI_0_2_AFFINITY_LEVEL_OFF: i32 = 2;

#[allow(dead_code)]
unsafe fn psci_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    if let Some(cpu_on) = psci_ops.cpu_on {
        #[cfg(CONFIG_XIP_KERNEL)]
        {
            let entry_point = (secondary_startup as usize)
                .wrapping_sub(XIP_VIRT_ADDR(CONFIG_XIP_PHYS_ADDR))
                .wrapping_add(CONFIG_XIP_PHYS_ADDR);
            return cpu_on(cpu_logical_map(cpu), entry_point);
        }
        #[cfg(not(CONFIG_XIP_KERNEL))]
        {
            return cpu_on(cpu_logical_map(cpu), virt_to_idmap(secondary_startup as *const ()));
        }
    }
    ENODEV
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn psci_cpu_disable(cpu: u32) -> i32 {
    /* Fail early if we don't have CPU_OFF support */
    if psci_ops.cpu_off.is_none() {
        return EOPNOTSUPP;
    }

    /* Trusted OS will deny CPU_OFF */
    if psci_tos_resident_on(cpu) {
        return EPERM;
    }

    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn psci_cpu_die(_cpu: u32) {
    let state = PSCI_POWER_STATE_TYPE_POWER_DOWN << PSCI_0_2_POWER_STATE_TYPE_SHIFT;

    if let Some(cpu_off) = psci_ops.cpu_off {
        cpu_off(state);
    }

    /* We should never return */
    panic(b"psci: cpu %d failed to shutdown\n\0".as_ptr());
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn psci_cpu_kill(cpu: u32) -> i32 {
    let mut err: i32 = 0;

    let affinity_info = match psci_ops.affinity_info {
        Some(function) => function,
        None => return 1,
    };

    /*
     * cpu_kill could race with cpu_die and we can
     * potentially end up declaring this cpu undead
     * while it is dying. So, try again a few times.
     */
    for _i in 0..10 {
        err = affinity_info(cpu_logical_map(cpu), 0);
        if err == PSCI_0_2_AFFINITY_LEVEL_OFF {
            pr_info(b"CPU%d killed.\n\0".as_ptr(), cpu);
            return 1;
        }

        msleep(10);
        pr_info(b"Retrying again to check for CPU kill\n\0".as_ptr());
    }

    pr_warn(
        b"CPU%d may not have shut down cleanly (AFFINITY_INFO reports %d)\n\0".as_ptr(),
        cpu,
        err,
    );
    /* Make platform_cpu_kill() fail. */
    0
}

pub unsafe fn psci_smp_available() -> bool {
    /* is cpu_on available at least? */
    psci_ops.cpu_on.is_some()
}

#[repr(C)]
pub struct SmpOperations {
    pub smp_boot_secondary: unsafe fn(u32, *mut TaskStruct) -> i32,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_disable: unsafe fn(u32) -> i32,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_die: unsafe fn(u32),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_kill: unsafe fn(u32) -> i32,
}

pub static psci_smp_ops: SmpOperations = SmpOperations {
    .smp_boot_secondary: psci_boot_secondary,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    .cpu_disable: psci_cpu_disable,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    .cpu_die: psci_cpu_die,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    .cpu_kill: psci_cpu_kill,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
