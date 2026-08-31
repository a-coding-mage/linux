/* ADJ_FREQ Skew change test
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2012
 *		Licensed under the GPLv2
 *
 *  NOTE: This is a meta-test which cranks the ADJ_FREQ knob and
 *  then uses other tests to detect problems. Thus this test requires
 *  that the raw_skew, inconsistency-check and nanosleep tests be
 *  present in the same directory it is run from.
 *
 *  To build:
 *	$ gcc change_skew.c -o change_skew -lrt
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

use std::ffi::c_char;

extern "C" {
    fn adjtimex(buf: *mut timex) -> i32;
    fn system(command: *const c_char) -> i32;
    fn printf(format: *const c_char, ...) -> i32;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

const ADJ_OFFSET: u32 = 0x0001;
const ADJ_FREQUENCY: u32 = 0x0002;

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timex {
    modes: u32,
    offset: i64,
    freq: i64,
    maxerror: i64,
    esterror: i64,
    status: i32,
    constant: i64,
    precision: i64,
    tolerance: i64,
    time: timeval,
    tick: i64,
    ppsfreq: i64,
    jitter: i64,
    shift: i32,
    stabil: i64,
    jitcnt: i64,
    calcnt: i64,
    errcnt: i64,
    stbcnt: i64,
    tai: i32,
    __unused: [i32; 11],
}

fn change_skew_test(ppm: i32) -> i32 {
    let mut tx: timex = unsafe { std::mem::zeroed() };
    let mut ret: i32;

    tx.modes = ADJ_FREQUENCY;
    tx.freq = (ppm << 16) as i64;

    ret = unsafe { adjtimex(&mut tx) };
    if ret < 0 {
        unsafe {
            printf(c"Error adjusting freq\n".as_ptr());
        }
        return ret;
    }

    ret = unsafe { system(c"./raw_skew".as_ptr()) };
    ret |= unsafe { system(c"./inconsistency-check".as_ptr()) };
    ret |= unsafe { system(c"./nanosleep".as_ptr()) };

    ret
}

fn main() {
    let mut tx: timex = unsafe { std::mem::zeroed() };
    let mut i: usize;
    let mut ret: i32;

    let ppm: [i32; 5] = [0, 250, 500, -250, -500];

    /* Kill ntpd */
    ret = unsafe { system(c"killall -9 ntpd".as_ptr()) };

    /* Make sure there's no offset adjustment going on */
    tx.modes = ADJ_OFFSET;
    tx.offset = 0;
    ret = unsafe { adjtimex(&mut tx) };

    if ret < 0 {
        unsafe {
            printf(c"Maybe you're not running as root?\n".as_ptr());
        }
        std::process::exit(-1);
    }

    i = 0;
    while i < 5 {
        unsafe {
            printf(c"Using %i ppm adjustment\n".as_ptr(), ppm[i]);
        }
        ret = change_skew_test(ppm[i]);
        if ret != 0 {
            break;
        }
        i += 1;
    }

    /* Set things back */
    tx.modes = ADJ_FREQUENCY;
    tx.offset = 0;
    unsafe {
        adjtimex(&mut tx);
    }

    if ret != 0 {
        unsafe {
            printf(c"[FAIL]".as_ptr());
            ksft_exit_fail();
        }
    }
    unsafe {
        printf(c"[OK]".as_ptr());
        ksft_exit_pass();
    }
}
