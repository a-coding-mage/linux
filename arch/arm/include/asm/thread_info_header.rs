/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from arch/arm/include/asm/thread_info.h. */

/* Kernel dependencies supplied by other translated headers. */

#[cfg(CONFIG_KASAN)]
pub const THREAD_SIZE_ORDER: usize = 2;
#[cfg(not(CONFIG_KASAN))]
pub const THREAD_SIZE_ORDER: usize = 1;

pub const THREAD_SIZE: usize = PAGE_SIZE << THREAD_SIZE_ORDER;
pub const THREAD_START_SP: usize = THREAD_SIZE - 8;

#[cfg(CONFIG_VMAP_STACK)]
pub const THREAD_ALIGN: usize = 2 * THREAD_SIZE;
#[cfg(not(CONFIG_VMAP_STACK))]
pub const THREAD_ALIGN: usize = THREAD_SIZE;

pub const OVERFLOW_STACK_SIZE: usize = SZ_4K;

#[repr(C)]
pub struct task_struct;

extern "C" {
    pub static mut __entry_task: *mut task_struct;
}

#[repr(C)]
pub struct cpu_context_save {
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub sl: u32,
    pub fp: u32,
    pub sp: u32,
    pub pc: u32,
    pub extra: [u32; 2], /* Xscale 'acc' register, etc */
}

/* Low level task data that entry.S needs immediate access to. */
#[repr(C)]
pub struct thread_info {
    pub flags: usize, /* low level flags */
    pub preempt_count: i32, /* 0 => preemptable, <0 => bug */
    pub cpu: u32, /* cpu */
    pub cpu_domain: u32, /* cpu domain */
    pub cpu_context: cpu_context_save, /* cpu context */
    pub abi_syscall: u32, /* ABI type and syscall nr */
    pub tp_value: [usize; 2], /* TLS registers */
    pub fpstate: fp_state,
    pub vfpstate: vfp_state,
    #[cfg(CONFIG_ARM_THUMBEE)]
    pub thumbee_state: usize, /* ThumbEE Handler Base register */
}

/* INIT_THREAD_INFO(tsk): flags = 0, preempt_count = INIT_PREEMPT_COUNT. */
pub const fn init_thread_info() -> (usize, i32) {
    (0, INIT_PREEMPT_COUNT)
}

#[inline]
pub unsafe fn thread_task(ti: *mut thread_info) -> *mut task_struct {
    ti as *mut task_struct
}

extern "C" {
    pub fn task_thread_info(tsk: *mut task_struct) -> *mut thread_info;
}

#[inline]
pub unsafe fn thread_saved_pc(tsk: *mut task_struct) -> usize {
    (*task_thread_info(tsk)).cpu_context.pc as usize
}
#[inline]
pub unsafe fn thread_saved_sp(tsk: *mut task_struct) -> usize {
    (*task_thread_info(tsk)).cpu_context.sp as usize
}
#[cfg(not(CONFIG_THUMB2_KERNEL))]
#[inline]
pub unsafe fn thread_saved_fp(tsk: *mut task_struct) -> usize {
    (*task_thread_info(tsk)).cpu_context.fp as usize
}
#[cfg(CONFIG_THUMB2_KERNEL)]
#[inline]
pub unsafe fn thread_saved_fp(tsk: *mut task_struct) -> usize {
    (*task_thread_info(tsk)).cpu_context.r7 as usize
}

extern "C" {
    pub fn iwmmxt_task_disable(ti: *mut thread_info);
    pub fn iwmmxt_task_copy(ti: *mut thread_info, from: *mut core::ffi::c_void);
    pub fn iwmmxt_task_restore(ti: *mut thread_info, from: *mut core::ffi::c_void);
    pub fn iwmmxt_task_release(ti: *mut thread_info);
    pub fn iwmmxt_task_switch(ti: *mut thread_info);
    pub fn iwmmxt_undef_handler(regs: *mut pt_regs, instr: u32) -> i32;
    pub fn register_undef_hook(hook: *mut undef_hook);
}

#[repr(C)]
pub struct undef_hook {
    pub instr_mask: u32,
    pub instr_val: u32,
    pub cpsr_mask: u32,
    pub cpsr_val: u32,
    pub fn_: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> i32>,
}

#[inline]
pub unsafe fn register_iwmmxt_undef_handler() {
    static mut IWMMXT_UNDEF_HOOK: undef_hook = undef_hook {
        instr_mask: 0x0c000e00,
        instr_val: 0x0c000000,
        cpsr_mask: MODE_MASK | PSR_T_BIT,
        cpsr_val: USR_MODE,
        fn_: Some(iwmmxt_undef_handler),
    };
    register_undef_hook(&mut IWMMXT_UNDEF_HOOK);
}

extern "C" {
    pub fn vfp_sync_hwstate(ti: *mut thread_info);
    pub fn vfp_flush_hwstate(ti: *mut thread_info);
    pub fn vfp_preserve_user_clear_hwstate(user: *mut user_vfp, exc: *mut user_vfp_exc) -> i32;
    pub fn vfp_restore_user_hwstate(user: *mut user_vfp, exc: *mut user_vfp_exc) -> i32;
}

pub enum fp_state {}
pub enum vfp_state {}
pub enum pt_regs {}
pub enum user_vfp {}
pub enum user_vfp_exc {}

pub const TIF_SIGPENDING: usize = 0;
pub const TIF_NEED_RESCHED: usize = 1;
pub const TIF_NOTIFY_RESUME: usize = 2;
pub const TIF_UPROBE: usize = 3;
pub const TIF_NOTIFY_SIGNAL: usize = 4;
pub const TIF_USING_IWMMXT: usize = 17;
pub const TIF_MEMDIE: usize = 18;
pub const TIF_RESTORE_SIGMASK: usize = 19;
pub const TIF_SYSCALL_TRACE: usize = 20;
pub const TIF_SYSCALL_AUDIT: usize = 21;
pub const TIF_SYSCALL_TRACEPOINT: usize = 22;
pub const TIF_SECCOMP: usize = 23;

pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_UPROBE: usize = 1 << TIF_UPROBE;
pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_SECCOMP: usize = 1 << TIF_SECCOMP;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_USING_IWMMXT: usize = 1 << TIF_USING_IWMMXT;
pub const _TIF_SYSCALL_WORK: usize = _TIF_SYSCALL_TRACE | _TIF_SYSCALL_AUDIT |
    _TIF_SYSCALL_TRACEPOINT | _TIF_SECCOMP;
pub const _TIF_WORK_MASK: usize = _TIF_NEED_RESCHED | _TIF_SIGPENDING |
    _TIF_NOTIFY_RESUME | _TIF_UPROBE | _TIF_NOTIFY_SIGNAL;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
