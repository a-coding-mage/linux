/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::{c_char, c_int, c_void};

/*
 * Copyright 2011 The Chromium Authors, All Rights Reserved.
 * Copyright 2008 Jon Loeliger, Freescale Semiconductor, Inc.
 */

/* C headers supplying these types and functions are external dependencies. */
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub const ARRAY_SIZE: usize = 0; /* Use array_size!(x) for the C sizeof expression. */

#[macro_export]
macro_rules! array_size {
    ($x:expr) => {
        core::mem::size_of_val(&$x) / core::mem::size_of_val(&$x[0])
    };
}

#[macro_export]
macro_rules! stringify {
    ($s:ident) => { stringify!($s) };
}

extern "C" {
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn vfprintf(stream: *mut FILE, format: *const c_char, ap: *mut c_void) -> c_int;
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn exit(status: c_int) -> !;
    pub static mut stderr: *mut FILE;
}

/* C's noreturn, printf-format attributes have no direct Rust equivalent. */
pub unsafe fn die(str_: *const c_char, _args: ...) -> ! {
    fprintf(stderr, b"FATAL ERROR: \0".as_ptr() as *const c_char);
    vfprintf(stderr, str_, core::ptr::null_mut());
    exit(1)
}

/** Writes path to fp, escaping spaces with a backslash. */
extern "C" {
    pub fn fprint_path_escaped(fp: *mut FILE, path: *const c_char);
}

pub unsafe fn xmalloc(len: usize) -> *mut c_void {
    let new = malloc(len);
    if new.is_null() {
        die(b"malloc() failed\n\0".as_ptr() as *const c_char);
    }
    new
}

pub unsafe fn xrealloc(p: *mut c_void, len: usize) -> *mut c_void {
    let new = realloc(p, len);
    if new.is_null() {
        die(b"realloc() failed (len=%zd)\n\0".as_ptr() as *const c_char);
    }
    new
}

extern "C" {
    pub fn xstrdup(s: *const c_char) -> *mut c_char;
    pub fn xstrndup(s: *const c_char, len: usize) -> *mut c_char;
    pub fn xasprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    pub fn xasprintf_append(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    pub fn xavsprintf_append(strp: *mut *mut c_char, fmt: *const c_char, ap: *mut c_void) -> c_int;
    pub fn join_path(path: *const c_char, name: *const c_char) -> *mut c_char;

    pub fn util_is_printable_string(data: *const c_void, len: c_int) -> bool;
    pub fn get_escape_char(s: *const c_char, i: *mut c_int) -> c_char;
    pub fn utilfdt_read(filename: *const c_char, len: *mut usize) -> *mut c_char;
    pub fn utilfdt_read_err(filename: *const c_char, buffp: *mut *mut c_char, len: *mut usize) -> c_int;
    pub fn utilfdt_write(filename: *const c_char, blob: *const c_void) -> c_int;
    pub fn utilfdt_write_err(filename: *const c_char, blob: *const c_void) -> c_int;
    pub fn utilfdt_decode_type(fmt: *const c_char, type_: *mut c_int, size: *mut c_int) -> c_int;
    pub fn utilfdt_print_data(data: *const c_char, len: c_int);
    pub fn util_version() -> !;
    pub fn util_usage(
        errmsg: *const c_char,
        synopsis: *const c_char,
        short_opts: *const c_char,
        long_opts: *const option,
        opts_help: *const *const c_char,
    ) -> !;
    pub fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
}

pub const USAGE_TYPE_MSG: &str = "<type>\ts=string, i=int, u=unsigned, x=hex, r=raw\n\tOptional modifier prefix:\n\t\thh or b=byte, h=2 byte, l=4 byte (default)";

#[macro_export]
macro_rules! usage {
    ($errmsg:expr) => {
        unsafe { util_usage($errmsg, usage_synopsis, usage_short_opts, usage_long_opts, usage_opts_help) }
    };
}

#[macro_export]
macro_rules! util_getopt_long {
    () => {
        unsafe { getopt_long(argc, argv, usage_short_opts, usage_long_opts, core::ptr::null_mut()) }
    };
}

pub const a_argument: c_int = 1;
pub const USAGE_COMMON_SHORT_OPTS: &[u8] = b"hV\0";

/* USAGE_COMMON_LONG_OPTS: {"help", no_argument, NULL, 'h'},
 * {"version", no_argument, NULL, 'V'}, {NULL, no_argument, NULL, 0x0} */
/* USAGE_COMMON_OPTS_HELP: "Print this help and exit", "Print version and exit", NULL */
/* case_USAGE_COMMON_FLAGS: 'h' => usage(NULL), 'V' => util_version(), '?' => usage("unknown option") */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
