/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not included here.

/*
 * Various counters maintained by the scheduler and fork(),
 * exposed via /proc, sys.c or used by drivers via these APIs.
 *
 * ( Note that all these values are acquired without locking,
 *   so they can only be relied on in narrow circumstances. )
 */

use core::ffi::c_ulong;

pub const CONFIG_SCHED_INFO: bool = cfg!(feature = "CONFIG_SCHED_INFO");

unsafe extern "C" {
    pub static mut total_forks: c_ulong;
    pub static mut nr_threads: i32;

    // DECLARE_PER_CPU(unsigned long, process_counts);
    pub static mut process_counts: c_ulong;

    pub fn nr_processes() -> i32;
    pub fn nr_running() -> u32;
    pub fn single_task_running() -> bool;
    pub fn nr_iowait() -> u32;
    pub fn nr_iowait_cpu(cpu: i32) -> u32;
}

#[inline]
pub fn sched_info_on() -> bool {
    CONFIG_SCHED_INFO
}

#[cfg(feature = "CONFIG_SCHEDSTATS")]
unsafe extern "C" {
    pub fn force_schedstat_enabled();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
