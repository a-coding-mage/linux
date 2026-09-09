/*
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Translated from the C header guard `_ASM_NIOS2_STRING_H`.

// The declarations below are present only when the original `__KERNEL__`
// preprocessor condition is enabled.  The Rust feature is the corresponding
// build-time condition.
#[cfg(feature = "kernel")]
pub const __HAVE_ARCH_MEMSET: bool = true;
#[cfg(feature = "kernel")]
pub const __HAVE_ARCH_MEMCPY: bool = true;
#[cfg(feature = "kernel")]
pub const __HAVE_ARCH_MEMMOVE: bool = true;

#[cfg(feature = "kernel")]
unsafe extern "C" {
    pub fn memset(
        s: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        count: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memcpy(
        d: *mut core::ffi::c_void,
        s: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memmove(
        d: *mut core::ffi::c_void,
        s: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
