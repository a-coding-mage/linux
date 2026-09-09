/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Thread support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* C header dependencies are supplied by the surrounding translation unit. */

pub const THREAD_SHIFT: usize = 12;
pub const THREAD_SIZE: usize = 1usize << THREAD_SHIFT;
/* PAGE_SHIFT is supplied by asm/page.h. */
pub const THREAD_SIZE_ORDER: usize = THREAD_SHIFT - PAGE_SHIFT;

/* Opaque declarations supplied by other headers. */
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/*
 * This is union'd with the "bottom" of the kernel stack.
 * It keeps track of thread info which is handy for routines
 * to access quickly.
 */
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,       /* main task structure */
    pub flags: libc::c_ulong,         /* low level flags */
    pub cpu: u32,                     /* current cpu */
    pub preempt_count: libc::c_int,   /* 0=>preemptible,<0=>BUG */
    /*
     * used for syscalls somehow;
     * seems to have a function pointer and four arguments
     */
    /* Points to the current pt_regs frame  */
    pub regs: *mut pt_regs,
    /*
     * saved kernel sp at switch_to time;
     * not sure if this is used (it's not in the VM model it seems;
     * see thread_struct)
     */
    pub sp: libc::c_ulong,
}

/* PAGE_SHIFT is an external build-time dependency from asm/page.h. */

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: &mut $tsk as *mut _,
            flags: 0,
            cpu: 0,
            preempt_count: 1,
            sp: 0,
            regs: core::ptr::null_mut(),
        }
    };
}

/* Tacky preprocessor trickery; the stringized register name is build-defined. */
#[macro_export]
macro_rules! qqstr {
    ($s:ident) => { stringify!($s) };
}
#[macro_export]
macro_rules! qstr {
    ($s:ident) => { stringify!($s) };
}

/* The C declaration uses an architecture-specific register constraint. */
extern "C" {
    pub static mut __current_thread_info: *mut thread_info;
}

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    __current_thread_info
}

/*
 * thread information flags
 * - these are process state flags that various assembly files
 *   may need to access
 * - pending work-to-be-done flags are in LSW
 * - other flags in MSW
 */
pub const TIF_SYSCALL_TRACE: u32 = 0;
pub const TIF_NOTIFY_RESUME: u32 = 1;
pub const TIF_SIGPENDING: u32 = 2;
pub const TIF_NEED_RESCHED: u32 = 3;
pub const TIF_SINGLESTEP: u32 = 4;
pub const TIF_RESTORE_SIGMASK: u32 = 6;
pub const TIF_NOTIFY_SIGNAL: u32 = 7;
/* true if poll_idle() is polling TIF_NEED_RESCHED */
pub const TIF_MEMDIE: u32 = 17;

pub const _TIF_SYSCALL_TRACE: u32 = 1u32 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: u32 = 1u32 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: u32 = 1u32 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1u32 << TIF_NEED_RESCHED;
pub const _TIF_SINGLESTEP: u32 = 1u32 << TIF_SINGLESTEP;
pub const _TIF_NOTIFY_SIGNAL: u32 = 1u32 << TIF_NOTIFY_SIGNAL;

/* work to do on interrupt/exception return - All but TIF_SYSCALL_TRACE */
pub const _TIF_WORK_MASK: u32 = 0x0000_FFFFu32 & !_TIF_SYSCALL_TRACE;

/* work to do on any return to u-space */
pub const _TIF_ALLWORK_MASK: u32 = 0x0000_FFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
