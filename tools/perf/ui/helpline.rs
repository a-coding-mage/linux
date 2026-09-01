// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, <stdlib.h>, <string.h>
// Local dependencies: "helpline.h", "ui.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_void, VaListImpl};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_helpline {
    pub pop: Option<extern "C" fn()>,
    pub push: Option<extern "C" fn(msg: *const c_char)>,
    pub show: Option<extern "C" fn(fmt: *const c_char, ap: VaListImpl<'_>) -> c_int>,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: VaListImpl<'_>) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: VaListImpl<'_>) -> c_int;
    fn free(ptr: *mut c_void);
}

#[unsafe(no_mangle)]
pub static mut ui_helpline__current: [c_char; 512] = [0; 512];

extern "C" fn nop_helpline__pop() {}

extern "C" fn nop_helpline__push(_msg: *const c_char) {}

extern "C" fn nop_helpline__show(_fmt: *const c_char, _ap: VaListImpl<'_>) -> c_int {
    0
}

static mut default_helpline_fns: ui_helpline = ui_helpline {
    pop: Some(nop_helpline__pop),
    push: Some(nop_helpline__push),
    show: Some(nop_helpline__show),
};

#[unsafe(no_mangle)]
pub static mut helpline_fns: *mut ui_helpline = &raw mut default_helpline_fns;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__pop() {
    unsafe {
        ((*helpline_fns).pop.unwrap())();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__push(msg: *const c_char) {
    unsafe {
        ((*helpline_fns).push.unwrap())(msg);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__vpush(fmt: *const c_char, ap: VaListImpl<'_>) {
    let mut s: *mut c_char = std::ptr::null_mut();

    unsafe {
        if vasprintf(&mut s, fmt, ap) < 0 {
            vfprintf(stderr, fmt, ap);
        } else {
            ui_helpline__push(s);
            free(s as *mut c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__fpush(fmt: *const c_char, mut ap: ...) {
    unsafe {
        ui_helpline__vpush(fmt, ap.as_va_list());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__puts(msg: *const c_char) {
    unsafe {
        ui_helpline__pop();
        ui_helpline__push(msg);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__vshow(fmt: *const c_char, ap: VaListImpl<'_>) -> c_int {
    unsafe { ((*helpline_fns).show.unwrap())(fmt, ap) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_helpline__printf(fmt: *const c_char, mut ap: ...) {
    unsafe {
        ui_helpline__pop();
        ui_helpline__vpush(fmt, ap.as_va_list());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
