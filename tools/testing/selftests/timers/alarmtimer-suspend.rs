/* alarmtimer suspend test
 *		John Stultz (john.stultz@linaro.org)
 *              (C) Copyright Linaro 2013
 *              Licensed under the GPLv2
 *
 *   This test makes sure the alarmtimer & RTC wakeup code is
 *   functioning.
 *
 *  To build:
 *	$ gcc alarmtimer-suspend.c -o alarmtimer-suspend -lrt
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

// C dependencies: stdio.h, unistd.h, time.h, string.h, signal.h, stdlib.h,
// pthread.h, errno.h, clock-helpers.h, kselftest.h.

use libc::{
    c_char, c_int, c_long, c_void, clockid_t, itimerspec, sigevent, sigset_t, time_t, timer_t,
    timespec, CLOCK_BOOTTIME_ALARM, CLOCK_REALTIME_ALARM, SIGEV_SIGNAL, TIMER_ABSTIME,
};

const NSEC_PER_SEC: i64 = 1_000_000_000;
const UNREASONABLE_LAT: i64 = NSEC_PER_SEC * 5; /* hopefully we resume in 5 secs */

const SUSPEND_SECS: i64 = 15;
static mut alarmcount: c_int = 0;
static mut alarm_clock_id: c_int = 0;
static mut start_time: timespec = timespec {
    tv_sec: 0 as time_t,
    tv_nsec: 0 as c_long,
};

extern "C" {
    fn clock_name(clockid: clockid_t) -> *const c_char;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

unsafe fn errno_location() -> *mut c_int {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        libc::__errno_location()
    }

    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        core::ptr::null_mut()
    }
}

unsafe fn timespec_sub(a: timespec, b: timespec) -> i64 {
    let mut ret: i64 = NSEC_PER_SEC * b.tv_sec as i64 + b.tv_nsec as i64;

    ret -= NSEC_PER_SEC * a.tv_sec as i64 + a.tv_nsec as i64;
    ret
}

static mut final_ret: c_int = 0;

extern "C" fn sigalarm(_signo: c_int) {
    unsafe {
        let mut delta_ns: i64;
        let mut ts: timespec = core::mem::zeroed();

        libc::clock_gettime(alarm_clock_id, &mut ts);
        alarmcount += 1;

        delta_ns = timespec_sub(start_time, ts);
        delta_ns -= NSEC_PER_SEC * SUSPEND_SECS * alarmcount as i64;

        libc::printf(
            b"ALARM(%i): %ld:%ld latency: %lld ns \0".as_ptr() as *const c_char,
            alarmcount,
            ts.tv_sec,
            ts.tv_nsec,
            delta_ns,
        );

        if delta_ns > UNREASONABLE_LAT {
            libc::printf(b"[FAIL]\n\0".as_ptr() as *const c_char);
            final_ret = -1;
        } else {
            libc::printf(b"[OK]\n\0".as_ptr() as *const c_char);
        }
    }
}

fn main() {
    unsafe {
        let mut tm1: timer_t = core::mem::zeroed();
        let mut its1: itimerspec = core::mem::zeroed();
        let mut its2: itimerspec = core::mem::zeroed();
        let mut se: sigevent = core::mem::zeroed();
        let mut act: libc::sigaction = core::mem::zeroed();
        let signum: c_int = libc::SIGRTMAX();

        /* Set up signal handler: */
        libc::sigfillset(&mut act.sa_mask as *mut sigset_t);
        act.sa_flags = 0;
        act.sa_sigaction = sigalarm as usize;
        libc::sigaction(signum, &act, core::ptr::null_mut());

        /* Set up timer: */
        libc::memset(
            &mut se as *mut sigevent as *mut c_void,
            0,
            core::mem::size_of::<sigevent>(),
        );
        se.sigev_notify = SIGEV_SIGNAL;
        se.sigev_signo = signum;
        se.sigev_value.sival_int = 0;

        alarm_clock_id = CLOCK_REALTIME_ALARM;
        while alarm_clock_id <= CLOCK_BOOTTIME_ALARM {
            alarmcount = 0;
            if libc::timer_create(alarm_clock_id, &mut se, &mut tm1) == -1 {
                libc::printf(
                    b"timer_create failed, %s unsupported?: %s\n\0".as_ptr() as *const c_char,
                    clock_name(alarm_clock_id),
                    libc::strerror(*errno_location()),
                );
                break;
            }

            libc::clock_gettime(alarm_clock_id, &mut start_time);
            libc::printf(
                b"Start time (%s): %ld:%ld\n\0".as_ptr() as *const c_char,
                clock_name(alarm_clock_id),
                start_time.tv_sec,
                start_time.tv_nsec,
            );
            libc::printf(
                b"Setting alarm for every %i seconds\n\0".as_ptr() as *const c_char,
                SUSPEND_SECS as c_int,
            );
            its1.it_value = start_time;
            its1.it_value.tv_sec += SUSPEND_SECS as time_t;
            its1.it_interval.tv_sec = SUSPEND_SECS as time_t;
            its1.it_interval.tv_nsec = 0;

            libc::timer_settime(tm1, TIMER_ABSTIME, &its1, &mut its2);

            while alarmcount < 5 {
                libc::sleep(1); /* First 5 alarms, do nothing */
            }

            libc::printf(b"Starting suspend loops\n\0".as_ptr() as *const c_char);
            while alarmcount < 10 {
                let ret: c_int;

                libc::sleep(3);
                ret = libc::system(b"echo mem > /sys/power/state\0".as_ptr() as *const c_char);
                if ret != 0 {
                    break;
                }
            }
            libc::timer_delete(tm1);

            alarm_clock_id += 1;
        }
        if final_ret != 0 {
            ksft_exit_fail();
        }
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
