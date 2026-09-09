/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_mode_t = u16;

pub type __kernel_ipc_pid_t = u16;

pub type __kernel_uid_t = u16;
pub type __kernel_gid_t = u16;

pub type __kernel_old_dev_t = u16;

// Dependency corresponding to: #include <asm-generic/posix_types.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
