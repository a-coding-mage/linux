/* Measure mqueue timeout latency
 *              by: john stultz (john.stultz@linaro.org)
 *		(C) Copyright Linaro 2013
 *
 *		Inspired with permission from example test by:
 *			Romain Francoise <romain@orebokech.com>
 *              Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc mqueue-lat.c -o mqueue-lat -lrt
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

/* C dependencies: stdio.h, stdlib.h, time.h, sys/time.h, sys/timex.h,
 * string.h, signal.h, errno.h, mqueue.h, "clock-helpers.h", "kselftest.h".
 */

const TARGET_TIMEOUT: libc::c_longlong = 100000000; /* 100ms in nanoseconds */
const UNRESONABLE_LATENCY: libc::c_longlong = 40000000; /* 40ms in nanosecs */

const NSEC_PER_SEC: libc::c_longlong = 1000000000;

extern "C" {
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

unsafe fn errno_location() -> *mut libc::c_int {
    #[cfg(any(target_env = "gnu", target_env = "musl"))]
    {
        libc::__errno_location()
    }

    #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
    {
        /* TODO: map errno location for this target's C library. */
        core::ptr::null_mut()
    }
}

pub unsafe fn timespec_sub(a: libc::timespec, b: libc::timespec) -> libc::c_longlong {
    let mut ret: libc::c_longlong =
        NSEC_PER_SEC * b.tv_sec as libc::c_longlong + b.tv_nsec as libc::c_longlong;

    ret -= NSEC_PER_SEC * a.tv_sec as libc::c_longlong + a.tv_nsec as libc::c_longlong;
    ret
}

pub unsafe fn timespec_add(
    mut ts: libc::timespec,
    ns: libc::c_ulonglong,
) -> libc::timespec {
    ts.tv_nsec += ns as libc::c_long;
    while ts.tv_nsec >= NSEC_PER_SEC as libc::c_long {
        ts.tv_nsec -= NSEC_PER_SEC as libc::c_long;
        ts.tv_sec += 1;
    }
    ts
}

pub unsafe fn mqueue_lat_test() -> libc::c_int {
    let q: libc::mqd_t;
    let mut attr: libc::mq_attr = core::mem::zeroed();
    let mut start: libc::timespec = core::mem::zeroed();
    let mut end: libc::timespec = core::mem::zeroed();
    let mut now: libc::timespec = core::mem::zeroed();
    let mut target: libc::timespec;
    let mut i: libc::c_int;
    let count: libc::c_int;
    let mut ret: libc::c_int;

    q = libc::mq_open(
        b"/foo\0".as_ptr() as *const libc::c_char,
        libc::O_CREAT | libc::O_RDONLY,
        0o666,
        core::ptr::null_mut::<libc::mq_attr>(),
    );
    if q < 0 {
        libc::perror(b"mq_open\0".as_ptr() as *const libc::c_char);
        return -1;
    }
    libc::mq_getattr(q, &mut attr);

    count = 100;
    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut start);

    i = 0;
    while i < count {
        let mut buf: Vec<libc::c_char> = vec![0; attr.mq_msgsize as usize];

        libc::clock_gettime(libc::CLOCK_REALTIME, &mut now);
        target = now;
        target = timespec_add(now, TARGET_TIMEOUT as libc::c_ulonglong); /* 100ms */

        ret = libc::mq_timedreceive(
            q,
            buf.as_mut_ptr(),
            core::mem::size_of_val(buf.as_slice()),
            core::ptr::null_mut(),
            &target,
        ) as libc::c_int;
        if ret < 0 && *errno_location() != libc::ETIMEDOUT {
            libc::perror(b"mq_timedreceive\0".as_ptr() as *const libc::c_char);
            return -1;
        }

        i += 1;
    }
    libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut end);

    libc::mq_close(q);

    if timespec_sub(start, end) / count as libc::c_longlong
        > TARGET_TIMEOUT + UNRESONABLE_LATENCY
    {
        return -1;
    }

    0
}

fn main() {
    unsafe {
        let ret: libc::c_int;

        libc::printf(
            b"Mqueue latency :                          \0".as_ptr() as *const libc::c_char,
        );
        libc::fflush(libc::stdout);

        ret = mqueue_lat_test();
        if ret < 0 {
            libc::printf(b"[FAILED]\n\0".as_ptr() as *const libc::c_char);
            ksft_exit_fail();
        }
        libc::printf(b"[OK]\n\0".as_ptr() as *const libc::c_char);
        ksft_exit_pass();
    }
}
