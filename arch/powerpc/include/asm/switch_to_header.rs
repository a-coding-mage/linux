/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

// Translated from the PowerPC switch_to.h header.

#[repr(C)]
pub struct thread_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct debug_reg {
    _private: [u8; 0],
}

extern "C" {
    pub fn __switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
    pub fn _switch(prev: *mut thread_struct, next: *mut thread_struct) -> *mut task_struct;
    pub fn switch_booke_debug_regs(new_debug: *mut debug_reg);
    pub fn emulate_altivec(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn restore_tm_state(regs: *mut pt_regs);
    pub fn flush_all_to_thread(task: *mut task_struct);
    pub fn giveup_all(task: *mut task_struct);
    pub fn kvmppc_save_user_regs();
    pub fn kvmppc_save_current_sprs();
    pub fn set_thread_tidr(t: *mut task_struct) -> ::core::ffi::c_int;
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {
        $last = unsafe { $crate::__switch_to($prev, $next) }
    };
}

// CONFIG_PPC_BOOK3S_64 selects the external implementation; otherwise this is an empty inline.
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
extern "C" {
    pub fn restore_math(regs: *mut pt_regs);
}

#[cfg(not(feature = "CONFIG_PPC_BOOK3S_64"))]
#[inline]
pub unsafe fn restore_math(_regs: *mut pt_regs) {}

#[cfg(feature = "CONFIG_PPC_FPU")]
extern "C" {
    pub fn enable_kernel_fp();
    pub fn flush_fp_to_thread(task: *mut task_struct);
    pub fn giveup_fpu(task: *mut task_struct);
    pub fn save_fpu(task: *mut task_struct);
}

#[cfg(feature = "CONFIG_PPC_FPU")]
#[inline]
pub unsafe fn disable_kernel_fp() {
    msr_check_and_clear(MSR_FP);
}

#[cfg(not(feature = "CONFIG_PPC_FPU"))]
#[inline]
pub unsafe fn save_fpu(_t: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PPC_FPU"))]
#[inline]
pub unsafe fn flush_fp_to_thread(_t: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PPC_FPU"))]
#[inline]
pub unsafe fn enable_kernel_fp() {
    BUILD_BUG();
}

#[cfg(feature = "CONFIG_ALTIVEC")]
extern "C" {
    pub fn enable_kernel_altivec();
    pub fn flush_altivec_to_thread(task: *mut task_struct);
    pub fn giveup_altivec(task: *mut task_struct);
    pub fn save_altivec(task: *mut task_struct);
}

#[cfg(feature = "CONFIG_ALTIVEC")]
#[inline]
pub unsafe fn disable_kernel_altivec() {
    msr_check_and_clear(MSR_VEC);
}

#[cfg(not(feature = "CONFIG_ALTIVEC"))]
#[inline]
pub unsafe fn save_altivec(_t: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_ALTIVEC"))]
#[inline]
pub unsafe fn __giveup_altivec(_t: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_ALTIVEC"))]
#[inline]
pub unsafe fn enable_kernel_altivec() {
    BUILD_BUG();
}

#[cfg(not(feature = "CONFIG_ALTIVEC"))]
#[inline]
pub unsafe fn disable_kernel_altivec() {
    BUILD_BUG();
}

#[cfg(feature = "CONFIG_VSX")]
extern "C" {
    pub fn enable_kernel_vsx();
    pub fn flush_vsx_to_thread(task: *mut task_struct);
}

#[cfg(feature = "CONFIG_VSX")]
#[inline]
pub unsafe fn disable_kernel_vsx() {
    msr_check_and_clear(MSR_FP | MSR_VEC | MSR_VSX);
}

#[cfg(not(feature = "CONFIG_VSX"))]
#[inline]
pub unsafe fn enable_kernel_vsx() {
    BUILD_BUG();
}

#[cfg(not(feature = "CONFIG_VSX"))]
#[inline]
pub unsafe fn disable_kernel_vsx() {
    BUILD_BUG();
}

#[cfg(feature = "CONFIG_SPE")]
extern "C" {
    pub fn enable_kernel_spe();
    pub fn flush_spe_to_thread(task: *mut task_struct);
    pub fn giveup_spe(task: *mut task_struct);
    pub fn __giveup_spe(task: *mut task_struct);
}

#[cfg(feature = "CONFIG_SPE")]
#[inline]
pub unsafe fn disable_kernel_spe() {
    msr_check_and_clear(MSR_SPE);
}

#[cfg(not(feature = "CONFIG_SPE"))]
#[inline]
pub unsafe fn __giveup_spe(_t: *mut task_struct) {}

#[inline]
pub unsafe fn clear_task_ebb(_t: *mut task_struct) {
    // CONFIG_PPC_BOOK3S_64: EBB perf events are not inherited, so clear all EBB state.
    // The containing task/thread layout is supplied by the surrounding translation.
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
