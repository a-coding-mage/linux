/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/thread_info.h
 *
 * Copyright (C) 2002 Russell King.
 * Copyright (C) 2012 ARM Ltd.
 */

/* C header dependencies: linux/compiler.h, asm/memory.h,
 * asm/stack_pointer.h, and asm/types.h are supplied externally. */

#[cfg(not(feature = "assembler"))]
pub struct task_struct;

/* low level task data that entry.S needs immediate access to. */
#[repr(C)]
pub struct thread_info {
    pub flags: usize, /* low level flags */
    #[cfg(feature = "CONFIG_ARM64_SW_TTBR0_PAN")]
    pub ttbr0: u64, /* saved TTBR0_EL1 */
    pub preempt: thread_info_preempt,
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    pub scs_base: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    pub scs_sp: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_ARM64_MPAM")]
    pub mpam_partid_pmg: u64,
    pub cpu: u32,
}

#[repr(C)]
pub union thread_info_preempt {
    pub preempt_count: u64, /* 0 => preemptible, <0 => bug */
    pub preempt: thread_info_preempt_fields,
}

#[repr(C)]
pub struct thread_info_preempt_fields {
    #[cfg(feature = "CONFIG_CPU_BIG_ENDIAN")]
    pub need_resched: u32,
    #[cfg(feature = "CONFIG_CPU_BIG_ENDIAN")]
    pub count: u32,
    #[cfg(not(feature = "CONFIG_CPU_BIG_ENDIAN"))]
    pub count: u32,
    #[cfg(not(feature = "CONFIG_CPU_BIG_ENDIAN"))]
    pub need_resched: u32,
}

#[macro_export]
macro_rules! thread_saved_pc {
    ($tsk:expr) => { ($tsk.thread.cpu_context.pc as usize) };
}
#[macro_export]
macro_rules! thread_saved_sp {
    ($tsk:expr) => { ($tsk.thread.cpu_context.sp as usize) };
}
#[macro_export]
macro_rules! thread_saved_fp {
    ($tsk:expr) => { ($tsk.thread.cpu_context.fp as usize) };
}

pub extern "C" fn arch_setup_new_exec();

pub const TIF_SIGPENDING: usize = 0; /* signal pending */
pub const TIF_NEED_RESCHED: usize = 1; /* rescheduling necessary */
pub const TIF_NEED_RESCHED_LAZY: usize = 2; /* Lazy rescheduling needed */
pub const TIF_NOTIFY_RESUME: usize = 3; /* callback before returning to user */
pub const TIF_FOREIGN_FPSTATE: usize = 4; /* CPU's FP state is not current's */
pub const TIF_UPROBE: usize = 5; /* uprobe breakpoint or singlestep */
pub const TIF_MTE_ASYNC_FAULT: usize = 6; /* MTE Asynchronous Tag Check Fault */
pub const TIF_NOTIFY_SIGNAL: usize = 7; /* signal notifications exist */
pub const TIF_SYSCALL_TRACE: usize = 8; /* syscall trace active */
pub const TIF_SYSCALL_AUDIT: usize = 9; /* syscall auditing */
pub const TIF_SYSCALL_TRACEPOINT: usize = 10; /* syscall tracepoint for ftrace */
pub const TIF_SECCOMP: usize = 11; /* syscall secure computing */
pub const TIF_SYSCALL_EMU: usize = 12; /* syscall emulation active */
pub const TIF_PATCH_PENDING: usize = 13; /* pending live patching update */
pub const TIF_MEMDIE: usize = 18; /* is terminating due to OOM killer */
pub const TIF_FREEZE: usize = 19;
pub const TIF_RESTORE_SIGMASK: usize = 20;
pub const TIF_SINGLESTEP: usize = 21;
pub const TIF_32BIT: usize = 22; /* 32bit process */
pub const TIF_SVE: usize = 23; /* Scalable Vector Extension in use */
pub const TIF_SVE_VL_INHERIT: usize = 24; /* Inherit SVE vl_onexec across exec */
pub const TIF_SSBD: usize = 25; /* Wants SSB mitigation */
pub const TIF_TAGGED_ADDR: usize = 26; /* Allow tagged user addresses */
pub const TIF_SME: usize = 27; /* SME in use */
pub const TIF_SME_VL_INHERIT: usize = 28; /* Inherit SME vl_onexec across exec */
pub const TIF_KERNEL_FPSTATE: usize = 29; /* Task is in a kernel mode FPSIMD section */
pub const TIF_TSC_SIGSEGV: usize = 30; /* SIGSEGV on counter-timer access */
pub const TIF_LAZY_MMU_PENDING: usize = 31; /* Ops pending for lazy mmu mode exit */

pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_NEED_RESCHED_LAZY: usize = 1 << TIF_NEED_RESCHED_LAZY;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_FOREIGN_FPSTATE: usize = 1 << TIF_FOREIGN_FPSTATE;
pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_SECCOMP: usize = 1 << TIF_SECCOMP;
pub const _TIF_SYSCALL_EMU: usize = 1 << TIF_SYSCALL_EMU;
pub const _TIF_PATCH_PENDING: usize = 1 << TIF_PATCH_PENDING;
pub const _TIF_UPROBE: usize = 1 << TIF_UPROBE;
pub const _TIF_SINGLESTEP: usize = 1 << TIF_SINGLESTEP;
pub const _TIF_32BIT: usize = 1 << TIF_32BIT;
pub const _TIF_SVE: usize = 1 << TIF_SVE;
pub const _TIF_MTE_ASYNC_FAULT: usize = 1 << TIF_MTE_ASYNC_FAULT;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_TSC_SIGSEGV: usize = 1 << TIF_TSC_SIGSEGV;
pub const _TIF_SYSCALL_WORK: usize = _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT |
    _TIF_SYSCALL_TRACEPOINT | _TIF_SECCOMP | _TIF_SYSCALL_EMU;

/* INIT_SCS and INIT_THREAD_INFO preserve the source build-time configuration. */
#[macro_export]
macro_rules! INIT_SCS {
    () => {
        #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
        scs_base: init_shadow_call_stack,
        #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
        scs_sp: init_shadow_call_stack,
    };
}

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {{
        let _ = $tsk;
        thread_info {
            flags: _TIF_FOREIGN_FPSTATE,
            preempt: thread_info_preempt { preempt_count: INIT_PREEMPT_COUNT },
            INIT_SCS!()
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
