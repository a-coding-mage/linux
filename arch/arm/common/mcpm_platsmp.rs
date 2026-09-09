// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-vexpress/mcpm_platsmp.c
 *
 * Created by:  Nicolas Pitre, November 2012
 * Copyright:   (C) 2012-2013  Linaro Limited
 *
 * Code to handle secondary CPU bringup and hotplug for the cluster power API.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    fn cpu_logical_map(cpu: u32) -> u32;
    fn mcpm_set_entry_vector(cpu: u32, cluster: u32, entry: Option<unsafe extern "C" fn()>);
    fn mcpm_cpu_power_up(cpu: u32, cluster: u32) -> i32;
    fn arch_send_wakeup_ipi_mask(mask: *const cpumask);
    fn dsb_sev();
    fn mcpm_cpu_powered_up();
    fn mcpm_wait_for_cpu_powerdown(cpu: u32, cluster: u32) -> i32;
    fn read_cpuid_mpidr() -> u32;
    fn mcpm_cpu_power_down() -> !;
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn smp_set_ops(ops: *const smp_operations);
    fn pr_debug(fmt: *const u8, ...);
    fn secondary_startup();
}

#[inline]
unsafe fn mpidr_affinity_level(mpidr: u32, level: u32) -> u32 {
    (mpidr >> (level * 8)) & 0xff
}

#[repr(C)]
struct smp_operations {
    smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    smp_secondary_init: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Option<unsafe extern "C" fn(u32) -> bool>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_can_disable: Option<unsafe extern "C" fn(u32) -> bool>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Option<unsafe extern "C" fn(u32)>,
}

unsafe extern "C" fn cpu_to_pcpu(cpu: u32, pcpu: *mut u32, pcluster: *mut u32) {
    let mpidr: u32;

    mpidr = cpu_logical_map(cpu);
    *pcpu = mpidr_affinity_level(mpidr, 0);
    *pcluster = mpidr_affinity_level(mpidr, 1);
}

unsafe extern "C" fn mcpm_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let (mut pcpu, mut pcluster, ret): (u32, u32, i32);

    cpu_to_pcpu(cpu, &mut pcpu, &mut pcluster);

    pr_debug(
        b"%s: logical CPU %d is physical CPU %d cluster %d\n\0".as_ptr(),
        b"mcpm_boot_secondary\0".as_ptr(),
        cpu,
        pcpu,
        pcluster,
    );

    mcpm_set_entry_vector(pcpu, pcluster, None);
    ret = mcpm_cpu_power_up(pcpu, pcluster);
    if ret != 0 {
        return ret;
    }
    mcpm_set_entry_vector(pcpu, pcluster, Some(secondary_startup));
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    dsb_sev();
    0
}

unsafe extern "C" fn mcpm_secondary_init(_cpu: u32) {
    mcpm_cpu_powered_up();
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn mcpm_cpu_kill(cpu: u32) -> bool {
    let (mut pcpu, mut pcluster): (u32, u32);

    cpu_to_pcpu(cpu, &mut pcpu, &mut pcluster);

    mcpm_wait_for_cpu_powerdown(pcpu, pcluster) == 0
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn mcpm_cpu_can_disable(_cpu: u32) -> bool {
    /* We assume all CPUs may be shut down. */
    true
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn mcpm_cpu_die(_cpu: u32) {
    let mpidr: u32;
    let (pcpu, pcluster): (u32, u32);
    mpidr = read_cpuid_mpidr();
    pcpu = mpidr_affinity_level(mpidr, 0);
    pcluster = mpidr_affinity_level(mpidr, 1);
    mcpm_set_entry_vector(pcpu, pcluster, None);
    mcpm_cpu_power_down();
}

// __initconst
static mcpm_smp_ops: smp_operations = smp_operations {
    smp_boot_secondary: Some(mcpm_boot_secondary),
    smp_secondary_init: Some(mcpm_secondary_init),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(mcpm_cpu_kill),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_can_disable: Some(mcpm_cpu_can_disable),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(mcpm_cpu_die),
};

pub unsafe extern "C" fn mcpm_smp_set_ops() {
    smp_set_ops(&mcpm_smp_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
