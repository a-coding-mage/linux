/* SPDX-License-Identifier: GPL-2.0 */

/* Defined in C only when __NR_fork is not already provided. */
pub const __NR_fork: u32 = 57;

/* Defined in C only when __NR_execve is not already provided. */
pub const __NR_execve: u32 = 59;

/* Defined in C only when __NR_getppid is not already provided. */
pub const __NR_getppid: u32 = 110;

/* Defined in C only when __NR_getpgid is not already provided. */
pub const __NR_getpgid: u32 = 121;

/* Defined in C only when __NR_capget is not already provided. */
pub const __NR_capget: u32 = 125;

/* Defined in C only when __NR_gettid is not already provided. */
pub const __NR_gettid: u32 = 186;

/* Defined in C only when __NR_futex is not already provided. */
pub const __NR_futex: u32 = 202;

/* Defined in C only when __NR_perf_event_open is not already provided. */
pub const __NR_perf_event_open: u32 = 298;

/* Defined in C only when __NR_setns is not already provided. */
pub const __NR_setns: u32 = 308;

/* Defined in C only when __NR_getcpu is not already provided. */
pub const __NR_getcpu: u32 = 309;

/* Defined in C only when __NR_seccomp is not already provided. */
pub const __NR_seccomp: u32 = 317;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
