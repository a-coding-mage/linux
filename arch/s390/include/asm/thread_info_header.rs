/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 2002, 2006
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

// Translated from the C header. The original include guard is omitted.
// Dependencies supplied by other headers: PAGE_SIZE, STACK_FRAME_OVERHEAD,
// __PT_SIZE, and BIT.

/* General size of kernel stacks */
#[cfg(any(CONFIG_KASAN, CONFIG_KMSAN))]
pub const THREAD_SIZE_ORDER: u32 = 4;
#[cfg(not(any(CONFIG_KASAN, CONFIG_KMSAN)))]
pub const THREAD_SIZE_ORDER: u32 = 2;

pub const BOOT_STACK_SIZE: usize = PAGE_SIZE << 2;
pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;

pub const STACK_INIT_OFFSET: usize = THREAD_SIZE - STACK_FRAME_OVERHEAD - __PT_SIZE;

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - this struct shares the supervisor stack pages
 * - if the contents of this structure are changed, the assembly constants must also be changed
 */
#[repr(C)]
pub struct thread_info {
    pub flags: ::core::ffi::c_ulong,       /* low level flags */
    pub syscall_work: ::core::ffi::c_ulong, /* SYSCALL_WORK_ flags */
    pub cpu: ::core::ffi::c_uint,          /* current CPU */
    pub sie: ::core::ffi::c_uchar,         /* running in SIE context */
}

/* macros/functions for gaining access to the thread information structure */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        thread_info {
            flags: 0,
            ..unsafe { ::core::mem::zeroed() }
        }
    };
}

pub struct task_struct;

extern "C" {
    pub fn arch_setup_new_exec();
}

/*
 * thread information flags bit numbers
 *
 * Tell the generic TIF infrastructure which special bits s390 supports
 */
// HAVE_TIF_NEED_RESCHED_LAZY
// HAVE_TIF_RESTORE_SIGMASK
// HAVE_TIF_POLLING_NRFLAG
// The generic TIF infrastructure is supplied by asm-generic/thread_info_tif.h.

/* Architecture specific bits */
pub const TIF_ASCE_PRIMARY: u32 = 16;       /* primary asce is kernel asce */
pub const TIF_GUARDED_STORAGE: u32 = 17;    /* load guarded storage control block */
pub const TIF_ISOLATE_BP_GUEST: u32 = 18;   /* Run KVM guests with isolated BP */
pub const TIF_PER_TRAP: u32 = 19;           /* Need to handle PER trap on exit to usermode */
pub const TIF_SINGLE_STEP: u32 = 21;        /* This task is single stepped */
pub const TIF_BLOCK_STEP: u32 = 22;         /* This task is block stepped */
pub const TIF_UPROBE_SINGLESTEP: u32 = 23;  /* This task is uprobe single stepped */

pub const _TIF_ASCE_PRIMARY: _ = BIT(TIF_ASCE_PRIMARY);
pub const _TIF_GUARDED_STORAGE: _ = BIT(TIF_GUARDED_STORAGE);
pub const _TIF_ISOLATE_BP_GUEST: _ = BIT(TIF_ISOLATE_BP_GUEST);
pub const _TIF_PER_TRAP: _ = BIT(TIF_PER_TRAP);
pub const _TIF_SINGLE_STEP: _ = BIT(TIF_SINGLE_STEP);
pub const _TIF_BLOCK_STEP: _ = BIT(TIF_BLOCK_STEP);
pub const _TIF_UPROBE_SINGLESTEP: _ = BIT(TIF_UPROBE_SINGLESTEP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
