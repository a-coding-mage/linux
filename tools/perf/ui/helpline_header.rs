/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

/*
 * C dependency intent:
 *   #include <stdio.h>
 *   #include <stdarg.h>
 *
 * The concrete representation of C va_list is target-specific and supplied by
 * the surrounding build/bindings context.
 */
pub type va_list = *mut c_void;

#[repr(C)]
pub struct ui_helpline {
    pub pop: Option<unsafe extern "C" fn()>,
    pub push: Option<unsafe extern "C" fn(msg: *const c_char)>,
    pub show: Option<unsafe extern "C" fn(fmt: *const c_char, ap: va_list) -> c_int>,
}

unsafe extern "C" {
    pub static mut helpline_fns: *mut ui_helpline;

    pub fn ui_helpline__init();

    pub fn ui_helpline__pop();
    pub fn ui_helpline__push(msg: *const c_char);
    pub fn ui_helpline__vpush(fmt: *const c_char, ap: va_list);
    pub fn ui_helpline__fpush(fmt: *const c_char, ...);
    pub fn ui_helpline__puts(msg: *const c_char);
    pub fn ui_helpline__printf(fmt: *const c_char, ...);
    pub fn ui_helpline__vshow(fmt: *const c_char, ap: va_list) -> c_int;

    pub static mut ui_helpline__current: [c_char; 512];
    pub static mut ui_helpline__last_msg: [c_char; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
