/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1994 Linus Torvalds
 *
 * Pentium III FXSR, SSE support
 * General FPU state handling cleanups
 *	Gareth Hughes <gareth@valinux.com>, May 2000
 * x86-64 work by Andi Kleen 2002
 */

// Translated from the C header. Types and functions supplied by other headers
// remain external dependencies.

pub const KFPU_387: u32 = 1u32 << 0;
pub const KFPU_MXCSR: u32 = 1u32 << 1;

extern "C" {
    pub fn kernel_fpu_begin_mask(kfpu_mask: u32);
    pub fn kernel_fpu_end();
    pub fn irq_fpu_usable() -> bool;
    pub fn fpregs_mark_activate();
}

#[inline]
pub unsafe fn kernel_fpu_begin() {
    #[cfg(target_pointer_width = "64")]
    {
        // Any 64-bit code that uses 387 instructions must explicitly request KFPU_387.
        kernel_fpu_begin_mask(KFPU_MXCSR);
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        // 32-bit kernel code may use 387 operations as well as SSE2, etc.
        kernel_fpu_begin_mask(KFPU_387 | KFPU_MXCSR);
    }
}

#[inline]
pub unsafe fn fpregs_lock() {
    // CONFIG_PREEMPT_RT selects the preemption path in the original build.
    #[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
    local_bh_disable();
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    preempt_disable();
}

#[inline]
pub unsafe fn fpregs_unlock() {
    #[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
    local_bh_enable();
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    preempt_enable();
}

extern "C" {
    pub fn fpregs_lock_and_load();
}

#[cfg(feature = "CONFIG_X86_DEBUG_FPU")]
extern "C" {
    pub fn fpregs_assert_state_consistent();
}
#[cfg(not(feature = "CONFIG_X86_DEBUG_FPU"))]
#[inline]
pub fn fpregs_assert_state_consistent() {}

extern "C" {
    pub fn switch_fpu_return();
    pub fn cpu_has_xfeatures(xfeatures_mask: u64, feature_name: *mut *const core::ffi::c_char) -> i32;
    pub fn fpu__exception_code(fpu: *mut fpu, trap_nr: i32) -> i32;
    pub fn fpu_sync_fpstate(fpu: *mut fpu);
    pub fn fpu_reset_from_exception_fixup();
    pub fn fpu__init_cpu();
    pub fn fpu__init_system();
    pub fn fpu__init_check_bugs();
    pub fn fpu__resume_cpu();
    pub fn fpstate_clear_xstate_component(fpstate: *mut fpstate, xfeature: u32);
    pub fn xstate_get_guest_group_perm() -> u64;
    pub fn get_xsave_addr(xsave: *mut xregs_state, xfeature_nr: i32) -> *mut core::ffi::c_void;
    pub fn fpu_alloc_guest_fpstate(gfpu: *mut fpu_guest) -> bool;
    pub fn fpu_free_guest_fpstate(gfpu: *mut fpu_guest);
    pub fn fpu_swap_kvm_fpstate(gfpu: *mut fpu_guest, enter_guest: bool) -> i32;
    pub fn fpu_enable_guest_xfd_features(guest_fpu: *mut fpu_guest, xfeatures: u64) -> i32;
    pub fn fpu_copy_guest_fpstate_to_uabi(gfpu: *mut fpu_guest, buf: *mut core::ffi::c_void, size: u32, xfeatures: u64, pkru: u32);
    pub fn fpu_copy_uabi_to_guest_fpstate(gfpu: *mut fpu_guest, buf: *const core::ffi::c_void, xcr0: u64, vpkru: *mut u32) -> i32;
    pub fn fpu_xstate_prctl(option: i32, arg2: usize) -> isize;
    pub fn fpu_idle_fpregs();
}

#[cfg(feature = "CONFIG_X86_64")]
extern "C" {
    pub fn fpstate_free(fpu: *mut fpu);
}
#[cfg(not(feature = "CONFIG_X86_64"))]
#[inline]
pub unsafe fn fpstate_free(_fpu: *mut fpu) {}

#[cfg(feature = "CONFIG_X86_64")]
extern "C" {
    pub fn fpu_update_guest_xfd(guest_fpu: *mut fpu_guest, xfd: u64);
    pub fn fpu_sync_guest_vmexit_xfd_state();
}
#[cfg(not(feature = "CONFIG_X86_64"))]
#[inline]
pub unsafe fn fpu_update_guest_xfd(_guest_fpu: *mut fpu_guest, _xfd: u64) {}
#[cfg(not(feature = "CONFIG_X86_64"))]
#[inline]
pub unsafe fn fpu_sync_guest_vmexit_xfd_state() {}

#[inline]
pub unsafe fn fpstate_set_confidential(gfpu: *mut fpu_guest) {
    (*(*gfpu).fpstate).is_confidential = true;
}

#[inline]
pub unsafe fn fpstate_is_confidential(gfpu: *mut fpu_guest) -> bool {
    (*(*gfpu).fpstate).is_confidential
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
