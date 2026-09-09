// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int, c_void};

// Symbols and constants supplied by the surrounding UML OS layer/libc.
unsafe extern "C" {
    fn grantpt(fd: c_int) -> c_int;
    fn unlockpt(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn initial_thread_cb(cb: unsafe extern "C" fn(*mut c_void), arg: *mut c_void);
    fn printk(fmt: *const c_char, ...);
}

const O_RDWR: c_int = 0o2;

// UM_KERN_ERR is a C preprocessor logging-prefix macro from the UML headers.

#[repr(C)]
struct grantpt_info {
    fd: c_int,
    res: c_int,
    err: c_int,
}

unsafe extern "C" fn grantpt_cb(arg: *mut c_void) {
    let info = arg as *mut grantpt_info;

    (*info).res = grantpt((*info).fd);
    (*info).err = *__errno_location();
}

pub unsafe extern "C" fn get_pty() -> c_int {
    let mut info: grantpt_info;
    let fd: c_int;
    let mut err: c_int;

    fd = open(b"/dev/ptmx\0".as_ptr() as *const c_char, O_RDWR);
    if fd < 0 {
        err = -(*__errno_location());
        printk(
            b"get_pty : Couldn't open /dev/ptmx - err = %d\n\0".as_ptr()
                as *const c_char,
            *__errno_location(),
        );
        return err;
    }

    info = grantpt_info {
        fd,
        res: 0,
        err: 0,
    };
    initial_thread_cb(grantpt_cb, &mut info as *mut grantpt_info as *mut c_void);

    if info.res < 0 {
        err = -info.err;
        printk(
            b"get_pty : Couldn't grant pty - errno = %d\n\0".as_ptr()
                as *const c_char,
            -info.err,
        );
        close(fd);
        return err;
    }

    if unlockpt(fd) < 0 {
        err = -(*__errno_location());
        printk(
            b"get_pty : Couldn't unlock pty - errno = %d\n\0".as_ptr()
                as *const c_char,
            *__errno_location(),
        );
        close(fd);
        return err;
    }
    fd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
