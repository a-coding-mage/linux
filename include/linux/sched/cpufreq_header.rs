/* SPDX-License-Identifier: GPL-2.0 */

// Original dependency: <linux/types.h>
// The following declarations are conditioned on CONFIG_CPU_FREQ in the C
// header; preserve that build-time condition at the integration boundary.

pub const SCHED_CPUFREQ_IOWAIT: u32 = 1u32 << 0;

#[repr(C)]
pub struct cpufreq_policy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct update_util_data {
    pub func: Option<unsafe extern "C" fn(
        data: *mut update_util_data,
        time: u64,
        flags: u32,
    )>,
}

unsafe extern "C" {
    pub fn cpufreq_add_update_util_hook(
        cpu: i32,
        data: *mut update_util_data,
        func: Option<unsafe extern "C" fn(
            data: *mut update_util_data,
            time: u64,
            flags: u32,
        )>,
    );

    pub fn cpufreq_remove_update_util_hook(cpu: i32);

    pub fn cpufreq_this_cpu_can_update(policy: *mut cpufreq_policy) -> bool;
}

#[inline]
pub fn map_util_freq(util: usize, freq: usize, cap: usize) -> usize {
    freq.wrapping_mul(util) / cap
}

#[inline]
pub fn map_util_perf(util: usize) -> usize {
    util.wrapping_add(util >> 2)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
