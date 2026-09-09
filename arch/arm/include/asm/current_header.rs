/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 Keith Packard <keithp@keithp.com>
 * Copyright (c) 2021 Google, LLC <ardb@kernel.org>
 */

//! ARM current-task pointer declarations.
//!
//! The original header is excluded during assembly builds.  Its assembler
//! alternatives are retained below as conditional intent; the compiler-side
//! interface is represented by the external current pointer.

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

extern "C" {
    pub static mut __current: *mut task_struct;
}

/// Return the current task pointer.
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    /*
     * CONFIG_CURRENT_POINTER_IN_TPIDRURO (and CONFIG_SMP) use the ARM
     * TPIDRURO register, with the ARMv6 relocation/alternative sequences
     * from the C header.  Rust inline assembly for those build-specific
     * kernel alternatives is intentionally left to the target integration;
     * the externally supplied pointer is the corresponding fallback.
     */
    __current
}

/// C header equivalent of `current`.
#[inline(always)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
