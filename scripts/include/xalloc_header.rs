/* SPDX-License-Identifier: GPL-2.0-only */

use std::ffi::{c_char, c_void};

pub type size_t = usize;

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn exit(status: i32) -> !;
}

#[inline]
pub unsafe fn xmalloc(size: size_t) -> *mut c_void {
    let p = unsafe { malloc(size) };

    if p.is_null() {
        unsafe { exit(1) };
    }
    p
}

#[inline]
pub unsafe fn xcalloc(nmemb: size_t, size: size_t) -> *mut c_void {
    let p = unsafe { calloc(nmemb, size) };

    if p.is_null() {
        unsafe { exit(1) };
    }
    p
}

#[inline]
pub unsafe fn xrealloc(mut p: *mut c_void, size: size_t) -> *mut c_void {
    p = unsafe { realloc(p, size) };
    if p.is_null() {
        unsafe { exit(1) };
    }
    p
}

#[inline]
pub unsafe fn xstrdup(s: *const c_char) -> *mut c_char {
    let p = unsafe { strdup(s) };

    if p.is_null() {
        unsafe { exit(1) };
    }
    p
}

#[inline]
pub unsafe fn xstrndup(s: *const c_char, n: size_t) -> *mut c_char {
    let p = unsafe { strndup(s, n) };

    if p.is_null() {
        unsafe { exit(1) };
    }
    p
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
