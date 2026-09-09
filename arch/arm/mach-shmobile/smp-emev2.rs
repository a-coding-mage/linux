// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for Emma Mobile EV2
 *
 * Copyright (C) 2012  Renesas Solutions Corp.
 * Copyright (C) 2012  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const EMEV2_SCU_BASE: usize = 0x1e000000;
const EMEV2_SMU_BASE: usize = 0xe0110000;
const SMU_GENERAL_REG0: usize = 0x7c0;

extern "C" {
    static shmobile_boot_vector: c_void;

    fn arch_send_wakeup_ipi_mask(mask: *const c_void);
    fn cpumask_of(cpu: u32) -> *const c_void;
    fn cpu_logical_map(cpu: u32) -> u32;
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iowrite32(value: u32, addr: *mut c_void);
    fn iounmap(addr: *mut c_void);
    fn __pa(addr: *const c_void) -> u32;
    fn shmobile_smp_scu_prepare_cpus(base: usize, max_cpus: u32);
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> i32>,
}

unsafe extern "C" fn emev2_boot_secondary(_cpu: u32, _idle: *mut task_struct) -> i32 {
    arch_send_wakeup_ipi_mask(cpumask_of(cpu_logical_map(_cpu)));
    0
}

unsafe extern "C" fn emev2_smp_prepare_cpus(max_cpus: u32) {
    let mut smu: *mut c_void;

    /* Tell ROM loader about our vector (in headsmp.S) */
    smu = ioremap(EMEV2_SMU_BASE, PAGE_SIZE);
    if !smu.is_null() {
        iowrite32(__pa(&shmobile_boot_vector), smu.add(SMU_GENERAL_REG0));
        iounmap(smu);
    }

    /* setup EMEV2 specific SCU bits */
    shmobile_smp_scu_prepare_cpus(EMEV2_SCU_BASE, max_cpus);
}

pub const emev2_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(emev2_smp_prepare_cpus),
    smp_boot_secondary: Some(emev2_boot_secondary),
};

// Supplied by the surrounding kernel translation.
const PAGE_SIZE: usize = 4096;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
