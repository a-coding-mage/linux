/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This file contains the system call numbers.
 */

// The UAPI definitions are supplied by the surrounding translation unit.
pub const NR_syscalls: usize = __NR_syscalls as usize;

// These architecture feature markers correspond to the C preprocessor
// definitions used when building the system-call table.
pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_OLD_READDIR: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_ALARM: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_IPC: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_SIGNAL: bool = true;
pub const __ARCH_WANT_SYS_TIME32: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_WAITPID: bool = true;
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;
pub const __ARCH_WANT_SYS_FADVISE64: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_LLSEEK: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_OLD_GETRLIMIT: bool = true;
pub const __ARCH_WANT_SYS_OLD_UNAME: bool = true;
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;

// CONFIG_PPC32 conditional definitions.
#[cfg(CONFIG_PPC32)]
pub const __ARCH_WANT_OLD_STAT: bool = true;
#[cfg(CONFIG_PPC32)]
pub const __ARCH_WANT_SYS_OLD_SELECT: bool = true;

// CONFIG_PPC64 conditional definitions.
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_SYS_TIME: bool = true;
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_SYS_UTIME: bool = true;
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_SYS_NEWFSTATAT: bool = true;
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_COMPAT_STAT: bool = true;
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_COMPAT_FALLOCATE: bool = true;
#[cfg(CONFIG_PPC64)]
pub const __ARCH_WANT_COMPAT_SYS_SENDFILE: bool = true;

pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
