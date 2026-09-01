// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Red Hat, Inc.*/
// Translated from C. Original dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <linux/limits.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const XATTR_SIZE_MAX: usize = 65536;

unsafe extern "C" {
    fn bpf_strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn bpf_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn bpf_strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn bpf_strchr(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strchrnul(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strnchr(s: *const c_char, n: usize, c: c_int) -> c_int;
    fn bpf_strrchr(s: *const c_char, c: c_int) -> c_int;
    fn bpf_strlen(s: *const c_char) -> c_int;
    fn bpf_strnlen(s: *const c_char, n: usize) -> c_int;
    fn bpf_strspn(s: *const c_char, accept: *const c_char) -> c_int;
    fn bpf_strcspn(s: *const c_char, reject: *const c_char) -> c_int;
    fn bpf_strstr(haystack: *const c_char, needle: *const c_char) -> c_int;
    fn bpf_strcasestr(haystack: *const c_char, needle: *const c_char) -> c_int;
    fn bpf_strnstr(haystack: *const c_char, needle: *const c_char, n: usize) -> c_int;
    fn bpf_strncasestr(haystack: *const c_char, needle: *const c_char, n: usize) -> c_int;
}

#[no_mangle]
pub static mut long_str: [c_char; XATTR_SIZE_MAX + 1] = [0; XATTR_SIZE_MAX + 1];

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strcmp(long_str.as_ptr(), long_str.as_ptr()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strcasecmp(long_str.as_ptr(), long_str.as_ptr()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strncasecmp(long_str.as_ptr(), long_str.as_ptr(), long_str.len()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strchr(long_str.as_ptr(), b'b' as c_int) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strchrnul(long_str.as_ptr(), b'b' as c_int) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strnchr(long_str.as_ptr(), long_str.len(), b'b' as c_int) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strrchr(long_str.as_ptr(), b'b' as c_int) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strlen_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strlen(long_str.as_ptr()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnlen_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strnlen(long_str.as_ptr(), long_str.len()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_str_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strspn(long_str.as_ptr(), b"a\0".as_ptr() as *const c_char) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn_accept_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strspn(b"b\0".as_ptr() as *const c_char, long_str.as_ptr()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_str_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strcspn(long_str.as_ptr(), b"b\0".as_ptr() as *const c_char) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn_reject_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strcspn(b"b\0".as_ptr() as *const c_char, long_str.as_ptr()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strstr(long_str.as_ptr(), b"hello\0".as_ptr() as *const c_char) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strcasestr(long_str.as_ptr(), b"hello\0".as_ptr() as *const c_char) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strnstr(long_str.as_ptr(), b"hello\0".as_ptr() as *const c_char, long_str.len()) }
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_too_long(ctx: *mut c_void) -> c_int {
    unsafe { bpf_strncasestr(long_str.as_ptr(), b"hello\0".as_ptr() as *const c_char, long_str.len()) }
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
