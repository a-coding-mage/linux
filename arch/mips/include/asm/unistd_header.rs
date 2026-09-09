/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 *
 * Changed system calls macros _syscall5 - _syscall7 to push args 5 to 7 onto
 * the stack. Robin Farine for ACN S.A, Copyright (C) 1996 by ACN S.A
 */

// Dependencies supplied by the corresponding UAPI and syscall-number headers:
// <uapi/asm/unistd.h>, <asm/unistd_nr_n32.h>, <asm/unistd_nr_n64.h>,
// and <asm/unistd_nr_o32.h>.

pub const __NR_N32_Linux: i32 = 6000;
pub const __NR_64_Linux: i32 = 5000;
pub const __NR_O32_Linux: i32 = 4000;

// Build-time configuration condition from CONFIG_MIPS32_N32.
#[cfg(feature = "CONFIG_MIPS32_N32")]
pub const NR_syscalls: i32 = __NR_N32_Linux + __NR_N32_Linux_syscalls;

// Build-time configuration condition from CONFIG_64BIT.
#[cfg(all(not(feature = "CONFIG_MIPS32_N32"), feature = "CONFIG_64BIT"))]
pub const NR_syscalls: i32 = __NR_64_Linux + __NR_64_Linux_syscalls;

// Build-time fallback when neither CONFIG_MIPS32_N32 nor CONFIG_64BIT is set.
#[cfg(all(not(feature = "CONFIG_MIPS32_N32"), not(feature = "CONFIG_64BIT")))]
pub const NR_syscalls: i32 = __NR_O32_Linux + __NR_O32_Linux_syscalls;

// The following declarations correspond to the non-assembler branch.
pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_OLD_READDIR: bool = true;
pub const __ARCH_WANT_SYS_ALARM: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_IPC: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_UTIME: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_WAITPID: bool = true;
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_OLD_UNAME: bool = true;
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;

// Build-time condition from CONFIG_32BIT.
#[cfg(feature = "CONFIG_32BIT")]
pub const __ARCH_WANT_STAT64: bool = true;

// Build-time inverse condition: CONFIG_64BIT.
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const __ARCH_WANT_COMPAT_STAT: bool = true;

// Build-time conditions from CONFIG_32BIT or CONFIG_MIPS32_O32.
#[cfg(any(feature = "CONFIG_32BIT", feature = "CONFIG_MIPS32_O32"))]
pub const __ARCH_WANT_SYS_TIME32: bool = true;

pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;

/* whitelists for checksyscalls */
pub const __IGNORE_fadvise64_64: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
