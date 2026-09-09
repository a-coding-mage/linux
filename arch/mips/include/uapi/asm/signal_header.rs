/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2003 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

pub const _NSIG: usize = 128;
pub const _NSIG_BPW: usize = core::mem::size_of::<usize>() * 8;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigset_t {
    pub sig: [usize; _NSIG_WORDS],
}

pub type old_sigset_t = usize; /* at least 32 bits */

pub const SIGHUP: i32 = 1; /* Hangup (POSIX).  */
pub const SIGINT: i32 = 2; /* Interrupt (ANSI).  */
pub const SIGQUIT: i32 = 3; /* Quit (POSIX).  */
pub const SIGILL: i32 = 4; /* Illegal instruction (ANSI).  */
pub const SIGTRAP: i32 = 5; /* Trace trap (POSIX).  */
pub const SIGIOT: i32 = 6; /* IOT trap (4.2 BSD).  */
pub const SIGABRT: i32 = SIGIOT; /* Abort (ANSI).  */
pub const SIGEMT: i32 = 7;
pub const SIGFPE: i32 = 8; /* Floating-point exception (ANSI).  */
pub const SIGKILL: i32 = 9; /* Kill, unblockable (POSIX).  */
pub const SIGBUS: i32 = 10; /* BUS error (4.2 BSD).  */
pub const SIGSEGV: i32 = 11; /* Segmentation violation (ANSI).  */
pub const SIGSYS: i32 = 12;
pub const SIGPIPE: i32 = 13; /* Broken pipe (POSIX).  */
pub const SIGALRM: i32 = 14; /* Alarm clock (POSIX).  */
pub const SIGTERM: i32 = 15; /* Termination (ANSI). */
pub const SIGUSR1: i32 = 16; /* User-defined signal 1 (POSIX).  */
pub const SIGUSR2: i32 = 17; /* User-defined signal 2 (POSIX).  */
pub const SIGCHLD: i32 = 18; /* Child status has changed (POSIX).  */
pub const SIGCLD: i32 = SIGCHLD; /* Same as SIGCHLD (System V).  */
pub const SIGPWR: i32 = 19; /* Power failure restart (System V).  */
pub const SIGWINCH: i32 = 20; /* Window size change (4.3 BSD, Sun).  */
pub const SIGURG: i32 = 21; /* Urgent condition on socket (4.2 BSD).  */
pub const SIGIO: i32 = 22; /* I/O now possible (4.2 BSD). */
pub const SIGPOLL: i32 = SIGIO; /* Pollable event occurred (System V). */
pub const SIGSTOP: i32 = 23; /* Stop, unblockable (POSIX).  */
pub const SIGTSTP: i32 = 24; /* Keyboard stop (POSIX).  */
pub const SIGCONT: i32 = 25; /* Continue (POSIX).  */
pub const SIGTTIN: i32 = 26; /* Background read from tty (POSIX).  */
pub const SIGTTOU: i32 = 27; /* Background write from tty (POSIX).  */
pub const SIGVTALRM: i32 = 28; /* Virtual alarm clock (4.2 BSD).  */
pub const SIGPROF: i32 = 29; /* Profiling alarm clock (4.2 BSD).  */
pub const SIGXCPU: i32 = 30; /* CPU limit exceeded (4.2 BSD).  */
pub const SIGXFSZ: i32 = 31; /* File size limit exceeded (4.2 BSD). */

/* These should not be considered constants from userland.  */
pub const SIGRTMIN: i32 = 32;
pub const SIGRTMAX: usize = _NSIG;

/*
 * SA_RESTORER used to be defined as 0x04000000 but only the O32 ABI ever
 * supported its use and no libc was using it, so the entire sa-restorer
 * functionality was removed with lmo commit 39bffc12c3580ab for 2.5.48
 * retaining only the SA_RESTORER definition as a reminder to avoid
 * accidental reuse of the mask bit.
 */
pub const SA_ONSTACK: u32 = 0x08000000;
pub const SA_RESETHAND: u32 = 0x80000000;
pub const SA_RESTART: u32 = 0x10000000;
pub const SA_SIGINFO: u32 = 0x00000008;
pub const SA_NODEFER: u32 = 0x40000000;
pub const SA_NOCLDWAIT: u32 = 0x00010000;
pub const SA_NOCLDSTOP: u32 = 0x00000001;

pub const SA_NOMASK: u32 = SA_NODEFER;
pub const SA_ONESHOT: u32 = SA_RESETHAND;

pub const MINSIGSTKSZ: usize = 2048;
pub const SIGSTKSZ: usize = 8192;

pub const SIG_BLOCK: i32 = 1; /* for blocking signals */
pub const SIG_UNBLOCK: i32 = 2; /* for unblocking signals */
pub const SIG_SETMASK: i32 = 3; /* for setting the signal mask */

/* Definitions from <asm-generic/signal-defs.h> are supplied externally. */

/* The following declaration is present only when __KERNEL__ is not defined. */
#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct sigaction {
    pub sa_flags: u32,
    pub sa_handler: __sighandler_t,
    pub sa_mask: sigset_t,
}

/* IRIX compatible stack_t */
#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_size: usize,
    pub ss_flags: i32,
}

pub type stack_t = sigaltstack;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
