/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: supplied by asm/types.h and asm/page.h.
// Build-time condition preserved from PAGE_SHIFT and CONFIG_4KSTACKS.

#[cfg(page_shift_lt_13)]
#[cfg(feature = "CONFIG_4KSTACKS")]
pub const THREAD_SIZE_ORDER: usize = 0;

#[cfg(page_shift_lt_13)]
#[cfg(not(feature = "CONFIG_4KSTACKS"))]
pub const THREAD_SIZE_ORDER: usize = 1;

#[cfg(not(page_shift_lt_13))]
pub const THREAD_SIZE_ORDER: usize = 0;

pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,       /* main task structure */
    pub flags: c_ulong,
    pub preempt_count: i32,           /* 0 => preemptable, <0 => BUG */
    pub cpu: u32,                     /* should always be 0 on m68k */
    pub tp_value: c_ulong,            /* thread pointer */
}

// Dependency declarations supplied by other translated files.
#[allow(non_camel_case_types)]
pub enum task_struct {}
pub type c_ulong = usize;
extern "Rust" {
    pub static PAGE_SIZE: usize;
    pub static INIT_PREEMPT_COUNT: i32;
}

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: &mut $tsk as *mut _,
            preempt_count: $crate::INIT_PREEMPT_COUNT,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

/* how to get the thread information struct from C */
#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    let ti: *mut thread_info;
    core::arch::asm!(
        "move.l %sp, {0} \n\t",
        "and.l  {1}, {0}",
        out(reg) ti,
        in(reg) (!(THREAD_SIZE - 1)),
    );
    ti
}

/* entry.S relies on these definitions!
 * bits 0-7 are tested at every exception exit
 * bits 8-15 are also tested at syscall exit
 */
pub const TIF_NOTIFY_SIGNAL: u32 = 4;
pub const TIF_NOTIFY_RESUME: u32 = 5;  /* callback before returning to user */
pub const TIF_SIGPENDING: u32 = 6;     /* signal pending */
pub const TIF_NEED_RESCHED: u32 = 7;   /* rescheduling necessary */
pub const TIF_SECCOMP: u32 = 13;       /* seccomp syscall filtering active */
pub const TIF_DELAYED_TRACE: u32 = 14; /* single step a syscall */
pub const TIF_SYSCALL_TRACE: u32 = 15; /* syscall trace active */
pub const TIF_MEMDIE: u32 = 16;        /* is terminating due to OOM killer */
pub const TIF_RESTORE_SIGMASK: u32 = 18; /* restore signal mask in do_signal */

pub const _TIF_NOTIFY_RESUME: u32 = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: u32 = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1 << TIF_NEED_RESCHED;
pub const _TIF_SECCOMP: u32 = 1 << TIF_SECCOMP;
pub const _TIF_DELAYED_TRACE: u32 = 1 << TIF_DELAYED_TRACE;
pub const _TIF_SYSCALL_TRACE: u32 = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_MEMDIE: u32 = 1 << TIF_MEMDIE;
pub const _TIF_RESTORE_SIGMASK: u32 = 1 << TIF_RESTORE_SIGMASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
