/* SPDX-License-Identifier: GPL-2.0 */

// C header guard-style condition: define only if __NR_fork is not already defined.
pub const __NR_fork: u32 = 2;

// C header guard-style condition: define only if __NR_execve is not already defined.
pub const __NR_execve: u32 = 11;

// C header guard-style condition: define only if __NR_getppid is not already defined.
pub const __NR_getppid: u32 = 64;

// C header guard-style condition: define only if __NR_getpgid is not already defined.
pub const __NR_getpgid: u32 = 132;

// C header guard-style condition: define only if __NR_capget is not already defined.
pub const __NR_capget: u32 = 184;

// C header guard-style condition: define only if __NR_gettid is not already defined.
pub const __NR_gettid: u32 = 224;

// C header guard-style condition: define only if __NR_futex is not already defined.
pub const __NR_futex: u32 = 240;

// C header guard-style condition: define only if __NR_getcpu is not already defined.
pub const __NR_getcpu: u32 = 318;

// C header guard-style condition: define only if __NR_perf_event_open is not already defined.
pub const __NR_perf_event_open: u32 = 336;

// C header guard-style condition: define only if __NR_setns is not already defined.
pub const __NR_setns: u32 = 346;

// C header guard-style condition: define only if __NR_seccomp is not already defined.
pub const __NR_seccomp: u32 = 354;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
