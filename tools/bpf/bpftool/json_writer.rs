// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * Simple streaming JSON writer
 *
 * This takes care of the annoying bits of JSON syntax like the commas
 * after elements
 *
 * Authors:	Stephen Hemminger <stephen@networkplumber.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![feature(c_variadic)]

use core::ffi::VaList;
use std::ffi::{c_char, c_double, c_int, c_ulonglong, c_void};
use std::mem;
use std::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList<'_, '_>) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    #[cfg(TEST)]
    static mut stdout: *mut FILE;
}

type json_writer_t = json_writer;

#[repr(C)]
pub struct json_writer {
    out: *mut FILE,  /* output file */
    depth: u32,      /* nesting */
    pretty: bool,    /* optional whitepace */
    sep: c_char,     /* either nul or comma */
}

unsafe fn putc(c: c_int, stream: *mut FILE) -> c_int {
    unsafe { fputc(c, stream) }
}

/* indentation for pretty print */
unsafe fn jsonw_indent(self_: *mut json_writer_t) {
    let mut i: u32 = 0;
    while i < unsafe { (*self_).depth } {
        unsafe {
            fputs(c"    ".as_ptr(), (*self_).out);
        }
        i = i.wrapping_add(1);
    }
}

/* end current line and indent if pretty printing */
unsafe fn jsonw_eol(self_: *mut json_writer_t) {
    if !unsafe { (*self_).pretty } {
        return;
    }

    unsafe {
        putc('\n' as c_int, (*self_).out);
        jsonw_indent(self_);
    }
}

/* If current object is not empty print a comma */
unsafe fn jsonw_eor(self_: *mut json_writer_t) {
    if unsafe { (*self_).sep } != '\0' as c_char {
        unsafe {
            putc((*self_).sep as c_int, (*self_).out);
        }
    }
    unsafe {
        (*self_).sep = ',' as c_char;
    }
}

/* Output JSON encoded string */
/* Handles C escapes, does not do Unicode */
unsafe fn jsonw_puts(self_: *mut json_writer_t, mut str_: *const c_char) {
    unsafe {
        putc('"' as c_int, (*self_).out);
    }
    while unsafe { *str_ } != 0 {
        match unsafe { *str_ as u8 } {
            b'\t' => unsafe {
                fputs(c"\\t".as_ptr(), (*self_).out);
            },
            b'\n' => unsafe {
                fputs(c"\\n".as_ptr(), (*self_).out);
            },
            b'\r' => unsafe {
                fputs(c"\\r".as_ptr(), (*self_).out);
            },
            b'\x0c' => unsafe {
                fputs(c"\\f".as_ptr(), (*self_).out);
            },
            b'\x08' => unsafe {
                fputs(c"\\b".as_ptr(), (*self_).out);
            },
            b'\\' => unsafe {
                fputs(c"\\\\".as_ptr(), (*self_).out);
            },
            b'"' => unsafe {
                fputs(c"\\\"".as_ptr(), (*self_).out);
            },
            _ => unsafe {
                putc(*str_ as c_int, (*self_).out);
            },
        }
        str_ = unsafe { str_.add(1) };
    }
    unsafe {
        putc('"' as c_int, (*self_).out);
    }
}

/* Create a new JSON stream */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_new(f: *mut FILE) -> *mut json_writer_t {
    let self_ = unsafe { malloc(mem::size_of::<json_writer>()) as *mut json_writer_t };
    if !self_.is_null() {
        unsafe {
            (*self_).out = f;
            (*self_).depth = 0;
            (*self_).pretty = false;
            (*self_).sep = '\0' as c_char;
        }
    }
    self_
}

