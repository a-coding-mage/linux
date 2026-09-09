// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SMP operations for Alpine platform.
 *
 * Copyright (C) 2015 Annapurna Labs Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    static secondary_startup: unsafe extern "C" fn();

    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> phys_addr_t;
    fn alpine_cpu_pm_init();
    fn cpu_logical_map(cpu: u32) -> u32;
    fn alpine_cpu_wakeup(cpu: u32, addr: u32) -> i32;
    fn pr_err(fmt: *const core::ffi::c_char, ...) -> i32;
}

type phys_addr_t = usize;

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

const EINVAL: i32 = 22;

unsafe extern "C" fn alpine_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let addr: phys_addr_t;

    addr = __pa_symbol(secondary_startup);

    if addr > (u32::MAX as phys_addr_t) {
        // The original kernel format is "%pa" and receives &addr.
        pr_err(b"FAIL: resume address over 32bit (%pa)\0".as_ptr() as *const core::ffi::c_char, &addr);
        return -EINVAL;
    }

    alpine_cpu_wakeup(cpu_logical_map(cpu), addr as u32)
}

unsafe extern "C" fn alpine_smp_prepare_cpus(_max_cpus: u32) {
    alpine_cpu_pm_init();
}

#[used]
#[link_section = ".init.rodata"]
pub static alpine_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(alpine_smp_prepare_cpus),
    smp_boot_secondary: Some(alpine_boot_secondary),
};

// CPU_METHOD_OF_DECLARE(alpine_smp, "al,alpine-smp", &alpine_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
