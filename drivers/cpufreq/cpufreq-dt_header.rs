/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Linaro
 * Viresh Kumar <viresh.kumar@linaro.org>
 */

// <linux/types.h>

#[repr(C)]
pub struct cpufreq_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpufreq_dt_platform_data {
    pub have_governor_per_policy: bool,

    pub get_intermediate:
        Option<unsafe extern "C" fn(policy: *mut cpufreq_policy, index: core::ffi::c_uint) -> core::ffi::c_uint>,
    pub target_intermediate:
        Option<unsafe extern "C" fn(policy: *mut cpufreq_policy, index: core::ffi::c_uint) -> core::ffi::c_int>,
    pub suspend: Option<unsafe extern "C" fn(policy: *mut cpufreq_policy) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(policy: *mut cpufreq_policy) -> core::ffi::c_int>,
}

unsafe extern "C" {
    pub fn cpufreq_dt_pdev_register(dev: *mut device) -> *mut platform_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
