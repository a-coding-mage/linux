/* valid adjtimex test
 *              by: John Stultz <john.stultz@linaro.org>
 *              (C) Copyright Linaro 2015
 *              Licensed under the GPLv2
 *
 *  This test validates adjtimex interface with valid
 *  and invalid test data.
 *
 *  Usage: valid-adjtimex
 *
 *  To build:
 *	$ gcc valid-adjtimex.c -o valid-adjtimex -lrt
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

use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

type clockid_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timex {
    modes: u32,
    offset: c_long,
    freq: c_long,
    maxerror: c_long,
    esterror: c_long,
    status: c_int,
    constant: c_long,
    precision: c_long,
    tolerance: c_long,
    time: timeval,
    tick: c_long,
    ppsfreq: c_long,
    jitter: c_long,
    shift: c_int,
    stabil: c_long,
    jitcnt: c_long,
    calcnt: c_long,
    errcnt: c_long,
    stbcnt: c_long,
    tai: c_int,
    __unused: [c_int; 11],
}

const ADJ_FREQUENCY: u32 = 0x0002;
const ADJ_STATUS: u32 = 0x0010;
const ADJ_NANO: u32 = 0x2000;
const ADJ_SETOFFSET: u32 = 0x0100;

const CLOCK_REALTIME: clockid_t = 0;
const NSEC_PER_SEC: i64 = 1_000_000_000;
const USEC_PER_SEC: i64 = 1_000_000;

#[cfg(target_arch = "x86_64")]
const __NR_clock_adjtime: c_long = 305;
#[cfg(target_arch = "x86")]
const __NR_clock_adjtime: c_long = 343;
#[cfg(target_arch = "aarch64")]
const __NR_clock_adjtime: c_long = 266;
#[cfg(target_arch = "arm")]
const __NR_clock_adjtime: c_long = 372;
#[cfg(target_arch = "riscv64")]
const __NR_clock_adjtime: c_long = 266;

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn adjtimex(tx: *mut timex) -> c_int;

    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

unsafe fn clock_adjtime(id: clockid_t, tx: *mut timex) -> c_int {
    syscall(__NR_clock_adjtime, id, tx) as c_int
}

/* clear NTP time_status & time_state */
unsafe fn clear_time_state() -> c_int {
    let mut tx: timex = mem::zeroed();
    let ret: c_int;

    tx.modes = ADJ_STATUS;
    tx.status = 0;
    ret = adjtimex(&mut tx);
    ret
}

const NUM_FREQ_VALID: usize = 32;
const NUM_FREQ_OUTOFRANGE: usize = 4;
const NUM_FREQ_INVALID: usize = 2;

const SHIFTED_PPM: c_long = 1 << 16;

static valid_freq: [c_long; NUM_FREQ_VALID] = [
    -499 * SHIFTED_PPM,
    -450 * SHIFTED_PPM,
    -400 * SHIFTED_PPM,
    -350 * SHIFTED_PPM,
    -300 * SHIFTED_PPM,
    -250 * SHIFTED_PPM,
    -200 * SHIFTED_PPM,
    -150 * SHIFTED_PPM,
    -100 * SHIFTED_PPM,
    -75 * SHIFTED_PPM,
    -50 * SHIFTED_PPM,
    -25 * SHIFTED_PPM,
    -10 * SHIFTED_PPM,
    -5 * SHIFTED_PPM,
    -1 * SHIFTED_PPM,
    -1000,
    1 * SHIFTED_PPM,
    5 * SHIFTED_PPM,
    10 * SHIFTED_PPM,
    25 * SHIFTED_PPM,
    50 * SHIFTED_PPM,
    75 * SHIFTED_PPM,
    100 * SHIFTED_PPM,
    150 * SHIFTED_PPM,
    200 * SHIFTED_PPM,
    250 * SHIFTED_PPM,
    300 * SHIFTED_PPM,
    350 * SHIFTED_PPM,
    400 * SHIFTED_PPM,
    450 * SHIFTED_PPM,
    499 * SHIFTED_PPM,
];

static outofrange_freq: [c_long; NUM_FREQ_OUTOFRANGE] = [
    -1000 * SHIFTED_PPM,
    -550 * SHIFTED_PPM,
    550 * SHIFTED_PPM,
    1000 * SHIFTED_PPM,
];

