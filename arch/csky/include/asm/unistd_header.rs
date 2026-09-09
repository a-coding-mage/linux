/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/asm/unistd.h>

pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;

pub const NR_syscalls: usize = __NR_syscalls;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
