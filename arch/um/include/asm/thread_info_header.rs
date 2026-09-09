/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/* C header guard: __UM_THREAD_INFO_H */

pub const THREAD_SIZE_ORDER: usize = CONFIG_KERNEL_STACK_ORDER;
pub const THREAD_SIZE: usize = (1usize << CONFIG_KERNEL_STACK_ORDER) * PAGE_SIZE;

/* The following declarations depend on symbols supplied by the included headers. */
#[repr(C)]
pub struct thread_info {
    pub flags: ::core::ffi::c_ulong, /* low level flags */
    pub cpu: u32,                    /* current CPU */
    pub preempt_count: ::core::ffi::c_int, /* 0 => preemptable, <0 => BUG */
}

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {{
        let _ = &$tsk;
        $crate::thread_info {
            flags: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
        }
    }};
}

pub const TIF_SYSCALL_TRACE: usize = 0; /* syscall trace active */
pub const TIF_SIGPENDING: usize = 1; /* signal pending */
pub const TIF_NEED_RESCHED: usize = 2; /* rescheduling necessary */
pub const TIF_NOTIFY_SIGNAL: usize = 3; /* signal notifications exist */
pub const TIF_RESTART_BLOCK: usize = 4;
pub const TIF_MEMDIE: usize = 5; /* is terminating due to OOM killer */
pub const TIF_SYSCALL_AUDIT: usize = 6;
pub const TIF_RESTORE_SIGMASK: usize = 7;
pub const TIF_NOTIFY_RESUME: usize = 8;
pub const TIF_SECCOMP: usize = 9; /* secure computing */
pub const TIF_SINGLESTEP: usize = 10; /* single stepping userspace */
pub const TIF_SYSCALL_TRACEPOINT: usize = 11; /* syscall tracepoint instrumentation */

pub const _TIF_SYSCALL_TRACE: usize = 1usize << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: usize = 1usize << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1usize << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_SIGNAL: usize = 1usize << TIF_NOTIFY_SIGNAL;
pub const _TIF_MEMDIE: usize = 1usize << TIF_MEMDIE;
pub const _TIF_SYSCALL_AUDIT: usize = 1usize << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_RESUME: usize = 1usize << TIF_NOTIFY_RESUME;
pub const _TIF_SECCOMP: usize = 1usize << TIF_SECCOMP;
pub const _TIF_SINGLESTEP: usize = 1usize << TIF_SINGLESTEP;

pub const _TIF_WORK_MASK: usize = _TIF_NEED_RESCHED
    | _TIF_SIGPENDING
    | _TIF_NOTIFY_SIGNAL
    | _TIF_NOTIFY_RESUME;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
