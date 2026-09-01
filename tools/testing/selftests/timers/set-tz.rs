/* Set tz value
 *              by: John Stultz <john.stultz@linaro.org>
 *              (C) Copyright Linaro 2016
 *              Licensed under the GPLv2
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

/* C dependencies:
 * stdio.h, stdlib.h, time.h, sys/time.h, sys/timex.h, string.h,
 * signal.h, unistd.h, and "kselftest.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

type c_char = i8;
type c_int = i32;
type c_long = i64;
type c_void = core::ffi::c_void;

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn settimeofday(tv: *const timeval, tz: *const timezone) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int;
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn set_tz(min: c_int, dst: c_int) -> c_int {
    let mut tz: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };

    tz.tz_minuteswest = min;
    tz.tz_dsttime = dst;

    unsafe { settimeofday(core::ptr::null(), &tz) }
}

#[no_mangle]
pub unsafe extern "C" fn get_tz_min() -> c_int {
    let mut tz: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        core::ptr::write_bytes(
            &mut tz as *mut timezone as *mut u8,
            0,
            core::mem::size_of::<timezone>(),
        );
        gettimeofday(&mut tv, &mut tz);
    }
    tz.tz_minuteswest
}

#[no_mangle]
pub unsafe extern "C" fn get_tz_dst() -> c_int {
    let mut tz: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        core::ptr::write_bytes(
            &mut tz as *mut timezone as *mut u8,
            0,
            core::mem::size_of::<timezone>(),
        );
        gettimeofday(&mut tv, &mut tz);
    }
    tz.tz_dsttime
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let min: c_int;
    let dst: c_int;

    unsafe {
        min = get_tz_min();
        dst = get_tz_dst();
        printf(
            b"tz_minuteswest started at %i, dst at %i\n\0".as_ptr() as *const c_char,
            min,
            dst,
        );

        printf(b"Checking tz_minuteswest can be properly set: \0".as_ptr() as *const c_char);
        fflush(stdout);
        i = -15 * 60;
        while i < 15 * 60 {
            ret = set_tz(i, dst);
            ret = get_tz_min();
            if ret != i {
                printf(
                    b"[FAILED] expected: %i got %i\n\0".as_ptr() as *const c_char,
                    i,
                    ret,
                );
                set_tz(min, dst);
                ksft_exit_fail();
            }
            i += 30;
        }
        printf(b"[OK]\n\0".as_ptr() as *const c_char);

        printf(b"Checking invalid tz_minuteswest values are caught: \0".as_ptr() as *const c_char);
        fflush(stdout);

        if set_tz(-15 * 60 - 1, dst) == 0 {
            printf(
                b"[FAILED] %i didn't return failure!\n\0".as_ptr() as *const c_char,
                -15 * 60 - 1,
            );
            set_tz(min, dst);
            ksft_exit_fail();
        }

        if set_tz(15 * 60 + 1, dst) == 0 {
            printf(
                b"[FAILED] %i didn't return failure!\n\0".as_ptr() as *const c_char,
                15 * 60 + 1,
            );
            set_tz(min, dst);
            ksft_exit_fail();
        }

        if set_tz(-24 * 60, dst) == 0 {
            printf(
                b"[FAILED] %i didn't return failure!\n\0".as_ptr() as *const c_char,
                -24 * 60,
            );
            set_tz(min, dst);
            ksft_exit_fail();
        }

        if set_tz(24 * 60, dst) == 0 {
            printf(
                b"[FAILED] %i didn't return failure!\n\0".as_ptr() as *const c_char,
                24 * 60,
            );
            set_tz(min, dst);
            ksft_exit_fail();
        }

        printf(b"[OK]\n\0".as_ptr() as *const c_char);

        set_tz(min, dst);
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
