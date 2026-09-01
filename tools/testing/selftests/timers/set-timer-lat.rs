/* set_timer latency test
 *		John Stultz (john.stultz@linaro.org)
 *              (C) Copyright Linaro 2014
 *              Licensed under the GPLv2
 *
 *   This test makes sure the set_timer api is correct
 *
 *  To build:
 *	$ gcc set-timer-lat.c -o set-timer-lat -lrt
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

// C dependencies: errno.h, stdio.h, unistd.h, time.h, string.h, signal.h,
// stdlib.h, pthread.h, "clock-helpers.h", and "kselftest.h".

use libc::{
    c_char, c_int, c_long, c_void, clockid_t, itimerspec, sigaction, sigevent, time_t, timer_t,
    timespec, timeval,
};
use std::mem;
use std::ptr;

/* CLOCK_HWSPECIFIC == CLOCK_SGI_CYCLE (Deprecated) */
const CLOCK_HWSPECIFIC: c_int = 10;

const UNRESONABLE_LATENCY: i64 = 40000000; /* 40ms in nanosecs */

const TIMER_SECS: c_int = 1;
const NSEC_PER_SEC: i64 = 1000000000;

static mut alarmcount: c_int = 0;
static mut clock_id: c_int = 0;
static mut start_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut max_latency_ns: i64 = 0;
static mut timer_fired_early: c_int = 0;

extern "C" {
    fn clock_name(clockid: clockid_t) -> *const c_char;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
    fn __errno_location() -> *mut c_int;
}

fn timespec_sub(a: timespec, b: timespec) -> i64 {
    let mut ret: i64 = NSEC_PER_SEC * b.tv_sec as i64 + b.tv_nsec as i64;

    ret -= NSEC_PER_SEC * a.tv_sec as i64 + a.tv_nsec as i64;
    ret
}

extern "C" fn sigalarm(_signo: c_int) {
    let mut ts: timespec = unsafe { mem::zeroed() };

    unsafe {
        libc::clock_gettime(clock_id as clockid_t, &mut ts);
        alarmcount += 1;

        let mut delta_ns = timespec_sub(start_time, ts);
        delta_ns -= NSEC_PER_SEC * TIMER_SECS as i64 * alarmcount as i64;

        if delta_ns < 0 {
            timer_fired_early = 1;
        }

        if delta_ns > max_latency_ns {
            max_latency_ns = delta_ns;
        }
    }
}

unsafe fn describe_timer(flags: c_int, interval: c_int) {
    libc::printf(
        b"%-22s %s %s \0".as_ptr() as *const c_char,
        clock_name(clock_id as clockid_t),
        if flags != 0 {
            b"ABSTIME\0".as_ptr()
        } else {
            b"RELTIME\0".as_ptr()
        } as *const c_char,
        if interval != 0 {
            b"PERIODIC\0".as_ptr()
        } else {
            b"ONE-SHOT\0".as_ptr()
        } as *const c_char,
    );
}

unsafe fn setup_timer(clock_id_arg: c_int, flags: c_int, interval: c_int, tm1: *mut timer_t) -> c_int {
    let mut se: sigevent = mem::zeroed();
    let mut its1: itimerspec = mem::zeroed();
    let mut its2: itimerspec = mem::zeroed();
    let mut err: c_int;

    /* Set up timer: */
    ptr::write_bytes(
        &mut se as *mut sigevent as *mut c_void,
        0,
        mem::size_of::<sigevent>(),
    );
    se.sigev_notify = libc::SIGEV_SIGNAL;
    se.sigev_signo = libc::SIGRTMAX();
    se.sigev_value.sival_ptr = ptr::null_mut();

    max_latency_ns = 0;
    alarmcount = 0;
    timer_fired_early = 0;

    err = libc::timer_create(clock_id_arg as clockid_t, &mut se, tm1);
    if err != 0 {
        if clock_id_arg == libc::CLOCK_REALTIME_ALARM || clock_id_arg == libc::CLOCK_BOOTTIME_ALARM {
            libc::printf(
                b"%-22s %s missing CAP_WAKE_ALARM?    : [UNSUPPORTED]\n\0".as_ptr()
                    as *const c_char,
                clock_name(clock_id_arg as clockid_t),
                if flags != 0 {
                    b"ABSTIME\0".as_ptr()
                } else {
                    b"RELTIME\0".as_ptr()
                } as *const c_char,
            );
            /* Indicate timer isn't set, so caller doesn't wait */
            return 1;
        }
        libc::printf(
            b"%s - timer_create() failed\n\0".as_ptr() as *const c_char,
            clock_name(clock_id_arg as clockid_t),
        );
        return -1;
    }

    libc::clock_gettime(clock_id_arg as clockid_t, &mut start_time);
    if flags != 0 {
        its1.it_value = start_time;
        its1.it_value.tv_sec += TIMER_SECS as time_t;
    } else {
        its1.it_value.tv_sec = TIMER_SECS as time_t;
        its1.it_value.tv_nsec = 0 as c_long;
    }
    its1.it_interval.tv_sec = interval as time_t;
    its1.it_interval.tv_nsec = 0 as c_long;

    err = libc::timer_settime(*tm1, flags, &its1, &mut its2);
    if err != 0 {
        libc::printf(
            b"%s - timer_settime() failed\n\0".as_ptr() as *const c_char,
            clock_name(clock_id_arg as clockid_t),
        );
        return -1;
    }

    0
}

