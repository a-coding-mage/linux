/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the Alpha kernel thread_info.h header. */
/* C-only includes and build-time __KERNEL__/__ASSEMBLER__ guards are omitted. */

#[repr(C)]
pub struct thread_info {
    pub pcb: pcb_struct,                 /* palcode state */
    pub task: *mut task_struct,          /* main task structure */
    pub flags: u32,                      /* low level flags */
    pub ieee_state: u32,                 /* see fpu.h */
    pub cpu: c_uint,                     /* current CPU */
    pub preempt_count: i32,              /* 0 => preemptable, <0 => BUG */
    pub status: u32,                     /* thread-synchronous flags */
    pub bpt_nsaved: i32,
    pub bpt_addr: [c_ulong; 2],          /* breakpoint handling */
    pub bpt_insn: [u32; 2],
    pub fp: [c_ulong; 32],
}

/* External types supplied by the corresponding architecture headers. */
pub type c_uint = u32;
pub type c_ulong = usize;
pub type __u32 = u32;

/* These declarations are supplied externally. */
extern "C" {
    pub static mut __current_thread_info: *mut thread_info;
    pub static mut current_stack_pointer: *mut c_ulong;
    pub fn __save_fpu();
}

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        thread_info {
            task: &mut $tsk as *mut _,
            preempt_count: INIT_PREEMPT_COUNT,
            ..::core::mem::zeroed()
        }
    };
}

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    __current_thread_info
}

pub const THREAD_SIZE_ORDER: usize = 1;
pub const THREAD_SIZE: usize = 2 * PAGE_SIZE;

pub const TIF_SYSCALL_TRACE: u32 = 0;
pub const TIF_NOTIFY_RESUME: u32 = 1;
pub const TIF_SIGPENDING: u32 = 2;
pub const TIF_NEED_RESCHED: u32 = 3;
pub const TIF_SYSCALL_AUDIT: u32 = 4;
pub const TIF_NOTIFY_SIGNAL: u32 = 5;
pub const TIF_SECCOMP: u32 = 6;
pub const TIF_SYSCALL_TRACEPOINT: u32 = 7;
pub const TIF_DIE_IF_KERNEL: u32 = 9;
pub const TIF_MEMDIE: u32 = 13;
pub const TIF_POLLING_NRFLAG: u32 = 14;

pub const _TIF_SYSCALL_TRACE: u32 = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: u32 = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u32 = 1 << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_RESUME: u32 = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SYSCALL_AUDIT: u32 = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_NOTIFY_SIGNAL: u32 = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_SECCOMP: u32 = 1 << TIF_SECCOMP;
pub const _TIF_POLLING_NRFLAG: u32 = 1 << TIF_POLLING_NRFLAG;
pub const _TIF_SYSCALL_TRACEPOINT: u32 = 1 << TIF_SYSCALL_TRACEPOINT;

/* CONFIG_AUDITSYSCALL selects the alternate definition. */
#[cfg(CONFIG_AUDITSYSCALL)]
pub const _TIF_SYSCALL_WORK: u32 = _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SECCOMP | _TIF_SYSCALL_TRACEPOINT;
#[cfg(not(CONFIG_AUDITSYSCALL))]
pub const _TIF_SYSCALL_WORK: u32 = _TIF_SYSCALL_TRACE | _TIF_SECCOMP | _TIF_SYSCALL_TRACEPOINT;

pub const _TIF_WORK_MASK: u32 = _TIF_SIGPENDING | _TIF_NEED_RESCHED | _TIF_NOTIFY_RESUME | _TIF_NOTIFY_SIGNAL;

pub const TS_UAC_NOPRINT: u32 = 0x0001;
pub const TS_UAC_NOFIX: u32 = 0x0002;
pub const TS_UAC_SIGBUS: u32 = 0x0004;
pub const TS_SAVED_FP: u32 = 0x0008;
pub const TS_RESTORE_FP: u32 = 0x0010;

#[inline]
pub unsafe fn save_fpu() {
    let ti = current_thread_info();
    if ((*ti).status & TS_SAVED_FP) == 0 {
        (*ti).status |= TS_SAVED_FP;
        __save_fpu();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
