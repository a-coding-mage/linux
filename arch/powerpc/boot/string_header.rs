/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header guarded by _PPC_BOOT_STRING_H_.

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C <stddef.h> size_t.
pub type size_t = usize;

unsafe extern "C" {
    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn strnlen(s: *const c_char, count: size_t) -> size_t;

    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
    pub fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
