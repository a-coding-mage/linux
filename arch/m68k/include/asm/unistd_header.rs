/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: symbols from <uapi/asm/unistd.h> are supplied externally.

pub const NR_syscalls: usize = __NR_syscalls;

pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_OLD_READDIR: bool = true;
pub const __ARCH_WANT_OLD_STAT: bool = true;
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
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_OLD_GETRLIMIT: bool = true;
pub const __ARCH_WANT_SYS_OLD_MMAP: bool = true;
pub const __ARCH_WANT_SYS_OLD_SELECT: bool = true;
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
