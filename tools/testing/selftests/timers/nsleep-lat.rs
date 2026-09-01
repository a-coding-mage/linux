/* Measure nanosleep timer latency
 *              by: john stultz (john.stultz@linaro.org)
 *		(C) Copyright Linaro 2013
 *              Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc nsleep-lat.c -o nsleep-lat -lrt
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

/* Dependencies from C headers:
 * stdio.h, stdlib.h, time.h, sys/time.h, sys/timex.h, string.h, signal.h,
 * clock-helpers.h, and kselftest.h.
 */

type c_int = i32;
type c_char = i8;
type c_long = i64;
type c_longlong = i64;
type clockid_t = c_int;
type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

extern "C" {
    static NSEC_PER_MSEC: c_longlong;
    static NSEC_PER_SEC: c_longlong;

    static CLOCK_REALTIME: clockid_t;
    static CLOCK_MONOTONIC: clockid_t;
    static CLOCK_BOOTTIME: clockid_t;
    static CLOCK_BOOTTIME_ALARM: clockid_t;
    static CLOCK_REALTIME_ALARM: clockid_t;
    static CLOCK_TAI: clockid_t;
    static TIMER_ABSTIME: c_int;

    static KSFT_SKIP: c_int;
    static KSFT_FAIL: c_int;
    static KSFT_PASS: c_int;

    fn clock_gettime(clockid: clockid_t, tp: *mut timespec) -> c_int;
    fn clock_nanosleep(
        clockid: clockid_t,
        flags: c_int,
        request: *const timespec,
        remain: *mut timespec,
    ) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_report(ret: c_int, fmt: *const c_char, ...);
    fn ksft_finished();
    fn clock_name(clockid: clockid_t) -> *const c_char;
}

unsafe fn unresonable_latency() -> c_longlong {
    40 * NSEC_PER_MSEC
}

pub unsafe fn timespec_add(mut ts: timespec, ns: u64) -> timespec {
    ts.tv_nsec += ns as c_long;
    while ts.tv_nsec >= NSEC_PER_SEC as c_long {
        ts.tv_nsec -= NSEC_PER_SEC as c_long;
        ts.tv_sec += 1;
    }
    ts
}

pub unsafe fn timespec_sub(a: timespec, b: timespec) -> c_longlong {
    let mut ret: c_longlong = NSEC_PER_SEC * b.tv_sec as c_longlong + b.tv_nsec as c_longlong;

    ret -= NSEC_PER_SEC * a.tv_sec as c_longlong + a.tv_nsec as c_longlong;
    ret
}

pub unsafe fn nanosleep_lat_test(clockid: c_int, ns: c_longlong) -> c_int {
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut target: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut latency: c_longlong = 0;
    let mut i: c_int;
    let count: c_int;

    target.tv_sec = (ns / NSEC_PER_SEC) as c_long;
    target.tv_nsec = (ns % NSEC_PER_SEC) as c_long;

    if clock_gettime(clockid, &mut start) != 0 {
        return KSFT_SKIP;
    }
    if clock_nanosleep(clockid, 0, &target, core::ptr::null_mut()) != 0 {
        return KSFT_SKIP;
    }

    count = 10;

    /* First check relative latency */
    if clock_gettime(clockid, &mut start) != 0 {
        return KSFT_FAIL;
    }

    i = 0;
    while i < count {
        if clock_nanosleep(clockid, 0, &target, core::ptr::null_mut()) != 0 {
            return KSFT_FAIL;
        }
        i += 1;
    }

    if clock_gettime(clockid, &mut end) != 0 {
        return KSFT_FAIL;
    }

    if ((timespec_sub(start, end) / count as c_longlong) - ns) > unresonable_latency() {
        ksft_print_msg(
            b"Large rel latency: %lld ns :\0".as_ptr() as *const c_char,
            (timespec_sub(start, end) / count as c_longlong) - ns,
        );
        return KSFT_FAIL;
    }

    /* Next check absolute latency */
    i = 0;
    while i < count {
        if clock_gettime(clockid, &mut start) != 0 {
            return KSFT_FAIL;
        }
        target = timespec_add(start, ns as u64);
        if clock_nanosleep(clockid, TIMER_ABSTIME, &target, core::ptr::null_mut()) != 0 {
            return KSFT_FAIL;
        }
        if clock_gettime(clockid, &mut end) != 0 {
            return KSFT_FAIL;
        }
        latency += timespec_sub(target, end);
        i += 1;
    }

    if latency / count as c_longlong > unresonable_latency() {
        ksft_print_msg(
            b"Large abs latency: %lld ns :\0".as_ptr() as *const c_char,
            latency / count as c_longlong,
        );
        return KSFT_FAIL;
    }

    KSFT_PASS
}

pub unsafe fn main_0(_argc: c_int, _argv: *mut *mut c_char) {
    let mut length: c_longlong;
    let mut clockid: c_int;
    let mut ret: c_int = 0;

    static TESTED_CLOCKS: [clockid_t; 6] = [
        CLOCK_REALTIME,
        CLOCK_MONOTONIC,
        CLOCK_BOOTTIME,
        CLOCK_BOOTTIME_ALARM,
        CLOCK_REALTIME_ALARM,
        CLOCK_TAI,
    ];

    ksft_print_header();
    ksft_set_plan(TESTED_CLOCKS.len() as c_int);

    let mut clock_index: size_t = 0;
    while clock_index < TESTED_CLOCKS.len() {
        clockid = TESTED_CLOCKS[clock_index];

        length = 10;
        while length <= NSEC_PER_SEC * 10 {
            ret = nanosleep_lat_test(clockid, length);
            if ret != 0 {
                break;
            }
            length *= 100;
        }

        ksft_test_result_report(ret, b"%s\n\0".as_ptr() as *const c_char, clock_name(clockid));
        clock_index += 1;
    }

    ksft_finished();
}

fn main() {
    unsafe {
        main_0(0, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
