/* SPDX-License-Identifier: GPL-2.0 */
/* thread_info.h: MIPS low-level thread information
 *
 * Copyright (C) 2002  David Howells (dhowells@redhat.com)
 * - Incorporating suggestions made by Linus Torvalds and Dave Miller
 */

/* C header guards and __KERNEL__/__ASSEMBLER__ build conditions omitted. */

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - this struct shares the supervisor stack pages
 * - if the contents of this structure are changed, the assembly constants
 *   must also be changed
 */
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,       /* main task structure */
    pub flags: ::core::ffi::c_ulong,  /* low level flags */
    pub tp_value: ::core::ffi::c_ulong, /* thread pointer */
    pub cpu: u32,                     /* current CPU */
    pub preempt_count: ::core::ffi::c_int, /* 0 => preemptible, <0 => BUG */
    pub regs: *mut pt_regs,
    pub syscall: ::core::ffi::c_long, /* syscall number */
}

/*
 * macros/functions for gaining access to the thread information structure
 */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {
        $crate::thread_info {
            task: &mut $tsk as *mut _,
            flags: _TIF_FIXADE as _,
            cpu: 0,
            preempt_count: INIT_PREEMPT_COUNT,
            tp_value: 0,
            regs: ::core::ptr::null_mut(),
            syscall: 0,
        }
    };
}

/*
 * A pointer to the struct thread_info for the currently executing thread is
 * held in register $28/$gp.
 */
extern "C" {
    pub static mut __current_thread_info: *mut thread_info;
}

#[inline]
pub unsafe fn current_thread_info() -> *mut thread_info {
    __current_thread_info
}

/* The C source declares current_stack_pointer as a register variable when
 * CONFIG_ARCH_HAS_CURRENT_STACK_POINTER is enabled. */

/* thread information allocation */
/* THREAD_SIZE_ORDER is selected by the build-time page-size and bitness
 * configuration; the alternatives are preserved below. */
#[cfg(all(CONFIG_PAGE_SIZE_4KB, CONFIG_32BIT))]
pub const THREAD_SIZE_ORDER: usize = 1;
#[cfg(all(CONFIG_PAGE_SIZE_4KB, CONFIG_64BIT))]
pub const THREAD_SIZE_ORDER: usize = 2;
#[cfg(CONFIG_PAGE_SIZE_8KB)]
pub const THREAD_SIZE_ORDER: usize = 1;
#[cfg(CONFIG_PAGE_SIZE_16KB)]
pub const THREAD_SIZE_ORDER: usize = 0;
#[cfg(CONFIG_PAGE_SIZE_32KB)]
pub const THREAD_SIZE_ORDER: usize = 0;
#[cfg(CONFIG_PAGE_SIZE_64KB)]
pub const THREAD_SIZE_ORDER: usize = 0;

pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;
pub const THREAD_MASK: usize = THREAD_SIZE - 1usize;
pub const STACK_WARN: usize = THREAD_SIZE / 8;

/* thread information flags */
pub const TIF_SIGPENDING: u32 = 1;
pub const TIF_NEED_RESCHED: u32 = 2;
pub const TIF_SYSCALL_AUDIT: u32 = 3;
pub const TIF_SECCOMP: u32 = 4;
pub const TIF_NOTIFY_RESUME: u32 = 5;
pub const TIF_UPROBE: u32 = 6;
pub const TIF_NOTIFY_SIGNAL: u32 = 7;
pub const TIF_RESTORE_SIGMASK: u32 = 9;
pub const TIF_USEDFPU: u32 = 16;
pub const TIF_MEMDIE: u32 = 18;
pub const TIF_NOHZ: u32 = 19;
pub const TIF_FIXADE: u32 = 20;
pub const TIF_LOGADE: u32 = 21;
pub const TIF_32BIT_REGS: u32 = 22;
pub const TIF_32BIT_ADDR: u32 = 23;
pub const TIF_FPUBOUND: u32 = 24;
pub const TIF_LOAD_WATCH: u32 = 25;
pub const TIF_SYSCALL_TRACEPOINT: u32 = 26;
pub const TIF_32BIT_FPREGS: u32 = 27;
pub const TIF_HYBRID_FPREGS: u32 = 28;
pub const TIF_USEDMSA: u32 = 29;
pub const TIF_MSA_CTX_LIVE: u32 = 30;
pub const TIF_SYSCALL_TRACE: u32 = 31;

