/* Time bounds setting test
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2012
 *		Licensed under the GPLv2
 *
 *  NOTE: This is a meta-test which sets the time to edge cases then
 *  uses other tests to detect problems. Thus this test requires that
 *  the inconsistency-check and nanosleep tests be present in the same
 *  directory it is run from.
 *
 *  To build:
 *	$ gcc set-2038.c -o set-2038 -lrt
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

// C dependencies: stdio.h, stdlib.h, unistd.h, time.h, sys/time.h,
// clock-helpers.h, kselftest.h.

use std::ffi::{c_char, c_int, c_long, c_void};

type time_t = c_long;
type suseconds_t = c_long;

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

unsafe extern "C" {
    fn settimeofday(tv: *const timeval, tz: *const c_void) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

const NSEC_PER_SEC: i64 = 1_000_000_000;

const KTIME_MAX: i64 = !((1_u64 << 63) as i64);
const KTIME_SEC_MAX: i64 = KTIME_MAX / NSEC_PER_SEC;

const YEAR_1901: i64 = -0x7fffffff_i64;
const YEAR_1970: i64 = 1;
const YEAR_2038: i64 = 0x7fffffff_i64; /*overflows 32bit time_t */
const YEAR_2262: i64 = KTIME_SEC_MAX; /*overflows 64bit ktime_t */
const YEAR_MAX: i64 = ((1_u64 << 63) - 1) as i64; /*overflows 64bit time_t */

fn is32bits() -> c_int {
    (std::mem::size_of::<c_long>() == 4) as c_int
}

unsafe fn settime(time_value: i64) -> c_int {
    let mut now: timeval = timeval {
        tv_sec: time_value as time_t,
        tv_usec: 0,
    };
    let ret: c_int;

    ret = unsafe { settimeofday(&mut now, std::ptr::null()) };

    println!(
        "Setting time to 0x{:x}: {}",
        time_value as c_long as usize,
        ret
    );
    ret
}

unsafe fn do_tests() -> c_int {
    let mut ret: c_int;

    ret = unsafe { system(c"date".as_ptr()) };
    ret = ret | unsafe { system(c"./inconsistency-check -c 0 -t 20".as_ptr()) };
    ret = ret | unsafe { system(c"./nanosleep".as_ptr()) };
    ret = ret | unsafe { system(c"./nsleep-lat".as_ptr()) };
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut opt: c_int;
    let mut dangerous: c_int = 0;
    let start: time_t;

    /* Process arguments */
    loop {
        opt = unsafe { getopt(argc, argv, c"d".as_ptr()) };
        if opt == -1 {
            break;
        }
        match opt {
            100 => {
                dangerous = 1;
            }
            _ => {}
        }
    }

    start = unsafe { time(std::ptr::null_mut()) };

    /* First test that crazy values don't work */
    if unsafe { settime(YEAR_1901) } == 0 {
        ret = -1;
        goto_out(ret, start);
    }
    if unsafe { settime(YEAR_MAX) } == 0 {
        ret = -1;
        goto_out(ret, start);
    }
    if is32bits() == 0 && unsafe { settime(YEAR_2262) } == 0 {
        ret = -1;
        goto_out(ret, start);
    }

    /* Now test behavior near edges */
    unsafe {
        settime(YEAR_1970);
    }
    ret = unsafe { do_tests() };
    if ret != 0 {
        goto_out(ret, start);
    }

    unsafe {
        settime(YEAR_2038 - 600);
    }
    ret = unsafe { do_tests() };
    if ret != 0 {
        goto_out(ret, start);
    }

    /* The rest of the tests can blowup on 32bit systems */
    if is32bits() != 0 && dangerous == 0 {
        goto_out(ret, start);
    }
    /* Test rollover behavior 32bit edge */
    unsafe {
        settime(YEAR_2038 - 10);
    }
    ret = unsafe { do_tests() };
    if ret != 0 {
        goto_out(ret, start);
    }

    unsafe {
        settime(YEAR_2262 - 600);
    }
    ret = unsafe { do_tests() };

    goto_out(ret, start);
}

unsafe fn goto_out(ret: c_int, start: time_t) -> ! {
    /* restore clock */
    unsafe {
        settime(start as i64);
    }
    if ret != 0 {
        unsafe {
            ksft_exit_fail();
        }
    }
    unsafe {
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
