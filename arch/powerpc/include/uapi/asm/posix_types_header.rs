/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

#[cfg(target_arch = "powerpc64")]
pub type __kernel_old_dev_t = ::core::ffi::c_ulong;

/* C self-referential macro: #define __kernel_old_dev_t __kernel_old_dev_t */

#[cfg(not(target_arch = "powerpc64"))]
pub type __kernel_ipc_pid_t = ::core::ffi::c_short;

/* C self-referential macro: #define __kernel_ipc_pid_t __kernel_ipc_pid_t */

/* Dependency equivalent of: #include <asm-generic/posix_types.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
