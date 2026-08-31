// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/util.c. Original dependencies:
// "util.h", "../util/debug.h", and <stdio.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

// External C/runtime declarations supplied by the surrounding project/libc.
pub enum FILE {}

extern "C" {
    static mut stderr: *mut FILE;
    static mut quiet: bool;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, args: va_list) -> c_int;
}

// va_list is supplied by the platform C ABI. The exact representation is
// external to this isolated translation unit.
pub type va_list = *mut c_void;

#[repr(C)]
pub struct perf_error_ops {
    pub error: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
    pub warning: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
}

/*
 * Default error logging functions
 */
unsafe extern "C" fn perf_stdio__error(format: *const c_char, args: va_list) -> c_int {
    unsafe {
        fprintf(stderr, b"Error:\n\0".as_ptr() as *const c_char);
        vfprintf(stderr, format, args);
    }
    0
}

unsafe extern "C" fn perf_stdio__warning(format: *const c_char, args: va_list) -> c_int {
    unsafe {
        if quiet {
            return 0;
        }

        fprintf(stderr, b"Warning:\n\0".as_ptr() as *const c_char);
        vfprintf(stderr, format, args);
    }
    0
}

static mut default_eops: perf_error_ops = perf_error_ops {
    error: Some(perf_stdio__error),
    warning: Some(perf_stdio__warning),
};

static mut perf_eops: *mut perf_error_ops = unsafe { &raw mut default_eops };

pub unsafe extern "C" fn ui__error(format: *const c_char, mut args: ...) -> c_int {
    let ret: c_int;

    unsafe {
        ret = ((*perf_eops).error.unwrap())(format, args.as_va_list() as va_list);
    }

    ret
}

pub unsafe extern "C" fn ui__warning(format: *const c_char, mut args: ...) -> c_int {
    let ret: c_int;
    unsafe {
        if quiet {
            return 0;
        }

        ret = ((*perf_eops).warning.unwrap())(format, args.as_va_list() as va_list);
    }

    ret
}

/**
 * perf_error__register - Register error logging functions
 * @eops: The pointer to error logging function struct
 *
 * Register UI-specific error logging functions. Before calling this,
 * other logging functions should be unregistered, if any.
 */
pub unsafe extern "C" fn perf_error__register(eops: *mut perf_error_ops) -> c_int {
    unsafe {
        if !core::ptr::addr_eq(perf_eops, &raw mut default_eops) {
            return -1;
        }

        perf_eops = eops;
    }
    0
}

/**
 * perf_error__unregister - Unregister error logging functions
 * @eops: The pointer to error logging function struct
 *
 * Unregister already registered error logging functions.
 */
pub unsafe extern "C" fn perf_error__unregister(eops: *mut perf_error_ops) -> c_int {
    unsafe {
        if perf_eops != eops {
            return -1;
        }

        perf_eops = &raw mut default_eops;
    }
    0
}
