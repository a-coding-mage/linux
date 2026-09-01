/* Time inconsistency check test
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2003, 2004, 2005, 2012
 *		(C) Copyright Linaro Limited 2015
 *		Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc inconsistency-check.c -o inconsistency-check -lrt
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 */

// C dependencies: stdio.h, unistd.h, stdlib.h, time.h, sys/time.h,
// sys/timex.h, string.h, signal.h, "clock-helpers.h", "kselftest.h"

use std::ffi::c_char;
use std::os::raw::{c_int, c_long, c_ulong, c_ulonglong, c_void};
use std::ptr;

/* CLOCK_HWSPECIFIC == CLOCK_SGI_CYCLE (Deprecated) */
const CLOCK_HWSPECIFIC: c_int = 10;

const CALLS_PER_LOOP: usize = 64;
const NSEC_PER_SEC: c_long = 1_000_000_000;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_TAI: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stdout: *mut c_void;

    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn ctime(timep: *const c_long) -> *mut c_char;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn fflush(stream: *mut c_void) -> c_int;

    fn clock_name(clockid: c_int) -> *const c_char;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

/* returns 1 if a <= b, 0 otherwise */
#[inline]
fn in_order(a: timespec, b: timespec) -> c_int {
    /* use unsigned to avoid false positives on 2038 rollover */
    if (a.tv_sec as c_ulong) < (b.tv_sec as c_ulong) {
        return 1;
    }
    if (a.tv_sec as c_ulong) > (b.tv_sec as c_ulong) {
        return 0;
    }
    if a.tv_nsec > b.tv_nsec {
        return 0;
    }
    1
}

unsafe fn consistency_test(clock_type: c_int, seconds: c_ulong) -> c_int {
    let mut list = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; CALLS_PER_LOOP];
    let mut i: c_int;
    let mut inconsistent: c_int;
    let mut now: c_long;
    let then: c_long;
    let mut t: c_long;
    let start_str: *mut c_char;

    unsafe {
        clock_gettime(clock_type, &mut list[0]);
    }
    then = list[0].tv_sec;
    now = then;

    /* timestamp start of test */
    unsafe {
        t = time(ptr::null_mut());
        start_str = ctime(&t);
    }

    while seconds == c_ulong::MAX || ((now - then) as c_ulong) < seconds {
        inconsistent = -1;

        /* Fill list */
        i = 0;
        while i < CALLS_PER_LOOP as c_int {
            unsafe {
                clock_gettime(clock_type, &mut list[i as usize]);
            }
            i += 1;
        }

        /* Check for inconsistencies */
        i = 0;
        while i < (CALLS_PER_LOOP - 1) as c_int {
            if in_order(list[i as usize], list[(i + 1) as usize]) == 0 {
                inconsistent = i;
            }
            i += 1;
        }

        /* display inconsistency */
        if inconsistent >= 0 {
            let mut delta: c_ulonglong;

            unsafe {
                ksft_print_msg(b"\\%s\n\0".as_ptr() as *const c_char, start_str);
            }
            i = 0;
            while i < CALLS_PER_LOOP as c_int {
                if i == inconsistent {
                    unsafe {
                        ksft_print_msg(b"--------------------\n\0".as_ptr() as *const c_char);
                    }
                }
                unsafe {
                    ksft_print_msg(
                        b"%lu:%lu\n\0".as_ptr() as *const c_char,
                        list[i as usize].tv_sec as c_ulong,
                        list[i as usize].tv_nsec as c_ulong,
                    );
                }
                if i == inconsistent + 1 {
                    unsafe {
                        ksft_print_msg(b"--------------------\n\0".as_ptr() as *const c_char);
                    }
                }
                i += 1;
            }
            delta = (list[inconsistent as usize].tv_sec * NSEC_PER_SEC) as c_ulonglong;
            delta = delta.wrapping_add(list[inconsistent as usize].tv_nsec as c_ulonglong);
            delta = delta.wrapping_sub(
                (list[(inconsistent + 1) as usize].tv_sec * NSEC_PER_SEC) as c_ulonglong,
            );
            delta = delta.wrapping_sub(list[(inconsistent + 1) as usize].tv_nsec as c_ulonglong);
            unsafe {
                ksft_print_msg(b"Delta: %llu ns\n\0".as_ptr() as *const c_char, delta);
                fflush(ptr::null_mut());
            }
            /* timestamp inconsistency*/
            unsafe {
                t = time(ptr::null_mut());
                ksft_print_msg(b"%s\n\0".as_ptr() as *const c_char, ctime(&t));
            }
            return -1;
        }
        now = list[0].tv_sec;
    }
    0
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut clockid: c_int;
    let mut opt: c_int;
    let mut userclock: c_int = CLOCK_REALTIME;
    let mut maxclocks: c_int = CLOCK_TAI + 1;
    let mut runtime: c_int = 10;
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    /* Process arguments */
    loop {
        unsafe {
            opt = getopt(argc, argv, b"t:c:\0".as_ptr() as *const c_char);
        }
        if opt == -1 {
            break;
        }
        match opt {
            116 => {
                unsafe {
                    runtime = atoi(optarg);
                }
            }
            99 => {
                unsafe {
                    userclock = atoi(optarg);
                }
                maxclocks = userclock + 1;
            }
            _ => unsafe {
                printf(
                    b"Usage: %s [-t <secs>] [-c <clockid>]\n\0".as_ptr() as *const c_char,
                    *argv,
                );
                printf(b"\t-t: Number of seconds to run\n\0".as_ptr() as *const c_char);
                printf(b"\t-c: clockid to use (default, all clockids)\n\0".as_ptr() as *const c_char);
                exit(-1);
            },
        }
    }

    unsafe {
        setbuf(stdout, ptr::null_mut());

        ksft_print_header();
        ksft_set_plan(maxclocks - userclock);
    }

    clockid = userclock;
    while clockid < maxclocks {
        let skip_clock = clockid == CLOCK_HWSPECIFIC
            || unsafe { clock_gettime(clockid, &mut ts) } != 0;

        if skip_clock {
            unsafe {
                ksft_test_result_skip(
                    b"%-31s\n\0".as_ptr() as *const c_char,
                    clock_name(clockid),
                );
            }
            clockid += 1;
            continue;
        }

        if unsafe { consistency_test(clockid, runtime as c_ulong) } != 0 {
            unsafe {
                ksft_test_result_fail(
                    b"%-31s\n\0".as_ptr() as *const c_char,
                    clock_name(clockid),
                );
                ksft_exit_fail();
            }
        } else {
            unsafe {
                ksft_test_result_pass(
                    b"%-31s\n\0".as_ptr() as *const c_char,
                    clock_name(clockid),
                );
            }
        }
        clockid += 1;
    }
    unsafe {
        ksft_exit_pass();
    }
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(ptr::null_mut());

    unsafe {
        main_0((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
