/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: asm-generic/seccomp.h

pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_XTENSA;
pub const SECCOMP_ARCH_NATIVE_NR: usize = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "xtensa";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
