/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGIOT: i32 = 6;
pub const SIGSTKFLT: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGBUS: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGXCPU: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGUSR1: i32 = 16;
pub const SIGUSR2: i32 = 17;
pub const SIGCHLD: i32 = 18;
pub const SIGPWR: i32 = 19;
pub const SIGVTALRM: i32 = 20;
pub const SIGPROF: i32 = 21;
pub const SIGIO: i32 = 22;
pub const SIGPOLL: i32 = SIGIO;
pub const SIGWINCH: i32 = 23;
pub const SIGSTOP: i32 = 24;
pub const SIGTSTP: i32 = 25;
pub const SIGCONT: i32 = 26;
pub const SIGTTIN: i32 = 27;
pub const SIGTTOU: i32 = 28;
pub const SIGURG: i32 = 29;
pub const SIGXFSZ: i32 = 30;
pub const SIGUNUSED: i32 = 31;
pub const SIGSYS: i32 = 31;

/* These should not be considered constants from userland. */
pub const SIGRTMIN: i32 = 32;
pub const SIGRTMAX: i32 = _NSIG;

pub const SA_ONSTACK: u32 = 0x00000001;
pub const SA_RESETHAND: u32 = 0x00000004;
pub const SA_NOCLDSTOP: u32 = 0x00000008;
pub const SA_SIGINFO: u32 = 0x00000010;
pub const SA_NODEFER: u32 = 0x00000020;
pub const SA_RESTART: u32 = 0x00000040;
pub const SA_NOCLDWAIT: u32 = 0x00000080;

pub const SA_NOMASK: u32 = SA_NODEFER;
pub const SA_ONESHOT: u32 = SA_RESETHAND;

pub const MINSIGSTKSZ: usize = 2048;
pub const SIGSTKSZ: usize = 8192;

/* Supplied by <asm-generic/signal-defs.h>. */

pub const _NSIG: i32 = 64;
pub const _NSIG_BPW: usize = core::mem::size_of::<core::ffi::c_ulong>() * 8;
pub const _NSIG_WORDS: usize = (_NSIG as usize) / _NSIG_BPW;

pub type old_sigset_t = core::ffi::c_ulong;

#[repr(C)]
pub struct sigset_t {
    pub sig: [core::ffi::c_ulong; _NSIG_WORDS],
}

/* Avoid too many header ordering problems. */
#[repr(C)]
pub struct siginfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_flags: core::ffi::c_int,
    pub ss_size: usize,
}

pub type stack_t = sigaltstack;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
