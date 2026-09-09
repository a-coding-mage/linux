/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Translated from asm/video.h.  The Linux compiler and asm-generic/video.h
// dependencies are supplied by the surrounding translation.

#[inline]
pub unsafe fn fb_memcpy_fromio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) {
    // C casts away __iomem/volatile qualification before calling memcpy.
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
}

// The C macro expands to the function of the same name.

#[inline]
pub unsafe fn fb_memcpy_toio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) {
    // C casts away __iomem/volatile qualification before calling memcpy.
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
}

// The C macro expands to the function of the same name.

#[inline]
pub unsafe fn fb_memset_io(addr: *mut core::ffi::c_void, c: i32, n: usize) {
    core::ptr::write_bytes(addr as *mut u8, c as u8, n);
}

// #define fb_memset fb_memset_io
pub use fb_memset_io as fb_memset;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
