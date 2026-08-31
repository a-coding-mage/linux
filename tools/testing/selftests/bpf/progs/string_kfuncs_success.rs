// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Red Hat, Inc.*/
// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "errno.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

const ENOENT: c_int = 2;

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
    fn bpf_strnstr(haystack: *const c_char, needle: *const c_char, len: usize) -> c_int;
    fn bpf_strncasestr(haystack: *const c_char, needle: *const c_char, len: usize) -> c_int;
}

#[no_mangle]
pub static mut r#str: [u8; 12] = *b"hello world\0";

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! str_ptr {
    () => {
        unsafe { r#str.as_ptr() as *const c_char }
    };
}

// #define __test(retval) SEC("syscall") __success __retval(retval)

/* Functional tests */
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_eq(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(str_ptr!(), cstr!("hello world")) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcmp_neq(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcmp(str_ptr!(), cstr!("hello")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_eq1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(str_ptr!(), cstr!("hello world")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_eq2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(str_ptr!(), cstr!("HELLO WORLD")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_eq3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(str_ptr!(), cstr!("HELLO world")) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_neq1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(str_ptr!(), cstr!("hello")) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasecmp_neq2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasecmp(str_ptr!(), cstr!("HELLO")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_eq1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("hello world"), 11) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_eq2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("HELLO WORLD"), 11) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_eq3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("HELLO world"), 11) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_eq4(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("hello"), 5) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_eq5(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("hello world!"), 11) } }
// __test(-1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_neq1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("hello!"), 6) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasecmp_neq2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasecmp(str_ptr!(), cstr!("abc"), 3) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(str_ptr!(), b'e' as c_int) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(str_ptr!(), b'\0' as c_int) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchr_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchr(str_ptr!(), b'x' as c_int) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchrnul(str_ptr!(), b'e' as c_int) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strchrnul_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strchrnul(str_ptr!(), b'x' as c_int) } }
// __test(1)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(str_ptr!(), 5, b'e' as c_int) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(str_ptr!(), 12, b'\0' as c_int) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnchr_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnchr(str_ptr!(), 5, b'w' as c_int) } }
// __test(9)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(str_ptr!(), b'l' as c_int) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_null(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(str_ptr!(), b'\0' as c_int) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strrchr_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strrchr(str_ptr!(), b'x' as c_int) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strlen(_ctx: *mut c_void) -> c_int { unsafe { bpf_strlen(str_ptr!()) } }
// __test(11)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnlen(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnlen(str_ptr!(), 12) } }
// __test(5)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strspn(_ctx: *mut c_void) -> c_int { unsafe { bpf_strspn(str_ptr!(), cstr!("ehlo")) } }
// __test(2)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcspn(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcspn(str_ptr!(), cstr!("lo")) } }
// __test(6)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(str_ptr!(), cstr!("world")) } }
// __test(6)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_found(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(str_ptr!(), cstr!("woRLD")) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(str_ptr!(), cstr!("hi")) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_notfound(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(str_ptr!(), cstr!("hi")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strstr_empty(_ctx: *mut c_void) -> c_int { unsafe { bpf_strstr(str_ptr!(), cstr!("")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strcasestr_empty(_ctx: *mut c_void) -> c_int { unsafe { bpf_strcasestr(str_ptr!(), cstr!("")) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_found1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(cstr!(""), cstr!(""), 0) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_found2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(str_ptr!(), cstr!("hello"), 5) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_found3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(str_ptr!(), cstr!("hello"), 6) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_notfound1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(str_ptr!(), cstr!("hi"), 10) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_notfound2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(str_ptr!(), cstr!("hello"), 4) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_notfound3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(cstr!(""), cstr!("a"), 0) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strnstr_empty(_ctx: *mut c_void) -> c_int { unsafe { bpf_strnstr(str_ptr!(), cstr!(""), 1) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_found1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(cstr!(""), cstr!(""), 0) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_found2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(str_ptr!(), cstr!("heLLO"), 5) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_found3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(str_ptr!(), cstr!("heLLO"), 6) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_notfound1(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(str_ptr!(), cstr!("hi"), 10) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_notfound2(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(str_ptr!(), cstr!("hello"), 4) } }
// __test(-ENOENT)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_notfound3(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(cstr!(""), cstr!("a"), 0) } }
// __test(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn test_strncasestr_empty(_ctx: *mut c_void) -> c_int { unsafe { bpf_strncasestr(str_ptr!(), cstr!(""), 1) } }

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
