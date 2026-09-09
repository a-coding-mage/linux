/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/asm/unistd.h> dependency.

/// Number of system calls, provided by the UAPI dependency.
pub const NR_syscalls: usize = __NR_syscalls as usize;

// Architecture feature markers from the C header:
// __ARCH_WANT_NEW_STAT
// __ARCH_WANT_OLD_READDIR
// __ARCH_WANT_STAT64
// __ARCH_WANT_SYS_GETHOSTNAME
// __ARCH_WANT_SYS_FADVISE64
// __ARCH_WANT_SYS_GETPGRP
// __ARCH_WANT_SYS_OLDUMOUNT
// __ARCH_WANT_SYS_SIGPENDING
// __ARCH_WANT_SYS_UTIME
// __ARCH_WANT_SYS_FORK
// __ARCH_WANT_SYS_VFORK
// __ARCH_WANT_SYS_CLONE

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
