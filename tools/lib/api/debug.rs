// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void, VaListImpl};

// C dependencies: <stdio.h>, <stdarg.h>, "debug.h", "debug-internal.h"

pub type FILE = c_void;

pub type libapi_print_fn_t = Option<unsafe extern "C" fn(format: *const c_char, ...) -> c_int>;

extern "C" {
    static mut stderr: *mut FILE;

    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: VaListImpl<'_, '_>) -> c_int;
}

unsafe extern "C" fn __base_pr(format: *const c_char, mut args: ...) -> c_int {
    let err: c_int;

    err = vfprintf(stderr, format, args.as_va_list());
    err
}

#[no_mangle]
pub static mut __pr_warn: libapi_print_fn_t = Some(__base_pr);

#[no_mangle]
pub static mut __pr_info: libapi_print_fn_t = Some(__base_pr);

#[no_mangle]
pub static mut __pr_debug: libapi_print_fn_t = None;

#[no_mangle]
pub unsafe extern "C" fn libapi_set_print(
    warn: libapi_print_fn_t,
    info: libapi_print_fn_t,
    debug: libapi_print_fn_t,
) {
    __pr_warn = warn;
    __pr_info = info;
    __pr_debug = debug;
}
