/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/seccomp.h
 *
 * Copyright (C) 2014 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

// Dependency supplied by the corresponding Rust translation of
// <asm/unistd_compat_32.h>.

#[cfg(feature = "CONFIG_COMPAT")]
pub const __NR_seccomp_read_32: _ = __NR_compat32_read;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __NR_seccomp_write_32: _ = __NR_compat32_write;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __NR_seccomp_exit_32: _ = __NR_compat32_exit;
#[cfg(feature = "CONFIG_COMPAT")]
pub const __NR_seccomp_sigreturn_32: _ = __NR_compat32_rt_sigreturn;

// Declarations supplied by <asm-generic/seccomp.h> remain external
// dependencies of this header translation.

pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_AARCH64;
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "aarch64";

#[cfg(feature = "CONFIG_COMPAT")]
pub const SECCOMP_ARCH_COMPAT: _ = AUDIT_ARCH_ARM;
#[cfg(feature = "CONFIG_COMPAT")]
pub const SECCOMP_ARCH_COMPAT_NR: _ = __NR_compat32_syscalls;
#[cfg(feature = "CONFIG_COMPAT")]
pub const SECCOMP_ARCH_COMPAT_NAME: &str = "arm";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
