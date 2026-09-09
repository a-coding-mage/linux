/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 SiFive
 */

// Dependencies supplied by linux/preempt.h and asm/neon.h.
extern "C" {
    pub fn in_task() -> bool;
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn kernel_neon_begin(arg: *mut core::ffi::c_void);
    pub fn kernel_neon_end(arg: *mut core::ffi::c_void);
    pub fn cpu_has_neon() -> bool;
}

#[inline]
pub unsafe fn kernel_fpu_available() -> bool {
    cpu_has_neon()
}

#[inline]
pub unsafe fn kernel_fpu_begin() {
    // Equivalent intent of BUG_ON(!in_task()).
    if !in_task() {
        core::hint::unreachable_unchecked();
    }
    preempt_disable();
    kernel_neon_begin(core::ptr::null_mut());
}

#[inline]
pub unsafe fn kernel_fpu_end() {
    kernel_neon_end(core::ptr::null_mut());
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
