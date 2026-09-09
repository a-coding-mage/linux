/* SPDX-License-Identifier: GPL-2.0-only */

// C dependency: <asm-generic/seccomp.h>

pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_ARM;
pub const SECCOMP_ARCH_NATIVE_NR: u32 = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "arm";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
