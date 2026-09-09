// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding UML/Linux translation.
unsafe extern "C" {
    fn os_open_file(filename: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn os_read_file(fd: c_int, buf: *mut c_void, size: c_int) -> c_int;
    fn os_close_file(fd: c_int);
    fn os_file_size(filename: *const c_char, size: *mut u64) -> c_int;
    fn memblock_alloc_or_panic(size: u64, align: usize) -> *mut c_void;
    fn memblock_free(area: *mut c_void, size: u64);
    fn printk(fmt: *const c_char, ...);
}

// These build-time constants/macros are supplied by the surrounding headers.
extern "Rust" {
    static SMP_CACHE_BYTES: usize;
    fn OPENFLAGS() -> c_int;
    fn of_read(flags: c_int) -> c_int;
}

unsafe fn __uml_load_file(filename: *const c_char, buf: *mut c_void, size: c_int) -> c_int {
    let fd: c_int;
    let n: c_int;

    fd = os_open_file(filename, of_read(OPENFLAGS()), 0);
    if fd < 0 {
        printk(
            b"Opening '%s' failed - err = %d\n\0".as_ptr() as *const c_char,
            filename,
            -fd,
        );
        return -1;
    }
    n = os_read_file(fd, buf, size);
    if n != size {
        printk(
            b"Read of %d bytes from '%s' failed, err = %d\n\0".as_ptr() as *const c_char,
            size,
            filename,
            -n,
        );
        return -1;
    }

    os_close_file(fd);
    0
}

pub unsafe fn uml_load_file(filename: *const c_char, size: *mut u64) -> *mut c_void {
    let area: *mut c_void;
    let err: c_int;

    *size = 0;

    if filename.is_null() {
        return core::ptr::null_mut();
    }

    err = os_file_size(filename, size);
    if err != 0 {
        return core::ptr::null_mut();
    }

    if *size == 0 {
        printk(
            b"\"%s\" is empty\n\0".as_ptr() as *const c_char,
            filename,
        );
        return core::ptr::null_mut();
    }

    area = memblock_alloc_or_panic(*size, SMP_CACHE_BYTES);

    if __uml_load_file(filename, area, *size as c_int) != 0 {
        memblock_free(area, *size);
        return core::ptr::null_mut();
    }

    area
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
