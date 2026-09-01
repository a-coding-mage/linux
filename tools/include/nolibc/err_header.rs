/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * formatted error message for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* C header dependencies removed: "nolibc.h", "errno.h", "stdarg.h", "sys.h". */

use core::ffi::{c_char, c_int, VaListImpl};

extern "C" {
    static mut stderr: *mut FILE;
    static program_invocation_short_name: *const c_char;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: VaListImpl<'_>) -> c_int;
    fn exit(status: c_int) -> !;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

pub unsafe fn vwarn(fmt: *const c_char, args: VaListImpl<'_>) {
    unsafe {
        fprintf(stderr, b"%s: \0".as_ptr() as *const c_char, program_invocation_short_name);
        vfprintf(stderr, fmt, args);
        fprintf(stderr, b": %m\n\0".as_ptr() as *const c_char);
    }
}

pub unsafe fn vwarnx(fmt: *const c_char, args: VaListImpl<'_>) {
    unsafe {
        fprintf(stderr, b"%s: \0".as_ptr() as *const c_char, program_invocation_short_name);
        vfprintf(stderr, fmt, args);
        fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
    }
}

pub unsafe extern "C" fn warn(fmt: *const c_char, mut args: ...) {
    unsafe {
        vwarn(fmt, args.as_va_list());
    }
}

pub unsafe extern "C" fn warnx(fmt: *const c_char, mut args: ...) {
    unsafe {
        vwarnx(fmt, args.as_va_list());
    }
}

pub unsafe fn verr(eval: c_int, fmt: *const c_char, args: VaListImpl<'_>) -> ! {
    unsafe {
        vwarn(fmt, args);
        exit(eval);
    }
}

pub unsafe fn verrx(eval: c_int, fmt: *const c_char, args: VaListImpl<'_>) -> ! {
    unsafe {
        warnx(fmt, args);
        exit(eval);
    }
}

pub unsafe extern "C" fn err(eval: c_int, fmt: *const c_char, mut args: ...) -> ! {
    unsafe {
        verr(eval, fmt, args.as_va_list());
    }
}

pub unsafe extern "C" fn errx(eval: c_int, fmt: *const c_char, mut args: ...) -> ! {
    unsafe {
        verrx(eval, fmt, args.as_va_list());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
