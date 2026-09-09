/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the surrounding translated headers are intentionally
// referenced here rather than reimplemented.

/* On the Sparc the signal handlers get passed a 'sub-signal' code
 * for certain signal types, which we document here.
 */
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SUBSIG_STACK: u32 = 0;
pub const SUBSIG_ILLINST: u32 = 2;
pub const SUBSIG_PRIVINST: u32 = 3;
#[inline]
pub const fn SUBSIG_BADTRAP(t: u32) -> u32 { 0x80u32.wrapping_add(t) }

pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;

pub const SIGEMT: u32 = 7;
pub const SUBSIG_TAG: u32 = 10;

pub const SIGFPE: u32 = 8;
pub const SUBSIG_FPDISABLED: u32 = 0x400;
pub const SUBSIG_FPERROR: u32 = 0x404;
pub const SUBSIG_FPINTOVFL: u32 = 0x001;
pub const SUBSIG_FPSTSIG: u32 = 0x002;
pub const SUBSIG_IDIVZERO: u32 = 0x014;
pub const SUBSIG_FPINEXACT: u32 = 0x0c4;
pub const SUBSIG_FPDIVZERO: u32 = 0x0c8;
pub const SUBSIG_FPUNFLOW: u32 = 0x0cc;
pub const SUBSIG_FPOPERROR: u32 = 0x0d0;
pub const SUBSIG_FPOVFLOW: u32 = 0x0d4;

pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SUBSIG_BUSTIMEOUT: u32 = 1;
pub const SUBSIG_ALIGNMENT: u32 = 2;
pub const SUBSIG_MISCERROR: u32 = 5;

pub const SIGSEGV: u32 = 11;
pub const SUBSIG_NOMAPPING: u32 = 3;
pub const SUBSIG_PROTECTION: u32 = 4;
pub const SUBSIG_SEGERROR: u32 = 5;

pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;

/* SunOS values which deviate from the Linux/i386 ones */
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGPOLL: u32 = SIGIO;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGLOST: u32 = 29;
pub const SIGPWR: u32 = SIGLOST;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;

/* Most things should be clean enough to redefine this at will.  */
pub const __OLD_NSIG: usize = 32;
pub const __NEW_NSIG: usize = 64;
#[cfg(__arch64__)]
pub const _NSIG_BPW: usize = 64;
#[cfg(not(__arch64__))]
pub const _NSIG_BPW: usize = 32;
pub const _NSIG_WORDS: usize = __NEW_NSIG / _NSIG_BPW;
pub const SIGRTMIN: usize = 32;
pub const SIGRTMAX: usize = __NEW_NSIG;

// The following aliases mirror the C preprocessor branches for kernel and
// POSIX1B builds; the referenced types are provided by dependent headers.
#[cfg(any(feature = "kernel", feature = "want_posix1b_signals"))]
pub const _NSIG: usize = __NEW_NSIG;
#[cfg(not(any(feature = "kernel", feature = "want_posix1b_signals")))]
pub const _NSIG: usize = __OLD_NSIG;
#[cfg(not(any(feature = "kernel", feature = "want_posix1b_signals")))]
pub const NSIG: usize = _NSIG;

pub type __old_sigset_t = libc::c_ulong;

#[repr(C)]
pub struct __new_sigset_t {
    pub sig: [libc::c_ulong; _NSIG_WORDS],
}

/* A SunOS sigstack */
#[repr(C)]
pub struct sigstack {
    /* XXX 32-bit pointers pinhead XXX */
    pub the_stack: *mut libc::c_char,
    pub cur_status: libc::c_int,
}

/* Sigvec flags */
pub const _SV_SSTACK: u32 = 1;
pub const _SV_INTR: u32 = 2;
pub const _SV_RESET: u32 = 4;
pub const _SV_IGNCHILD: u32 = 8;

/*
 * sa_flags values: SA_STACK is not currently supported, but will allow the
 * usage of signal stacks by using the (now obsolete) sa_restorer field in the
 * sigaction structure as a stack pointer. This is now possible due to the
 * changes in signal handling. LBT 010493.
 * SA_RESTART flag to get restarting signals (which were the default long ago)
 */
pub const SA_NOCLDSTOP: u32 = _SV_IGNCHILD;
pub const SA_STACK: u32 = _SV_SSTACK;
pub const SA_ONSTACK: u32 = _SV_SSTACK;
pub const SA_RESTART: u32 = _SV_INTR;
pub const SA_RESETHAND: u32 = _SV_RESET;
pub const SA_NODEFER: u32 = 0x20;
pub const SA_NOCLDWAIT: u32 = 0x100;
pub const SA_SIGINFO: u32 = 0x200;
pub const SIG_BLOCK: u32 = 0x01;
pub const SIG_UNBLOCK: u32 = 0x02;
pub const SIG_SETMASK: u32 = 0x04;
pub const MINSIGSTKSZ: usize = 4096;
pub const SIGSTKSZ: usize = 16384;

// __sighandler_t, __sigrestore_t, __kernel_size_t, and the sigaction types
// are supplied by the translated dependency headers.
#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct __new_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: libc::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: __new_sigset_t,
}

#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct __old_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: __old_sigset_t,
    pub sa_flags: libc::c_ulong,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: __kernel_size_t,
}

pub type stack_t = sigaltstack;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