unsafe fn check_timer_latency(flags: c_int, interval: c_int) -> c_int {
    let mut err: c_int = 0;

    describe_timer(flags, interval);
    libc::printf(
        b"timer fired early: %7d : \0".as_ptr() as *const c_char,
        timer_fired_early,
    );
    if timer_fired_early == 0 {
        libc::printf(b"[OK]\n\0".as_ptr() as *const c_char);
    } else {
        libc::printf(b"[FAILED]\n\0".as_ptr() as *const c_char);
        err = -1;
    }

    describe_timer(flags, interval);
    libc::printf(
        b"max latency: %10lld ns : \0".as_ptr() as *const c_char,
        max_latency_ns,
    );

    if max_latency_ns < UNRESONABLE_LATENCY {
        libc::printf(b"[OK]\n\0".as_ptr() as *const c_char);
    } else {
        libc::printf(b"[FAILED]\n\0".as_ptr() as *const c_char);
        err = -1;
    }
    err
}

unsafe fn check_alarmcount(flags: c_int, interval: c_int) -> c_int {
    describe_timer(flags, interval);
    libc::printf(
        b"count: %19d : \0".as_ptr() as *const c_char,
        alarmcount,
    );
    if alarmcount == 1 {
        libc::printf(b"[OK]\n\0".as_ptr() as *const c_char);
        return 0;
    }
    libc::printf(b"[FAILED]\n\0".as_ptr() as *const c_char);
    -1
}

unsafe fn do_timer(clock_id_arg: c_int, flags: c_int) -> c_int {
    let mut tm1: timer_t = mem::zeroed();
    const interval: c_int = TIMER_SECS;
    let mut err: c_int;

    err = setup_timer(clock_id_arg, flags, interval, &mut tm1);
    /* Unsupported case - return 0 to not fail the test */
    if err != 0 {
        return if err == 1 { 0 } else { err };
    }

    while alarmcount < 5 {
        libc::sleep(1);
    }

    libc::timer_delete(tm1);
    check_timer_latency(flags, interval)
}

unsafe fn do_timer_oneshot(clock_id_arg: c_int, flags: c_int) -> c_int {
    let mut tm1: timer_t = mem::zeroed();
    const interval: c_int = 0;
    let mut timeout: timeval = mem::zeroed();
    let mut err: c_int;

    err = setup_timer(clock_id_arg, flags, interval, &mut tm1);
    /* Unsupported case - return 0 to not fail the test */
    if err != 0 {
        return if err == 1 { 0 } else { err };
    }

    ptr::write_bytes(
        &mut timeout as *mut timeval as *mut c_void,
        0,
        mem::size_of::<timeval>(),
    );
    timeout.tv_sec = 5;
    loop {
        err = libc::select(
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut timeout,
        );
        if !(err == -1 && *__errno_location() == libc::EINTR) {
            break;
        }
    }

    libc::timer_delete(tm1);
    err = check_timer_latency(flags, interval);
    err |= check_alarmcount(flags, interval);
    err
}

fn main() {
    unsafe {
        let mut act: sigaction = mem::zeroed();
        let signum: c_int = libc::SIGRTMAX();
        let mut ret: c_int = 0;
        let max_clocks: c_int = libc::CLOCK_TAI + 1;

        /* Set up signal handler: */
        libc::sigfillset(&mut act.sa_mask);
        act.sa_flags = 0;
        act.sa_sigaction = sigalarm as usize;
        libc::sigaction(signum, &act, ptr::null_mut());

        libc::printf(
            b"Setting timers for every %i seconds\n\0".as_ptr() as *const c_char,
            TIMER_SECS,
        );
        clock_id = 0;
        while clock_id < max_clocks {
            if clock_id == libc::CLOCK_PROCESS_CPUTIME_ID
                || clock_id == libc::CLOCK_THREAD_CPUTIME_ID
                || clock_id == libc::CLOCK_MONOTONIC_RAW
                || clock_id == libc::CLOCK_REALTIME_COARSE
                || clock_id == libc::CLOCK_MONOTONIC_COARSE
                || clock_id == CLOCK_HWSPECIFIC
            {
                clock_id += 1;
                continue;
            }

            ret |= do_timer(clock_id, libc::TIMER_ABSTIME);
            ret |= do_timer(clock_id, 0);
            ret |= do_timer_oneshot(clock_id, libc::TIMER_ABSTIME);
            ret |= do_timer_oneshot(clock_id, 0);
            clock_id += 1;
        }
        if ret != 0 {
            ksft_exit_fail();
        }
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
