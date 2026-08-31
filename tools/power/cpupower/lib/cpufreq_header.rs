/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  cpufreq.h - definitions for libcpufreq
 *
 *  Copyright (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 */

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong};

#[repr(C)]
pub struct cpufreq_policy {
    pub min: c_ulong,
    pub max: c_ulong,
    pub governor: *mut c_char,
}

#[repr(C)]
pub struct cpufreq_available_governors {
    pub governor: *mut c_char,
    pub next: *mut cpufreq_available_governors,
    pub first: *mut cpufreq_available_governors,
}

#[repr(C)]
pub struct cpufreq_available_frequencies {
    pub frequency: c_ulong,
    pub next: *mut cpufreq_available_frequencies,
    pub first: *mut cpufreq_available_frequencies,
}

#[repr(C)]
pub struct cpufreq_affected_cpus {
    pub cpu: c_uint,
    pub next: *mut cpufreq_affected_cpus,
    pub first: *mut cpufreq_affected_cpus,
}

#[repr(C)]
pub struct cpufreq_stats {
    pub frequency: c_ulong,
    pub time_in_state: c_ulonglong,
    pub next: *mut cpufreq_stats,
    pub first: *mut cpufreq_stats,
}

unsafe extern "C" {
    /*
     * determine current CPU frequency
     * - _kernel variant means kernel's opinion of CPU frequency
     * - _hardware variant means actual hardware CPU frequency,
     *    which is only available to root.
     *
     * returns 0 on failure, else frequency in kHz.
     */
    pub fn cpufreq_get_freq_kernel(cpu: c_uint) -> c_ulong;

    pub fn cpufreq_get_freq_hardware(cpu: c_uint) -> c_ulong;

    /*
     * determine CPU transition latency
     *
     * returns 0 on failure, else transition latency in 10^(-9) s = nanoseconds
     */
    pub fn cpufreq_get_transition_latency(cpu: c_uint) -> c_ulong;

    /*
     * determine energy performance preference
     *
     * returns NULL on failure, else the string that represents the energy performance
     * preference requested.
     */
    pub fn cpufreq_get_energy_performance_preference(cpu: c_uint) -> *mut c_char;
    pub fn cpufreq_put_energy_performance_preference(ptr: *mut c_char);

    /*
     * determine hardware CPU frequency limits
     *
     * These may be limited further by thermal, energy or other
     * considerations by cpufreq policy notifiers in the kernel.
     */
    pub fn cpufreq_get_hardware_limits(
        cpu: c_uint,
        min: *mut c_ulong,
        max: *mut c_ulong,
    ) -> c_int;

    /*
     * determine CPUfreq driver used
     *
     * Remember to call cpufreq_put_driver when no longer needed
     * to avoid memory leakage, please.
     */
    pub fn cpufreq_get_driver(cpu: c_uint) -> *mut c_char;

    pub fn cpufreq_put_driver(ptr: *mut c_char);

    /*
     * determine CPUfreq policy currently used
     *
     * Remember to call cpufreq_put_policy when no longer needed
     * to avoid memory leakage, please.
     */
    pub fn cpufreq_get_policy(cpu: c_uint) -> *mut cpufreq_policy;

    pub fn cpufreq_put_policy(policy: *mut cpufreq_policy);

    /*
     * determine CPUfreq governors currently available
     *
     * may be modified by modprobe'ing or rmmod'ing other governors. Please
     * free allocated memory by calling cpufreq_put_available_governors
     * after use.
     */
    pub fn cpufreq_get_available_governors(cpu: c_uint) -> *mut cpufreq_available_governors;

    pub fn cpufreq_put_available_governors(first: *mut cpufreq_available_governors);

    /*
     * determine CPU frequency states available
     *
     * Only present on _some_ ->target() cpufreq drivers. For information purposes
     * only. Please free allocated memory by calling
     * cpufreq_put_frequencies after use.
     */
    pub fn cpufreq_get_available_frequencies(cpu: c_uint) -> *mut cpufreq_available_frequencies;

    pub fn cpufreq_put_available_frequencies(first: *mut cpufreq_available_frequencies);

    pub fn cpufreq_get_boost_frequencies(cpu: c_uint) -> *mut cpufreq_available_frequencies;

    pub fn cpufreq_put_boost_frequencies(first: *mut cpufreq_available_frequencies);

    /*
     * determine affected CPUs
     *
     * Remember to call cpufreq_put_affected_cpus when no longer needed
     * to avoid memory leakage, please.
     */
    pub fn cpufreq_get_affected_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus;

    pub fn cpufreq_put_affected_cpus(first: *mut cpufreq_affected_cpus);

    /*
     * determine related CPUs
     *
     * Remember to call cpufreq_put_related_cpus when no longer needed
     * to avoid memory leakage, please.
     */
    pub fn cpufreq_get_related_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus;

    pub fn cpufreq_put_related_cpus(first: *mut cpufreq_affected_cpus);

    /*
     * determine stats for cpufreq subsystem
     *
     * This is not available in all kernel versions or configurations.
     */
    pub fn cpufreq_get_stats(cpu: c_uint, total_time: *mut c_ulonglong) -> *mut cpufreq_stats;

    pub fn cpufreq_put_stats(stats: *mut cpufreq_stats);

    pub fn cpufreq_get_transitions(cpu: c_uint) -> c_ulong;

    /*
     * set new cpufreq policy
     *
     * Tries to set the passed policy as new policy as close as possible,
     * but results may differ depending e.g. on governors being available.
     */
    pub fn cpufreq_set_policy(cpu: c_uint, policy: *mut cpufreq_policy) -> c_int;

    /*
     * modify a policy by only changing min/max freq or governor
     *
     * Does not check whether result is what was intended.
     */
    pub fn cpufreq_modify_policy_min(cpu: c_uint, min_freq: c_ulong) -> c_int;
    pub fn cpufreq_modify_policy_max(cpu: c_uint, max_freq: c_ulong) -> c_int;
    pub fn cpufreq_modify_policy_governor(cpu: c_uint, governor: *mut c_char) -> c_int;

    /*
     * set a specific frequency
     *
     * Does only work if userspace governor can be used and no external
     * interference (other calls to this function or to set/modify_policy)
     * occurs. Also does not work on ->range() cpufreq drivers.
     */
    pub fn cpufreq_set_frequency(cpu: c_uint, target_frequency: c_ulong) -> c_int;

    /*
     * get the sysfs value from specific table
     *
     * Read the value with the sysfs file name from specific table. Does
     * only work if the cpufreq driver has the specific sysfs interfaces.
     */
    pub fn cpufreq_get_sysfs_value_from_table(
        cpu: c_uint,
        table: *mut *const c_char,
        index: c_uint,
        size: c_uint,
    ) -> c_ulong;
}

/*
 * C macro:
 * #define cpufreq_get(cpu) cpufreq_get_freq_kernel(cpu);
 */
#[inline]
pub unsafe fn cpufreq_get(cpu: c_uint) -> c_ulong {
    unsafe { cpufreq_get_freq_kernel(cpu) }
}
