/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the generic UAPI signal header.
// `__BITS_PER_LONG` and the included Linux/architecture types are supplied by
// other translated headers.
pub const _NSIG: usize = 64;
pub const _NSIG_BPW: usize = __BITS_PER_LONG;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

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

// These should not be considered constants from userland.
pub const SIGRTMIN: i32 = 32;
// In C, SIGRTMAX is defined only when it has not already been supplied by the
// build environment; preserve that conditional intent for the containing build.
#[cfg(not(feature = "SIGRTMAX"))]
pub const SIGRTMAX: usize = _NSIG;

// In C, these are defined only when either value is absent from the build.
#[cfg(any(feature = "MINSIGSTKSZ", feature = "SIGSTKSZ"))]
#[allow(dead_code)]
const _SIGNAL_STACK_SIZE_CONSTANTS_SUPPLIED_EXTERNALLY: () = ();
#[cfg(not(any(feature = "MINSIGSTKSZ", feature = "SIGSTKSZ")))]
pub const MINSIGSTKSZ: usize = 2048;
#[cfg(not(any(feature = "MINSIGSTKSZ", feature = "SIGSTKSZ")))]
pub const SIGSTKSZ: usize = 8192;

#[repr(C)]
pub struct sigset_t {
    pub sig: [::core::ffi::c_ulong; _NSIG_WORDS],
}

// Not actually used, but required for linux/syscalls.h.
pub type old_sigset_t = ::core::ffi::c_ulong;

// The asm-generic signal definitions are supplied by the corresponding
// translated dependency.  If SA_RESTORER is defined by that dependency:
#[cfg(feature = "SA_RESTORER")]
pub const __ARCH_HAS_SA_RESTORER: bool = true;

// `__KERNEL__` excludes this declaration in kernel builds.
#[cfg(not(feature = "__KERNEL__"))]
#[repr(C)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: ::core::ffi::c_ulong,
    #[cfg(feature = "SA_RESTORER")]
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t, // mask last for extensibility
}

#[repr(C)]
pub struct sigaltstack {
    pub ss_sp: *mut ::core::ffi::c_void,
    pub ss_flags: ::core::ffi::c_int,
    pub ss_size: __kernel_size_t,
}

pub type stack_t = sigaltstack;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
