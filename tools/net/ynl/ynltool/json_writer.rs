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
#![feature(c_variadic)]

use core::ffi::{c_char, c_double, c_int, c_uchar, c_uint, c_ulonglong, c_void, VaListImpl};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

pub type uint64_t = u64;
pub type int64_t = i64;
pub type va_list = VaListImpl<'static>;

#[repr(C)]
pub struct json_writer {
	pub out: *mut FILE,
	pub depth: c_uint,
	pub pretty: bool,
	pub sep: c_char,
}

pub type json_writer_t = json_writer;

unsafe extern "C" {
	fn malloc(size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
	fn putc(c: c_int, stream: *mut FILE) -> c_int;
	fn fflush(stream: *mut FILE) -> c_int;
	fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaListImpl<'_>) -> c_int;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
	bytes.as_ptr() as *const c_char
}

unsafe fn jsonw_indent(self_: *mut json_writer_t) {
	let mut i: c_uint = 0;

	while i < (*self_).depth {
		fputs(cstr(b"    \0"), (*self_).out);
		i = i.wrapping_add(1);
	}
}

unsafe fn jsonw_eol(self_: *mut json_writer_t) {
	if !(*self_).pretty {
		return;
	}

	putc('\n' as c_int, (*self_).out);
	jsonw_indent(self_);
}

unsafe fn jsonw_eor(self_: *mut json_writer_t) {
	if (*self_).sep != '\0' as c_char {
		putc((*self_).sep as c_int, (*self_).out);
	}
	(*self_).sep = ',' as c_char;
}

unsafe fn jsonw_puts(self_: *mut json_writer_t, mut str_: *const c_char) {
	putc('"' as c_int, (*self_).out);
	while *str_ != 0 {
		match *str_ as c_uchar as c_char {
			x if x == '\t' as c_char => {
				fputs(cstr(b"\\t\0"), (*self_).out);
			}
			x if x == '\n' as c_char => {
				fputs(cstr(b"\\n\0"), (*self_).out);
			}
			x if x == '\r' as c_char => {
				fputs(cstr(b"\\r\0"), (*self_).out);
			}
			x if x == '\x0c' as c_char => {
				fputs(cstr(b"\\f\0"), (*self_).out);
			}
			x if x == '\x08' as c_char => {
				fputs(cstr(b"\\b\0"), (*self_).out);
			}
			x if x == '\\' as c_char => {
				fputs(cstr(b"\\\\\0"), (*self_).out);
			}
			x if x == '"' as c_char => {
				fputs(cstr(b"\\\"\0"), (*self_).out);
			}
			_ => {
				putc(*str_ as c_int, (*self_).out);
			}
		}
		str_ = str_.add(1);
	}
	putc('"' as c_int, (*self_).out);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_new(f: *mut FILE) -> *mut json_writer_t {
	let self_ = malloc(size_of::<json_writer_t>()) as *mut json_writer_t;

	if !self_.is_null() {
		(*self_).out = f;
		(*self_).depth = 0;
		(*self_).pretty = false;
		(*self_).sep = '\0' as c_char;
	}
	self_
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_destroy(self_p: *mut *mut json_writer_t) {
	let self_ = *self_p;

	assert!((*self_).depth == 0);
	fputs(cstr(b"\n\0"), (*self_).out);
	fflush((*self_).out);
	free(self_ as *mut c_void);
	*self_p = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_pretty(self_: *mut json_writer_t, on: bool) {
	(*self_).pretty = on;
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_reset(self_: *mut json_writer_t) {
	assert!((*self_).depth == 0);
	(*self_).sep = '\0' as c_char;
}

unsafe fn jsonw_begin(self_: *mut json_writer_t, c: c_int) {
	jsonw_eor(self_);
	putc(c, (*self_).out);
	(*self_).depth = (*self_).depth.wrapping_add(1);
	(*self_).sep = '\0' as c_char;
}

unsafe fn jsonw_end(self_: *mut json_writer_t, c: c_int) {
	assert!((*self_).depth > 0);

	(*self_).depth = (*self_).depth.wrapping_sub(1);
	if (*self_).sep != '\0' as c_char {
		jsonw_eol(self_);
	}
	putc(c, (*self_).out);
	(*self_).sep = ',' as c_char;
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_name(self_: *mut json_writer_t, name: *const c_char) {
	jsonw_eor(self_);
	jsonw_eol(self_);
	(*self_).sep = '\0' as c_char;
	jsonw_puts(self_, name);
	putc(':' as c_int, (*self_).out);
	if (*self_).pretty {
		putc(' ' as c_int, (*self_).out);
	}
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_vprintf_enquote(
	self_: *mut json_writer_t,
	fmt: *const c_char,
	ap: VaListImpl<'_>,
) {
	jsonw_eor(self_);
	putc('"' as c_int, (*self_).out);
	vfprintf((*self_).out, fmt, ap);
	putc('"' as c_int, (*self_).out);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_printf(
	self_: *mut json_writer_t,
	fmt: *const c_char,
	mut ap: ...
) {
	jsonw_eor(self_);
	vfprintf((*self_).out, fmt, ap.as_va_list());
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_start_object(self_: *mut json_writer_t) {
	jsonw_begin(self_, '{' as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_end_object(self_: *mut json_writer_t) {
	jsonw_end(self_, '}' as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_start_array(self_: *mut json_writer_t) {
	jsonw_begin(self_, '[' as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_end_array(self_: *mut json_writer_t) {
	jsonw_end(self_, ']' as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_string(self_: *mut json_writer_t, value: *const c_char) {
	jsonw_eor(self_);
	jsonw_puts(self_, value);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_bool(self_: *mut json_writer_t, val: bool) {
	jsonw_printf(
		self_,
		cstr(b"%s\0"),
		if val { cstr(b"true\0") } else { cstr(b"false\0") },
	);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_null(self_: *mut json_writer_t) {
	jsonw_printf(self_, cstr(b"null\0"));
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_float_fmt(
	self_: *mut json_writer_t,
	fmt: *const c_char,
	num: c_double,
) {
	jsonw_printf(self_, fmt, num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_float(self_: *mut json_writer_t, num: c_double) {
	jsonw_printf(self_, cstr(b"%g\0"), num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_hu(self_: *mut json_writer_t, num: u16) {
	jsonw_printf(self_, cstr(b"%hu\0"), num as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_uint(self_: *mut json_writer_t, num: uint64_t) {
	jsonw_printf(self_, cstr(b"%llu\0"), num as c_ulonglong);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_lluint(self_: *mut json_writer_t, num: c_ulonglong) {
	jsonw_printf(self_, cstr(b"%llu\0"), num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_int(self_: *mut json_writer_t, num: int64_t) {
	jsonw_printf(self_, cstr(b"%lld\0"), num as i64);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_string_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	val: *const c_char,
) {
	jsonw_name(self_, prop);
	jsonw_string(self_, val);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_bool_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	val: bool,
) {
	jsonw_name(self_, prop);
	jsonw_bool(self_, val);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_float_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	val: c_double,
) {
	jsonw_name(self_, prop);
	jsonw_float(self_, val);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_float_field_fmt(
	self_: *mut json_writer_t,
	prop: *const c_char,
	fmt: *const c_char,
	val: c_double,
) {
	jsonw_name(self_, prop);
	jsonw_float_fmt(self_, fmt, val);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_uint_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	num: uint64_t,
) {
	jsonw_name(self_, prop);
	jsonw_uint(self_, num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_hu_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	num: u16,
) {
	jsonw_name(self_, prop);
	jsonw_hu(self_, num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_lluint_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	num: c_ulonglong,
) {
	jsonw_name(self_, prop);
	jsonw_lluint(self_, num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_int_field(
	self_: *mut json_writer_t,
	prop: *const c_char,
	num: int64_t,
) {
	jsonw_name(self_, prop);
	jsonw_int(self_, num);
}

#[no_mangle]
pub unsafe extern "C" fn jsonw_null_field(self_: *mut json_writer_t, prop: *const c_char) {
	jsonw_name(self_, prop);
	jsonw_null(self_);
}
