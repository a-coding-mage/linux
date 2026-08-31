/* CLOCK_MONOTONIC vs CLOCK_MONOTONIC_RAW skew test
 *		by: john stultz (johnstul@us.ibm.com)
 *		    John Stultz <john.stultz@linaro.org>
 *		(C) Copyright IBM 2012
 *		(C) Copyright Linaro Limited 2015
 *		Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc raw_skew.c -o raw_skew -lrt
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

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_MONOTONIC_RAW: c_int = 4;
const NSEC_PER_SEC: i64 = 1_000_000_000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timex {
    pub modes: c_uint,
    pub offset: c_long,
    pub freq: c_long,
    pub maxerror: c_long,
    pub esterror: c_long,
    pub status: c_int,
    pub constant: c_long,
    pub precision: c_long,
    pub tolerance: c_long,
    pub time: timeval,
    pub tick: c_long,
    pub ppsfreq: c_long,
    pub jitter: c_long,
    pub shift: c_int,
    pub stabil: c_long,
    pub jitcnt: c_long,
    pub calcnt: c_long,
    pub errcnt: c_long,
    pub stbcnt: c_long,
    pub tai: c_int,
    pub __unused: [c_int; 11],
}

unsafe extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn adjtimex(buf: *mut timex) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn abs(i: c_int) -> c_int;

    static mut stdout: *mut c_void;

    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

fn shift_right(x: i64, s: i32) -> i64 {
    if x < 0 {
        -((-x) >> s)
    } else {
        x >> s
    }
}

fn llabs(mut val: i64) -> i64 {
    if val < 0 {
        val = -val;
    }
    val
}

fn ts_to_nsec(ts: timespec) -> u64 {
    (ts.tv_sec as u64)
        .wrapping_mul(NSEC_PER_SEC as u64)
        .wrapping_add(ts.tv_nsec as u64)
}

fn nsec_to_ts(ns: i64) -> timespec {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    ts.tv_sec = (ns / NSEC_PER_SEC) as c_long;
    ts.tv_nsec = (ns % NSEC_PER_SEC) as c_long;
    ts
}

fn diff_timespec(start: timespec, end: timespec) -> i64 {
    let start_ns: i64;
    let end_ns: i64;

    start_ns = ts_to_nsec(start) as i64;
    end_ns = ts_to_nsec(end) as i64;
    end_ns - start_ns
}

unsafe fn get_monotonic_and_raw(mon: *mut timespec, raw: *mut timespec) {
    let mut start = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut mid = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut diff: i64 = 0;
    let mut tmp: i64;
    let mut i: c_int;

    i = 0;
    while i < 3 {
        let newdiff: i64;

        unsafe {
            clock_gettime(CLOCK_MONOTONIC, &mut start);
            clock_gettime(CLOCK_MONOTONIC_RAW, &mut mid);
            clock_gettime(CLOCK_MONOTONIC, &mut end);
        }

        newdiff = diff_timespec(start, end);
        if diff == 0 || newdiff < diff {
            diff = newdiff;
            unsafe {
                *raw = mid;
            }
            tmp = ((ts_to_nsec(start).wrapping_add(ts_to_nsec(end))) / 2) as i64;
            unsafe {
                *mon = nsec_to_ts(tmp);
            }
        }

        i += 1;
    }
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut mon = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut raw = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut start: timespec;
    let mut end: timespec;
    let mut delta1: i64;
    let mut delta2: i64;
    let interval: i64;
    let mut eppm: i64;
    let mut ppm: i64;
    let mut tx1 = timex {
        modes: 0,
        offset: 0,
        freq: 0,
        maxerror: 0,
        esterror: 0,
        status: 0,
        constant: 0,
        precision: 0,
        tolerance: 0,
        time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        tick: 0,
        ppsfreq: 0,
        jitter: 0,
        shift: 0,
        stabil: 0,
        jitcnt: 0,
        calcnt: 0,
        errcnt: 0,
        stbcnt: 0,
        tai: 0,
        __unused: [0; 11],
    };
    let mut tx2 = tx1;

    unsafe {
        setbuf(stdout, core::ptr::null_mut());
    }

    if unsafe { clock_gettime(CLOCK_MONOTONIC_RAW, &mut raw) } != 0 {
        unsafe {
            printf(c"ERR: NO CLOCK_MONOTONIC_RAW\n".as_ptr());
        }
        return -1;
    }

    tx1.modes = 0;
    unsafe {
        adjtimex(&mut tx1);
        get_monotonic_and_raw(&mut mon, &mut raw);
    }
    start = mon;
    delta1 = diff_timespec(mon, raw);

    if tx1.offset != 0 {
        unsafe {
            printf(c"WARNING: ADJ_OFFSET in progress, this will cause inaccurate results\n".as_ptr());
        }
    }

    unsafe {
        printf(c"Estimating clock drift: ".as_ptr());
        fflush(stdout);
        sleep(120);
    }

    unsafe {
        get_monotonic_and_raw(&mut mon, &mut raw);
    }
    end = mon;
    tx2.modes = 0;
    unsafe {
        adjtimex(&mut tx2);
    }
    delta2 = diff_timespec(mon, raw);

    interval = diff_timespec(start, end);

    /* calculate measured ppm between MONOTONIC and MONOTONIC_RAW */
    eppm = ((delta2 - delta1) * NSEC_PER_SEC) / interval;
    eppm = -eppm;
    unsafe {
        printf(
            c"%lld.%i(est)".as_ptr(),
            eppm / 1000,
            abs((eppm % 1000) as c_int),
        );
    }

    /* Avg the two actual freq samples adjtimex gave us */
    ppm = (tx1.freq + tx2.freq) as i64 * 1000 / 2;
    ppm = shift_right(ppm, 16);
    unsafe {
        printf(
            c" %lld.%i(act)".as_ptr(),
            ppm / 1000,
            abs((ppm % 1000) as c_int),
        );
    }

    if llabs(eppm - ppm) > 1000 {
        if tx1.offset != 0 || tx2.offset != 0 || tx1.freq != tx2.freq || tx1.tick != tx2.tick {
            unsafe {
                printf(c"\t[SKIP]\n".as_ptr());
                ksft_exit_skip(
                    c"The clock was adjusted externally. Shutdown NTPd or other time sync daemons\n"
                        .as_ptr(),
                );
            }
        }
        unsafe {
            printf(c"\t[FAILED]\n".as_ptr());
            ksft_exit_fail();
        }
    }
    unsafe {
        printf(c"\t[OK]\n".as_ptr());
        ksft_exit_pass();
    }
}

fn main() {
    unsafe {
        main_impl(0, core::ptr::null_mut());
    }
}
