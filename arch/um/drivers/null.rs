// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the surrounding channel and OS interfaces.
type __u8 = u8;

#[repr(C)]
pub struct chan_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct chan_ops {
    pub type_: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut c_char, c_int, *const chan_opts) -> *mut c_void>,
    pub open: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void, *mut *mut c_char) -> c_int>,
    pub close: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn(c_int, *mut __u8, *mut c_void) -> c_int>,
    pub write: Option<unsafe extern "C" fn(c_int, *const __u8, c_int, *mut c_void) -> c_int>,
    pub console_write: Option<unsafe extern "C" fn(c_int, *const __u8, c_int, *mut c_void) -> c_int>,
    pub window_size: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub winch: c_int,
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn generic_close(fd: c_int, data: *mut c_void);
    fn generic_write(fd: c_int, data: *const __u8, len: c_int, unused: *mut c_void) -> c_int;
    fn generic_console_write(
        fd: c_int,
        data: *const __u8,
        len: c_int,
        unused: *mut c_void,
    ) -> c_int;
    fn generic_window_size(fd: c_int, data: *mut c_void) -> c_int;
}

/* This address is used only as a unique identifier */
static mut null_chan: c_int = 0;

unsafe extern "C" fn null_init(
    _str: *mut c_char,
    _device: c_int,
    _opts: *const chan_opts,
) -> *mut c_void {
    core::ptr::addr_of_mut!(null_chan).cast()
}

unsafe extern "C" fn null_open(
    _input: c_int,
    _output: c_int,
    _primary: c_int,
    _d: *mut c_void,
    dev_out: *mut *mut c_char,
) -> c_int {
    *dev_out = core::ptr::null_mut();

    let fd = open(b"/dev/null\0".as_ptr().cast(), 2);
    if fd < 0 {
        -(*__errno_location())
    } else {
        fd
    }
}

unsafe extern "C" fn null_read(_fd: c_int, _c_out: *mut __u8, _unused: *mut c_void) -> c_int {
    -19
}

unsafe extern "C" fn null_free(_data: *mut c_void) {}

pub static null_ops: chan_ops = chan_ops {
    type_: b"null\0".as_ptr().cast(),
    init: Some(null_init),
    open: Some(null_open),
    close: Some(generic_close),
    read: Some(null_read),
    write: Some(generic_write),
    console_write: Some(generic_console_write),
    window_size: Some(generic_window_size),
    free: Some(null_free),
    winch: 0,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
