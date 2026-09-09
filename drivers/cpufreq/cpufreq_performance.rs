// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/cpufreq/cpufreq_performance.c
 *
 *  Copyright (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
 */

// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.

use core::ffi::c_void;

// Supplied by the cpufreq, init, and module dependencies.
#[repr(C)]
pub struct cpufreq_policy {
    pub max: u32,
}

pub type cpufreq_limits_fn = unsafe extern "C" fn(policy: *mut cpufreq_policy);

#[repr(C)]
pub struct cpufreq_governor {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub flags: u32,
    pub limits: Option<cpufreq_limits_fn>,
}

unsafe extern "C" {
    fn __cpufreq_driver_target(
        policy: *mut cpufreq_policy,
        target_freq: u32,
        relation: u32,
    );
    fn pr_debug(fmt: *const u8, ...);
}

// These constants and symbols are supplied by the kernel dependencies.
unsafe extern "C" {
    static THIS_MODULE: c_void;
}

const CPUFREQ_RELATION_H: u32 = 0;
const CPUFREQ_GOV_STRICT_TARGET: u32 = 0;

unsafe extern "C" fn cpufreq_gov_performance_limits(policy: *mut cpufreq_policy) {
    unsafe {
        pr_debug(b"setting to %u kHz\n\0".as_ptr(), (*policy).max);
        __cpufreq_driver_target(policy, (*policy).max, CPUFREQ_RELATION_H);
    }
}

static mut cpufreq_gov_performance: cpufreq_governor = cpufreq_governor {
    name: b"performance\0".as_ptr(),
    owner: core::ptr::addr_of!(THIS_MODULE) as *mut c_void,
    flags: CPUFREQ_GOV_STRICT_TARGET,
    limits: Some(cpufreq_gov_performance_limits),
};

// Preserved from: #ifdef CONFIG_CPU_FREQ_DEFAULT_GOV_PERFORMANCE
#[cfg(CONFIG_CPU_FREQ_DEFAULT_GOV_PERFORMANCE)]
#[no_mangle]
pub unsafe extern "C" fn cpufreq_default_governor() -> *mut cpufreq_governor {
    unsafe { core::ptr::addr_of_mut!(cpufreq_gov_performance) }
}

// Preserved from: #ifndef CONFIG_CPU_FREQ_GOV_PERFORMANCE_MODULE
#[cfg(not(CONFIG_CPU_FREQ_GOV_PERFORMANCE_MODULE))]
#[no_mangle]
pub unsafe extern "C" fn cpufreq_fallback_governor() -> *mut cpufreq_governor {
    unsafe { core::ptr::addr_of_mut!(cpufreq_gov_performance) }
}

// MODULE_AUTHOR("Dominik Brodowski <linux@brodo.de>");
// MODULE_DESCRIPTION("CPUfreq policy governor 'performance'");
// MODULE_LICENSE("GPL");

// cpufreq_governor_init(cpufreq_gov_performance);
// cpufreq_governor_exit(cpufreq_gov_performance);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
