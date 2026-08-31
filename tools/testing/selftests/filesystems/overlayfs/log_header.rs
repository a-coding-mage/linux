/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard removed in Rust translation. */

use std::os::raw::c_char;

unsafe extern "C" {
    pub fn ksft_print_msg(fmt: *const c_char, ...);
    pub fn ksft_test_result_error(fmt: *const c_char, ...);
    pub fn ksft_test_result_fail(fmt: *const c_char, ...);
}

macro_rules! pr_msg {
    ($fmt:literal, $lvl:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            ksft_print_msg(
                concat!("[%s] (%s:%d)\t", $fmt, "\n\0").as_ptr() as *const c_char,
                $lvl,
                concat!(file!(), "\0").as_ptr() as *const c_char,
                line!() as i32
                $(, $arg)*,
            );
        }
    }};
}

macro_rules! pr_p {
    ($func:ident, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $func!(concat!($fmt, ": %m") $(, $arg)*)
    };
}

macro_rules! pr_err {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            ksft_test_result_error(
                concat!($fmt, "\n\0").as_ptr() as *const c_char
                $(, $arg)*,
            );
        }
        -1
    }};
}

macro_rules! pr_fail {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            ksft_test_result_fail(
                concat!($fmt, "\0").as_ptr() as *const c_char
                $(, $arg)*,
            );
        }
        -1
    }};
}

macro_rules! pr_perror {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        pr_p!(pr_err, $fmt $(, $arg)*)
    };
}
