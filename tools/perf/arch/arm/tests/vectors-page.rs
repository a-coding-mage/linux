// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, <string.h>, <linux/compiler.h>
// Local dependencies: "debug.h", "tests/tests.h", "util/find-map.c"

use core::ffi::{c_char, c_int, c_void};

const VECTORS__MAP_NAME: &[u8] = b"[vectors]\0";

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn find_map(start: *mut *mut c_void, end: *mut *mut c_void, name: *const c_char) -> c_int;

    static TEST_FAIL: c_int;
    static TEST_OK: c_int;
}

unsafe extern "C" fn test__vectors_page(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let mut start: *mut c_void = core::ptr::null_mut();
    let mut end: *mut c_void = core::ptr::null_mut();

    let _ = test;
    let _ = subtest;

    if unsafe {
        find_map(
            &mut start,
            &mut end,
            VECTORS__MAP_NAME.as_ptr() as *const c_char,
        )
    } != 0
    {
        unsafe extern "C" {
            fn pr_err(fmt: *const c_char, ...);
        }

        unsafe {
            pr_err(
                b"%s not found, is CONFIG_KUSER_HELPERS enabled?\n\0".as_ptr() as *const c_char,
                VECTORS__MAP_NAME.as_ptr() as *const c_char,
            );
        }
        return unsafe { TEST_FAIL };
    }

    unsafe { TEST_OK }
}

// DEFINE_SUITE("Vectors page", vectors_page);
unsafe extern "C" {
    static mut vectors_page: test_suite;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
