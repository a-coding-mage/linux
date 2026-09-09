/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007-2008 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header guard: _ASM_MICROBLAZE_UNISTD_H
// Dependency: <uapi/asm/unistd.h>

// The following feature markers are defined only when __ASSEMBLER__ is not
// defined in the C source.

// #define __ARCH_WANT_OLD_READDIR
// #define __ARCH_WANT_OLD_STAT
pub const __ARCH_WANT_NEW_STAT: i32 = 1;
pub const __ARCH_WANT_STAT64: i32 = 1;
pub const __ARCH_WANT_SYS_ALARM: i32 = 1;
pub const __ARCH_WANT_SYS_GETHOSTNAME: i32 = 1;
pub const __ARCH_WANT_SYS_PAUSE: i32 = 1;
pub const __ARCH_WANT_SYS_SIGNAL: i32 = 1;
pub const __ARCH_WANT_SYS_TIME32: i32 = 1;
pub const __ARCH_WANT_SYS_UTIME32: i32 = 1;
pub const __ARCH_WANT_SYS_WAITPID: i32 = 1;
pub const __ARCH_WANT_SYS_SOCKETCALL: i32 = 1;
pub const __ARCH_WANT_SYS_FADVISE64: i32 = 1;
pub const __ARCH_WANT_SYS_GETPGRP: i32 = 1;
pub const __ARCH_WANT_SYS_NICE: i32 = 1;
// #define __ARCH_WANT_SYS_OLD_GETRLIMIT
pub const __ARCH_WANT_SYS_OLDUMOUNT: i32 = 1;
pub const __ARCH_WANT_SYS_SIGPENDING: i32 = 1;
pub const __ARCH_WANT_SYS_SIGPROCMASK: i32 = 1;
pub const __ARCH_WANT_SYS_CLONE: i32 = 1;
pub const __ARCH_WANT_SYS_VFORK: i32 = 1;
pub const __ARCH_WANT_SYS_FORK: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
