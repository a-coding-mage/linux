/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-sh/processor.h
 *
 * Copyright (C) 1999, 2000  Niibe Yutaka
 * Copyright (C) 2002, 2003  Paul Mundt
 */

// C header dependencies are supplied by the surrounding translation unit.

/* Core Processor Version Register */
pub const CCN_PVR: usize = 0xff000030;
pub const CCN_CVR: usize = 0xff000040;
pub const CCN_PRR: usize = 0xff000044;

/* User space process size: 2GB. */
pub const TASK_SIZE: usize = 0x7c000000;
pub const STACK_TOP: usize = TASK_SIZE;
pub const STACK_TOP_MAX: usize = STACK_TOP;

/* PAGE_ALIGN(TASK_SIZE / 3), as supplied by the target page definitions. */
pub const TASK_UNMAPPED_BASE: usize = page_align(TASK_SIZE / 3);

/* Bit of SR register */
pub const SR_DSP: usize = 0x00001000;
pub const SR_IMASK: usize = 0x000000f0;
pub const SR_FD: usize = 0x00008000;
pub const SR_MD: usize = 0x40000000;
pub const SR_USER_MASK: usize = 0x00000303; // M, Q, S, T bits

#[repr(C)]
pub struct sh_dsp_struct {
    pub dsp_regs: [usize; 14],
    pub status: isize,
}

#[repr(C)]
pub struct sh_fpu_hard_struct {
    pub fp_regs: [usize; 16],
    pub xfp_regs: [usize; 16],
    pub fpscr: usize,
    pub fpul: usize,
    pub status: isize, /* software status information */
}

#[repr(C)]
pub struct sh_fpu_soft_struct {
    pub fp_regs: [usize; 16],
    pub xfp_regs: [usize; 16],
    pub fpscr: usize,
    pub fpul: usize,
    pub lookahead: u8,
    pub entry_pc: usize,
}

#[repr(C)]
pub union thread_xstate {
    pub hardfpu: sh_fpu_hard_struct,
    pub softfpu: sh_fpu_soft_struct,
}

#[repr(C)]
pub struct thread_struct {
    /* Saved registers when thread is descheduled */
    pub sp: usize,
    pub pc: usize,
    /* Various thread flags, see SH_THREAD_xxx */
    pub flags: usize,
    /* Save middle states of ptrace breakpoints */
    pub ptrace_bps: [*mut perf_event; HBP_NUM],
    // CONFIG_SH_DSP conditionally includes dsp_status here.
    #[cfg(CONFIG_SH_DSP)]
    pub dsp_status: sh_dsp_struct,
    /* Extended processor state */
    pub xstate: *mut thread_xstate,
    /* Number of consecutive context switches that used the FPU. */
    pub fpu_counter: u8,
}

/* INIT_THREAD initializer; init_stack is supplied by the surrounding code. */
// pub const INIT_THREAD: thread_struct = thread_struct { ... };

/* Forward declaration, a strange C thing */
pub struct task_struct;

unsafe extern "C" {
    pub fn start_thread(regs: *mut pt_regs, new_pc: usize, new_sp: usize);
}

/* FPU lazy state save handling. */
#[inline]
pub unsafe fn disable_fpu() {
    let mut dummy: usize;
    core::arch::asm!(
        "stc sr, {0}",
        "or {1}, {0}",
        "ldc {0}, sr",
        out(reg) dummy,
        in(reg) SR_FD,
    );
}

#[inline]
pub unsafe fn enable_fpu() {
    let mut dummy: usize;
    core::arch::asm!(
        "stc sr, {0}",
        "and {1}, {0}",
        "ldc {0}, sr",
        out(reg) dummy,
        in(reg) !SR_FD,
    );
}

/* Double precision, NANS as NANS, rounding to nearest, no exceptions */
pub const FPSCR_INIT: usize = 0x00080000;
pub const FPSCR_CAUSE_MASK: usize = 0x0001f000; /* Cause bits */
pub const FPSCR_FLAG_MASK: usize = 0x0000007c; /* Flag bits */

/* C macro: (tsk->thread.pc) */
#[macro_export]
macro_rules! thread_saved_pc {
    ($tsk:expr) => { $tsk.thread.pc };
}

unsafe extern "C" {
    pub fn show_trace(
        tsk: *mut task_struct,
        sp: *mut usize,
        regs: *mut pt_regs,
        loglvl: *const core::ffi::c_char,
    );
}

#[cfg(CONFIG_DUMP_CODE)]
unsafe extern "C" {
    pub fn show_code(regs: *mut pt_regs);
}

#[cfg(not(CONFIG_DUMP_CODE))]
#[inline]
pub unsafe fn show_code(_regs: *mut pt_regs) {}

unsafe extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> usize;
}

#[inline]
pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> usize {
    (*task_pt_regs(tsk)).pc
}

#[inline]
pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> usize {
    (*task_pt_regs(tsk)).regs[15]
}

#[cfg(any(CONFIG_CPU_SH2A, CONFIG_CPU_SH4))]
pub const PREFETCH_STRIDE: usize = L1_CACHE_BYTES;

#[cfg(any(CONFIG_CPU_SH2A, CONFIG_CPU_SH4))]
#[inline]
pub unsafe fn prefetch(x: *const core::ffi::c_void) {
    core::intrinsics::prefetch_read_data(x, 3);
}

#[cfg(any(CONFIG_CPU_SH2A, CONFIG_CPU_SH4))]
#[inline]
pub unsafe fn prefetchw(x: *const core::ffi::c_void) {
    core::intrinsics::prefetch_write_data(x, 3);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
