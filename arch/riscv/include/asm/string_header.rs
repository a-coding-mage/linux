/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Regents of the University of California
 */

// Translated from the guarded C header `_ASM_RISCV_STRING_H`.

pub const __HAVE_ARCH_MEMSET: bool = true;
pub const __HAVE_ARCH_MEMCPY: bool = true;
pub const __HAVE_ARCH_MEMMOVE: bool = true;

unsafe extern "C" {
    // `asmlinkage` is a platform calling-convention annotation in the source.
    pub fn memset(dest: *mut core::ffi::c_void, value: core::ffi::c_int, n: usize)
        -> *mut core::ffi::c_void;
    pub fn __memset(dest: *mut core::ffi::c_void, value: core::ffi::c_int, n: usize)
        -> *mut core::ffi::c_void;
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    pub fn __memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    pub fn __memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

// The following declarations are omitted when CONFIG_KASAN_GENERIC or
// CONFIG_KASAN_SW_TAGS is enabled in the C build.
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRCMP: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRLEN: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRNCMP: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRNLEN: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRCHR: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRRCHR: bool = true;

#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
unsafe extern "C" {
    pub fn strcmp(cs: *const core::ffi::c_char, ct: *const core::ffi::c_char)
        -> core::ffi::c_int;
    pub fn strlen(s: *const core::ffi::c_char) -> usize;
    pub fn strncmp(
        cs: *const core::ffi::c_char,
        ct: *const core::ffi::c_char,
        count: usize,
    ) -> core::ffi::c_int;
    pub fn strnlen(s: *const core::ffi::c_char, maxlen: usize) -> usize;
    pub fn strchr(s: *const core::ffi::c_char, c: core::ffi::c_int)
        -> *mut core::ffi::c_char;
    pub fn strrchr(s: *const core::ffi::c_char, c: core::ffi::c_int)
        -> *mut core::ffi::c_char;
}

// For those files which don't want to check by kasan.
// When CONFIG_KASAN is defined and __SANITIZE_ADDRESS__ is not defined,
// the C header provides the following function-like macro substitutions:
//   memcpy(dst, src, len) -> __memcpy(dst, src, len)
//   memset(s, c, n)       -> __memset(s, c, n)
//   memmove(dst, src, len) -> __memmove(dst, src, len)
// It also defines __NO_FORTIFY unless already defined because FORTIFY_SOURCE
// uses __builtin_memcpy and related builtins.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
