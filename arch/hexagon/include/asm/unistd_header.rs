/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Architecture syscall feature-selection macros from the C header.
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;

pub const __ARCH_BROKEN_SYS_CLONE3: bool = true;

// C dependency: <uapi/asm/unistd.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
