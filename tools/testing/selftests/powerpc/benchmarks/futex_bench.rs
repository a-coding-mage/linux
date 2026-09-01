// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016, Anton Blanchard, Michael Ellerman, IBM Corp.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

type c_char = i8;
type c_int = i32;
type c_long = i64;
type c_uint = u32;
type time_t = i64;
type c_void = core::ffi::c_void;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

const ITERATIONS: u64 = 100000000;

const CLOCK_MONOTONIC: c_int = 1;
const FUTEX_WAKE: c_int = 1;
const __NR_futex: c_long = 221;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;

    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe fn futex(
    A: *mut c_uint,
    B: c_int,
    C: c_int,
    D: *mut c_void,
    E: *mut c_void,
    F: c_int,
) -> c_long {
    unsafe { syscall(__NR_futex, A, B, C, D, E, F) }
}

unsafe extern "C" fn test_futex() -> c_int {
    let mut ts_start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts_end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: u64 = ITERATIONS;

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
    }

    while {
        let old = i;
        i = i.wrapping_sub(1);
        old != 0
    } {
        let mut addr: c_uint = 0;
        unsafe {
            futex(
                &mut addr,
                FUTEX_WAKE,
                1,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                0,
            );
        }
    }

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts_end);

        printf(
            b"time = %.6f\n\0".as_ptr() as *const c_char,
            (ts_end.tv_sec - ts_start.tv_sec) as f64
                + (ts_end.tv_nsec - ts_start.tv_nsec) as f64 / 1e9f64,
        );
    }

    0
}

fn main() {
    unsafe {
        test_harness_set_timeout(300);
        std::process::exit(test_harness(
            test_futex,
            b"futex_bench\0".as_ptr() as *const c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
