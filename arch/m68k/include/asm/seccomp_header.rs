/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency equivalent of <asm-generic/seccomp.h>.

pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_M68K;
pub const SECCOMP_ARCH_NATIVE_NR: usize = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "m68k";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
