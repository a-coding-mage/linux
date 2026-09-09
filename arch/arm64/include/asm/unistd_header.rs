/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/* The following declarations are enabled when CONFIG_COMPAT is configured. */
#[cfg(feature = "compat")]
pub const __ARCH_WANT_COMPAT_STAT: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_COMPAT_STAT64: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_NICE: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_COMPAT_SYS_SENDFILE: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_FORK: bool = true;
#[cfg(feature = "compat")]
pub const __ARCH_WANT_SYS_VFORK: bool = true;

/*
 * The following SVCs are ARM private.
 */
#[cfg(feature = "compat")]
pub const __ARM_NR_COMPAT_BASE: u32 = 0x0f0000;
#[cfg(feature = "compat")]
pub const __ARM_NR_compat_cacheflush: u32 = __ARM_NR_COMPAT_BASE + 2;
#[cfg(feature = "compat")]
pub const __ARM_NR_compat_set_tls: u32 = __ARM_NR_COMPAT_BASE + 5;
#[cfg(feature = "compat")]
pub const __ARM_NR_COMPAT_END: u32 = __ARM_NR_COMPAT_BASE + 0x800;

pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_NEW_STAT: bool = true;

/* Declarations from <asm/unistd_64.h> are supplied by the surrounding build. */

pub const NR_syscalls: usize = __NR_syscalls as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