pub const _TIF_SYSCALL_TRACE: u64 = 1u64 << TIF_SYSCALL_TRACE;
pub const _TIF_SIGPENDING: u64 = 1u64 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: u64 = 1u64 << TIF_NEED_RESCHED;
pub const _TIF_SYSCALL_AUDIT: u64 = 1u64 << TIF_SYSCALL_AUDIT;
pub const _TIF_SECCOMP: u64 = 1u64 << TIF_SECCOMP;
pub const _TIF_NOTIFY_RESUME: u64 = 1u64 << TIF_NOTIFY_RESUME;
pub const _TIF_UPROBE: u64 = 1u64 << TIF_UPROBE;
pub const _TIF_NOTIFY_SIGNAL: u64 = 1u64 << TIF_NOTIFY_SIGNAL;
pub const _TIF_USEDFPU: u64 = 1u64 << TIF_USEDFPU;
pub const _TIF_NOHZ: u64 = 1u64 << TIF_NOHZ;
pub const _TIF_FIXADE: u64 = 1u64 << TIF_FIXADE;
pub const _TIF_LOGADE: u64 = 1u64 << TIF_LOGADE;
pub const _TIF_32BIT_REGS: u64 = 1u64 << TIF_32BIT_REGS;
pub const _TIF_32BIT_ADDR: u64 = 1u64 << TIF_32BIT_ADDR;
pub const _TIF_FPUBOUND: u64 = 1u64 << TIF_FPUBOUND;
pub const _TIF_LOAD_WATCH: u64 = 1u64 << TIF_LOAD_WATCH;
pub const _TIF_32BIT_FPREGS: u64 = 1u64 << TIF_32BIT_FPREGS;
pub const _TIF_HYBRID_FPREGS: u64 = 1u64 << TIF_HYBRID_FPREGS;
pub const _TIF_USEDMSA: u64 = 1u64 << TIF_USEDMSA;
pub const _TIF_MSA_CTX_LIVE: u64 = 1u64 << TIF_MSA_CTX_LIVE;
pub const _TIF_SYSCALL_TRACEPOINT: u64 = 1u64 << TIF_SYSCALL_TRACEPOINT;

pub const _TIF_WORK_SYSCALL_ENTRY: u64 = _TIF_NOHZ | _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SYSCALL_TRACEPOINT | _TIF_SECCOMP;
pub const _TIF_WORK_SYSCALL_EXIT: u64 = _TIF_NOHZ | _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT | _TIF_SYSCALL_TRACEPOINT;
pub const _TIF_WORK_MASK: u64 = _TIF_SIGPENDING | _TIF_NEED_RESCHED | _TIF_NOTIFY_RESUME | _TIF_UPROBE | _TIF_NOTIFY_SIGNAL;
pub const _TIF_ALLWORK_MASK: u64 = _TIF_NOHZ | _TIF_WORK_MASK | _TIF_WORK_SYSCALL_EXIT | _TIF_SYSCALL_TRACEPOINT;

/* Processor-id COP0 register selection. */
#[cfg(CONFIG_MIPS_PGD_C0_CONTEXT)]
pub const SMP_CPUID_REG: (u32, u32) = (20, 0);
#[cfg(CONFIG_MIPS_PGD_C0_CONTEXT)]
pub const ASM_SMP_CPUID_REG: &str = "$20";
#[cfg(CONFIG_MIPS_PGD_C0_CONTEXT)]
pub const SMP_CPUID_PTRSHIFT: u32 = 48;
#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
pub const SMP_CPUID_REG: (u32, u32) = (4, 0);
#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
pub const ASM_SMP_CPUID_REG: &str = "$4";
#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
pub const SMP_CPUID_PTRSHIFT: u32 = 23;

#[cfg(CONFIG_64BIT)]
pub const SMP_CPUID_REGSHIFT: u32 = SMP_CPUID_PTRSHIFT + 3;
#[cfg(not(CONFIG_64BIT))]
pub const SMP_CPUID_REGSHIFT: u32 = SMP_CPUID_PTRSHIFT + 2;

/* Assembly macro aliases retained as symbolic constants. */
pub const ASM_CPUID_MFC0: &str = "MFC0";
pub const UASM_i_CPUID_MFC0: &str = "UASM_i_MFC0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
