/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: symbols from <asm/unistd.h> and
// <asm-generic/seccomp.h> are supplied by other translated files.

#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_RISCV64;

#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;

#[cfg(CONFIG_64BIT)]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "riscv64";

#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_RISCV32;

#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;

#[cfg(not(CONFIG_64BIT))]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "riscv32";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
