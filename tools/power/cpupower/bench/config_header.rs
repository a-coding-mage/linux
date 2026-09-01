/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

/* initial loop count for the load calibration */
pub const GAUGECOUNT: i32 = 1500;

/*
 * default scheduling policy SCHED_OTHER
 *
 * SCHED_OTHER is supplied by the system scheduling bindings.
 */
pub const SCHEDULER: i32 = SCHED_OTHER;

pub const PRIORITY_DEFAULT: i32 = 0;

unsafe extern "C" {
    pub fn sched_get_priority_max(policy: i32) -> i32;
    pub fn sched_get_priority_min(policy: i32) -> i32;
}

#[inline]
pub unsafe fn PRIORITY_HIGH() -> i32 {
    unsafe { sched_get_priority_max(SCHEDULER) }
}

#[inline]
pub unsafe fn PRIORITY_LOW() -> i32 {
    unsafe { sched_get_priority_min(SCHEDULER) }
}

/*
 * enable further debug messages
 *
 * Original C macro:
 *   #ifdef DEBUG
 *   #define dprintf printf
 *   #else
 *   #define dprintf(...) do { } while (0)
 *   #endif
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
