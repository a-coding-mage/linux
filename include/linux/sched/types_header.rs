/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/types.h>

/**
 * struct task_cputime - collected CPU time counts
 * @stime:              time spent in kernel mode, in nanoseconds
 * @utime:              time spent in user mode, in nanoseconds
 * @sum_exec_runtime:   total time spent on the CPU, in nanoseconds
 *
 * This structure groups together three kinds of CPU time that are tracked for
 * threads and thread groups.  Most things considering CPU time want to group
 * these counts together and treat all three of them in parallel.
 */
#[repr(C)]
pub struct task_cputime {
    pub stime: u64,
    pub utime: u64,
    pub sum_exec_runtime: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
