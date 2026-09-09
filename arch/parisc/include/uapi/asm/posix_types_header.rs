/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

/* Equivalent to the C header's !__LP64__ build-time condition. */
#[cfg(not(target_pointer_width = "64"))]
pub type __kernel_mode_t = u16;

pub type __kernel_ipc_pid_t = u16;

pub type __kernel_off64_t = i64;
pub type __kernel_ino64_t = u64;

/* Dependency supplied by asm-generic/posix_types.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
