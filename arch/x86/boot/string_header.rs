/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: BOOT_STRING_H

// Undef any of these macros coming from string_32.h.
// #undef memcpy
// #undef memset
// #undef memcmp

// C size_t is represented by usize.
extern "C" {
    pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn memset(dst: *mut core::ffi::c_void, c: core::ffi::c_int, len: usize) -> *mut core::ffi::c_void;
    pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, len: usize) -> core::ffi::c_int;
    pub fn bcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, len: usize) -> core::ffi::c_int;

    // Access builtin version by default.
    // #define memcpy(d,s,l) __builtin_memcpy(d,s,l)
    // #define memset(d,c,l) __builtin_memset(d,c,l)
    // #define memcmp __builtin_memcmp

    pub fn strcmp(str1: *const core::ffi::c_char, str2: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn strncmp(cs: *const core::ffi::c_char, ct: *const core::ffi::c_char, count: usize) -> core::ffi::c_int;
    pub fn strlen(s: *const core::ffi::c_char) -> usize;
    pub fn strstr(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strchr(s: *const core::ffi::c_char, c: core::ffi::c_int) -> *mut core::ffi::c_char;
    pub fn strnlen(s: *const core::ffi::c_char, maxlen: usize) -> usize;
    pub fn simple_strtoull(
        cp: *const core::ffi::c_char,
        endp: *mut *mut core::ffi::c_char,
        base: core::ffi::c_uint,
    ) -> core::ffi::c_ulonglong;
    pub fn simple_strtol(
        cp: *const core::ffi::c_char,
        endp: *mut *mut core::ffi::c_char,
        base: core::ffi::c_uint,
    ) -> core::ffi::c_long;

    pub fn boot_kstrtoul(
        s: *const core::ffi::c_char,
        base: core::ffi::c_uint,
        res: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
