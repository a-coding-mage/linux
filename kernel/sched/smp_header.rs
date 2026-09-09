/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Scheduler internal SMP callback types and methods between the scheduler
 * and other internal parts of the core kernel.
 *
 * C dependency: <linux/types.h>
 */

use core::ffi::c_void;

extern "C" {
    pub fn sched_ttwu_pending(arg: *mut c_void);

    pub fn call_function_single_prep_ipi(cpu: i32) -> bool;
}

/* CONFIG_SMP build-time condition. */
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn flush_smp_call_function_queue();
}

/* CONFIG_SMP build-time condition: the non-SMP inline function is empty. */
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_smp_call_function_queue() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
