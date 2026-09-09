/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 *
 * Author: Meng Li <li.meng@amd.com>
 */

// Dependencies supplied by the Linux kernel headers:
// linux/pm_qos.h, linux/platform_profile.h

// #if IS_MODULE(CONFIG_X86_AMD_PSTATE_UT)
// EXPORT_SYMBOL_FOR_PSTATE_UT(symbol) exports symbol for the amd-pstate-ut module.
// #else
// EXPORT_SYMBOL_FOR_PSTATE_UT(symbol) is empty.
// #endif

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerfCachedFields {
    pub highest_perf: u8,
    pub nominal_perf: u8,
    pub lowest_nonlinear_perf: u8,
    pub lowest_perf: u8,
    pub min_limit_perf: u8,
    pub max_limit_perf: u8,
    pub bios_min_perf: u8,
}

#[repr(C)]
pub union perf_cached {
    pub fields: PerfCachedFields,
    pub val: u64,
}

#[repr(C)]
pub struct amd_aperf_mperf {
    pub aperf: u64,
    pub mperf: u64,
    pub tsc: u64,
}

#[repr(C)]
pub struct amd_cpudata {
    pub cpu: i32,
    pub req: [freq_qos_request; 2],
    pub cppc_req_cached: u64,
    pub cppc_req2_cached: u64,
    pub perf: perf_cached,
    pub prefcore_ranking: u8,
    pub floor_perf_cnt: u8,
    pub bios_floor_perf: u8,
    pub min_limit_freq: u32,
    pub max_limit_freq: u32,
    pub nominal_freq: u32,
    pub max_freq: u32,
    pub lowest_nonlinear_freq: u32,
    pub floor_freq: u32,
    pub cur: amd_aperf_mperf,
    pub prev: amd_aperf_mperf,
    pub freq: u64,
    pub boost_supported: bool,
    pub hw_prefcore: bool,
    pub policy: u32,
    pub suspended: bool,
    pub epp_default_ac: u8,
    pub epp_default_dc: u8,
    pub dynamic_epp: bool,
    pub raw_epp: bool,
    pub power_nb: notifier_block,
    pub current_profile: platform_profile_option,
    pub ppdev: *mut device,
    pub profile_name: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amd_pstate_mode {
    AMD_PSTATE_UNDEFINED = 0,
    AMD_PSTATE_DISABLE,
    AMD_PSTATE_PASSIVE,
    AMD_PSTATE_ACTIVE,
    AMD_PSTATE_GUIDED,
    AMD_PSTATE_MAX,
}

extern "C" {
    pub fn amd_pstate_get_mode_string(mode: amd_pstate_mode) -> *const i8;
    pub fn amd_pstate_get_status() -> i32;
    pub fn amd_pstate_update_status(buf: *const i8, size: usize) -> i32;
    pub fn store_energy_performance_preference(
        policy: *mut cpufreq_policy,
        buf: *const i8,
        count: usize,
    ) -> isize;
    pub fn show_energy_performance_preference(
        policy: *mut cpufreq_policy,
        buf: *mut i8,
    ) -> isize;
    pub fn amd_pstate_clear_dynamic_epp(policy: *mut cpufreq_policy);
    pub fn store_amd_pstate_floor_freq(
        policy: *mut cpufreq_policy,
        buf: *const i8,
        count: usize,
    ) -> isize;
    pub fn show_amd_pstate_floor_freq(policy: *mut cpufreq_policy, buf: *mut i8) -> isize;
    pub fn amd_pstate_get_current_attrs() -> *mut *mut freq_attr;
}

// External kernel types supplied by included headers.
extern "C" {
    type freq_qos_request;
    type notifier_block;
    type platform_profile_option;
    type device;
    type cpufreq_policy;
    type freq_attr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
