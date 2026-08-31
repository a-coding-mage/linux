// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint};
use core::mem::size_of_val;

// Dependencies from:
// #include <linux/compiler.h>
// #include <linux/kernel.h>
// #include "tests.h"
// #include "debug.h"
// #include "print_binary.h"

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    static TEST_FAIL: c_int;
    static TEST_OK: c_int;

    fn is_printable_array(buf: *mut c_char, len: c_uint) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
struct test_case {
    buf: *mut c_char,
    len: c_uint,
    ret: c_int,
}

unsafe extern "C" fn test__is_printable_array(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    let mut buf1: [c_char; 6] = [
        b'k' as c_char,
        b'r' as c_char,
        4 as c_char,
        b'v' as c_char,
        b'a' as c_char,
        0,
    ];
    let mut buf2: [c_char; 6] = [
        b'k' as c_char,
        b'r' as c_char,
        b'a' as c_char,
        b'v' as c_char,
        4 as c_char,
        0,
    ];
    let t: [test_case; 7] = [
        test_case {
            buf: c"krava".as_ptr() as *mut c_char,
            len: size_of_val(c"krava".to_bytes_with_nul()) as c_uint,
            ret: 1,
        },
        test_case {
            buf: c"krava".as_ptr() as *mut c_char,
            len: (size_of_val(c"krava".to_bytes_with_nul()) - 1) as c_uint,
            ret: 0,
        },
        test_case {
            buf: c"".as_ptr() as *mut c_char,
            len: size_of_val(c"".to_bytes_with_nul()) as c_uint,
            ret: 1,
        },
        test_case {
            buf: c"".as_ptr() as *mut c_char,
            len: 0,
            ret: 0,
        },
        test_case {
            buf: core::ptr::null_mut(),
            len: 0,
            ret: 0,
        },
        test_case {
            buf: buf1.as_mut_ptr(),
            len: size_of_val(&buf1) as c_uint,
            ret: 0,
        },
        test_case {
            buf: buf2.as_mut_ptr(),
            len: size_of_val(&buf2) as c_uint,
            ret: 0,
        },
    ];
    let mut i: c_uint;

    i = 0;
    while (i as usize) < t.len() {
        let ret: c_int;

        ret = is_printable_array(t[i as usize].buf as *mut c_char, t[i as usize].len);
        if ret != t[i as usize].ret {
            pr_err(c"failed: test %u\n".as_ptr(), i);
            return TEST_FAIL;
        }

        i = i.wrapping_add(1);
    }

    TEST_OK
}

// DEFINE_SUITE("is_printable_array", is_printable_array);
