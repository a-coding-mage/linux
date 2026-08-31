/* SPDX-License-Identifier: GPL-2.0 */
/* For debugging general purposes */

use core::ffi::{c_char, c_int, c_void};

/* C header dependencies removed: stdarg.h, stdbool.h, stdio.h, linux/compiler.h. */

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

pub type va_list = *mut c_void;

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: [u64; 0],
}

unsafe extern "C" {
    pub static mut verbose: c_int;
    pub static mut debug_kmaps: c_int;
    pub static mut debug_peo_args: c_int;
    pub static mut quiet: bool;
    pub static mut dump_trace: bool;
    pub static mut debug_ordered_events: c_int;
    pub static mut debug_data_convert: c_int;
    pub static mut debug_type_profile: c_int;

    pub fn dump_printf(fmt: *const c_char, ...) -> c_int;
    pub fn trace_event(event: *mut perf_event);

    pub fn ui__error(format: *const c_char, ...) -> c_int;
    pub fn ui__warning(format: *const c_char, ...) -> c_int;

    pub fn pr_stat(fmt: *const c_char, ...);

    pub fn eprintf(level: c_int, var: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn eprintf_time(level: c_int, var: c_int, t: u64, fmt: *const c_char, ...) -> c_int;
    pub fn veprintf(level: c_int, var: c_int, fmt: *const c_char, args: va_list) -> c_int;

    pub fn perf_debug_option(str_: *const c_char) -> c_int;
    pub fn debug_file() -> *mut FILE;
    pub fn debug_set_file(file: *mut FILE);
    pub fn debug_set_display_time(set: bool);
    pub fn perf_debug_setup();
    pub fn perf_quiet_option() -> c_int;

    pub fn __dump_stack(file: *mut FILE, stackdump: *mut *mut c_void, stackdump_size: usize);
    pub fn dump_stack();
    pub fn sighandler_dump_stack(sig: c_int);
}

pub const STRERR_BUFSIZE: usize = 128; /* For the buffer size of str_error_r */

macro_rules! pr_fmt {
    ($fmt:expr) => {
        $fmt
    };
}

macro_rules! pr_err {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf(0, verbose, pr_fmt!($fmt) $(, $args)*) }
    };
}

macro_rules! pr_warning {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf(0, verbose, pr_fmt!($fmt) $(, $args)*) }
    };
}

macro_rules! pr_warning_once {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        static mut __WARNED: c_int = 0;
        unsafe {
            if __WARNED == 0 {
                pr_warning!($fmt $(, $args)*);
                __WARNED = 1;
            }
        }
    }};
}

macro_rules! pr_info {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf(0, verbose, pr_fmt!($fmt) $(, $args)*) }
    };
}

macro_rules! pr_debug {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf(1, verbose, pr_fmt!($fmt) $(, $args)*) }
    };
}

macro_rules! pr_debugN {
    ($n:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf($n, verbose, pr_fmt!($fmt) $(, $args)*) }
    };
}

macro_rules! pr_debug2 {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        pr_debugN!(2, pr_fmt!($fmt) $(, $args)*)
    };
}

macro_rules! pr_debug3 {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        pr_debugN!(3, pr_fmt!($fmt) $(, $args)*)
    };
}

macro_rules! pr_debug4 {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        pr_debugN!(4, pr_fmt!($fmt) $(, $args)*)
    };
}

/* Special macro to print perf_event_open arguments/return value. */
macro_rules! pr_debug2_peo {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if debug_peo_args != 0 {
                pr_debugN!(0, pr_fmt!($fmt) $(, $args)*);
            } else {
                pr_debugN!(2, pr_fmt!($fmt) $(, $args)*);
            }
        }
    }};
}

macro_rules! pr_time_N {
    ($n:expr, $var:expr, $t:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe { eprintf_time($n, $var, $t, $fmt $(, $args)*) }
    };
}

macro_rules! pr_oe_time {
    ($t:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        pr_time_N!(1, debug_ordered_events, $t, pr_fmt!($fmt) $(, $args)*)
    };
}

macro_rules! pr_oe_time2 {
    ($t:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        pr_time_N!(2, debug_ordered_events, $t, pr_fmt!($fmt) $(, $args)*)
    };
}

macro_rules! ui__warning_once {
    ($format:expr $(, $args:expr)* $(,)?) => {{
        static mut __WARNED: c_int = 0;
        unsafe {
            if __WARNED == 0 {
                ui__warning($format $(, $args)*);
                __WARNED = 1;
            }
        }
    }};
}
