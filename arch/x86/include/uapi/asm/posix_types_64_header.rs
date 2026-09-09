/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_old_uid_t = u16;
pub type __kernel_old_gid_t = u16;

pub type __kernel_old_dev_t = u64;

// Dependency equivalent of <asm-generic/posix_types.h> is supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
