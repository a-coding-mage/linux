/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies: asm/types.h, asm/page.h, asm/processor.h,
 * and abi/switch_context.h are supplied by other translated units. */

#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
    pub dump_exec_domain: *mut core::ffi::c_void,
    pub flags: usize,
    pub preempt_count: core::ffi::c_int,
    pub tp_value: usize,
    pub restart_block: restart_block,
    pub regs: *mut pt_regs,
    pub cpu: core::ffi::c_uint,
}

/* Corresponds to INIT_THREAD_INFO(tsk). */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        thread_info {
            task: &mut $tsk,
            dump_exec_domain: core::ptr::null_mut(),
            flags: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            tp_value: 0,
            restart_block: restart_block {
                fn_: do_no_restart_syscall,
            },
            regs: core::ptr::null_mut(),
            cpu: 0,
        }
    };
}

pub const THREAD_SIZE_ORDER: usize = THREAD_SHIFT - PAGE_SHIFT;

#[inline]
pub unsafe fn thread_saved_fp(tsk: *mut task_struct) -> usize {
    (*((*tsk).thread.sp as *mut switch_stack)).r8 as usize
}

#[inline]
pub unsafe fn thread_saved_sp(tsk: *mut task_struct) -> usize {
    (*tsk).thread.sp as usize
}

#[inline]
pub unsafe fn thread_saved_lr(tsk: *mut task_struct) -> usize {
    (*((*tsk).thread.sp as *mut switch_stack)).r15 as usize
}

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    let mut sp: usize;
    core::arch::asm!("mov {0}, sp", out(reg) sp);
    (sp & !(THREAD_SIZE - 1)) as *mut thread_info
}

pub const TIF_SIGPENDING: usize = 0; // signal pending
pub const TIF_NOTIFY_RESUME: usize = 1; // callback before returning to user
pub const TIF_NEED_RESCHED: usize = 2; // rescheduling necessary
pub const TIF_UPROBE: usize = 3; // uprobe breakpoint or singlestep
pub const TIF_SYSCALL_TRACE: usize = 4; // syscall trace active
pub const TIF_SYSCALL_TRACEPOINT: usize = 5; // syscall tracepoint instrumentation
pub const TIF_SYSCALL_AUDIT: usize = 6; // syscall auditing
pub const TIF_NOTIFY_SIGNAL: usize = 7; // signal notifications exist
pub const TIF_POLLING_NRFLAG: usize = 16; // poll_idle() is TIF_NEED_RESCHED
pub const TIF_MEMDIE: usize = 18; // is terminating due to OOM killer
pub const TIF_RESTORE_SIGMASK: usize = 20; // restore signal mask in do_signal()
pub const TIF_SECCOMP: usize = 21; // secure computing

pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_UPROBE: usize = 1 << TIF_UPROBE;
pub const _TIF_POLLING_NRFLAG: usize = 1 << TIF_POLLING_NRFLAG;
pub const _TIF_MEMDIE: usize = 1 << TIF_MEMDIE;
pub const _TIF_RESTORE_SIGMASK: usize = 1 << TIF_RESTORE_SIGMASK;
pub const _TIF_SECCOMP: usize = 1 << TIF_SECCOMP;

pub const _TIF_WORK_MASK: usize =
    _TIF_NEED_RESCHED | _TIF_SIGPENDING | _TIF_NOTIFY_RESUME | _TIF_UPROBE | _TIF_NOTIFY_SIGNAL;
pub const _TIF_SYSCALL_WORK: usize =
    _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SYSCALL_TRACEPOINT | _TIF_SECCOMP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
