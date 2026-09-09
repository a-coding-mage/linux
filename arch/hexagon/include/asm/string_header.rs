/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C header guard: _ASM_STRING_H_

// The following declarations are present only when the C __KERNEL__
// conditional is enabled.
pub const __HAVE_ARCH_MEMCPY: bool = true;

unsafe extern "C" {
    pub fn memcpy(
        __to: *mut core::ffi::c_void,
        __from: *const core::ffi::c_void,
        __n: usize,
    ) -> *mut core::ffi::c_void;
}

/* ToDo: use dczeroa, accelerate the compiler-constant zero case */
pub const __HAVE_ARCH_MEMSET: bool = true;

unsafe extern "C" {
    pub fn memset(
        __to: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        __n: usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
