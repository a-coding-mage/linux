/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include <perf/core.h> */

use core::ffi::c_char;

unsafe extern "C" {
    pub fn libperf_print(level: libperf_print_level, format: *const c_char, ...);
}

macro_rules! __pr {
    ($level:expr, $fmt:literal $(, $($arg:expr),* $(,)?)?) => {{
        unsafe {
            libperf_print(
                $level,
                concat!("libperf: ", $fmt, "\0").as_ptr() as *const c_char
                $(, $($arg),*)?
            );
        }
    }};
}

macro_rules! pr_err {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_ERR, $fmt $(, $($arg),*)?)
    };
}

macro_rules! pr_warning {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_WARN, $fmt $(, $($arg),*)?)
    };
}

macro_rules! pr_info {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_INFO, $fmt $(, $($arg),*)?)
    };
}

macro_rules! pr_debug {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_DEBUG, $fmt $(, $($arg),*)?)
    };
}

macro_rules! pr_debug2 {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_DEBUG2, $fmt $(, $($arg),*)?)
    };
}

macro_rules! pr_debug3 {
    ($fmt:literal $(, $($arg:expr),* $(,)?)?) => {
        __pr!(LIBPERF_DEBUG3, $fmt $(, $($arg),*)?)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
