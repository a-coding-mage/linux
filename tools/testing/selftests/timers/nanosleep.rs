/* Make sure timers don't return early
 *              by: john stultz (johnstul@us.ibm.com)
 *		    John Stultz (john.stultz@linaro.org)
 *              (C) Copyright IBM 2012
 *              (C) Copyright Linaro 2013 2015
 *              Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc nanosleep.c -o nanosleep -lrt
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

// C dependencies: errno.h, stdio.h, stdlib.h, time.h, sys/time.h,
// sys/timex.h, string.h, signal.h, "clock-helpers.h", "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_void};

type clockid_t = c_int;
type timer_t = *mut c_void;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct itimerspec {
    it_interval: timespec,
    it_value: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigset_t {
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigaction {
    sa_handler: sighandler_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
    sa_mask: sigset_t,
}

const NSEC_PER_SEC: c_long = 1_000_000_000;
const TIMER_ABSTIME: c_int = 1;
const CLOCK_REALTIME: clockid_t = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_TAI: clockid_t = 11;
const CLOCK_BOOTTIME: clockid_t = 7;
const CLOCK_REALTIME_ALARM: clockid_t = 8;
const CLOCK_BOOTTIME_ALARM: clockid_t = 9;
const SIGALRM: c_int = 14;
const EINTR: c_int = 4;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const SIG_DFL: sighandler_t = None;

unsafe extern "C" {
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn clock_nanosleep(
        clockid: clockid_t,
        flags: c_int,
        request: *const timespec,
        remain: *mut timespec,
    ) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn timer_create(clockid: clockid_t, sevp: *mut c_void, timerid: *mut timer_t) -> c_int;
    fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
    fn timer_delete(timerid: timer_t) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;

    static mut stdout: *mut c_void;

    fn clock_name(clockid: clockid_t) -> *const c_char;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_finished();
    fn ksft_exit_fail_msg(fmt: *const c_char, ...);
    fn ksft_test_result_report(ret: c_int, fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
}

/* returns 1 if a <= b, 0 otherwise */
#[inline]
fn in_order(a: timespec, b: timespec) -> c_int {
    if a.tv_sec < b.tv_sec {
        return 1;
    }
    if a.tv_sec > b.tv_sec {
        return 0;
    }
    if a.tv_nsec > b.tv_nsec {
        return 0;
    }
    1
}

fn timespec_add(mut ts: timespec, ns: u64) -> timespec {
    ts.tv_nsec += ns as c_long;
    while ts.tv_nsec >= NSEC_PER_SEC {
        ts.tv_nsec -= NSEC_PER_SEC;
        ts.tv_sec += 1;
    }
    ts
}

fn nanosleep_test(clockid: c_int, ns: i64) -> c_int {
    let mut now = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut target;
    let mut rel = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    /* First check abs time */
    unsafe {
        if clock_gettime(clockid, &mut now) != 0 {
            return KSFT_SKIP;
        }
    }
    target = timespec_add(now, ns as u64);

    unsafe {
        if clock_nanosleep(clockid, TIMER_ABSTIME, &target, core::ptr::null_mut()) != 0 {
            return KSFT_SKIP;
        }
        clock_gettime(clockid, &mut now);
    }

    if in_order(target, now) == 0 {
        return KSFT_FAIL;
    }

    /* Second check reltime */
    unsafe {
        clock_gettime(clockid, &mut now);
    }
    rel.tv_sec = 0;
    rel.tv_nsec = 0;
    rel = timespec_add(rel, ns as u64);
    target = timespec_add(now, ns as u64);
    unsafe {
        clock_nanosleep(clockid, 0, &rel, core::ptr::null_mut());
        clock_gettime(clockid, &mut now);
    }

    if in_order(target, now) == 0 {
        return KSFT_FAIL;
    }
    KSFT_PASS
}

unsafe extern "C" fn dummy_event_handler(_val: c_int) {
    /* No action needed */
}

