/* SPDX-License-Identifier: GPL-2.0 */

/* Includes in the original header:
 * <linux/compiler.h>, <stdio.h>, <stdarg.h>
 */

use core::ffi::{c_char, c_double, c_int, c_void};

/* "\033[1;38;5;2xx;48;5;2xxm\0" is 23 bytes */
pub const COLOR_MAXLEN: usize = 24;

pub const PERF_COLOR_NORMAL: &[u8] = b"\0";
pub const PERF_COLOR_RESET: &[u8] = b"\x1b[m\0";
pub const PERF_COLOR_BOLD: &[u8] = b"\x1b[1m\0";
pub const PERF_COLOR_RED: &[u8] = b"\x1b[31m\0";
pub const PERF_COLOR_GREEN: &[u8] = b"\x1b[32m\0";
pub const PERF_COLOR_YELLOW: &[u8] = b"\x1b[33m\0";
pub const PERF_COLOR_BLUE: &[u8] = b"\x1b[34m\0";
pub const PERF_COLOR_MAGENTA: &[u8] = b"\x1b[35m\0";
pub const PERF_COLOR_CYAN: &[u8] = b"\x1b[36m\0";
pub const PERF_COLOR_BG_RED: &[u8] = b"\x1b[41m\0";

pub const MIN_GREEN: c_double = 0.5;
pub const MIN_RED: c_double = 5.0;

pub const PERF_COLOR_DELETE_LINE: &[u8] = b"\x1b[A\x1b[2K\r\0";

/* Opaque C dependency types from <stdio.h> and <stdarg.h>. */
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

pub type va_list = *mut c_void;

unsafe extern "C" {
    /*
     * This variable stores the value of color.ui
     */
    pub static mut perf_use_color_default: c_int;

    pub fn perf_config_colorbool(
        var: *const c_char,
        value: *const c_char,
        stdout_is_tty: c_int,
    ) -> c_int;
    pub fn color_vsnprintf(
        bf: *mut c_char,
        size: usize,
        color: *const c_char,
        fmt: *const c_char,
        args: va_list,
    ) -> c_int;
    pub fn color_vfprintf(
        fp: *mut FILE,
        color: *const c_char,
        fmt: *const c_char,
        args: va_list,
    ) -> c_int;
    /* Original declaration used __printf(3, 4). */
    pub fn color_fprintf(
        fp: *mut FILE,
        color: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;
    /* Original declaration used __printf(4, 5). */
    pub fn color_snprintf(
        bf: *mut c_char,
        size: usize,
        color: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;
    pub fn value_color_snprintf(
        bf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        value: c_double,
    ) -> c_int;
    /* Original declaration used __printf(3, 4). */
    pub fn percent_color_snprintf(
        bf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> c_int;
    /* Original declaration used __printf(3, 4). */
    pub fn percent_color_len_snprintf(
        bf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> c_int;
    pub fn percent_color_fprintf(
        fp: *mut FILE,
        fmt: *const c_char,
        percent: c_double,
    ) -> c_int;
    pub fn get_percent_color(percent: c_double) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
