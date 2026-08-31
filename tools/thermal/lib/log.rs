// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
//
// C dependencies translated from:
// #include <stdarg.h>
// #include <stdio.h>
// #include <string.h>
// #include <syslog.h>
// #include "log.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

type va_list = *mut c_void;
type FILE = c_void;

const LOG_EMERG: c_int = 0;
const LOG_ALERT: c_int = 1;
const LOG_CRIT: c_int = 2;
const LOG_ERR: c_int = 3;
const LOG_WARNING: c_int = 4;
const LOG_NOTICE: c_int = 5;
const LOG_INFO: c_int = 6;
const LOG_DEBUG: c_int = 7;
const LOG_NDELAY: c_int = 0x08;
const LOG_USER: c_int = 1 << 3;

// From "log.h".
const TO_SYSLOG: c_int = 0x1;
const TO_STDERR: c_int = 0x2;
const TO_STDOUT: c_int = 0x4;

static mut __ident: *const c_char = b"unknown\0".as_ptr() as *const c_char;
static mut __options: c_int = 0;

static loglvl: [*const c_char; 8] = [
    b"EMERG\0".as_ptr() as *const c_char,
    b"ALERT\0".as_ptr() as *const c_char,
    b"CRITICAL\0".as_ptr() as *const c_char,
    b"ERROR\0".as_ptr() as *const c_char,
    b"WARN\0".as_ptr() as *const c_char,
    b"NOTICE\0".as_ptr() as *const c_char,
    b"INFO\0".as_ptr() as *const c_char,
    b"DEBUG\0".as_ptr() as *const c_char,
];

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn vsyslog(priority: c_int, format: *const c_char, ap: va_list);
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    fn openlog(ident: *const c_char, option: c_int, facility: c_int);
    fn setlogmask(maskpri: c_int) -> c_int;
    fn closelog();
}

const fn LOG_MASK(pri: c_int) -> c_int {
    1 << pri
}

const fn LOG_UPTO(pri: c_int) -> c_int {
    (1 << (pri + 1)) - 1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_str2level(lvl: *const c_char) -> c_int {
    let mut i: c_int = 0;

    while (i as usize) < loglvl.len() {
        if unsafe { strcmp(lvl, loglvl[i as usize]) } == 0 {
            return i;
        }
        i += 1;
    }

    LOG_DEBUG
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn logit(level: c_int, format: *const c_char, mut args: ...) {
    let ap: va_list = args.as_va_list().as_ptr() as va_list;

    if unsafe { __options } & TO_SYSLOG != 0 {
        unsafe { vsyslog(level, format, ap) };
    }

    if unsafe { __options } & TO_STDERR != 0 {
        unsafe { vfprintf(stderr, format, ap) };
    }

    if unsafe { __options } & TO_STDOUT != 0 {
        unsafe { vfprintf(stdout, format, ap) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_init(level: c_int, ident: *const c_char, options: c_int) -> c_int {
    if options == 0 {
        return -1;
    }

    if level > LOG_DEBUG {
        return -1;
    }

    if ident.is_null() {
        return -1;
    }

    unsafe {
        __ident = ident;
        __options = options;
    }

    if options & TO_SYSLOG != 0 {
        unsafe {
            openlog(__ident, options | LOG_NDELAY, LOG_USER);
            setlogmask(LOG_UPTO(level));
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_exit() {
    unsafe { closelog() };
}
