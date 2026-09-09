/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  arch/arm/include/asm/posix_types.h
 *
 *  Copyright (C) 1996-1998 Russell King.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 *  Changelog:
 *   27-06-1996 RMK Created
 */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_mode_t = u16;
// #define __kernel_mode_t __kernel_mode_t

pub type __kernel_ipc_pid_t = u16;
// #define __kernel_ipc_pid_t __kernel_ipc_pid_t

pub type __kernel_uid_t = u16;
pub type __kernel_gid_t = u16;
// #define __kernel_uid_t __kernel_uid_t

pub type __kernel_old_dev_t = u16;
// #define __kernel_old_dev_t __kernel_old_dev_t

// Dependency equivalent of: #include <asm-generic/posix_types.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
