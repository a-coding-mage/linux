/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from asm-generic/seccomp.h; its declarations are supplied externally. */

/* CONFIG_64BIT */
#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_PARISC64;
#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE_NR: u32 = NR_syscalls;
#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "parisc64";

/* CONFIG_COMPAT, when CONFIG_64BIT is enabled. */
#[cfg(all(CONFIG_64BIT, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT: u32 = AUDIT_ARCH_PARISC;
#[cfg(all(CONFIG_64BIT, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT_NR: u32 = NR_syscalls;
#[cfg(all(CONFIG_64BIT, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT_NAME: &str = "parisc";

/* !CONFIG_64BIT */
#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_PARISC;
#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE_NR: u32 = NR_syscalls;
#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "parisc";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