const LONG_MAX: c_long = c_long::MAX;
const LONG_MIN: c_long = -LONG_MAX - 1;

static invalid_freq: [c_long; NUM_FREQ_INVALID] = [
    LONG_MAX,
    LONG_MIN,
];

unsafe fn validate_freq() -> c_int {
    let mut tx: timex = mem::zeroed();
    let mut ret: c_int;
    let mut pass: c_int = 0;
    let mut i: usize;

    clear_time_state();

    ptr::write_bytes(&mut tx as *mut timex as *mut u8, 0, mem::size_of::<timex>());
    /* Set the leap second insert flag */

    printf(b"Testing ADJ_FREQ... \0".as_ptr() as *const c_char);
    fflush(stdout);
    i = 0;
    while i < NUM_FREQ_VALID {
        tx.modes = ADJ_FREQUENCY;
        tx.freq = valid_freq[i];

        ret = adjtimex(&mut tx);
        if ret < 0 {
            printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
            printf(
                b"Error: adjtimex(ADJ_FREQ, %ld - %ld ppm\n\0".as_ptr() as *const c_char,
                valid_freq[i],
                valid_freq[i] >> 16,
            );
            pass = -1;
            goto_out_validate_freq(&mut tx);
            return pass;
        }
        tx.modes = 0;
        ret = adjtimex(&mut tx);
        if tx.freq != valid_freq[i] {
            printf(
                b"Warning: freq value %ld not what we set it (%ld)!\n\0".as_ptr()
                    as *const c_char,
                tx.freq,
                valid_freq[i],
            );
        }
        i += 1;
    }
    i = 0;
    while i < NUM_FREQ_OUTOFRANGE {
        tx.modes = ADJ_FREQUENCY;
        tx.freq = outofrange_freq[i];

        ret = adjtimex(&mut tx);
        if ret < 0 {
            printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
            printf(
                b"Error: adjtimex(ADJ_FREQ, %ld - %ld ppm\n\0".as_ptr() as *const c_char,
                outofrange_freq[i],
                outofrange_freq[i] >> 16,
            );
            pass = -1;
            goto_out_validate_freq(&mut tx);
            return pass;
        }
        tx.modes = 0;
        ret = adjtimex(&mut tx);
        if tx.freq == outofrange_freq[i] {
            printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
            printf(
                b"ERROR: out of range value %ld actually set!\n\0".as_ptr() as *const c_char,
                tx.freq,
            );
            pass = -1;
            goto_out_validate_freq(&mut tx);
            return pass;
        }
        i += 1;
    }

    if mem::size_of::<c_long>() == 8 {
        /* this case only applies to 64bit systems */
        i = 0;
        while i < NUM_FREQ_INVALID {
            tx.modes = ADJ_FREQUENCY;
            tx.freq = invalid_freq[i];
            ret = adjtimex(&mut tx);
            if ret >= 0 {
                printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
                printf(
                    b"Error: No failure on invalid ADJ_FREQUENCY %ld\n\0".as_ptr()
                        as *const c_char,
                    invalid_freq[i],
                );
                pass = -1;
                goto_out_validate_freq(&mut tx);
                return pass;
            }
            i += 1;
        }
    }

    printf(b"[OK]\n\0".as_ptr() as *const c_char);

    /* reset freq to zero */
    tx.modes = ADJ_FREQUENCY;
    tx.freq = 0;
    ret = adjtimex(&mut tx);
    let _ = ret;

    pass
}

unsafe fn goto_out_validate_freq(tx: *mut timex) {
    /* reset freq to zero */
    (*tx).modes = ADJ_FREQUENCY;
    (*tx).freq = 0;
    let _ret = adjtimex(tx);
}

