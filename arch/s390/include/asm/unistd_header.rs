/* SPDX-License-Identifier: GPL-2.0 */
/*
 * S390 version
 *
 * Derived from "include/asm-i386/unistd.h"
 */

// Dependency supplied by the UAPI architecture header.
pub const NR_syscalls: _ = __NR_syscalls;

// Architecture syscall feature-selection markers from the C preprocessor
// header. Their presence is represented as unit constants in Rust.
pub const __ARCH_WANT_NEW_STAT: () = ();
pub const __ARCH_WANT_OLD_READDIR: () = ();
pub const __ARCH_WANT_SYS_ALARM: () = ();
pub const __ARCH_WANT_SYS_GETHOSTNAME: () = ();
pub const __ARCH_WANT_SYS_PAUSE: () = ();
pub const __ARCH_WANT_SYS_SIGNAL: () = ();
pub const __ARCH_WANT_SYS_UTIME: () = ();
pub const __ARCH_WANT_SYS_SOCKETCALL: () = ();
pub const __ARCH_WANT_SYS_IPC: () = ();
pub const __ARCH_WANT_SYS_FADVISE64: () = ();
pub const __ARCH_WANT_SYS_GETPGRP: () = ();
pub const __ARCH_WANT_SYS_NICE: () = ();
pub const __ARCH_WANT_SYS_OLD_GETRLIMIT: () = ();
pub const __ARCH_WANT_SYS_OLD_MMAP: () = ();
pub const __ARCH_WANT_SYS_OLDUMOUNT: () = ();
pub const __ARCH_WANT_SYS_SIGPENDING: () = ();
pub const __ARCH_WANT_SYS_SIGPROCMASK: () = ();
pub const __ARCH_WANT_SYS_FORK: () = ();
pub const __ARCH_WANT_SYS_VFORK: () = ();
pub const __ARCH_WANT_SYS_CLONE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
