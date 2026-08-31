/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Syscall support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 and
 * only version 2 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

/*
 *  The kernel pulls this unistd.h in three different ways:
 *  1.  the "normal" way which gets all the __NR defines
 *  2.  with __SYSCALL defined to produce function declarations
 *  3.  with __SYSCALL defined to produce syscall table initialization
 *  See also:  syscalltab.c
 */

pub use sys_mmap_pgoff as sys_mmap2;

pub const __ARCH_WANT_RENAMEAT: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SET_GET_RLIMIT: bool = true;
pub const __ARCH_WANT_SYS_EXECVE: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_TIME32_SYSCALLS: bool = true;

/* Depends on declarations and syscall constants from <asm-generic/unistd.h>. */
