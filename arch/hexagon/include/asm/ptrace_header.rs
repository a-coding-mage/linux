/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Ptrace definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency equivalent of: #include <uapi/asm/ptrace.h>

/* kprobe-based event tracer support */
unsafe extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn regs_query_register_name(offset: core::ffi::c_uint) -> *const core::ffi::c_char;
}

/// Equivalent of the `current_pt_regs()` macro.
#[inline]
pub unsafe fn current_pt_regs() -> *mut crate::pt_regs {
    (((crate::current_thread_info() as usize + crate::THREAD_SIZE) as *mut crate::pt_regs)
        .offset(-1))
}

// Equivalent of: #if CONFIG_HEXAGON_ARCH_VERSION >= 4
#[cfg(feature = "CONFIG_HEXAGON_ARCH_VERSION_GE_4")]
pub const ARCH_HAS_SINGLE_STEP: core::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
