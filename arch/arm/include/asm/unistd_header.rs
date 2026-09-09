/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/unistd.h
 *
 *  Copyright (C) 2001-2005 Russell King
 *
 * Please forward _all_ changes to this file to rmk@arm.linux.org.uk,
 * no matter what the change is.  Thanks!
 */

// C dependencies:
// #include <uapi/asm/unistd.h>
// #include <asm/unistd-nr.h>

pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;
pub const __ARCH_WANT_SYS_OLD_MMAP: bool = true;
pub const __ARCH_WANT_SYS_OLD_SELECT: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;

// C condition: !defined(CONFIG_AEABI) || defined(CONFIG_OABI_COMPAT).
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_TIME32: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_IPC: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_ALARM: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_OLD_GETRLIMIT: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_OLD_READDIR: bool = true;
#[cfg(any(not(feature = "CONFIG_AEABI"), feature = "CONFIG_OABI_COMPAT"))]
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;

pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;

/*
 * Unimplemented (or alternatively implemented) syscalls
 */
pub const __IGNORE_fadvise64_64: bool = true;

// C condition: defined(__ARM_EABI__).
#[cfg(feature = "__ARM_EABI__")]
pub const __IGNORE_getrlimit: bool = true;

/*
 * The following syscalls are obsolete and no longer available for EABI:
 *  __NR_time
 *  __NR_umount
 *  __NR_stime
 *  __NR_alarm
 *  __NR_utime
 *  __NR_getrlimit
 *  __NR_select
 *  __NR_readdir
 *  __NR_mmap
 *  __NR_socketcall
 *  __NR_syscall
 *  __NR_ipc
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
