// SPDX-License-Identifier: GPL-2.0
// C dependencies: <malloc.h>, <stdlib.h>, <string.h>, <time.h>, "utils.h"

use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_char, c_double, c_int, c_long, c_void};

const SIZE: usize = 256;
const ITERATIONS: c_int = 1000;
const ITERATIONS_BENCH: c_int = 100000;
const CLOCK_MONOTONIC: c_int = 1;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

unsafe extern "C" {
    fn memalign(alignment: usize, size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn srandom(seed: u32);
    fn random() -> c_long;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn test_strlen(s: *const c_void) -> c_int;
    fn test_harness(test: Option<extern "C" fn() -> c_int>, name: *const c_char) -> c_int;
}

/* test all offsets and lengths */
extern "C" fn test_one(s: *mut c_char) {
    let mut offset: c_ulong_compat = 0;

    while offset < SIZE as c_ulong_compat {
        let x: c_int;
        let y: c_int;
        let mut i: c_ulong_compat;

        unsafe {
            y = strlen(s.add(offset as usize)) as c_int;
            x = test_strlen(s.add(offset as usize) as *const c_void);

            if x != y {
                printf(
                    c"strlen() returned %d, should have returned %d (%p offset %ld)\n".as_ptr(),
                    x,
                    y,
                    s,
                    offset as c_long,
                );

                i = offset;
                while i < SIZE as c_ulong_compat {
                    printf(
                        c"%02x ".as_ptr(),
                        *s.add(i as usize) as c_int,
                    );
                    i += 1;
                }
                printf(c"\n".as_ptr());
            }
        }

        offset += 1;
    }
}

extern "C" fn bench_test(s: *mut c_char) {
    let mut ts_start = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts_end = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: c_int;

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts_start);

        i = 0;
        while i < ITERATIONS_BENCH {
            test_strlen(s as *const c_void);
            i += 1;
        }

        clock_gettime(CLOCK_MONOTONIC, &mut ts_end);

        printf(
            c"len %3.3d : time = %.6f\n".as_ptr(),
            test_strlen(s as *const c_void),
            (ts_end.tv_sec - ts_start.tv_sec) as c_double
                + (ts_end.tv_nsec - ts_start.tv_nsec) as c_double / 1e9f64,
        );
    }
}

extern "C" fn testcase() -> c_int {
    let s: *mut c_char;
    let mut i: c_ulong_compat;

    unsafe {
        s = memalign(128, SIZE) as *mut c_char;
        if s.is_null() {
            perror(c"memalign".as_ptr());
            exit(1);
        }

        srandom(1);

        memset(s as *mut c_void, 0, SIZE);
        i = 0;
        while i < SIZE as c_ulong_compat {
            let mut c: c_char;

            loop {
                c = (random() & 0x7f) as c_char;
                if c != 0 {
                    break;
                }
            }
            *s.add(i as usize) = c;
            test_one(s);
            i += 1;
        }

        i = 0;
        while i < ITERATIONS as c_ulong_compat {
            let mut j: c_ulong_compat;

            j = 0;
            while j < SIZE as c_ulong_compat {
                let mut c: c_char;

                loop {
                    c = (random() & 0x7f) as c_char;
                    if c != 0 {
                        break;
                    }
                }
                *s.add(j as usize) = c;
                j += 1;
            }
            j = 0;
            while j < size_of::<c_long>() as c_ulong_compat {
                *s.add(SIZE - 1 - j as usize) = 0;
                test_one(s);
                j += 1;
            }
            i += 1;
        }

        i = 0;
        while i < SIZE as c_ulong_compat {
            let mut c: c_char;

            loop {
                c = (random() & 0x7f) as c_char;
                if c != 0 {
                    break;
                }
            }
            *s.add(i as usize) = c;
            i += 1;
        }

        bench_test(s);

        *s.add(16) = 0;
        bench_test(s);

        *s.add(8) = 0;
        bench_test(s);

        *s.add(4) = 0;
        bench_test(s);

        *s.add(3) = 0;
        bench_test(s);

        *s.add(2) = 0;
        bench_test(s);

        *s.add(1) = 0;
        bench_test(s);
    }

    0
}

type c_ulong_compat = std::os::raw::c_ulong;

pub extern "C" fn main() -> c_int {
    unsafe { test_harness(Some(testcase), CStr::from_bytes_with_nul_unchecked(b"strlen\0").as_ptr()) }
}
