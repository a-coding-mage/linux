/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header guard: _ASM_MICROBLAZE_STRING_H

// The following declarations are present only when __KERNEL__ is defined in
// the C build, and CONFIG_OPT_LIB_FUNCTION is enabled.
// C: #define __HAVE_ARCH_MEMSET
// C: #define __HAVE_ARCH_MEMCPY
// C: #define __HAVE_ARCH_MEMMOVE

unsafe extern "C" {
    pub fn memset(
        destination: *mut core::ffi::c_void,
        value: i32,
        size: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memcpy(
        destination: *mut core::ffi::c_void,
        source: *const core::ffi::c_void,
        size: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memmove(
        destination: *mut core::ffi::c_void,
        source: *const core::ffi::c_void,
        size: usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
