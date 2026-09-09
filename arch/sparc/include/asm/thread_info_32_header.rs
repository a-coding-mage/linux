/* SPDX-License-Identifier: GPL-2.0 */
/*
 * thread_info.h: sparc low-level thread information
 * adapted from the ppc version by Pete Zaitcev, which was
 * adapted from the i386 version by Paul Mackerras
 *
 * Copyright (C) 2002  David Howells (dhowells@redhat.com)
 * Copyright (c) 2002  Pete Zaitcev (zaitcev@yahoo.com)
 * - Incorporating suggestions made by Linus Torvalds and Dave Miller
 */

/* C header guards and __KERNEL__/__ASSEMBLER__ conditionals are preserved by
 * the surrounding build configuration. */

/* External types and constants are supplied by the corresponding dependencies. */

pub const NSWINS: usize = 8;

/*
 * Low level task data.
 *
 * If you change this, change the TI_* offsets below to match.
 */
#[repr(C)]
pub struct thread_info {
    pub uwinmask: usize,
    pub task: *mut task_struct,
    pub flags: usize,
    pub cpu: i32,
    pub preempt_count: i32,
    pub softirq_count: i32,
    pub hardirq_count: i32,
    pub __unused: u32,

    /* Context switch saved kernel state. */
    pub ksp: usize,
    pub kpc: usize,
    pub kpsr: usize,
    pub kwim: usize,

    /* A place to store user windows and stack pointers
     * when the stack needs inspection.
     */
    pub reg_window: [reg_window32; NSWINS],
    pub rwbuf_stkptrs: [usize; NSWINS],
    pub w_saved: usize,
}

/* macros/functions for gaining access to the thread information structure */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        thread_info {
            uwinmask: 0,
            task: &mut $tsk as *mut _,
            flags: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            softirq_count: 0,
            hardirq_count: 0,
            __unused: 0,
            ksp: 0,
            kpc: 0,
            kpsr: 0,
            kwim: 0,
            reg_window: [unsafe { core::mem::zeroed() }; NSWINS],
            rwbuf_stkptrs: [0; NSWINS],
            w_saved: 0,
        }
    };
}

/* how to get the thread information struct from C */
pub static mut current_thread_info_reg: *mut thread_info = core::ptr::null_mut();

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    current_thread_info_reg
}

/* thread information allocation */
pub const THREAD_SIZE_ORDER: usize = 1;

/* Size of kernel stack for each process */
pub const THREAD_SIZE: usize = 2 * PAGE_SIZE;

/* Offsets in thread_info structure, used in assembly code */
pub const TI_UWINMASK: usize = 0x00;
pub const TI_TASK: usize = 0x04;
pub const TI_FLAGS: usize = 0x08;
pub const TI_CPU: usize = 0x0c;
pub const TI_PREEMPT: usize = 0x10;
pub const TI_SOFTIRQ: usize = 0x14;
pub const TI_HARDIRQ: usize = 0x18;
pub const TI_KSP: usize = 0x20;
pub const TI_KPC: usize = 0x24;
pub const TI_KPSR: usize = 0x28;
pub const TI_KWIM: usize = 0x2c;
pub const TI_REG_WINDOW: usize = 0x30;
pub const TI_RWIN_SPTRS: usize = 0x230;
pub const TI_W_SAVED: usize = 0x250;

/* thread information flag bit numbers */
pub const TIF_SYSCALL_TRACE: usize = 0;
pub const TIF_NOTIFY_RESUME: usize = 1;
pub const TIF_SIGPENDING: usize = 2;
pub const TIF_NEED_RESCHED: usize = 3;
pub const TIF_RESTORE_SIGMASK: usize = 4;
pub const TIF_NOTIFY_SIGNAL: usize = 5;
pub const TIF_USEDFPU: usize = 8;
pub const TIF_POLLING_NRFLAG: usize = 9;
pub const TIF_MEMDIE: usize = 10;

/* as above, but as bit values */
pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_USEDFPU: usize = 1 << TIF_USEDFPU;
pub const _TIF_POLLING_NRFLAG: usize = 1 << TIF_POLLING_NRFLAG;

pub const _TIF_DO_NOTIFY_RESUME_MASK: usize =
    _TIF_NOTIFY_RESUME | _TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL;

#[inline]
pub const fn is_32bit_task() -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
