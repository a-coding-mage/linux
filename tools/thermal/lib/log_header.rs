/* SPDX-License-Identifier: LGPL-2.1+ */
/* Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org> */

use core::ffi::{c_char, c_int};

/* C header dependency: <syslog.h> provides LOG_DEBUG, LOG_INFO, and related levels. */

/* C-only __maybe_unused attribute macro omitted. */

pub const TO_SYSLOG: c_int = 0x1;
pub const TO_STDOUT: c_int = 0x2;
pub const TO_STDERR: c_int = 0x4;

unsafe extern "C" {
    pub fn logit(level: c_int, format: *const c_char, ...);

    pub fn log_init(level: c_int, ident: *const c_char, options: c_int) -> c_int;
    pub fn log_str2level(lvl: *const c_char) -> c_int;
    pub fn log_exit();
}

#[macro_export]
macro_rules! DEBUG {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(
                libc::LOG_DEBUG,
                concat!("%s:%d: ", $fmt, "\0").as_ptr() as *const core::ffi::c_char,
                concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as core::ffi::c_int
                $(, $args)*
            )
        }
    };
}

#[macro_export]
macro_rules! INFO {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_INFO, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! NOTICE {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_NOTICE, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! WARN {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_WARNING, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! ERROR {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_ERR, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! CRITICAL {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_CRIT, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! ALERT {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_ALERT, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}

#[macro_export]
macro_rules! EMERG {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        unsafe {
            $crate::logit(libc::LOG_EMERG, concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char $(, $args)*)
        }
    };
}
