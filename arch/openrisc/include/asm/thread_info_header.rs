/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others. All original copyrights apply as per the original source
 * declaration.
 */

/* C header guard and __KERNEL__ conditional are preserved as source intent. */

/* THREAD_SIZE is the size of the task_struct/kernel_stack combination. */
pub const THREAD_SIZE_ORDER: usize = 0;
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

/*
 * Low-level task data that entry.S needs immediate access to. This structure
 * must fit entirely inside one cache line and shares the supervisor stack
 * pages; assembly constants must be updated if it changes.
 */
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
    pub flags: ::core::ffi::c_ulong,
    pub cpu: __u32,
    pub preempt_count: __s32,
    pub supervisor_stack: [__u8; 0],
    pub ksp: ::core::ffi::c_ulong,
}

/*
 * Macros/functions for gaining access to the thread information structure.
 * preempt_count needs to be 1 initially, until the scheduler is functional.
 */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: &mut $tsk as *mut _,
            flags: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            supervisor_stack: [],
            ksp: 0,
        }
    };
}

/* The C declaration binds this register variable to r10. */
unsafe extern "C" {
    pub static mut current_thread_info_reg: *mut thread_info;
}

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    // current_thread_info_reg is a register variable in the original target ABI.
    unsafe { current_thread_info_reg }
}

unsafe extern "C" {
    pub fn get_task_struct(task: *mut task_struct);
    pub fn put_task_struct(task: *mut task_struct);
}

#[inline]
pub unsafe fn get_thread_info(ti: *mut thread_info) {
    unsafe { get_task_struct((*ti).task) };
}

#[inline]
pub unsafe fn put_thread_info(ti: *mut thread_info) {
    unsafe { put_task_struct((*ti).task) };
}

/* Thread information flags. Pending work-to-be-done flags are in the LSW. */
pub const TIF_SYSCALL_TRACE: usize = 0;
pub const TIF_NOTIFY_RESUME: usize = 1;
pub const TIF_SIGPENDING: usize = 2;
pub const TIF_NEED_RESCHED: usize = 3;
pub const TIF_SINGLESTEP: usize = 4;
pub const TIF_NOTIFY_SIGNAL: usize = 5;
pub const TIF_SYSCALL_TRACEPOINT: usize = 8;
pub const TIF_RESTORE_SIGMASK: usize = 9;
pub const TIF_POLLING_NRFLAG: usize = 16;
pub const TIF_MEMDIE: usize = 17;

pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_SINGLESTEP: usize = 1 << TIF_SINGLESTEP;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_POLLING_NRFLAG: usize = 1 << TIF_POLLING_NRFLAG;

/* Work to do when returning from interrupt/exception. */
pub const _TIF_WORK_MASK: usize =
    0xff & !(_TIF_SYSCALL_TRACE | _TIF_SINGLESTEP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
