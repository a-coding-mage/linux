/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_size_t = usize;
pub type __kernel_ssize_t = isize;

pub type __kernel_old_dev_t = u16;

/* Preserved from the C header: these declarations exist only under __KERNEL__. */
#[cfg(feature = "__KERNEL__")]
pub type __kernel_old_uid_t = u16;
#[cfg(feature = "__KERNEL__")]
pub type __kernel_old_gid_t = u16;

pub type __kernel_ino_t = u32;
pub type __kernel_mode_t = u32;
pub type __kernel_ipc_pid_t = i32;
pub type __kernel_uid_t = u32;
pub type __kernel_gid_t = u32;
pub type __kernel_ptrdiff_t = isize;
pub type __kernel_sigset_t = usize; /* at least 32 bits */

/* The C header includes <asm-generic/posix_types.h>; its declarations are
 * supplied by the surrounding translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
