/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* thread information allocation */
#[cfg(feature = "CONFIG_KASAN")]
pub const KASAN_STACK_ORDER: usize = 1;
#[cfg(not(feature = "CONFIG_KASAN"))]
pub const KASAN_STACK_ORDER: usize = 0;

pub const THREAD_SIZE_ORDER: usize = CONFIG_THREAD_SIZE_ORDER + KASAN_STACK_ORDER;
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

/*
 * By aligning VMAP'd stacks to 2 * THREAD_SIZE, we can detect overflow by
 * checking sp & (1 << THREAD_SHIFT), which we can do cheaply in the entry
 * assembly.
 */
#[cfg(feature = "CONFIG_VMAP_STACK")]
pub const THREAD_ALIGN: usize = 2 * THREAD_SIZE;
#[cfg(not(feature = "CONFIG_VMAP_STACK"))]
pub const THREAD_ALIGN: usize = THREAD_SIZE;

pub const THREAD_SHIFT: usize = PAGE_SHIFT + THREAD_SIZE_ORDER;
pub const OVERFLOW_STACK_SIZE: usize = SZ_4K;
pub const IRQ_STACK_SIZE: usize = THREAD_SIZE;

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - if the members of this struct changes, the assembly constants
 *   in asm-offsets.c must be updated accordingly
 * - thread_info is included in task_struct at an offset of 0.  This means that
 *   tp points to both thread_info and task_struct.
 */
#[repr(C)]
pub struct thread_info {
    pub flags: core::ffi::c_ulong,
    pub preempt_count: core::ffi::c_int,
    /*
     * These stack pointers are overwritten on every system call or
     * exception.  SP is also saved to the stack it can be recovered when
     * overwritten.
     */
    pub kernel_sp: core::ffi::c_long,
    pub user_sp: core::ffi::c_long,
    pub cpu: core::ffi::c_int,
    pub syscall_work: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    pub scs_base: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    pub scs_sp: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_64BIT")]
    pub a0: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_64BIT")]
    pub a1: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_64BIT")]
    pub a2: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_RISCV_USER_CFI")]
    pub user_cfi_state: cfi_state,
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[macro_export]
macro_rules! INIT_SCS {
    () => {
        scs_base: init_shadow_call_stack,
        scs_sp: init_shadow_call_stack,
    };
}
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[macro_export]
macro_rules! INIT_SCS {
    () => {};
}

/*
 * macros/functions for gaining access to the thread information structure
 *
 * preempt_count needs to be 1 initially, until the scheduler is functional.
 */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {{
        let _ = $tsk;
        thread_info {
            flags: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            INIT_SCS!()
        }
    }};
}

unsafe extern "C" {
    pub fn arch_release_task_struct(tsk: *mut task_struct);
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> core::ffi::c_int;
}

/* thread information flags */
/*
 * Tell the generic TIF infrastructure which bits riscv supports.
 * HAVE_TIF_NEED_RESCHED_LAZY and HAVE_TIF_RESTORE_SIGMASK are feature markers.
 */
pub const HAVE_TIF_NEED_RESCHED_LAZY: bool = true;
pub const HAVE_TIF_RESTORE_SIGMASK: bool = true;

pub const TIF_32BIT: usize = 16;
pub const TIF_RISCV_V_DEFER_RESTORE: usize = 17;
pub const _TIF_RISCV_V_DEFER_RESTORE: usize = 1usize << TIF_RISCV_V_DEFER_RESTORE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
