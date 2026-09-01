/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Simple streaming JSON writer
 *
 * This takes care of the annoying bits of JSON syntax like the commas
 * after elements
 *
 * Authors:	Stephen Hemminger <stephen@networkplumber.org>
 */

use core::ffi::{c_char, c_ulonglong, c_void};

/* Dependencies in the original header:
 * <stdbool.h>, <stdint.h>, <stdarg.h>, and <stdio.h>.
 */

/* Opaque class structure */
#[repr(C)]
pub struct json_writer {
    _private: [u8; 0],
}

pub type json_writer_t = json_writer;

/* Opaque stdio FILE from <stdio.h>. */
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* C va_list from <stdarg.h>; exact representation is supplied by the C ABI. */
pub type va_list = *mut c_void;

unsafe extern "C" {
    /* Create a new JSON stream */
    pub fn jsonw_new(f: *mut FILE) -> *mut json_writer_t;
    /* End output to JSON stream */
    pub fn jsonw_destroy(self_p: *mut *mut json_writer_t);

    /* Cause output to have pretty whitespace */
    pub fn jsonw_pretty(self_: *mut json_writer_t, on: bool);

    /* Reset separator to create new JSON */
    pub fn jsonw_reset(self_: *mut json_writer_t);

    /* Add property name */
    pub fn jsonw_name(self_: *mut json_writer_t, name: *const c_char);

    /* Add value  */
    /* Original declarations used GCC printf format attributes. */
    pub fn jsonw_vprintf_enquote(
        self_: *mut json_writer_t,
        fmt: *const c_char,
        ap: va_list,
    );
    pub fn jsonw_printf(self_: *mut json_writer_t, fmt: *const c_char, ...);
    pub fn jsonw_string(self_: *mut json_writer_t, value: *const c_char);
    pub fn jsonw_bool(self_: *mut json_writer_t, value: bool);
    pub fn jsonw_float(self_: *mut json_writer_t, number: f64);
    pub fn jsonw_float_fmt(self_: *mut json_writer_t, fmt: *const c_char, num: f64);
    pub fn jsonw_uint(self_: *mut json_writer_t, number: u64);
    pub fn jsonw_hu(self_: *mut json_writer_t, number: u16);
    pub fn jsonw_int(self_: *mut json_writer_t, number: i64);
    pub fn jsonw_null(self_: *mut json_writer_t);
    pub fn jsonw_lluint(self_: *mut json_writer_t, num: c_ulonglong);

    /* Useful Combinations of name and value */
    pub fn jsonw_string_field(
        self_: *mut json_writer_t,
        prop: *const c_char,
        val: *const c_char,
    );
    pub fn jsonw_bool_field(self_: *mut json_writer_t, prop: *const c_char, value: bool);
    pub fn jsonw_float_field(self_: *mut json_writer_t, prop: *const c_char, num: f64);
    pub fn jsonw_uint_field(self_: *mut json_writer_t, prop: *const c_char, num: u64);
    pub fn jsonw_hu_field(self_: *mut json_writer_t, prop: *const c_char, num: u16);
    pub fn jsonw_int_field(self_: *mut json_writer_t, prop: *const c_char, num: i64);
    pub fn jsonw_null_field(self_: *mut json_writer_t, prop: *const c_char);
    pub fn jsonw_lluint_field(
        self_: *mut json_writer_t,
        prop: *const c_char,
        num: c_ulonglong,
    );
    pub fn jsonw_float_field_fmt(
        self_: *mut json_writer_t,
        prop: *const c_char,
        fmt: *const c_char,
        val: f64,
    );

    /* Collections */
    pub fn jsonw_start_object(self_: *mut json_writer_t);
    pub fn jsonw_end_object(self_: *mut json_writer_t);

    pub fn jsonw_start_array(self_: *mut json_writer_t);
    pub fn jsonw_end_array(self_: *mut json_writer_t);
}

/* Override default exception handling */
pub type jsonw_err_handler_fn = unsafe extern "C" fn(*const c_char);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
