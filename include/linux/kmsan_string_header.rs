/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KMSAN string functions API used in other headers.
 *
 * Copyright (C) 2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 *
 */

/*
 * KMSAN overrides the default memcpy/memset/memmove implementations in the
 * kernel, which requires having __msan_XXX function prototypes in several other
 * headers. Keep them in one place instead of open-coding.
 */
extern "C" {
    pub fn __msan_memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    pub fn __msan_memset(s: *mut core::ffi::c_void, c: core::ffi::c_int, n: usize) -> *mut core::ffi::c_void;
    pub fn __msan_memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
