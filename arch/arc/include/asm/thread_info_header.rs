/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Vineetg: Oct 2009
 *  No need for ARC specific thread_info allocator (kmalloc/free). This is
 *  anyways one page allocation, thus slab alloc can be short-circuited and
 *  the generic version (get_free_page) would be loads better.
 *
 * Sameer Dhavale: Codito Technologies 2004
 */

// Dependency supplied by asm/page.h.

#[cfg(feature = "CONFIG_16KSTACKS")]
pub const THREAD_SIZE_ORDER: usize = 1;
#[cfg(not(feature = "CONFIG_16KSTACKS"))]
pub const THREAD_SIZE_ORDER: usize = 0;

pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;
pub const THREAD_SHIFT: usize = PAGE_SHIFT << THREAD_SIZE_ORDER;

// Dependency supplied by linux/thread_info.h.

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - this struct shares the supervisor stack pages
 * - if the contents of this structure are changed, the assembly constants
 *   must also be changed
 */
#[repr(C)]
pub struct thread_info {
    pub flags: ::core::ffi::c_ulong,       /* low level flags */
    pub ksp: ::core::ffi::c_ulong,         /* kernel mode stack top in __switch_to */
    pub preempt_count: ::core::ffi::c_int, /* 0 => preemptible, <0 => BUG */
    pub cpu: ::core::ffi::c_int,           /* current CPU */
    pub thr_ptr: ::core::ffi::c_ulong,     /* TLS ptr */
    pub task: *mut task_struct,            /* main task structure */
}

/*
 * initilaize thread_info for any @tsk
 *  - this is not related to init_task per se
 */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: ::core::ptr::addr_of_mut!($tsk),
            flags: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            ksp: 0,
            thr_ptr: 0,
        }
    };
}

// Opaque type supplied by the kernel task definitions.
pub enum task_struct {}

pub unsafe fn current_thread_info() -> *mut thread_info {
    // C uses an ARC register variable: register unsigned long sp asm("sp").
    // This inline assembly preserves the architecture-specific stack-pointer access.
    let sp: usize;
    ::core::arch::asm!("", out("sp") sp);
    (sp & !(THREAD_SIZE - 1)) as *mut thread_info
}

/*
 * thread information flags
 * - these are process state flags that various assembly files may need to
 *   access
 * - pending work-to-be-done flags are in LSW
 * - other flags in MSW
 */
pub const TIF_RESTORE_SIGMASK: usize = 0; /* restore sig mask in do_signal() */
pub const TIF_NOTIFY_RESUME: usize = 1; /* resumption notification requested */
pub const TIF_SIGPENDING: usize = 2; /* signal pending */
pub const TIF_NEED_RESCHED: usize = 3; /* rescheduling necessary */
pub const TIF_SYSCALL_AUDIT: usize = 4; /* syscall auditing active */
pub const TIF_NOTIFY_SIGNAL: usize = 5; /* signal notifications exist */
pub const TIF_SYSCALL_TRACE: usize = 15; /* syscall trace active */
/* true if poll_idle() is polling TIF_NEED_RESCHED */
pub const TIF_MEMDIE: usize = 16;
pub const TIF_SYSCALL_TRACEPOINT: usize = 17; /* syscall tracepoint instrumentation */

pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_MEMDIE: usize = 1 << TIF_MEMDIE;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;

/* work to do on interrupt/exception return */
pub const _TIF_WORK_MASK: usize =
    _TIF_NEED_RESCHED | _TIF_SIGPENDING | _TIF_NOTIFY_RESUME | _TIF_NOTIFY_SIGNAL;

pub const _TIF_SYSCALL_WORK: usize = _TIF_SYSCALL_TRACE | _TIF_SYSCALL_TRACEPOINT;

/*
 * _TIF_ALLWORK_MASK includes SYSCALL_TRACE, but we don't need it.
 * SYSCALL_TRACE is anyway separately/unconditionally tested right after a
 * syscall, so all that remains to be tested is _TIF_WORK_MASK
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
