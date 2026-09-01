// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/ui/util.h.
// C include dependency intent: <stdarg.h> supplies va_list.

use std::ffi::{c_char, c_int, c_void};

// File-local Rust stand-in for C va_list from <stdarg.h>. The exact platform
// representation is supplied by the C ABI and external definitions.
pub type va_list = *mut c_void;

unsafe extern "C" {
    pub fn ui__getch(delay_secs: c_int) -> c_int;
    pub fn ui__popup_menu(argc: c_int, argv: *const *mut c_char, keyp: *mut c_int) -> c_int;
    pub fn ui__help_window(text: *const c_char) -> c_int;
    pub fn ui__dialog_yesno(msg: *const c_char) -> c_int;
    pub fn __ui__info_window(title: *const c_char, text: *const c_char, exit_msg: *const c_char);
    pub fn ui__info_window(title: *const c_char, text: *const c_char);
    pub fn ui__question_window(
        title: *const c_char,
        text: *const c_char,
        exit_msg: *const c_char,
        delay_secs: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct perf_error_ops {
    pub error: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
    pub warning: Option<unsafe extern "C" fn(format: *const c_char, args: va_list) -> c_int>,
}

unsafe extern "C" {
    pub fn perf_error__register(eops: *mut perf_error_ops) -> c_int;
    pub fn perf_error__unregister(eops: *mut perf_error_ops) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
