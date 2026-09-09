/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture, generic seccomp,
// and Linux audit headers are referenced here as external symbols.

pub const SECCOMP_ARCH_NATIVE: u32 = AUDIT_ARCH_ALPHA;
pub const SECCOMP_ARCH_NATIVE_NR: u32 = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "alpha";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
