/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent to <vdso/bits.h> is supplied externally.

/* Bits 16-31 are reserved for architecture specific purposes */

pub const TIF_NOTIFY_RESUME: usize = 0; // callback before returning to user
pub const _TIF_NOTIFY_RESUME: usize = 1usize << TIF_NOTIFY_RESUME;

pub const TIF_SIGPENDING: usize = 1; // signal pending
pub const _TIF_SIGPENDING: usize = 1usize << TIF_SIGPENDING;

pub const TIF_NOTIFY_SIGNAL: usize = 2; // signal notifications exist
pub const _TIF_NOTIFY_SIGNAL: usize = 1usize << TIF_NOTIFY_SIGNAL;

pub const TIF_MEMDIE: usize = 3; // is terminating due to OOM killer
pub const _TIF_MEMDIE: usize = 1usize << TIF_MEMDIE;

pub const TIF_NEED_RESCHED: usize = 4; // rescheduling necessary
pub const _TIF_NEED_RESCHED: usize = 1usize << TIF_NEED_RESCHED;

// Conditional on HAVE_TIF_NEED_RESCHED_LAZY in the source build.
#[cfg(have_tif_need_resched_lazy)]
pub const TIF_NEED_RESCHED_LAZY: usize = 5; // Lazy rescheduling needed
#[cfg(have_tif_need_resched_lazy)]
pub const _TIF_NEED_RESCHED_LAZY: usize = 1usize << TIF_NEED_RESCHED_LAZY;

// Conditional on HAVE_TIF_POLLING_NRFLAG in the source build.
#[cfg(have_tif_polling_nrflag)]
pub const TIF_POLLING_NRFLAG: usize = 6; // idle is polling for TIF_NEED_RESCHED
#[cfg(have_tif_polling_nrflag)]
pub const _TIF_POLLING_NRFLAG: usize = 1usize << TIF_POLLING_NRFLAG;

pub const TIF_USER_RETURN_NOTIFY: usize = 7; // notify kernel of userspace return
pub const _TIF_USER_RETURN_NOTIFY: usize = 1usize << TIF_USER_RETURN_NOTIFY;

pub const TIF_UPROBE: usize = 8; // breakpointed or singlestepping
pub const _TIF_UPROBE: usize = 1usize << TIF_UPROBE;

pub const TIF_PATCH_PENDING: usize = 9; // pending live patching update
pub const _TIF_PATCH_PENDING: usize = 1usize << TIF_PATCH_PENDING;

// Conditional on HAVE_TIF_RESTORE_SIGMASK in the source build.
#[cfg(have_tif_restore_sigmask)]
pub const TIF_RESTORE_SIGMASK: usize = 10; // Restore signal mask in do_signal()
#[cfg(have_tif_restore_sigmask)]
pub const _TIF_RESTORE_SIGMASK: usize = 1usize << TIF_RESTORE_SIGMASK;

pub const TIF_RSEQ: usize = 11; // Run RSEQ fast path
pub const _TIF_RSEQ: usize = 1usize << TIF_RSEQ;

pub const TIF_HRTIMER_REARM: usize = 12; // re-arm the timer
pub const _TIF_HRTIMER_REARM: usize = 1usize << TIF_HRTIMER_REARM;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
