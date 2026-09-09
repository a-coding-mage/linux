/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 ARM Ltd.
 */

use core::ffi::{c_char, c_void};

// The following declarations are omitted when CONFIG_KASAN_GENERIC or
// CONFIG_KASAN_SW_TAGS is enabled.
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRRCHR: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRCHR: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRCMP: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRNCMP: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRLEN: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_STRNLEN: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_MEMCMP: bool = true;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const __HAVE_ARCH_MEMCHR: bool = true;

pub const __HAVE_ARCH_MEMCPY: bool = true;
pub const __HAVE_ARCH_MEMMOVE: bool = true;
pub const __HAVE_ARCH_MEMSET: bool = true;

extern "C" {
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strrchr(s: *const c_char, c: i32) -> *mut c_char;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strchr(s: *const c_char, c: i32) -> *mut c_char;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strlen(s: *const c_char) -> usize;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn strnlen(s: *const c_char, maxlen: usize) -> usize;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32;
    #[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
    pub fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void;

    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn __memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn __memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    pub fn __memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;

    #[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
    pub fn memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize);
}

#[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
pub const __HAVE_ARCH_MEMCPY_FLUSHCACHE: bool = true;

// When KASAN is enabled and this translation unit is not address-sanitizer
// instrumented, the C macros redirect memcpy, memmove, and memset to their
// corresponding __mem* implementations. __NO_FORTIFY is also defined there.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
