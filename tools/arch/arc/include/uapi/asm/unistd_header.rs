/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

/******** no-legacy-syscalls-ABI *******/

/*
 * Non-typical guard macro to enable inclusion twice in ARCH sys.c
 * That is how the Generic syscall wrapper generator works
 *
 * C condition preserved from the source:
 * #if !defined(_UAPI_ASM_ARC_UNISTD_H) || defined(__SYSCALL)
 */

pub const _UAPI_ASM_ARC_UNISTD_H: bool = true;

pub const __ARCH_WANT_RENAMEAT: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SET_GET_RLIMIT: bool = true;
pub const __ARCH_WANT_SYS_EXECVE: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_TIME32_SYSCALLS: bool = true;

pub use sys_mmap_pgoff as sys_mmap2;

/* Depends on <asm-generic/unistd.h>. */

pub const NR_syscalls: usize = __NR_syscalls;

/* Generic syscall (fs/filesystems.c - lost in asm-generic/unistd.h */
pub const __NR_sysfs: usize = __NR_arch_specific_syscall + 3;

/* ARC specific syscall */
pub const __NR_cacheflush: usize = __NR_arch_specific_syscall + 0;
pub const __NR_arc_settls: usize = __NR_arch_specific_syscall + 1;
pub const __NR_arc_gettls: usize = __NR_arch_specific_syscall + 2;
pub const __NR_arc_usr_cmpxchg: usize = __NR_arch_specific_syscall + 4;

__SYSCALL!(__NR_cacheflush, sys_cacheflush);
__SYSCALL!(__NR_arc_settls, sys_arc_settls);
__SYSCALL!(__NR_arc_gettls, sys_arc_gettls);
__SYSCALL!(__NR_arc_usr_cmpxchg, sys_arc_usr_cmpxchg);
__SYSCALL!(__NR_sysfs, sys_sysfs);

/* #undef __SYSCALL */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
