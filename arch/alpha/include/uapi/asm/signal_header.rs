/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::ffi::c_void;

/* Avoid too many header ordering problems. */
#[repr(C)]
pub struct siginfo {
    _private: [u8; 0],
}

/* Here we must cater to libcs that poke about in kernel headers. */
#[cfg(not(__KERNEL__))]
pub const NSIG: usize = 32;

#[cfg(not(__KERNEL__))]
pub type sigset_t = ::core::ffi::c_ulong;

/*
 * Linux/AXP has different signal numbers that Linux/i386: I'm trying
 * to make it OSF/1 binary compatible, at least for normal binaries.
 */
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGEMT: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGBUS: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGSYS: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGURG: i32 = 16;
pub const SIGSTOP: i32 = 17;
pub const SIGTSTP: i32 = 18;
pub const SIGCONT: i32 = 19;
pub const SIGCHLD: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGIO: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGINFO: i32 = 29;
pub const SIGUSR1: i32 = 30;
pub const SIGUSR2: i32 = 31;

pub const SIGPOLL: i32 = SIGIO;
pub const SIGPWR: i32 = SIGINFO;
pub const SIGIOT: i32 = SIGABRT;

/* These should not be considered constants from userland. */
pub const SIGRTMIN: i32 = 32;
pub const SIGRTMAX: i32 = _NSIG;

pub const SA_ONSTACK: u32 = 0x00000001;
pub const SA_RESTART: u32 = 0x00000002;
pub const SA_NOCLDSTOP: u32 = 0x00000004;
pub const SA_NODEFER: u32 = 0x00000008;
pub const SA_RESETHAND: u32 = 0x00000010;
pub const SA_NOCLDWAIT: u32 = 0x00000020;
pub const SA_SIGINFO: u32 = 0x00000040;

pub const SA_ONESHOT: u32 = SA_RESETHAND;
pub const SA_NOMASK: u32 = SA_NODEFER;

pub const MINSIGSTKSZ: usize = 4096;
pub const SIGSTKSZ: usize = 16384;

pub const SIG_BLOCK: i32 = 1; /* for blocking signals */
pub const SIG_UNBLOCK: i32 = 2; /* for unblocking signals */
pub const SIG_SETMASK: i32 = 3; /* for setting the signal mask */

/* Declarations from <asm-generic/signal-defs.h> are supplied externally. */

#[cfg(not(__KERNEL__))]
pub type __sighandler_t = unsafe extern "C" fn(i32);

#[cfg(not(__KERNEL__))]
#[repr(C)]
pub union sigaction__u {
    pub _sa_handler: Option<__sighandler_t>,
    pub _sa_sigaction: Option<unsafe extern "C" fn(i32, *mut siginfo, *mut c_void)>,
}

#[cfg(not(__KERNEL__))]
#[repr(C)]
pub struct sigaction {
    pub _u: sigaction__u,
    pub sa_mask: sigset_t,
    pub sa_flags: i32,
}

#[cfg(not(__KERNEL__))]
pub type sa_handler = __sighandler_t;

#[cfg(not(__KERNEL__))]
pub type sa_sigaction = unsafe extern "C" fn(i32, *mut siginfo, *mut c_void);

pub type stack_t = sigaltstack;

#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut c_void,
    pub ss_flags: i32,
    pub ss_size: __kernel_size_t,
}

/* sigstack(2) is deprecated, and will be withdrawn in a future version
   of the X/Open CAE Specification.  Use sigaltstack instead.  It is only
   implemented here for OSF/1 compatibility. */
#[repr(C)]
pub struct sigstack {
    pub ss_sp: *mut c_void,
    pub ss_onstack: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
