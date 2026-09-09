// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for R-Mobile / SH-Mobile - r8a7779 portion
 *
 * Copyright (C) 2011  Renesas Solutions Corp.
 * Copyright (C) 2011  Magnus Damm
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const HPBREG_BASE: usize = 0xfe700000;
const AVECR: usize = 0x0040; /* ARM Reset Vector Address Register */
const R8A7779_SCU_BASE: usize = 0xf0000000;
const SZ_4K: usize = 0x1000;
const EIO: i32 = 5;

#[repr(C)]
pub struct task_struct;

extern "C" {
    fn cpu_logical_map(cpu: u32) -> u32;
    fn rcar_sysc_power_up_cpu(cpu: u32) -> i32;
    fn rcar_sysc_power_down_cpu(cpu: u32) -> i32;
    fn request_mem_region(start: usize, len: usize, name: *const u8) -> *mut c_void;
    fn ioremap(offset: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn __pa(addr: unsafe extern "C" fn()) -> usize;
    fn shmobile_boot_vector();
    fn writel(value: u32, addr: *mut u8);
    fn shmobile_smp_scu_prepare_cpus(base: usize, max_cpus: u32);
    fn shmobile_smp_scu_cpu_kill(cpu: u32) -> i32;
    fn shmobile_smp_scu_cpu_die(cpu: u32);
}

unsafe fn r8a7779_boot_secondary(mut cpu: u32, _idle: *mut task_struct) -> i32 {
    let mut ret = -EIO;

    cpu = cpu_logical_map(cpu);
    if cpu != 0 {
        ret = rcar_sysc_power_up_cpu(cpu);
    }

    ret
}

unsafe fn r8a7779_smp_prepare_cpus(max_cpus: u32) {
    let base: *mut c_void;

    if request_mem_region(0, SZ_4K, b"Boot Area\0".as_ptr()).is_null() {
        // pr_err("Failed to request boot area\n");
        return;
    }

    base = ioremap(HPBREG_BASE, 0x1000);

    /* Map the reset vector (in headsmp-scu.S, headsmp.S) */
    writel(
        __pa(shmobile_boot_vector) as u32,
        (base as *mut u8).add(AVECR),
    );

    /* setup r8a7779 specific SCU bits */
    shmobile_smp_scu_prepare_cpus(R8A7779_SCU_BASE, max_cpus);

    iounmap(base);
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn r8a7779_platform_cpu_kill(mut cpu: u32) -> i32 {
    let mut ret = -EIO;

    cpu = cpu_logical_map(cpu);
    if cpu != 0 {
        ret = rcar_sysc_power_down_cpu(cpu);
    }

    if ret != 0 { ret } else { 1 }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn r8a7779_cpu_kill(cpu: u32) -> i32 {
    if shmobile_smp_scu_cpu_kill(cpu) != 0 {
        return r8a7779_platform_cpu_kill(cpu);
    }

    0
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe fn(u32)>,
    pub smp_boot_secondary: Option<unsafe fn(u32, *mut task_struct) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe fn(u32) -> i32>,
}

// __initconst
#[no_mangle]
pub static r8a7779_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(r8a7779_smp_prepare_cpus),
    smp_boot_secondary: Some(r8a7779_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(shmobile_smp_scu_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(r8a7779_cpu_kill),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
