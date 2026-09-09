/*
 * NiosII low-level thread information
 *
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * Based on asm/thread_info_no.h from m68k which is:
 *
 * Copyright (C) 2002 David Howells <dhowells@redhat.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* The original declarations are guarded by __KERNEL__. */

/* Size of the kernel stack for each process. */
pub const THREAD_SIZE_ORDER: usize = 1;
pub const THREAD_SIZE: usize = 8192; /* 2 * PAGE_SIZE */

/*
 * Low level task data that entry.S needs immediate access to.
 * This struct should fit entirely inside of one cache line and shares the
 * supervisor stack pages.  Assembly constants must change if this changes.
 */
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
    pub flags: core::ffi::c_ulong,
    pub cpu: u32,
    pub preempt_count: core::ffi::c_int,
    pub regs: *mut pt_regs,
}

/* preempt_count needs to be 1 initially, until the scheduler is functional. */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: (&$tsk as *const _ as *mut task_struct),
            flags: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            regs: core::ptr::null_mut(),
        }
    };
}

/* How to get the thread information struct from C. */
pub unsafe fn current_thread_info() -> *mut thread_info {
    let sp: core::ffi::c_ulong;
    core::arch::asm!("mov {0}, sp", out(reg) sp);
    (sp & !(THREAD_SIZE as core::ffi::c_ulong - 1)) as *mut thread_info
}

/*
 * Thread information flags.
 * Pending work-to-be-done flags are in the LSW; other flags are in the MSW.
 */
pub const TIF_SYSCALL_TRACE: u32 = 0; /* syscall trace active */
pub const TIF_NOTIFY_RESUME: u32 = 1; /* resumption notification requested */
pub const TIF_SIGPENDING: u32 = 2; /* signal pending */
pub const TIF_NEED_RESCHED: u32 = 3; /* rescheduling necessary */
pub const TIF_MEMDIE: u32 = 4; /* is terminating due to OOM killer */
pub const TIF_SECCOMP: u32 = 5; /* secure computing */
pub const TIF_SYSCALL_AUDIT: u32 = 6; /* syscall auditing active */
pub const TIF_NOTIFY_SIGNAL: u32 = 7; /* signal notifications exist */
pub const TIF_RESTORE_SIGMASK: u32 = 9; /* restore signal mask in do_signal() */
pub const TIF_POLLING_NRFLAG: u32 = 16; /* true if poll_idle() is polling
                                            TIF_NEED_RESCHED */

pub const _TIF_SYSCALL_TRACE: u32 = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: u32 = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: u32 = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1 << TIF_NEED_RESCHED;
pub const _TIF_SECCOMP: u32 = 1 << TIF_SECCOMP;
pub const _TIF_SYSCALL_AUDIT: u32 = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_SIGNAL: u32 = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_RESTORE_SIGMASK: u32 = 1 << TIF_RESTORE_SIGMASK;
pub const _TIF_POLLING_NRFLAG: u32 = 1 << TIF_POLLING_NRFLAG;

/* Work to do on interrupt/exception return. */
pub const _TIF_WORK_MASK: u32 = 0x0000FFFE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
