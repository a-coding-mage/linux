/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header file for CPUFreq ondemand governor and related code.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependency supplied by cpufreq_governor.h.

#[repr(C)]
pub struct od_policy_dbs_info {
    pub policy_dbs: policy_dbs_info,
    pub freq_lo: ::core::ffi::c_uint,
    pub freq_lo_delay_us: ::core::ffi::c_uint,
    pub freq_hi_delay_us: ::core::ffi::c_uint,
    // C bit-field: unsigned int sample_type:1.
    pub sample_type: ::core::ffi::c_uint,
}

#[inline]
pub unsafe fn to_dbs_info(
    policy_dbs: *mut policy_dbs_info,
) -> *mut od_policy_dbs_info {
    // Equivalent to the C container_of(policy_dbs, struct od_policy_dbs_info,
    // policy_dbs) macro supplied by the dependency header.
    container_of!(policy_dbs, od_policy_dbs_info, policy_dbs)
}

#[repr(C)]
pub struct od_dbs_tuners {
    pub powersave_bias: ::core::ffi::c_uint,
}

/* CONFIG_X86 conditional declaration, preserved below. */
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn od_should_io_be_busy() -> bool {
    boot_cpu_data.x86_vendor == X86_VENDOR_INTEL
        && boot_cpu_data.x86_vfm >= INTEL_PENTIUM_PRO
}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub fn od_should_io_be_busy() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
