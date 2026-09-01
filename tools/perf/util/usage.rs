// SPDX-License-Identifier: GPL-2.0
/*
 * usage.c
 *
 * Various reporting routines.
 * Originally copied from GIT source.
 *
 * Copyright (C) Linus Torvalds, 2005
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[unsafe(no_mangle)]
pub static perf_usage_string: [u8; { b"perf [--version] [--help] [OPTIONS] COMMAND [ARGS]\0".len() }] =
    *b"perf [--version] [--help] [OPTIONS] COMMAND [ARGS]\0";

#[unsafe(no_mangle)]
pub static perf_more_info_string: [u8; {
    b"See 'perf help COMMAND' for more information on a specific command.\0".len()
}] =
    *b"See 'perf help COMMAND' for more information on a specific command.\0";

unsafe extern "C" fn usage_builtin(err: *const c_char) -> ! {
    unsafe {
        fprintf(stderr, c"\n Usage: %s\n".as_ptr(), err);
        exit(129);
    }
}

/* If we are in a dlopen()ed .so write to a global variable would segfault
 * (ugh), so keep things static. */
static mut usage_routine: unsafe extern "C" fn(err: *const c_char) -> ! = usage_builtin;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(err: *const c_char) {
    unsafe {
        usage_routine(err);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