unsafe fn set_offset(offset: i64, use_nano: c_int) -> c_int {
    let mut tmx: timex = mem::zeroed();
    let ret: c_int;

    tmx.modes = ADJ_SETOFFSET;
    if use_nano != 0 {
        tmx.modes |= ADJ_NANO;

        tmx.time.tv_sec = (offset / NSEC_PER_SEC) as c_long;
        tmx.time.tv_usec = (offset % NSEC_PER_SEC) as c_long;

        if offset < 0 && tmx.time.tv_usec != 0 {
            tmx.time.tv_sec -= 1;
            tmx.time.tv_usec += NSEC_PER_SEC as c_long;
        }
    } else {
        tmx.time.tv_sec = (offset / USEC_PER_SEC) as c_long;
        tmx.time.tv_usec = (offset % USEC_PER_SEC) as c_long;

        if offset < 0 && tmx.time.tv_usec != 0 {
            tmx.time.tv_sec -= 1;
            tmx.time.tv_usec += USEC_PER_SEC as c_long;
        }
    }

    ret = clock_adjtime(CLOCK_REALTIME, &mut tmx);
    if ret < 0 {
        printf(
            b"(sec: %ld  usec: %ld) \0".as_ptr() as *const c_char,
            tmx.time.tv_sec,
            tmx.time.tv_usec,
        );
        printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

unsafe fn set_bad_offset(sec: c_long, usec: c_long, use_nano: c_int) -> c_int {
    let mut tmx: timex = mem::zeroed();
    let ret: c_int;

    tmx.modes = ADJ_SETOFFSET;
    if use_nano != 0 {
        tmx.modes |= ADJ_NANO;
    }

    tmx.time.tv_sec = sec;
    tmx.time.tv_usec = usec;
    ret = clock_adjtime(CLOCK_REALTIME, &mut tmx);
    if ret >= 0 {
        printf(
            b"Invalid (sec: %ld  usec: %ld) did not fail! \0".as_ptr() as *const c_char,
            tmx.time.tv_sec,
            tmx.time.tv_usec,
        );
        printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

unsafe fn validate_set_offset() -> c_int {
    printf(b"Testing ADJ_SETOFFSET... \0".as_ptr() as *const c_char);
    fflush(stdout);

    /* Test valid values */
    if set_offset(NSEC_PER_SEC - 1, 1) != 0 {
        return -1;
    }

    if set_offset(-NSEC_PER_SEC + 1, 1) != 0 {
        return -1;
    }

    if set_offset(-NSEC_PER_SEC - 1, 1) != 0 {
        return -1;
    }

    if set_offset(5 * NSEC_PER_SEC, 1) != 0 {
        return -1;
    }

    if set_offset(-5 * NSEC_PER_SEC, 1) != 0 {
        return -1;
    }

    if set_offset(5 * NSEC_PER_SEC + NSEC_PER_SEC / 2, 1) != 0 {
        return -1;
    }

    if set_offset(-5 * NSEC_PER_SEC - NSEC_PER_SEC / 2, 1) != 0 {
        return -1;
    }

    if set_offset(USEC_PER_SEC - 1, 0) != 0 {
        return -1;
    }

    if set_offset(-USEC_PER_SEC + 1, 0) != 0 {
        return -1;
    }

    if set_offset(-USEC_PER_SEC - 1, 0) != 0 {
        return -1;
    }

    if set_offset(5 * USEC_PER_SEC, 0) != 0 {
        return -1;
    }

    if set_offset(-5 * USEC_PER_SEC, 0) != 0 {
        return -1;
    }

    if set_offset(5 * USEC_PER_SEC + USEC_PER_SEC / 2, 0) != 0 {
        return -1;
    }

    if set_offset(-5 * USEC_PER_SEC - USEC_PER_SEC / 2, 0) != 0 {
        return -1;
    }

    /* Test invalid values */
    if set_bad_offset(0, -1, 1) != 0 {
        return -1;
    }
    if set_bad_offset(0, -1, 0) != 0 {
        return -1;
    }
    if set_bad_offset(0, (2 * NSEC_PER_SEC) as c_long, 1) != 0 {
        return -1;
    }
    if set_bad_offset(0, (2 * USEC_PER_SEC) as c_long, 0) != 0 {
        return -1;
    }
    if set_bad_offset(0, NSEC_PER_SEC as c_long, 1) != 0 {
        return -1;
    }
    if set_bad_offset(0, USEC_PER_SEC as c_long, 0) != 0 {
        return -1;
    }
    if set_bad_offset(0, (-NSEC_PER_SEC) as c_long, 1) != 0 {
        return -1;
    }
    if set_bad_offset(0, (-USEC_PER_SEC) as c_long, 0) != 0 {
        return -1;
    }

    printf(b"[OK]\n\0".as_ptr() as *const c_char);
    0
}

fn main() {
    unsafe {
        if validate_freq() != 0 {
            ksft_exit_fail();
        }

        if validate_set_offset() != 0 {
            ksft_exit_fail();
        }

        ksft_exit_pass();
    }
}
