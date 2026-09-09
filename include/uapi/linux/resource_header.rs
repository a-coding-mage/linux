/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies: <linux/time_types.h>, <linux/types.h>

/*
 * Resource control/accounting header file for linux
 */

/*
 * Definition of struct rusage taken from BSD 4.3 Reno
 * 
 * We don't support all of these yet, but we might as well have them....
 * Otherwise, each time we add new items, programs which depend on this
 * structure will lose.  This reduces the chances of that happening.
 */
pub const RUSAGE_SELF: i32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_BOTH: i32 = -2; // sys_wait4() uses this
pub const RUSAGE_THREAD: i32 = 1; // only the calling thread

#[repr(C)]
pub struct rusage {
    pub ru_utime: __kernel_old_timeval, // user time used
    pub ru_stime: __kernel_old_timeval, // system time used
    pub ru_maxrss: __kernel_long_t,     // maximum resident set size
    pub ru_ixrss: __kernel_long_t,      // integral shared memory size
    pub ru_idrss: __kernel_long_t,      // integral unshared data size
    pub ru_isrss: __kernel_long_t,      // integral unshared stack size
    pub ru_minflt: __kernel_long_t,     // page reclaims
    pub ru_majflt: __kernel_long_t,     // page faults
    pub ru_nswap: __kernel_long_t,      // swaps
    pub ru_inblock: __kernel_long_t,    // block input operations
    pub ru_oublock: __kernel_long_t,    // block output operations
    pub ru_msgsnd: __kernel_long_t,     // messages sent
    pub ru_msgrcv: __kernel_long_t,     // messages received
    pub ru_nsignals: __kernel_long_t,   // signals received
    pub ru_nvcsw: __kernel_long_t,      // voluntary context switches
    pub ru_nivcsw: __kernel_long_t,     // involuntary "
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}

pub const RLIM64_INFINITY: u64 = !0u64;

#[repr(C)]
pub struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}

pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: i32 = 20;

pub const PRIO_PROCESS: i32 = 0;
pub const PRIO_PGRP: i32 = 1;
pub const PRIO_USER: i32 = 2;

/*
 * Limit the stack by to some sane default: root can always
 * increase this limit if needed..  8MB seems reasonable.
 */
pub const _STK_LIM: i32 = 8 * 1024 * 1024;

/*
 * Limit the amount of locked memory by some sane default:
 * root can always increase this limit if needed.
 *
 * The main use-cases are (1) preventing sensitive memory
 * from being swapped; (2) real-time operations; (3) via
 * IOURING_REGISTER_BUFFERS.
 *
 * The first two don't need much. The latter will take as
 * much as it can get. 8MB is a reasonably sane default.
 */
pub const MLOCK_LIMIT: i32 = 8 * 1024 * 1024;

/*
 * Due to binary compatibility, the actual resource numbers
 * may be different for different linux versions..
 */
// C dependency: <asm/resource.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
