/* SPDX-License-Identifier: GPL-2.0 */
/*
 * System calls under the Sparc.
 *
 * Don't be scared by the ugly clobbers, it is the only way I can
 * think of right now to force the arguments into fixed registers
 * before the trap into the system call with gcc 'asm' statements.
 *
 * Copyright (C) 1995, 2007 David S. Miller (davem@davemloft.net)
 *
 * SunOS compatibility based upon preliminary work which is:
 *
 * Copyright (C) 1995 Adrian M. Rodriguez (adrian@remus.rutgers.edu)
 */

// Dependency supplied by the corresponding uapi header:
// #include <uapi/asm/unistd.h>

// The C header defines this as __NR_syscalls from the uapi header.
pub const NR_syscalls: usize = __NR_syscalls;

#[cfg(not(__32bit_syscall_numbers__))]
pub const __NR_time: usize = 231; // Linux sparc32

pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_OLD_READDIR: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_ALARM: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_SIGNAL: bool = true;
pub const __ARCH_WANT_SYS_TIME32: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_WAITPID: bool = true;
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;
pub const __ARCH_WANT_SYS_FADVISE64: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;

#[cfg(__32bit_syscall_numbers__)]
pub const __ARCH_WANT_SYS_IPC: bool = true;

#[cfg(not(__32bit_syscall_numbers__))]
pub const __ARCH_WANT_SYS_TIME: bool = true;
#[cfg(not(__32bit_syscall_numbers__))]
pub const __ARCH_WANT_SYS_UTIME: bool = true;
#[cfg(not(__32bit_syscall_numbers__))]
pub const __ARCH_WANT_COMPAT_SYS_SENDFILE: bool = true;
#[cfg(not(__32bit_syscall_numbers__))]
pub const __ARCH_WANT_COMPAT_STAT: bool = true;

#[cfg(__32bit_syscall_numbers__)]
/* Sparc 32-bit only has the "setresuid32", "getresuid32" variants,
 * it never had the plain ones and there is no value to adding those
 * old versions into the syscall table.
 */
pub const __IGNORE_setresuid: bool = true;
#[cfg(__32bit_syscall_numbers__)]
pub const __IGNORE_getresuid: bool = true;
#[cfg(__32bit_syscall_numbers__)]
pub const __IGNORE_setresgid: bool = true;
#[cfg(__32bit_syscall_numbers__)]
pub const __IGNORE_getresgid: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
