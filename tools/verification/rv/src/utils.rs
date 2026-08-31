// SPDX-License-Identifier: GPL-2.0
/*
 * util functions.
 *
 * Copyright (C) 2022 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

#![feature(c_variadic)]

use std::ffi::{c_char, c_int, VaListImpl};

// C dependencies: <stdarg.h>, <stdio.h>, <utils.h>

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn vsnprintf(
        s: *mut c_char,
        n: usize,
        format: *const c_char,
        arg: VaListImpl,
    ) -> c_int;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub static mut config_debug: c_int = 0;

const MAX_MSG_LENGTH: usize = 1024;

/**
 * err_msg - print an error message to the stderr
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_msg(fmt: *const c_char, mut args: ...) {
    let mut message: [c_char; MAX_MSG_LENGTH] = [0; MAX_MSG_LENGTH];

    unsafe {
        vsnprintf(
            message.as_mut_ptr(),
            std::mem::size_of_val(&message),
            fmt,
            args.as_va_list(),
        );

        fprintf(stderr, c"%s".as_ptr(), message.as_ptr());
    }
}

/**
 * debug_msg - print a debug message to stderr if debug is set
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debug_msg(fmt: *const c_char, mut args: ...) {
    let mut message: [c_char; MAX_MSG_LENGTH] = [0; MAX_MSG_LENGTH];

    unsafe {
        if config_debug == 0 {
            return;
        }

        vsnprintf(
            message.as_mut_ptr(),
            std::mem::size_of_val(&message),
            fmt,
            args.as_va_list(),
        );

        fprintf(stderr, c"%s".as_ptr(), message.as_ptr());
    }
}
