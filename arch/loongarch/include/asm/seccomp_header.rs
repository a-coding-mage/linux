/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent:
// - asm/unistd.h
// - asm-generic/seccomp.h

#[cfg(feature = "CONFIG_32BIT")]
pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_LOONGARCH32;
#[cfg(feature = "CONFIG_32BIT")]
pub const SECCOMP_ARCH_NATIVE_NR: usize = NR_syscalls;
#[cfg(feature = "CONFIG_32BIT")]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "loongarch32";

#[cfg(not(feature = "CONFIG_32BIT"))]
pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_LOONGARCH64;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const SECCOMP_ARCH_NATIVE_NR: usize = NR_syscalls;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "loongarch64";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
