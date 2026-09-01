// SPDX-License-Identifier: GPL-2.0
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_double, c_int, c_ulong, VaList, VaListImpl};
use core::ptr;

pub type size_t = c_ulong;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn pager_in_use() -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: VaList) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn vscnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, args: VaList) -> c_int;
    fn fabs(x: c_double) -> c_double;
}

unsafe extern "C" {
    static PERF_COLOR_RESET: *const c_char;
    static PERF_COLOR_NORMAL: *const c_char;
    static PERF_COLOR_RED: *const c_char;
    static PERF_COLOR_GREEN: *const c_char;
    static MIN_RED: c_double;
    static MIN_GREEN: c_double;
}

#[unsafe(no_mangle)]
pub static mut perf_use_color_default: c_int = -1;

unsafe fn __color_vsnprintf(
    bf: *mut c_char,
    size: size_t,
    color: *const c_char,
    fmt: *const c_char,
    args: VaList,
    trail: *const c_char,
) -> c_int {
    let mut r: c_int = 0;

    /*
     * Auto-detect:
     */
    if perf_use_color_default < 0 {
        if isatty(1) != 0 || pager_in_use() != 0 {
            perf_use_color_default = 1;
        } else {
            perf_use_color_default = 0;
        }
    }

    if perf_use_color_default != 0 && *color != 0 {
        r += scnprintf(bf, size, c"%s".as_ptr(), color);
    }
    r += vscnprintf(bf.offset(r as isize), size - r as size_t, fmt, args);
    if perf_use_color_default != 0 && *color != 0 {
        r += scnprintf(
            bf.offset(r as isize),
            size - r as size_t,
            c"%s".as_ptr(),
            PERF_COLOR_RESET,
        );
    }
    if !trail.is_null() {
        r += scnprintf(
            bf.offset(r as isize),
            size - r as size_t,
            c"%s".as_ptr(),
            trail,
        );
    }
    r
}

/* Colors are not included in return value */
unsafe fn __color_vfprintf(
    fp: *mut FILE,
    color: *const c_char,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    let mut r: c_int = 0;

    /*
     * Auto-detect:
     */
    if perf_use_color_default < 0 {
        if isatty(fileno(fp)) != 0 || pager_in_use() != 0 {
            perf_use_color_default = 1;
        } else {
            perf_use_color_default = 0;
        }
    }

    if perf_use_color_default != 0 && *color != 0 {
        fprintf(fp, c"%s".as_ptr(), color);
    }
    r += vfprintf(fp, fmt, args);
    if perf_use_color_default != 0 && *color != 0 {
        fprintf(fp, c"%s".as_ptr(), PERF_COLOR_RESET);
    }
    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn color_vsnprintf(
    bf: *mut c_char,
    size: size_t,
    color: *const c_char,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    __color_vsnprintf(bf, size, color, fmt, args, ptr::null())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn color_vfprintf(
    fp: *mut FILE,
    color: *const c_char,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    __color_vfprintf(fp, color, fmt, args)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn color_snprintf(
    bf: *mut c_char,
    size: size_t,
    color: *const c_char,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let r: c_int;

    r = color_vsnprintf(bf, size, color, fmt, args.as_va_list());
    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn color_fprintf(
    fp: *mut FILE,
    color: *const c_char,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let r: c_int;

    r = color_vfprintf(fp, color, fmt, args.as_va_list());
    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_percent_color(percent: c_double) -> *const c_char {
    let mut color: *const c_char = PERF_COLOR_NORMAL;

    /*
     * We color high-overhead entries in red, mid-overhead
     * entries in green - and keep the low overhead places
     * normal:
     */
    if fabs(percent) >= MIN_RED {
        color = PERF_COLOR_RED;
    } else if fabs(percent) > MIN_GREEN {
        color = PERF_COLOR_GREEN;
    }
    color
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn percent_color_fprintf(
    fp: *mut FILE,
    fmt: *const c_char,
    percent: c_double,
) -> c_int {
    let r: c_int;
    let color: *const c_char;

    color = get_percent_color(percent);
    r = color_fprintf(fp, color, fmt, percent);

    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_color_snprintf(
    bf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    value: c_double,
) -> c_int {
    let color: *const c_char = get_percent_color(value);
    color_snprintf(bf, size, color, fmt, value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn percent_color_snprintf(
    bf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let percent: c_double;

    percent = args.arg::<c_double>();
    value_color_snprintf(bf, size, fmt, percent)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn percent_color_len_snprintf(
    bf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let len: c_int;
    let percent: c_double;
    let color: *const c_char;

    len = args.arg::<c_int>();
    percent = args.arg::<c_double>();

    color = get_percent_color(percent);
    color_snprintf(bf, size, color, fmt, len, percent)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
