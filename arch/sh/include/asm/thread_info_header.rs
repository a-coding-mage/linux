/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from SuperH thread_info.h. */

/* SuperH version
 * Copyright (C) 2002  Niibe Yutaka
 *
 * The copyright of original i386 version is:
 *
 *  Copyright (C) 2002  David Howells (dhowells@redhat.com)
 *  - Incorporating suggestions made by Linus Torvalds and Dave Miller
 */

/* Dependency supplied by the surrounding translation unit: asm/page.h */

/* Page fault error code bits */
pub const FAULT_CODE_WRITE: u32 = 1 << 0; /* write access */
pub const FAULT_CODE_INITIAL: u32 = 1 << 1; /* initial page write */
pub const FAULT_CODE_ITLB: u32 = 1 << 2; /* ITLB miss */
pub const FAULT_CODE_PROT: u32 = 1 << 3; /* protection fault */
pub const FAULT_CODE_USER: u32 = 1 << 4; /* user-mode access */

/* Dependency supplied by the surrounding translation unit: asm/processor.h */
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct, /* main task structure */
    pub flags: libc::c_ulong, /* low level flags */
    pub status: u32, /* thread synchronous flags */
    pub cpu: u32,
    pub preempt_count: libc::c_int, /* 0 => preemptable, <0 => BUG */
    pub previous_sp: libc::c_ulong, /* sp of previous stack in case of nested IRQ stacks */
    pub supervisor_stack: [u8; 0],
}

/* `struct task_struct` is supplied by asm/processor.h. */
pub enum task_struct {}

/* CONFIG_4KSTACKS selects THREAD_SHIFT = 12; otherwise it is 13. */
#[cfg(CONFIG_4KSTACKS)]
pub const THREAD_SHIFT: u32 = 12;
#[cfg(not(CONFIG_4KSTACKS))]
pub const THREAD_SHIFT: u32 = 13;

pub const THREAD_SIZE: usize = 1usize << THREAD_SHIFT;
pub const STACK_WARN: usize = THREAD_SIZE >> 3;

/* macros/functions for gaining access to the thread information structure */
pub const fn init_thread_info(tsk: *mut task_struct) -> thread_info {
    thread_info {
        task: tsk,
        flags: 0,
        status: 0,
        cpu: 0,
        preempt_count: INIT_PREEMPT_COUNT,
        previous_sp: 0,
        supervisor_stack: [],
    }
}

/* Dependency supplied by the surrounding translation unit. */
extern "C" {
    pub static mut current_stack_pointer: libc::c_ulong;
    pub fn init_thread_xstate();
}

/* PAGE_SHIFT is supplied by asm/page.h. */
pub const THREAD_SIZE_ORDER: u32 = THREAD_SHIFT - PAGE_SHIFT;

/* The C implementation obtains r15 (or r7_bank) using SuperH inline assembly. */
#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    #[cfg(CONFIG_CPU_HAS_SR_RB)]
    {
        /* `stc r7_bank, ti` is architecture-specific and supplied by the SH backend. */
        unimplemented!("SuperH r7_bank inline assembly")
    }
    #[cfg(not(CONFIG_CPU_HAS_SR_RB))]
    {
        let sp = current_stack_pointer;
        (sp & (!(THREAD_SIZE - 1))) as *mut thread_info
    }
}

/* Thread information flags. */
pub const TIF_SYSCALL_TRACE: u32 = 0; /* syscall trace active */
pub const TIF_SIGPENDING: u32 = 1; /* signal pending */
pub const TIF_NEED_RESCHED: u32 = 2; /* rescheduling necessary */
pub const TIF_NOTIFY_SIGNAL: u32 = 3; /* signal notifications exist */
pub const TIF_SINGLESTEP: u32 = 4; /* singlestepping active */
pub const TIF_SYSCALL_AUDIT: u32 = 5; /* syscall auditing active */
pub const TIF_SECCOMP: u32 = 6; /* secure computing */
pub const TIF_NOTIFY_RESUME: u32 = 7; /* callback before returning to user */
pub const TIF_SYSCALL_TRACEPOINT: u32 = 8; /* for ftrace syscall instrumentation */
pub const TIF_POLLING_NRFLAG: u32 = 17; /* true if poll_idle() is polling TIF_NEED_RESCHED */
pub const TIF_MEMDIE: u32 = 18; /* is terminating due to OOM killer */

pub const _TIF_SYSCALL_TRACE: u32 = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: u32 = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1 << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_SIGNAL: u32 = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_SINGLESTEP: u32 = 1 << TIF_SINGLESTEP;
pub const _TIF_SYSCALL_AUDIT: u32 = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SECCOMP: u32 = 1 << TIF_SECCOMP;
pub const _TIF_NOTIFY_RESUME: u32 = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SYSCALL_TRACEPOINT: u32 = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_POLLING_NRFLAG: u32 = 1 << TIF_POLLING_NRFLAG;

/* work to do in syscall trace */
pub const _TIF_WORK_SYSCALL_MASK: u32 = _TIF_SYSCALL_TRACE | _TIF_SINGLESTEP | _TIF_SYSCALL_AUDIT | _TIF_SECCOMP | _TIF_SYSCALL_TRACEPOINT;
/* work to do on any return to u-space */
pub const _TIF_ALLWORK_MASK: u32 = _TIF_SYSCALL_TRACE | _TIF_SIGPENDING | _TIF_NEED_RESCHED | _TIF_SYSCALL_AUDIT | _TIF_SINGLESTEP | _TIF_NOTIFY_RESUME | _TIF_SYSCALL_TRACEPOINT | _TIF_NOTIFY_SIGNAL;
/* work to do on interrupt/exception return */
pub const _TIF_WORK_MASK: u32 = _TIF_ALLWORK_MASK & !(_TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SINGLESTEP);

/* Thread-synchronous status. */
pub const TS_USEDFPU: u32 = 0x0002; /* FPU used by this task this quantum */

pub const TI_FLAG_FAULT_CODE_SHIFT: u32 = 24;

#[inline]
pub unsafe fn set_thread_fault_code(val: u32) {
    let ti = current_thread_info();
    (*ti).flags = ((*ti).flags & (!0usize >> (32 - TI_FLAG_FAULT_CODE_SHIFT)))
        | ((val as libc::c_ulong) << TI_FLAG_FAULT_CODE_SHIFT);
}

#[inline]
pub unsafe fn get_thread_fault_code() -> u32 {
    let ti = current_thread_info();
    ((*ti).flags >> TI_FLAG_FAULT_CODE_SHIFT) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
