// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2012 David Gibson, IBM Corporation.
 */

use core::ffi::{c_int, c_void};

// Declarations supplied by libfdt and its associated headers.
unsafe extern "C" {
    fn fdt_create(buf: *mut c_void, bufsize: c_int) -> c_int;
    fn fdt_finish_reservemap(fdt: *mut c_void) -> c_int;
    fn fdt_begin_node(fdt: *mut c_void, name: *const u8) -> c_int;
    fn fdt_end_node(fdt: *mut c_void) -> c_int;
    fn fdt_finish(fdt: *mut c_void) -> c_int;
    fn fdt_open_into(
        fdt: *const c_void,
        buf: *mut c_void,
        bufsize: c_int,
    ) -> c_int;
}

pub unsafe fn fdt_create_empty_tree(buf: *mut c_void, bufsize: c_int) -> c_int {
    let mut err: c_int;

    err = fdt_create(buf, bufsize);
    if err != 0 {
        return err;
    }

    err = fdt_finish_reservemap(buf);
    if err != 0 {
        return err;
    }

    err = fdt_begin_node(buf, b"\0".as_ptr());
    if err != 0 {
        return err;
    }

    err = fdt_end_node(buf);
    if err != 0 {
        return err;
    }

    err = fdt_finish(buf);
    if err != 0 {
        return err;
    }

    fdt_open_into(buf, buf, bufsize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
