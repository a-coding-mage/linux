/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TI DaVinci CPUFreq platform support.
 *
 * Copyright (C) 2009 Texas Instruments, Inc. https://www.ti.com/
 */

// Dependency supplied by the Linux cpufreq subsystem:
// use linux::cpufreq::cpufreq_frequency_table;

#[repr(C)]
pub struct davinci_cpufreq_config {
    pub freq_table: *mut cpufreq_frequency_table,
    pub set_voltage: Option<unsafe extern "C" fn(index: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

// The CONFIG_CPU_FREQ build-time condition is preserved through this cfg.
#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe extern "C" {
    pub fn davinci_cpufreq_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_CPU_FREQ"))]
#[inline]
pub unsafe fn davinci_cpufreq_init() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
