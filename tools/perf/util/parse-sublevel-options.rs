// Translated from C source:
// #include <stdlib.h>
// #include <stdint.h>
// #include <string.h>
// #include <stdio.h>
//
// #include "util/debug.h"
// #include "util/parse-sublevel-options.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct sublevel_option {
    pub name: *const c_char,
    pub value_ptr: *mut c_int,
}

unsafe extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn free(ptr: *mut core::ffi::c_void);
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;

    fn pr_err(fmt: *const c_char, ...) -> c_int;
}

unsafe fn parse_one_sublevel_option(
    str_: *const c_char,
    opts: *mut sublevel_option,
) -> c_int {
    let mut opt = opts;
    let mut vstr: *mut c_char;
    let s = unsafe { strdup(str_) };
    let mut v: c_int = 1;

    if s.is_null() {
        unsafe {
            pr_err(c"no memory\n".as_ptr());
        }
        return -1;
    }

    vstr = unsafe { strchr(s, '=' as c_int) };
    if !vstr.is_null() {
        unsafe {
            *vstr = 0;
        }
        vstr = unsafe { vstr.add(1) };
    }

    while unsafe { !(*opt).name.is_null() } {
        if unsafe { strcmp(s, (*opt).name) } == 0 {
            break;
        }
        opt = unsafe { opt.add(1) };
    }

    if unsafe { (*opt).name.is_null() } {
        unsafe {
            pr_err(c"Unknown option name '%s'\n".as_ptr(), s);
            free(s.cast());
        }
        return -1;
    }

    if !vstr.is_null() {
        v = unsafe { atoi(vstr) };
    }

    unsafe {
        *(*opt).value_ptr = v;
        free(s.cast());
    }
    0
}

/* parse options like --foo a=<n>,b,c... */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_parse_sublevel_options(
    str_: *const c_char,
    opts: *mut sublevel_option,
) -> c_int {
    let s = unsafe { strdup(str_) };
    let mut p: *mut c_char = core::ptr::null_mut();
    let mut ret: c_int;

    if s.is_null() {
        unsafe {
            pr_err(c"no memory\n".as_ptr());
        }
        return -1;
    }

    p = unsafe { strtok(s, c",".as_ptr()) };
    while !p.is_null() {
        ret = unsafe { parse_one_sublevel_option(p, opts) };
        if ret != 0 {
            unsafe {
                free(s.cast());
            }
            return ret;
        }

        p = unsafe { strtok(core::ptr::null_mut(), c",".as_ptr()) };
    }

    unsafe {
        free(s.cast());
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
