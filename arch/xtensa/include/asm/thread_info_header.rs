/*
 * include/asm-xtensa/thread_info.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C header dependencies: linux/stringify.h, asm/kmem_layout.h, asm/processor.h

pub const CURRENT_SHIFT: usize = KERNEL_STACK_SHIFT;

#[cfg(feature = "xtensa_have_coprocessors")]
#[repr(C)]
pub struct xtregs_coprocessor_t {
    pub cp0: xtregs_cp0_t,
    pub cp1: xtregs_cp1_t,
    pub cp2: xtregs_cp2_t,
    pub cp3: xtregs_cp3_t,
    pub cp4: xtregs_cp4_t,
    pub cp5: xtregs_cp5_t,
    pub cp6: xtregs_cp6_t,
    pub cp7: xtregs_cp7_t,
}

#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
    pub flags: ::core::ffi::c_ulong,
    pub status: ::core::ffi::c_ulong,
    pub cpu: u32,
    pub preempt_count: i32,
    #[cfg(feature = "xchal_have_exclusive")]
    pub atomctl8: ::core::ffi::c_ulong,
    #[cfg(feature = "config_user_abi_call0_probe")]
    pub ps_woe_fix_addr: ::core::ffi::c_ulong,
    pub cpenable: ::core::ffi::c_ulong,
    pub cp_owner_cpu: u32,
    #[cfg(feature = "xtensa_have_coprocessors")]
    pub xtregs_cp: xtregs_coprocessor_t,
    pub xtregs_user: xtregs_user_t,
}

/* INIT_THREAD_INFO(tsk) */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        thread_info {
            task: &mut $tsk,
            flags: 0,
            status: 0,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            ..unsafe { ::core::mem::zeroed() }
        }
    };
}

/* how to get the thread information struct from C */
#[inline(always)]
pub unsafe fn current_thread_info() -> *mut thread_info {
    let ti: *mut thread_info;
    ::core::arch::asm!(
        "extui {0}, a1, 0, {1}",
        "xor {0}, a1, {0}",
        out(reg) ti,
        const CURRENT_SHIFT,
    );
    ti
}

/* Assembly form: extui reg, sp, 0, CURRENT_SHIFT; xor reg, sp, reg */

pub const TIF_SYSCALL_TRACE: usize = 0;
pub const TIF_SIGPENDING: usize = 1;
pub const TIF_NEED_RESCHED: usize = 2;
pub const TIF_SINGLESTEP: usize = 3;
pub const TIF_SYSCALL_TRACEPOINT: usize = 4;
pub const TIF_NOTIFY_SIGNAL: usize = 5;
pub const TIF_RESTORE_SIGMASK: usize = 6;
pub const TIF_NOTIFY_RESUME: usize = 7;
pub const TIF_DB_DISABLED: usize = 8;
pub const TIF_SYSCALL_AUDIT: usize = 9;
pub const TIF_SECCOMP: usize = 10;
pub const TIF_MEMDIE: usize = 11;

pub const _TIF_SYSCALL_TRACE: ::core::ffi::c_ulong = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: ::core::ffi::c_ulong = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: ::core::ffi::c_ulong = 1 << TIF_NEED_RESCHED;
pub const _TIF_SINGLESTEP: ::core::ffi::c_ulong = 1 << TIF_SINGLESTEP;
pub const _TIF_SYSCALL_TRACEPOINT: ::core::ffi::c_ulong = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_NOTIFY_SIGNAL: ::core::ffi::c_ulong = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_NOTIFY_RESUME: ::core::ffi::c_ulong = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SYSCALL_AUDIT: ::core::ffi::c_ulong = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SECCOMP: ::core::ffi::c_ulong = 1 << TIF_SECCOMP;

pub const _TIF_WORK_MASK: ::core::ffi::c_ulong =
    _TIF_SYSCALL_TRACE | _TIF_SINGLESTEP | _TIF_SYSCALL_TRACEPOINT |
    _TIF_SYSCALL_AUDIT | _TIF_SECCOMP;

pub const THREAD_SIZE: usize = KERNEL_STACK_SIZE;
pub const THREAD_SIZE_ORDER: usize = KERNEL_STACK_SHIFT - PAGE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
