/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

pub unsafe fn test_start(name: *const c_char) {
    unsafe {
        printf(c"test: %s\n".as_ptr(), name);
    }
}

pub unsafe fn test_failure_detail(name: *const c_char, detail: *const c_char) {
    unsafe {
        printf(c"failure: %s [%s]\n".as_ptr(), name, detail);
    }
}

pub unsafe fn test_failure(name: *const c_char) {
    unsafe {
        printf(c"failure: %s\n".as_ptr(), name);
    }
}

pub unsafe fn test_error(name: *const c_char) {
    unsafe {
        printf(c"error: %s\n".as_ptr(), name);
    }
}

pub unsafe fn test_skip(name: *const c_char) {
    unsafe {
        printf(c"skip: %s\n".as_ptr(), name);
    }
}

pub unsafe fn test_success(name: *const c_char) {
    unsafe {
        printf(c"success: %s\n".as_ptr(), name);
    }
}

pub unsafe fn test_finish(name: *const c_char, status: c_int) {
    if status != 0 {
        unsafe {
            test_failure(name);
        }
    } else {
        unsafe {
            test_success(name);
        }
    }
}

pub unsafe fn test_set_git_version(value: *const c_char) {
    unsafe {
        printf(c"tags: git_version:%s\n".as_ptr(), value);
    }
}
