/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* Translation of the MicroBlaze kernel thread_info header. */

/* we have 8k stack */
pub const THREAD_SHIFT: u32 = 13;
pub const THREAD_SIZE: usize = 1usize << THREAD_SHIFT;
pub const THREAD_SIZE_ORDER: u32 = 1;

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - this struct shares the supervisor stack pages
 * - if the contents of this structure are changed, the assembly constants
 *   must also be changed
 */
#[repr(C)]
pub struct cpu_context {
    pub r1: u32, /* stack pointer */
    pub r2: u32,
    /* dedicated registers */
    pub r13: u32,
    pub r14: u32,
    pub r15: u32,
    pub r16: u32,
    pub r17: u32,
    pub r18: u32,
    /* non-volatile registers */
    pub r19: u32,
    pub r20: u32,
    pub r21: u32,
    pub r22: u32,
    pub r23: u32,
    pub r24: u32,
    pub r25: u32,
    pub r26: u32,
    pub r27: u32,
    pub r28: u32,
    pub r29: u32,
    pub r30: u32,
    /* r31 is used as current task pointer */
    /* special purpose registers */
    pub msr: u32,
    pub ear: u32,
    pub esr: u32,
    pub fsr: u32,
}

#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct, /* main task structure */
    pub flags: libc::c_ulong, /* low level flags */
    pub status: libc::c_ulong, /* thread-synchronous flags */
    pub cpu: u32, /* current CPU */
    pub preempt_count: i32, /* 0 => preemptable,< 0 => BUG*/
    pub cpu_context: cpu_context,
}

/* task_struct and INIT_PREEMPT_COUNT are supplied by other kernel headers. */
extern "C" {
    pub type task_struct;
}

/* macros/functions for gaining access to the thread information structure */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: &mut $tsk as *mut _,
            flags: 0,
            status: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            cpu_context: $crate::cpu_context {
                r1: 0, r2: 0, r13: 0, r14: 0, r15: 0, r16: 0, r17: 0, r18: 0,
                r19: 0, r20: 0, r21: 0, r22: 0, r23: 0, r24: 0, r25: 0,
                r26: 0, r27: 0, r28: 0, r29: 0, r30: 0,
                msr: 0, ear: 0, esr: 0, fsr: 0,
            },
        }
    };
}

/* how to get the thread information struct from C */
#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    /* The C source reads the MicroBlaze r1 register here. */
    let sp: usize;
    core::arch::asm!("", out("r1") sp, options(nomem, nostack, preserves_flags));
    (sp & !(THREAD_SIZE - 1)) as *mut thread_info
}

/* thread information allocation */

/*
 * thread information flags
 * - these are process state flags that various assembly files may need to access
 * - pending work-to-be-done flags are in LSW
 * - other flags in MSW
 */
pub const TIF_SYSCALL_TRACE: u32 = 0; /* syscall trace active */
pub const TIF_NOTIFY_RESUME: u32 = 1; /* resumption notification requested */
pub const TIF_SIGPENDING: u32 = 2; /* signal pending */
pub const TIF_NEED_RESCHED: u32 = 3; /* rescheduling necessary */
/* restore singlestep on return to user mode */
pub const TIF_SINGLESTEP: u32 = 4;
pub const TIF_NOTIFY_SIGNAL: u32 = 5; /* signal notifications exist */
pub const TIF_MEMDIE: u32 = 6; /* is terminating due to OOM killer */
pub const TIF_SYSCALL_AUDIT: u32 = 9; /* syscall auditing active */
pub const TIF_SECCOMP: u32 = 10; /* secure computing */

/* true if poll_idle() is polling TIF_NEED_RESCHED */
pub const TIF_POLLING_NRFLAG: u32 = 16;

pub const _TIF_SYSCALL_TRACE: u32 = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: u32 = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: u32 = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1 << TIF_NEED_RESCHED;
pub const _TIF_SINGLESTEP: u32 = 1 << TIF_SINGLESTEP;
pub const _TIF_NOTIFY_SIGNAL: u32 = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_POLLING_NRFLAG: u32 = 1 << TIF_POLLING_NRFLAG;
pub const _TIF_SYSCALL_AUDIT: u32 = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SECCOMP: u32 = 1 << TIF_SECCOMP;

/* work to do in syscall trace */
pub const _TIF_WORK_SYSCALL_MASK: u32 =
    _TIF_SYSCALL_TRACE | _TIF_SINGLESTEP | _TIF_SYSCALL_AUDIT | _TIF_SECCOMP;

/* work to do on interrupt/exception return */
pub const _TIF_WORK_MASK: u32 = 0x0000FFFE;

/* work to do on any return to u-space */
pub const _TIF_ALLWORK_MASK: u32 = 0x0000FFFF;

/*
 * Thread-synchronous status.
 *
 * This is different from the flags in that nobody else
 * ever touches our thread-synchronous status, so we don't
 * have to worry about atomic accesses.
 */
/* FPU was used by this task this quantum (SMP) */
pub const TS_USEDFPU: u32 = 0x0001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
