/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/posix_types.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Largely copied from include/asm-ppc/posix_types.h
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_ipc_pid_t = u16;
// C compatibility macro: #define __kernel_ipc_pid_t __kernel_ipc_pid_t

pub type __kernel_size_t = u32;
pub type __kernel_ssize_t = i32;
pub type __kernel_ptrdiff_t = i32;
// C compatibility macro: #define __kernel_size_t __kernel_size_t

pub type __kernel_old_uid_t = u16;
pub type __kernel_old_gid_t = u16;
// C compatibility macro: #define __kernel_old_uid_t __kernel_old_uid_t

pub type __kernel_old_dev_t = u16;
// C compatibility macro: #define __kernel_old_dev_t __kernel_old_dev_t

// Dependency corresponding to: #include <asm-generic/posix_types.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
