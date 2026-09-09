/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

// C preprocessor feature markers from the original header.
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_TIME32_SYSCALLS: bool = true;

// The original header includes <uapi/asm/unistd.h>; its declarations are
// supplied by the corresponding Rust dependency/module.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