/* End output to JSON stream */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_destroy(self_p: *mut *mut json_writer_t) {
    let self_ = unsafe { *self_p };

    assert!(unsafe { (*self_).depth } == 0);
    unsafe {
        fputs(c"\n".as_ptr(), (*self_).out);
        fflush((*self_).out);
        free(self_ as *mut c_void);
        *self_p = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_pretty(self_: *mut json_writer_t, on: bool) {
    unsafe {
        (*self_).pretty = on;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_reset(self_: *mut json_writer_t) {
    assert!(unsafe { (*self_).depth } == 0);
    unsafe {
        (*self_).sep = '\0' as c_char;
    }
}

/* Basic blocks */
unsafe fn jsonw_begin(self_: *mut json_writer_t, c: c_int) {
    unsafe {
        jsonw_eor(self_);
        putc(c, (*self_).out);
        (*self_).depth = (*self_).depth.wrapping_add(1);
        (*self_).sep = '\0' as c_char;
    }
}

unsafe fn jsonw_end(self_: *mut json_writer_t, c: c_int) {
    assert!(unsafe { (*self_).depth } > 0);

    unsafe {
        (*self_).depth = (*self_).depth.wrapping_sub(1);
        if (*self_).sep != '\0' as c_char {
            jsonw_eol(self_);
        }
        putc(c, (*self_).out);
        (*self_).sep = ',' as c_char;
    }
}

/* Add a JSON property name */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_name(self_: *mut json_writer_t, name: *const c_char) {
    unsafe {
        jsonw_eor(self_);
        jsonw_eol(self_);
        (*self_).sep = '\0' as c_char;
        jsonw_puts(self_, name);
        putc(':' as c_int, (*self_).out);
        if (*self_).pretty {
            putc(' ' as c_int, (*self_).out);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_vprintf_enquote(
    self_: *mut json_writer_t,
    fmt: *const c_char,
    ap: VaList<'_, '_>,
) {
    unsafe {
        jsonw_eor(self_);
        putc('"' as c_int, (*self_).out);
        vfprintf((*self_).out, fmt, ap);
        putc('"' as c_int, (*self_).out);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_printf(
    self_: *mut json_writer_t,
    fmt: *const c_char,
    mut ap: ...
) {
    unsafe {
        jsonw_eor(self_);
        vfprintf((*self_).out, fmt, ap.as_va_list());
    }
}

/* Collections */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_start_object(self_: *mut json_writer_t) {
    unsafe {
        jsonw_begin(self_, '{' as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_end_object(self_: *mut json_writer_t) {
    unsafe {
        jsonw_end(self_, '}' as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_start_array(self_: *mut json_writer_t) {
    unsafe {
        jsonw_begin(self_, '[' as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_end_array(self_: *mut json_writer_t) {
    unsafe {
        jsonw_end(self_, ']' as c_int);
    }
}

/* JSON value types */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_string(self_: *mut json_writer_t, value: *const c_char) {
    unsafe {
        jsonw_eor(self_);
        jsonw_puts(self_, value);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_bool(self_: *mut json_writer_t, val: bool) {
    unsafe {
        jsonw_printf(self_, c"%s".as_ptr(), if val { c"true".as_ptr() } else { c"false".as_ptr() });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_null(self_: *mut json_writer_t) {
    unsafe {
        jsonw_printf(self_, c"null".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_float_fmt(
    self_: *mut json_writer_t,
    fmt: *const c_char,
    num: c_double,
) {
    unsafe {
        jsonw_printf(self_, fmt, num);
    }
}

/* Original C condition: #ifdef notused */
#[cfg(notused)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_float(self_: *mut json_writer_t, num: c_double) {
    unsafe {
        jsonw_printf(self_, c"%g".as_ptr(), num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_hu(self_: *mut json_writer_t, num: u16) {
    unsafe {
        jsonw_printf(self_, c"%hu".as_ptr(), num as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_uint(self_: *mut json_writer_t, num: u64) {
    unsafe {
        jsonw_printf(self_, c"%llu".as_ptr(), num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_lluint(self_: *mut json_writer_t, num: c_ulonglong) {
    unsafe {
        jsonw_printf(self_, c"%llu".as_ptr(), num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_int(self_: *mut json_writer_t, num: i64) {
    unsafe {
        jsonw_printf(self_, c"%lld".as_ptr(), num);
    }
}

/* Basic name/value objects */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_string_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    val: *const c_char,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_string(self_, val);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_bool_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    val: bool,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_bool(self_, val);
    }
}

/* Original C condition: #ifdef notused */
#[cfg(notused)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_float_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    val: c_double,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_float(self_, val);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_float_field_fmt(
    self_: *mut json_writer_t,
    prop: *const c_char,
    fmt: *const c_char,
    val: c_double,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_float_fmt(self_, fmt, val);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_uint_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    num: u64,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_uint(self_, num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_hu_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    num: u16,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_hu(self_, num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_lluint_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    num: c_ulonglong,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_lluint(self_, num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_int_field(
    self_: *mut json_writer_t,
    prop: *const c_char,
    num: i64,
) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_int(self_, num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jsonw_null_field(self_: *mut json_writer_t, prop: *const c_char) {
    unsafe {
        jsonw_name(self_, prop);
        jsonw_null(self_);
    }
}

/* Original C condition: #ifdef TEST */
#[cfg(TEST)]
unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut argc = argc;
    let mut argv = argv;
    let mut wr: *mut json_writer_t = unsafe { jsonw_new(stdout) };

    unsafe {
        jsonw_start_object(wr);
        jsonw_pretty(wr, true);
        jsonw_name(wr, c"Vyatta".as_ptr());
        jsonw_start_object(wr);
        jsonw_string_field(wr, c"url".as_ptr(), c"http://vyatta.com".as_ptr());
        jsonw_uint_field(wr, c"downloads".as_ptr(), 2000000u64);
        jsonw_float_field(wr, c"stock".as_ptr(), 8.16);

        jsonw_name(wr, c"ARGV".as_ptr());
        jsonw_start_array(wr);
        while {
            argc -= 1;
            argc != 0
        } {
            argv = argv.add(1);
            jsonw_string(wr, *argv);
        }
        jsonw_end_array(wr);

        jsonw_name(wr, c"empty".as_ptr());
        jsonw_start_array(wr);
        jsonw_end_array(wr);

        jsonw_name(wr, c"NIL".as_ptr());
        jsonw_start_object(wr);
        jsonw_end_object(wr);

        jsonw_null_field(wr, c"my_null".as_ptr());

        jsonw_name(wr, c"special chars".as_ptr());
        jsonw_start_array(wr);
        jsonw_string_field(wr, c"slash".as_ptr(), c"/".as_ptr());
        jsonw_string_field(wr, c"newline".as_ptr(), c"\n".as_ptr());
        jsonw_string_field(wr, c"tab".as_ptr(), c"\t".as_ptr());
        jsonw_string_field(wr, c"ff".as_ptr(), c"\x0c".as_ptr());
        jsonw_string_field(wr, c"quote".as_ptr(), c"\"".as_ptr());
        jsonw_string_field(wr, c"tick".as_ptr(), c"'".as_ptr());
        jsonw_string_field(wr, c"backslash".as_ptr(), c"\\".as_ptr());
        jsonw_end_array(wr);

        jsonw_end_object(wr);

        jsonw_end_object(wr);
        jsonw_destroy(&mut wr);
    }
    return 0;
}
