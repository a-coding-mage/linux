/* Set tai offset
 *              by: John Stultz <john.stultz@linaro.org>
 *              (C) Copyright Linaro 2013
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

// C dependencies: stdio.h, stdlib.h, time.h, sys/time.h, sys/timex.h,
// string.h, signal.h, unistd.h, and "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;

const ADJ_TAI: c_uint = 0x0080;

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
    pub __glibc_reserved1: c_int,
    pub __glibc_reserved2: c_int,
    pub __glibc_reserved3: c_int,
    pub __glibc_reserved4: c_int,
    pub __glibc_reserved5: c_int,
    pub __glibc_reserved6: c_int,
    pub __glibc_reserved7: c_int,
    pub __glibc_reserved8: c_int,
    pub __glibc_reserved9: c_int,
    pub __glibc_reserved10: c_int,
    pub __glibc_reserved11: c_int,
}

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn adjtimex(buf: *mut timex) -> c_int;

    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

pub unsafe fn set_tai(offset: c_int) -> c_int {
    let mut tx: timex = mem::zeroed();

    memset(
        &mut tx as *mut timex as *mut c_void,
        0,
        mem::size_of::<timex>(),
    );

    tx.modes = ADJ_TAI;
    tx.constant = offset as c_long;

    adjtimex(&mut tx)
}

pub unsafe fn get_tai() -> c_int {
    let mut tx: timex = mem::zeroed();

    memset(
        &mut tx as *mut timex as *mut c_void,
        0,
        mem::size_of::<timex>(),
    );

    adjtimex(&mut tx);
    tx.tai
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;

    ret = get_tai();
    printf(
        b"tai offset started at %i\n\0".as_ptr() as *const c_char,
        ret,
    );

    printf(b"Checking tai offsets can be properly set: \0".as_ptr() as *const c_char);
    fflush(stdout);
    i = 1;
    while i <= 60 {
        ret = set_tai(i);
        ret = get_tai();
        if ret != i {
            printf(
                b"[FAILED] expected: %i got %i\n\0".as_ptr() as *const c_char,
                i,
                ret,
            );
            ksft_exit_fail();
        }
        i += 1;
    }
    printf(b"[OK]\n\0".as_ptr() as *const c_char);
    ksft_exit_pass();
}
