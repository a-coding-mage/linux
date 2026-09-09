/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * SA_FLAGS values:
 *
 * SA_NOCLDSTOP flag to turn off SIGCHLD when children stop.
 * SA_NOCLDWAIT flag on SIGCHLD to inhibit zombies.
 * SA_SIGINFO delivers the signal with SIGINFO structs.
 * SA_ONSTACK indicates that a registered stack_t will be used.
 * SA_RESTART flag to get restarting signals (which were the default long ago)
 * SA_NODEFER prevents the current signal from being masked in the handler.
 * SA_RESETHAND clears the handler when the signal is delivered.
 * SA_UNSUPPORTED is a flag bit that will never be supported. Kernels from
 * before the introduction of SA_UNSUPPORTED did not clear unknown bits from
 * sa_flags when read using the oldact argument to sigaction and rt_sigaction,
 * so this bit allows flag bit support to be detected from userspace while
 * allowing an old kernel to be distinguished from a kernel that supports every
 * flag bit.
 * SA_EXPOSE_TAGBITS exposes an architecture-defined set of tag bits in
 * siginfo.si_addr.
 *
 * SA_ONESHOT and SA_NOMASK are the historical Linux names for the Single
 * Unix names RESETHAND and NODEFER respectively.
 */
pub const SA_NOCLDSTOP: u32 = 0x00000001;
pub const SA_NOCLDWAIT: u32 = 0x00000002;
pub const SA_SIGINFO: u32 = 0x00000004;
/* 0x00000008 used on alpha, mips, parisc */
/* 0x00000010 used on alpha, parisc */
/* 0x00000020 used on alpha, parisc, sparc */
/* 0x00000040 used on alpha, parisc */
/* 0x00000080 used on parisc */
/* 0x00000100 used on sparc */
/* 0x00000200 used on sparc */
pub const SA_UNSUPPORTED: u32 = 0x00000400;
pub const SA_EXPOSE_TAGBITS: u32 = 0x00000800;
/* 0x00010000 used on mips */
/* 0x00800000 used for internal SA_IMMUTABLE */
/* 0x01000000 used on x86 */
/* 0x02000000 used on x86 */
/*
 * New architectures should not define the obsolete
 *     SA_RESTORER 0x04000000
 */
pub const SA_ONSTACK: u32 = 0x08000000;
pub const SA_RESTART: u32 = 0x10000000;
pub const SA_NODEFER: u32 = 0x40000000;
pub const SA_RESETHAND: u32 = 0x80000000;

pub const SA_NOMASK: u32 = SA_NODEFER;
pub const SA_ONESHOT: u32 = SA_RESETHAND;

pub const SIG_BLOCK: i32 = 0; /* for blocking signals */
pub const SIG_UNBLOCK: i32 = 1; /* for unblocking signals */
pub const SIG_SETMASK: i32 = 2; /* for setting the signal mask */

pub type __signalfn_t = unsafe extern "C" fn(i32);
pub type __sighandler_t = Option<__signalfn_t>;

pub type __restorefn_t = unsafe extern "C" fn();
pub type __sigrestore_t = Option<__restorefn_t>;

/* Function-pointer sentinel values corresponding to the C casts. */
pub const SIG_DFL: usize = 0; /* default signal handling */
pub const SIG_IGN: usize = 1; /* ignore signal */
pub const SIG_ERR: usize = usize::MAX; /* error return from signal */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
