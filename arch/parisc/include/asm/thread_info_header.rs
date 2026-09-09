/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the PA-RISC thread_info header. */

#[repr(C)]
pub struct thread_info {
    /* thread_info flags (see TIF_*) */
    pub flags: ::core::ffi::c_ulong,
    /* 0=premptable, <0=BUG; will also serve as bh-counter */
    pub preempt_count: ::core::ffi::c_int,
    #[cfg(feature = "CONFIG_SMP")]
    pub cpu: ::core::ffi::c_uint,
}

/* INIT_PREEMPT_COUNT is supplied by the corresponding dependency. */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            flags: 0,
            preempt_count: INIT_PREEMPT_COUNT,
        }
    };
}

/* Build-time configuration conditions from the C header are represented by
 * Cargo configuration features. */
#[cfg(feature = "CONFIG_IRQSTACKS")]
pub const THREAD_SIZE_ORDER: u32 = 2; /* PA-RISC requires at least 16k stack */
#[cfg(not(feature = "CONFIG_IRQSTACKS"))]
pub const THREAD_SIZE_ORDER: u32 = 3; /* PA-RISC requires at least 32k stack */

/* Be sure to hunt all references to this down when you change the size of
 * the kernel stack */
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;
pub const THREAD_SHIFT: usize = PAGE_SHIFT + THREAD_SIZE_ORDER;

/* thread information flags */
pub const TIF_SYSCALL_TRACE: u32 = 0; /* syscall trace active */
pub const TIF_SIGPENDING: u32 = 1; /* signal pending */
pub const TIF_NEED_RESCHED: u32 = 2; /* rescheduling necessary */
pub const TIF_POLLING_NRFLAG: u32 = 3; /* true if poll_idle() is polling TIF_NEED_RESCHED */
pub const TIF_32BIT: u32 = 4; /* 32 bit binary */
pub const TIF_MEMDIE: u32 = 5; /* is terminating due to OOM killer */
pub const TIF_NOTIFY_SIGNAL: u32 = 6; /* signal notifications exist */
pub const TIF_SYSCALL_AUDIT: u32 = 7; /* syscall auditing active */
pub const TIF_NOTIFY_RESUME: u32 = 8; /* callback before returning to user */
pub const TIF_SINGLESTEP: u32 = 9; /* single stepping? */
pub const TIF_BLOCKSTEP: u32 = 10; /* branch stepping? */
pub const TIF_SECCOMP: u32 = 11; /* secure computing */
pub const TIF_SYSCALL_TRACEPOINT: u32 = 12; /* syscall tracepoint instrumentation */
pub const TIF_NONBLOCK_WARNING: u32 = 13; /* warned about wrong O_NONBLOCK usage */

pub const _TIF_SYSCALL_TRACE: usize = 1usize << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: usize = 1usize << TIF_SIGPENDING;
pub const _TIF_NOTIFY_SIGNAL: usize = 1usize << TIF_NOTIFY_SIGNAL;
pub const _TIF_NEED_RESCHED: usize = 1usize << TIF_NEED_RESCHED;
pub const _TIF_POLLING_NRFLAG: usize = 1usize << TIF_POLLING_NRFLAG;
pub const _TIF_32BIT: usize = 1usize << TIF_32BIT;
pub const _TIF_SYSCALL_AUDIT: usize = 1usize << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_RESUME: usize = 1usize << TIF_NOTIFY_RESUME;
pub const _TIF_SINGLESTEP: usize = 1usize << TIF_SINGLESTEP;
pub const _TIF_BLOCKSTEP: usize = 1usize << TIF_BLOCKSTEP;
pub const _TIF_SECCOMP: usize = 1usize << TIF_SECCOMP;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1usize << TIF_SYSCALL_TRACEPOINT;

pub const _TIF_USER_WORK_MASK: usize = _TIF_SIGPENDING | _TIF_NOTIFY_RESUME |
    _TIF_NEED_RESCHED | _TIF_NOTIFY_SIGNAL;
pub const _TIF_SYSCALL_TRACE_MASK: usize = _TIF_SYSCALL_TRACE | _TIF_SINGLESTEP |
    _TIF_BLOCKSTEP | _TIF_SYSCALL_AUDIT | _TIF_SECCOMP | _TIF_SYSCALL_TRACEPOINT;

#[cfg(all(feature = "CONFIG_64BIT", feature = "CONFIG_COMPAT"))]
#[macro_export]
macro_rules! is_32bit_task { () => { test_thread_flag(TIF_32BIT) }; }

#[cfg(all(feature = "CONFIG_64BIT", not(feature = "CONFIG_COMPAT")))]
#[macro_export]
macro_rules! is_32bit_task { () => { 0 }; }

#[cfg(not(feature = "CONFIG_64BIT"))]
#[macro_export]
macro_rules! is_32bit_task { () => { 1 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
