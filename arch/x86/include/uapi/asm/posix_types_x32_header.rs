/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is only used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 *
 * These types should generally match the ones used by the 64-bit kernel,
 *
 */

pub type __kernel_long_t = i64;
pub type __kernel_ulong_t = u64;

/* C compatibility macro: #define __kernel_long_t __kernel_long_t */

/* Dependency corresponding to: #include <asm/posix_types_64.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
