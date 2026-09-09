/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/signal.h
 *
 * Swiped from SH.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C header guard: _UAPI_XTENSA_SIGNAL_H

pub const _NSIG: usize = 64;
pub const _NSIG_BPW: usize = 32;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

/* Avoid too many header ordering problems. */
pub struct siginfo;
pub type old_sigset_t = core::ffi::c_ulong; /* at least 32 bits */

#[repr(C)]
pub struct sigset_t {
    pub sig: [core::ffi::c_ulong; _NSIG_WORDS],
}

pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGIOT: i32 = 6;
pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGURG: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGIO: i32 = 29;
pub const SIGPOLL: i32 = SIGIO;
/* #define SIGLOST 29 */
pub const SIGPWR: i32 = 30;
pub const SIGSYS: i32 = 31;
pub const SIGUNUSED: i32 = 31;

/* These should not be considered constants from userland. */
pub const SIGRTMIN: i32 = 32;
pub const SIGRTMAX: i32 = (_NSIG - 1) as i32;

pub const SA_RESTORER: u32 = 0x04000000;

pub const MINSIGSTKSZ: usize = 2048;
pub const SIGSTKSZ: usize = 8192;

/* Contents of <asm-generic/signal-defs.h> are supplied externally. */

/* The __KERNEL__ conditional from the C header is preserved by this comment. */
#[repr(C)]
pub union sigaction_u {
    pub _sa_handler: __sighandler_t,
    pub _sa_sigaction: Option<unsafe extern "C" fn(i32, *mut siginfo, *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct sigaction {
    pub _u: sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: core::ffi::c_ulong,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

/* C field aliases: sa_handler = _u._sa_handler; sa_sigaction = _u._sa_sigaction. */

#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_flags: i32,
    pub ss_size: __kernel_size_t,
}

pub type stack_t = sigaltstack;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
