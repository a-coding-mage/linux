// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// C dependencies: stdio.h, stdlib.h, unistd.h, errno.h, termios.h,
// chan_user.h, os.h, and um_malloc.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct chan_opts {
    pub raw: c_int,
}

#[repr(C)]
pub struct chan_ops {
    pub type_: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut c_char, c_int, *const chan_opts) -> *mut c_void>,
    pub open: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void, *mut *mut c_char) -> c_int>,
    pub close: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn()>,
    pub console_write: Option<unsafe extern "C" fn()>,
    pub window_size: Option<unsafe extern "C" fn()>,
    pub free: Option<unsafe extern "C" fn()>,
    pub winch: c_int,
}

unsafe extern "C" {
    fn printk(fmt: *const c_char, ...);
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_ulong;
    fn uml_kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn isatty(fd: c_int) -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn raw(fd: c_int) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn generic_read();
    fn generic_write();
    fn generic_console_write();
    fn generic_window_size();
    fn generic_free();
}

const UM_GFP_KERNEL: c_int = 0;
const UM_KERN_ERR: &[u8] = b"";
const TCSAFLUSH: c_int = 2;

#[repr(C)]
struct fd_chan {
    fd: c_int,
    raw: c_int,
    tt: termios,
    str_: [c_char; 11],
}

unsafe extern "C" fn fd_init(str_: *mut c_char, _device: c_int, opts: *const chan_opts) -> *mut c_void {
    let mut end: *mut c_char = core::ptr::null_mut();
    let n: c_int;

    if *str_ != b':' as c_char {
        printk(c"fd_init : channel type 'fd' must specify a file descriptor\n".as_ptr());
        return core::ptr::null_mut();
    }
    let str_ = str_.add(1);
    n = strtoul(str_, &mut end, 0) as c_int;
    if (*end != 0) || (end == str_) {
        printk(c"fd_init : couldn't parse file descriptor '%s'\n".as_ptr(), str_);
        return core::ptr::null_mut();
    }

    let data = uml_kmalloc(core::mem::size_of::<fd_chan>(), UM_GFP_KERNEL) as *mut fd_chan;
    if data.is_null() {
        return core::ptr::null_mut();
    }
    (*data).fd = n;
    (*data).raw = (*opts).raw;
    data as *mut c_void
}

unsafe extern "C" fn fd_open(_input: c_int, _output: c_int, _primary: c_int, d: *mut c_void, dev_out: *mut *mut c_char) -> c_int {
    let data = d as *mut fd_chan;
    let mut err: c_int;

    if (*data).raw != 0 && isatty((*data).fd) != 0 {
        err = tcgetattr((*data).fd, &mut (*data).tt);
        if err != 0 { return err; }
        err = raw((*data).fd);
        if err != 0 { return err; }
    }
    sprintf((*data).str_.as_mut_ptr(), c"%d".as_ptr(), (*data).fd);
    *dev_out = (*data).str_.as_mut_ptr();
    (*data).fd
}

unsafe extern "C" fn fd_close(fd: c_int, d: *mut c_void) {
    let data = d as *mut fd_chan;
    if (*data).raw == 0 || isatty(fd) == 0 { return; }
    let err = tcsetattr(fd, TCSAFLUSH, &(*data).tt);
    if err != 0 {
        printk(c"Failed to restore terminal state - errno = %d\n".as_ptr(), -err);
    }
    (*data).raw = 0;
}

#[no_mangle]
pub static mut fd_ops: chan_ops = chan_ops {
    type_: c"fd".as_ptr(),
    init: Some(fd_init),
    open: Some(fd_open),
    close: Some(fd_close),
    read: None,
    write: None,
    console_write: None,
    window_size: None,
    free: None,
    winch: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
