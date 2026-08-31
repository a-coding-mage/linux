// SPDX-License-Identifier: GPL-2.0
// C dependencies: malloc.h, stdlib.h, string.h, assert.h, and "utils.h".

use core::ffi::{c_char, c_int, c_long, c_void};

type size_t = usize;

unsafe extern "C" {
    fn TEST_MEMMOVE(s1: *const c_void, s2: *const c_void, n: size_t) -> *mut c_void;

    fn memalign(alignment: size_t, size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn abort() -> !;

    fn test_harness(
        testcase: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

const BUF_LEN: size_t = 65536;
const MAX_OFFSET: size_t = 512;

fn max(a: size_t, b: size_t) -> size_t {
    if a >= b {
        return a;
    }
    b
}

unsafe extern "C" fn testcase_run() -> c_int {
    let mut i: size_t;
    let mut src_off: size_t;
    let mut dst_off: size_t;
    let mut len: size_t;

    let usermap = unsafe { memalign(BUF_LEN, BUF_LEN) as *mut c_char };
    let kernelmap = unsafe { memalign(BUF_LEN, BUF_LEN) as *mut c_char };

    assert!(!usermap.is_null());
    assert!(!kernelmap.is_null());

    unsafe {
        memset(usermap as *mut c_void, 0, BUF_LEN);
        memset(kernelmap as *mut c_void, 0, BUF_LEN);
    }

    i = 0;
    while i < BUF_LEN {
        unsafe {
            *usermap.add(i) = (i & 0xff) as c_char;
            *kernelmap.add(i) = (i & 0xff) as c_char;
        }
        i += 1;
    }

    src_off = 0;
    while src_off < MAX_OFFSET {
        dst_off = 0;
        while dst_off < MAX_OFFSET {
            len = 1;
            while len < MAX_OFFSET - max(src_off, dst_off) {
                unsafe {
                    memmove(
                        usermap.add(dst_off) as *mut c_void,
                        usermap.add(src_off) as *const c_void,
                        len,
                    );
                    TEST_MEMMOVE(
                        kernelmap.add(dst_off) as *const c_void,
                        kernelmap.add(src_off) as *const c_void,
                        len,
                    );
                    if memcmp(
                        usermap as *const c_void,
                        kernelmap as *const c_void,
                        MAX_OFFSET,
                    ) != 0
                    {
                        printf(
                            b"memmove failed at %ld %ld %ld\n\0".as_ptr()
                                as *const c_char,
                            src_off as c_long,
                            dst_off as c_long,
                            len as c_long,
                        );
                        abort();
                    }
                }
                len += 1;
            }
            dst_off += 1;
        }
        src_off += 1;
    }
    0
}

fn main() -> c_int {
    unsafe { test_harness(testcase_run, b"memmove\0".as_ptr() as *const c_char) }
}
