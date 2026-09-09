/* SPDX-License-Identifier: GPL-2.0 */
/* thread_info.h: PowerPC low-level thread information
 * adapted from the i386 version by Paul Mackerras
 *
 * Copyright (C) 2002  David Howells (dhowells@redhat.com)
 * - Incorporating suggestions made by Linus Torvalds and Dave Miller
 */

/* Dependencies: asm-const.h, page.h, cache.h, processor.h, accounting.h,
 * and ppc_asm.h are supplied by other translation units. */

#[cfg(all(feature = "kasan", feature = "thread_shift_lt_15"))]
pub const MIN_THREAD_SHIFT: usize = CONFIG_THREAD_SHIFT + 1;
#[cfg(not(all(feature = "kasan", feature = "thread_shift_lt_15")))]
pub const MIN_THREAD_SHIFT: usize = CONFIG_THREAD_SHIFT;

#[cfg(all(feature = "vmap_stack", feature = "min_thread_shift_lt_page_shift"))]
pub const THREAD_SHIFT: usize = PAGE_SHIFT;
#[cfg(not(all(feature = "vmap_stack", feature = "min_thread_shift_lt_page_shift")))]
pub const THREAD_SHIFT: usize = MIN_THREAD_SHIFT;

pub const THREAD_SIZE: usize = 1usize << THREAD_SHIFT;

/* By aligning VMAP'd stacks to 2 * THREAD_SIZE, overflow is detected by
 * checking sp & (1 << THREAD_SHIFT), cheaply in the entry assembly. */
#[cfg(feature = "vmap_stack")]
pub const THREAD_ALIGN_SHIFT: usize = THREAD_SHIFT + 1;
#[cfg(not(feature = "vmap_stack"))]
pub const THREAD_ALIGN_SHIFT: usize = THREAD_SHIFT;
pub const THREAD_ALIGN: usize = 1usize << THREAD_ALIGN_SHIFT;

pub const SLB_PRELOAD_NR: usize = 16;

#[repr(C)]
pub struct thread_info {
    pub preempt_count: i32, /* 0 => preemptable, <0 => BUG */
    #[cfg(feature = "smp")]
    pub cpu: u32,
    pub exit_flags: usize,  /* Exit Flags for entry/exit */
    pub syscall_work: usize, /* SYSCALL_WORK_ flags */
    pub local_flags: usize, /* private flags for thread */
    #[cfg(feature = "livepatch_64")]
    pub livepatch_sp: *mut usize,
    #[cfg(all(feature = "virt_cpu_accounting_native", feature = "ppc32"))]
    pub accounting: cpu_accounting_data,
    pub slb_preload_nr: u8,
    pub slb_preload_tail: u8,
    pub slb_preload_esid: [u32; SLB_PRELOAD_NR],
    pub flags: usize, /* ____cacheline_aligned_in_smp */
}

/* INIT_THREAD_INFO(tsk): preempt_count = INIT_PREEMPT_COUNT, flags = 0. */
pub const THREAD_SIZE_ORDER: usize = THREAD_SHIFT - PAGE_SHIFT;

extern "C" {
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32;
    pub fn arch_setup_new_exec();
}

/* thread information flag bit numbers */
pub const TIF_SYSCALL_TRACE: usize = 0;
pub const TIF_SIGPENDING: usize = 1;
pub const TIF_NEED_RESCHED: usize = 2;
pub const TIF_NOTIFY_SIGNAL: usize = 3;
pub const TIF_SYSCALL_EMU: usize = 4;
pub const TIF_RESTORE_TM: usize = 5;
pub const TIF_PATCH_PENDING: usize = 6;
pub const TIF_SYSCALL_AUDIT: usize = 7;
pub const TIF_SINGLESTEP: usize = 8;
pub const TIF_NEED_RESCHED_LAZY: usize = 9;
pub const TIF_SECCOMP: usize = 10;
pub const TIF_RESTOREALL: usize = 11;
pub const TIF_NOERROR: usize = 12;
pub const TIF_NOTIFY_RESUME: usize = 13;
pub const TIF_UPROBE: usize = 14;
pub const TIF_SYSCALL_TRACEPOINT: usize = 15;
pub const TIF_EMULATE_STACK_STORE: usize = 16;
pub const TIF_MEMDIE: usize = 17;
#[cfg(feature = "ppc64")]
pub const TIF_ELF2ABI: usize = 18;
pub const TIF_POLLING_NRFLAG: usize = 19;
pub const TIF_32BIT: usize = 20;
pub const TIF_SYSCALL_RET: usize = 21;

pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_NEED_RESCHED_LAZY: usize = 1 << TIF_NEED_RESCHED_LAZY;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_POLLING_NRFLAG: usize = 1 << TIF_POLLING_NRFLAG;
pub const _TIF_32BIT: usize = 1 << TIF_32BIT;
pub const _TIF_RESTORE_TM: usize = 1 << TIF_RESTORE_TM;
pub const _TIF_PATCH_PENDING: usize = 1 << TIF_PATCH_PENDING;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SINGLESTEP: usize = 1 << TIF_SINGLESTEP;
pub const _TIF_SECCOMP: usize = 1 << TIF_SECCOMP;
pub const _TIF_RESTOREALL: usize = 1 << TIF_RESTOREALL;
pub const _TIF_NOERROR: usize = 1 << TIF_NOERROR;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_UPROBE: usize = 1 << TIF_UPROBE;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_EMULATE_STACK_STORE: usize = 1 << TIF_EMULATE_STACK_STORE;
pub const _TIF_SYSCALL_EMU: usize = 1 << TIF_SYSCALL_EMU;
pub const _TIF_SYSCALL_DOTRACE: usize = _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SECCOMP | _TIF_SYSCALL_TRACEPOINT | _TIF_SYSCALL_EMU;
pub const _TIF_USER_WORK_MASK: usize = _TIF_SIGPENDING | _TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY | _TIF_NOTIFY_RESUME | _TIF_UPROBE | _TIF_RESTORE_TM | _TIF_PATCH_PENDING | _TIF_NOTIFY_SIGNAL;
pub const _TIF_PERSYSCALL_MASK: usize = _TIF_RESTOREALL | _TIF_NOERROR;

/* Bits in local_flags. */
pub const TLF_NAPPING: usize = 0;
pub const TLF_SLEEPING: usize = 1;
pub const TLF_RUNLATCH: usize = 4;
pub const _TLF_NAPPING: usize = 1 << TLF_NAPPING;
pub const _TLF_SLEEPING: usize = 1 << TLF_SLEEPING;
pub const _TLF_RUNLATCH: usize = 1 << TLF_RUNLATCH;

extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
    pub fn current_stack_pointer() -> *const core::ffi::c_void;
    pub fn test_thread_flag(flag: usize) -> bool;
    pub fn test_tsk_thread_flag(tsk: *mut task_struct, flag: usize) -> bool;
    pub fn clear_tsk_thread_flag(tsk: *mut task_struct, flag: usize);
}

#[inline]
pub unsafe fn clear_thread_local_flags(flags: u32) {
    (*current_thread_info()).local_flags &= !(flags as usize);
}

#[inline]
pub unsafe fn test_thread_local_flags(flags: u32) -> bool {
    ((*current_thread_info()).local_flags & flags as usize) != 0
}

#[cfg(feature = "compat")]
pub unsafe fn is_32bit_task() -> bool { test_thread_flag(TIF_32BIT) }
#[cfg(not(feature = "compat"))]
pub const fn is_32bit_task() -> bool { cfg!(feature = "ppc32") }

#[cfg(feature = "ppc64")]
pub unsafe fn is_elf2_task() -> bool { test_thread_flag(TIF_ELF2ABI) }
#[cfg(not(feature = "ppc64"))]
pub const fn is_elf2_task() -> bool { false }

/* Declaration-only external types and constants used by this header. */
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpu_accounting_data { _private: [u8; 0] }
extern "C" { pub static STACK_FRAME_PARAMS: usize; pub static GOOD_FRAME: i32; pub static BAD_STACK: i32; }

#[cfg(feature = "ppc32")]
extern "C" { pub static mut emergency_ctx: [*mut core::ffi::c_void; 0]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
