/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _XTENSA_UNISTD_H

// #include <uapi/asm/unistd.h>

pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;

pub const NR_syscalls: u32 = __NR_syscalls;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
