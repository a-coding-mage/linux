// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/* Dependencies from the original C file:
 * #include <test_progs.h>
 * #include "test_static_linked.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct test_static_linked {
    pub data: *mut test_static_linked__data,
    pub rodata: *mut test_static_linked__rodata,
}

#[repr(C)]
pub struct test_static_linked__data {
    pub var1: c_int,
    pub var2: c_int,
}

#[repr(C)]
pub struct test_static_linked__rodata {
    pub rovar1: c_int,
    pub rovar2: c_int,
}

unsafe extern "C" {
    fn test_static_linked__open() -> *mut test_static_linked;
    fn test_static_linked__load(skel: *mut test_static_linked) -> c_int;
    fn test_static_linked__attach(skel: *mut test_static_linked) -> c_int;
    fn test_static_linked__destroy(skel: *mut test_static_linked);

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn usleep(usec: c_uint) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn test_static_linked() {
    let mut err: c_int;
    let skel: *mut test_static_linked;

    skel = unsafe { test_static_linked__open() };
    if !unsafe { ASSERT_OK_PTR(skel.cast(), c"skel_open".as_ptr()) } {
        return;
    }

    unsafe {
        (*(*skel).rodata).rovar1 = 1;
        (*(*skel).rodata).rovar2 = 4;
    }

    err = unsafe { test_static_linked__load(skel) };
    if !unsafe { ASSERT_OK(err, c"skel_load".as_ptr()) } {
        unsafe { test_static_linked__destroy(skel) };
        return;
    }

    err = unsafe { test_static_linked__attach(skel) };
    if !unsafe { ASSERT_OK(err, c"skel_attach".as_ptr()) } {
        unsafe { test_static_linked__destroy(skel) };
        return;
    }

    /* trigger */
    unsafe {
        usleep(1);
    }

    unsafe {
        ASSERT_EQ((*(*skel).data).var1, 1 * 2 + 2 + 3, c"var1".as_ptr());
        ASSERT_EQ((*(*skel).data).var2, 4 * 3 + 5 + 6, c"var2".as_ptr());
    }

    unsafe { test_static_linked__destroy(skel) };
}
