/* adjtimex() tick adjustment test
 *		by:   John Stultz <john.stultz@linaro.org>
 *		(C) Copyright Linaro Limited 2015
 *		Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc adjtick.c -o adjtick -lrt
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

// C dependencies: stdio.h, unistd.h, stdlib.h, sys/time.h, sys/timex.h, time.h
// Local C dependencies: clock-helpers.h, kselftest.h

use std::ffi::{c_char, c_int, c_long, c_void};

const MILLION: c_long = 1000000;

const NSEC_PER_SEC: i64 = 1000000000;
const USEC_PER_SEC: c_long = 1000000;

const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_MONOTONIC_RAW: c_int = 4;
const _SC_CLK_TCK: c_int = 2;

const ADJ_OFFSET: c_int = 0x0001;
const ADJ_FREQUENCY: c_int = 0x0002;
const ADJ_STATUS: c_int = 0x0010;
const ADJ_TICK: c_int = 0x4000;
const STA_PLL: c_int = 0x0001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timex {
    pub modes: c_int,
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
    fn sleep(seconds: u32) -> u32;
    fn sysconf(name: c_int) -> c_long;
    fn adjtimex(buf: *mut timex) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;

    static mut stdout: *mut c_void;
}

static mut systick: c_long = 0;

pub fn llabs(mut val: i64) -> i64 {
    if val < 0 {
        val = -val;
    }
    val
}

pub fn ts_to_nsec(ts: timespec) -> u64 {
    (ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec) as u64
}

pub fn nsec_to_ts(ns: i64) -> timespec {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    ts.tv_sec = ns / NSEC_PER_SEC;
    ts.tv_nsec = ns % NSEC_PER_SEC;

    ts
}

pub fn diff_timespec(start: timespec, end: timespec) -> i64 {
    let start_ns: i64;
    let end_ns: i64;

    start_ns = ts_to_nsec(start) as i64;
    end_ns = ts_to_nsec(end) as i64;

    end_ns - start_ns
}

pub unsafe fn get_monotonic_and_raw(mon: *mut timespec, raw: *mut timespec) {
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut mid: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut diff: i64 = 0;
    let mut tmp: i64;
    let mut i: c_int;

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, mon);
        clock_gettime(CLOCK_MONOTONIC_RAW, raw);
    }

    /* Try to get a more tightly bound pairing */
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
            tmp = ((ts_to_nsec(start) + ts_to_nsec(end)) / 2) as i64;
            unsafe {
                *mon = nsec_to_ts(tmp);
            }
        }
        i += 1;
    }
}

pub unsafe fn get_ppm_drift() -> i64 {
    let mut mon_start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut raw_start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut mon_end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut raw_end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let delta1: i64;
    let delta2: i64;
    let eppm: i64;

    unsafe {
        get_monotonic_and_raw(&mut mon_start, &mut raw_start);

        sleep(15);

        get_monotonic_and_raw(&mut mon_end, &mut raw_end);
    }

    delta1 = diff_timespec(mon_start, mon_end);
    delta2 = diff_timespec(raw_start, raw_end);

    eppm = (delta1 * MILLION as i64) / delta2 - MILLION as i64;

    eppm
}

pub unsafe fn check_tick_adj(tickval: c_long) -> c_int {
    let eppm: i64;
    let ppm: i64;
    let mut tx1: timex = unsafe { std::mem::zeroed() };

    tx1.modes = ADJ_TICK;
    tx1.modes |= ADJ_OFFSET;
    tx1.modes |= ADJ_FREQUENCY;
    tx1.modes |= ADJ_STATUS;

    tx1.status = STA_PLL;
    tx1.offset = 0;
    tx1.freq = 0;
    tx1.tick = tickval;

    unsafe {
        adjtimex(&mut tx1);

        sleep(1);
    }

    ppm = ((tickval as i64 * MILLION as i64) / unsafe { systick } as i64) - MILLION as i64;
    unsafe {
        printf(
            c"Estimating tick (act: %ld usec, %lld ppm): ".as_ptr(),
            tickval,
            ppm,
        );
    }

    eppm = unsafe { get_ppm_drift() };
    unsafe {
        printf(
            c"%lld usec, %lld ppm".as_ptr(),
            systick + (systick * eppm as c_long / MILLION),
            eppm,
        );
        fflush(stdout);
    }

    tx1.modes = 0;
    unsafe {
        adjtimex(&mut tx1);
    }

    if tx1.offset != 0 || tx1.freq != 0 || tx1.tick != tickval {
        unsafe {
            printf(c"\t[ERROR]\n".as_ptr());
            printf(c"\tUnexpected adjtimex return values, make sure ntpd is not running.\n".as_ptr());
        }
        return -1;
    }

    /*
     * Here we use 100ppm difference as an error bound.
     * We likely should see better, but some coarse clocksources
     * cannot match the HZ tick size accurately, so we have a
     * internal correction factor that doesn't scale exactly
     * with the adjustment, resulting in > 10ppm error during
     * a 10% adjustment. 100ppm also gives us more breathing
     * room for interruptions during the measurement.
     */
    if llabs(eppm - ppm) > 100 {
        unsafe {
            printf(c"\t[FAILED]\n".as_ptr());
        }
        return -1;
    }
    unsafe {
        printf(c"\t[OK]\n".as_ptr());
    }

    0
}

fn main() {
    let mut raw: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut tick: c_long;
    let max: c_long;
    let interval: c_long;
    let mut err: c_long;
    let mut tx1: timex = unsafe { std::mem::zeroed() };

    err = 0;
    unsafe {
        setbuf(stdout, std::ptr::null_mut());
    }

    if unsafe { clock_gettime(CLOCK_MONOTONIC_RAW, &mut raw) } != 0 {
        unsafe {
            printf(c"ERR: NO CLOCK_MONOTONIC_RAW\n".as_ptr());
        }
        std::process::exit(-1);
    }

    unsafe {
        printf(c"Each iteration takes about 15 seconds\n".as_ptr());
    }

    unsafe {
        systick = sysconf(_SC_CLK_TCK);
        systick = USEC_PER_SEC / sysconf(_SC_CLK_TCK);
        max = systick / 10; /* +/- 10% */
        interval = max / 4; /* in 4 steps each side */

        tick = systick - max;
        while tick < systick + max {
            if check_tick_adj(tick) != 0 {
                err = 1;
                break;
            }
            tick += interval;
        }
    }

    /* Reset things to zero */
    tx1.modes = ADJ_TICK;
    tx1.modes |= ADJ_OFFSET;
    tx1.modes |= ADJ_FREQUENCY;

    tx1.offset = 0;
    tx1.freq = 0;
    tx1.tick = unsafe { systick };

    unsafe {
        adjtimex(&mut tx1);

        if err != 0 {
            ksft_exit_fail();
        }

        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
