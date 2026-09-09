/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

pub const __ARCH_WANT_SYS_CLONE: bool = true;

// CONFIG_COMPAT is a build-time configuration condition from the C header.
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_TRUNCATE64: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_FTRUNCATE64: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_FALLOCATE: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_PREAD64: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_PWRITE64: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_SYNC_FILE_RANGE: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_READAHEAD: bool = true;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __ARCH_WANT_COMPAT_FADVISE64_64: bool = true;

// Corresponds to: #if defined(__LP64__) && !defined(__SYSCALL_COMPAT)
#[cfg(all(target_pointer_width = "64", not(feature = "__SYSCALL_COMPAT")))]
pub const __ARCH_WANT_NEW_STAT: bool = true;
#[cfg(all(target_pointer_width = "64", not(feature = "__SYSCALL_COMPAT")))]
pub const __ARCH_WANT_SET_GET_RLIMIT: bool = true;

pub const __ARCH_WANT_MEMFD_SECRET: bool = true;

// The C header includes <uapi/asm/unistd.h>; its declarations are supplied externally.

pub const NR_syscalls: usize = __NR_syscalls as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
