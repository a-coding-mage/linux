/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: linux/types.h supplies the kernel ABI types used by this header.

pub const _NSIG: usize = 64;

#[cfg(target_pointer_width = "64")]
pub const _NSIG_BPW: usize = 64;
#[cfg(not(target_pointer_width = "64"))]
pub const _NSIG_BPW: usize = 32;

pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

pub type old_sigset_t = ::core::primitive::c_ulong;

#[repr(C)]
pub struct sigset_t {
	pub sig: [::core::primitive::c_ulong; _NSIG_WORDS],
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
pub const SIGPWR: i32 = 30;
pub const SIGSYS: i32 = 31;
pub const SIGUNUSED: i32 = 31;

/* These should not be considered constants from userland. */
pub const SIGRTMIN: i32 = 32;
pub const SIGRTMAX: usize = _NSIG;

pub const SA_RESTORER: u32 = 0x04000000;

#[cfg(target_pointer_width = "64")]
pub const MINSIGSTKSZ: usize = 8192;
#[cfg(target_pointer_width = "64")]
pub const SIGSTKSZ: usize = 32768;
#[cfg(not(target_pointer_width = "64"))]
pub const MINSIGSTKSZ: usize = 2048;
#[cfg(not(target_pointer_width = "64"))]
pub const SIGSTKSZ: usize = 8192;

// Dependency: asm-generic/signal-defs.h supplies additional signal definitions.

#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct old_sigaction {
	pub sa_handler: __sighandler_t,
	pub sa_mask: old_sigset_t,
	pub sa_flags: ::core::primitive::c_ulong,
	pub sa_restorer: __sigrestore_t,
}

#[cfg(not(feature = "kernel"))]
#[repr(C)]
pub struct sigaction {
	pub sa_handler: __sighandler_t,
	pub sa_flags: ::core::primitive::c_ulong,
	pub sa_restorer: __sigrestore_t,
	pub sa_mask: sigset_t, /* mask last for extensibility */
}

#[repr(C)]
pub struct stack_t {
	pub ss_sp: *mut ::core::ffi::c_void,
	pub ss_flags: ::core::ffi::c_int,
	pub ss_size: __kernel_size_t,
}

#[cfg(not(target_pointer_width = "64"))]
#[repr(C)]
pub struct sig_dbg_op {
	pub dbg_type: ::core::ffi::c_int,
	pub dbg_value: ::core::primitive::c_ulong,
}

#[cfg(not(target_pointer_width = "64"))]
pub const SIG_DBG_SINGLE_STEPPING: i32 = 1;

#[cfg(not(target_pointer_width = "64"))]
pub const SIG_DBG_BRANCH_TRACING: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