fn nanosleep_test_remaining(clockid: c_int) -> c_int {
    let mut rqtp = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rmtp = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut itimer = itimerspec {
        it_interval: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        it_value: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    };
    let mut sa = sigaction {
        sa_handler: None,
        sa_flags: 0,
        sa_restorer: None,
        sa_mask: sigset_t { __val: [0; 16] },
    };
    let mut timer: timer_t = core::ptr::null_mut();
    let mut ret: c_int;

    sa.sa_handler = Some(dummy_event_handler);
    unsafe {
        ret = sigaction(SIGALRM, &sa, core::ptr::null_mut());
    }
    if ret != 0 {
        return KSFT_FAIL;
    }

    unsafe {
        ret = timer_create(clockid, core::ptr::null_mut(), &mut timer);
    }
    if ret != 0 {
        return KSFT_FAIL;
    }

    itimer.it_value.tv_nsec = NSEC_PER_SEC / 4;
    unsafe {
        ret = timer_settime(timer, 0, &itimer, core::ptr::null_mut());
    }
    if ret != 0 {
        return KSFT_FAIL;
    }

    rqtp.tv_nsec = NSEC_PER_SEC / 2;
    unsafe {
        ret = clock_nanosleep(clockid, 0, &rqtp, &mut rmtp);
    }

    unsafe {
        if timer_delete(timer) != 0 {
            ksft_exit_fail_msg(
                b"Unable to delete the timeout timer for %s. This might interfere with following testcases.\n\0"
                    .as_ptr() as *const c_char,
                clock_name(clockid),
            );
        }
    }

    if ret != EINTR {
        return KSFT_FAIL;
    }

    sa.sa_handler = SIG_DFL;
    unsafe {
        ret = sigaction(SIGALRM, &sa, core::ptr::null_mut());
    }
    if ret != 0 {
        return KSFT_FAIL;
    }

    if in_order(
        timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        rmtp,
    ) == 0
    {
        return KSFT_FAIL;
    }

    if in_order(rmtp, rqtp) == 0 {
        return KSFT_FAIL;
    }

    KSFT_PASS
}

fn nanosleep_test_clock(clockid: clockid_t) {
    let mut length: i64 = 10;
    let mut ret: c_int;

    while length <= (NSEC_PER_SEC * 10) as i64 {
        ret = nanosleep_test(clockid, length);
        if ret != KSFT_PASS {
            unsafe {
                ksft_test_result_report(ret, b"%s\n\0".as_ptr() as *const c_char, clock_name(clockid));
                ksft_test_result_skip(
                    b"%s (remaining)\n\0".as_ptr() as *const c_char,
                    clock_name(clockid),
                );
            }
            return;
        }

        length *= 100;
    }
    unsafe {
        ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, clock_name(clockid));
    }

    ret = nanosleep_test_remaining(clockid);
    unsafe {
        ksft_test_result_report(
            ret,
            b"%s (remaining)\n\0".as_ptr() as *const c_char,
            clock_name(clockid),
        );
    }
}

fn main() {
    let mut clockid: c_int;

    static TESTED_CLOCKS: [clockid_t; 6] = [
        CLOCK_REALTIME,
        CLOCK_MONOTONIC,
        CLOCK_BOOTTIME,
        CLOCK_BOOTTIME_ALARM,
        CLOCK_REALTIME_ALARM,
        CLOCK_TAI,
    ];

    unsafe {
        ksft_print_header();
        ksft_set_plan((TESTED_CLOCKS.len() * 2) as c_int);
    }

    let mut clock_index: usize = 0;
    while clock_index < TESTED_CLOCKS.len() {
        clockid = TESTED_CLOCKS[clock_index];

        unsafe {
            fflush(stdout);
        }

        nanosleep_test_clock(clockid);
        clock_index += 1;
    }
    unsafe {
        ksft_finished();
    }
}
