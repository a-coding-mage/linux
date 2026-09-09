/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard and assembler-only conditional omitted; this is the Rust translation. */

/* Dependency supplied by the surrounding translation unit: asm/ptrace.h */

pub struct task_struct;

#[cfg(feature = "CONFIG_SH_FPU")]
#[inline]
pub unsafe fn release_fpu(regs: *mut pt_regs) {
    (*regs).sr |= SR_FD;
}

#[cfg(feature = "CONFIG_SH_FPU")]
#[inline]
pub unsafe fn grab_fpu(regs: *mut pt_regs) {
    (*regs).sr &= !SR_FD;
}

#[cfg(feature = "CONFIG_SH_FPU")]
extern "C" {
    pub fn save_fpu(__tsk: *mut task_struct);
    pub fn restore_fpu(__tsk: *mut task_struct);
    pub fn fpu_state_restore(regs: *mut pt_regs);
    pub fn __fpu_state_restore();
}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn save_fpu(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn restore_fpu(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn release_fpu(_regs: *mut pt_regs) {}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn grab_fpu(_regs: *mut pt_regs) {}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn fpu_state_restore(_regs: *mut pt_regs) {}

#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[inline]
pub unsafe fn __fpu_state_restore(_regs: *mut pt_regs) {}

pub struct user_regset;

extern "C" {
    pub fn do_fpu_inst(_: u16, _: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn init_fpu(_: *mut task_struct) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn __unlazy_fpu(tsk: *mut task_struct, regs: *mut pt_regs) {
    if task_thread_info(tsk).status & TS_USEDFPU != 0 {
        task_thread_info(tsk).status &= !TS_USEDFPU;
        save_fpu(tsk);
        release_fpu(regs);
    } else {
        (*tsk).thread.fpu_counter = 0;
    }
}

#[inline]
pub unsafe fn unlazy_fpu(tsk: *mut task_struct, regs: *mut pt_regs) {
    preempt_disable();
    __unlazy_fpu(tsk, regs);
    preempt_enable();
}

#[inline]
pub unsafe fn clear_fpu(tsk: *mut task_struct, regs: *mut pt_regs) {
    preempt_disable();
    if task_thread_info(tsk).status & TS_USEDFPU != 0 {
        task_thread_info(tsk).status &= !TS_USEDFPU;
        release_fpu(regs);
    }
    preempt_enable();
}

extern "C" {
    pub fn float_raise(flags: u32);
    pub fn float_rounding_mode() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
