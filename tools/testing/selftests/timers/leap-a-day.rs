/* Leap second stress test
 *              by: John Stultz (john.stultz@linaro.org)
 *              (C) Copyright IBM 2012
 *              (C) Copyright 2013, 2015 Linaro Limited
 *              Licensed under the GPLv2
 *
 *  This test signals the kernel to insert a leap second
 *  every day at midnight GMT. This allows for stressing the
 *  kernel's leap-second behavior, as well as how well applications
 *  handle the leap-second discontinuity.
 *
 *  Usage: leap-a-day [-w] [-i <num>] [-t]
 *
 *  Options:
 *	-w:	Only set the leap-second flag and wait for the leap second
 *		each iteration, instead of advancing the time. By default the
 *		date is set to 10 seconds before midnight GMT, which speeds up
 *		the number of leapsecond transitions tested, but because it
 *		calls settimeofday frequently, advancing the time by 24 hours
 *		every ~16 seconds, it may cause application disruption.
 *
 *	-i:	Number of iterations to run (-1 = infinite, default: 10)
 *
 *	-t:	Print TAI time.
 *
 *  Other notes: Disabling NTP prior to running this is advised, as the two
 *		 may conflict in their commands to the kernel.
 *
 *  To build:
 *	$ gcc leap-a-day.c -o leap-a-day -lrt
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

use libc::{
    c_char, c_int, c_long, c_void, clock_gettime, clock_nanosleep, ctime, ctime_r, exit, getopt,
    memset, printf, sigaction, sigfillset, signal, size_t, strlen, time_t, timer_create,
    timer_settime, timer_t, timespec, timeval, ADJ_MAXERROR, ADJ_STATUS, CLOCK_MONOTONIC,
    CLOCK_REALTIME, SIGEV_SIGNAL, SIGINT, SIGKILL, TIMER_ABSTIME,
};

/* From clock-helpers.h. */
const NSEC_PER_SEC: c_long = 1_000_000_000;

const CLOCK_TAI: c_int = 11;

static mut NEXT_LEAP: time_t = 0;
static mut ERROR_FOUND: c_int = 0;

extern "C" {
    static mut optarg: *mut c_char;

    fn adjtimex(buf: *mut libc::timex) -> c_int;
    fn settimeofday(tv: *const timeval, tz: *const c_void) -> c_int;

    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

/* returns 1 if a <= b, 0 otherwise */
#[inline]
unsafe fn in_order(a: timespec, b: timespec) -> c_int {
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

unsafe fn timespec_add(mut ts: timespec, ns: u64) -> timespec {
    ts.tv_nsec += ns as c_long;
    while ts.tv_nsec >= NSEC_PER_SEC {
        ts.tv_nsec -= NSEC_PER_SEC;
        ts.tv_sec += 1;
    }
    ts
}

unsafe fn time_state_str(state: c_int) -> *const c_char {
    match state {
        libc::TIME_OK => b"TIME_OK\0".as_ptr() as *const c_char,
        libc::TIME_INS => b"TIME_INS\0".as_ptr() as *const c_char,
        libc::TIME_DEL => b"TIME_DEL\0".as_ptr() as *const c_char,
        libc::TIME_OOP => b"TIME_OOP\0".as_ptr() as *const c_char,
        libc::TIME_WAIT => b"TIME_WAIT\0".as_ptr() as *const c_char,
        libc::TIME_BAD => b"TIME_BAD\0".as_ptr() as *const c_char,
        _ => b"ERROR\0".as_ptr() as *const c_char,
    }
}

/* clear NTP time_status & time_state */
unsafe fn clear_time_state() -> c_int {
    let mut tx: libc::timex = std::mem::zeroed();
    let mut ret: c_int;

    /*
     * We have to call adjtime twice here, as kernels
     * prior to 6b1859dba01c7 (included in 3.5 and
     * -stable), had an issue with the state machine
     * and wouldn't clear the STA_INS/DEL flag directly.
     */
    tx.modes = ADJ_STATUS;
    tx.status = libc::STA_PLL;
    ret = adjtimex(&mut tx);

    /* Clear maxerror, as it can cause UNSYNC to be set */
    tx.modes = ADJ_MAXERROR;
    tx.maxerror = 0;
    ret = adjtimex(&mut tx);

    /* Clear the status */
    tx.modes = ADJ_STATUS;
    tx.status = 0;
    ret = adjtimex(&mut tx);

    ret
}

/* Make sure we cleanup on ctrl-c */
unsafe extern "C" fn handler(_unused: c_int) {
    clear_time_state();
    exit(0);
}

unsafe extern "C" fn sigalarm(_signo: c_int) {
    let mut tx: libc::timex = std::mem::zeroed();
    let ret: c_int;

    tx.modes = 0;
    ret = adjtimex(&mut tx);

    if tx.time.tv_sec < NEXT_LEAP {
        printf(
            b"Error: Early timer expiration! (Should be %ld)\n\0".as_ptr() as *const c_char,
            NEXT_LEAP,
        );
        ERROR_FOUND = 1;
        printf(
            b"adjtimex: %10ld sec + %6ld us (%i)\t%s\n\0".as_ptr() as *const c_char,
            tx.time.tv_sec,
            tx.time.tv_usec,
            tx.tai,
            time_state_str(ret),
        );
    }
    if ret != libc::TIME_WAIT {
        printf(
            b"Error: Timer seeing incorrect NTP state? (Should be TIME_WAIT)\n\0".as_ptr()
                as *const c_char,
        );
        ERROR_FOUND = 1;
        printf(
            b"adjtimex: %10ld sec + %6ld us (%i)\t%s\n\0".as_ptr() as *const c_char,
            tx.time.tv_sec,
            tx.time.tv_usec,
            tx.tai,
            time_state_str(ret),
        );
    }
}

/* Test for known hrtimer failure */
unsafe fn test_hrtimer_failure() {
    let mut now: timespec = std::mem::zeroed();
    let mut target: timespec;

    clock_gettime(CLOCK_REALTIME, &mut now);
    target = timespec_add(now, (NSEC_PER_SEC / 2) as u64);
    clock_nanosleep(CLOCK_REALTIME, TIMER_ABSTIME, &target, std::ptr::null_mut());
    clock_gettime(CLOCK_REALTIME, &mut now);

    if in_order(target, now) == 0 {
        printf(
            b"ERROR: hrtimer early expiration failure observed.\n\0".as_ptr() as *const c_char,
        );
        ERROR_FOUND = 1;
    }
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut tm1: timer_t = std::mem::zeroed();
    let mut its1: libc::itimerspec = std::mem::zeroed();
    let mut se: libc::sigevent = std::mem::zeroed();
    let mut act: sigaction = std::mem::zeroed();
    let signum: c_int = libc::SIGRTMAX();
    let mut settime: c_int = 1;
    let mut tai_time: c_int = 0;
    let mut insert: c_int = 1;
    let mut iterations: c_int = 10;
    let mut opt: c_int;

    /* Process arguments */
    loop {
        opt = getopt(argc, argv, b"wti:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }

        match opt {
            c if c == 'w' as c_int => {
                printf(
                    b"Only setting leap-flag, not changing time. It could take up to a day for leap to trigger.\n\0"
                        .as_ptr() as *const c_char,
                );
                settime = 0;
            }
            c if c == 'i' as c_int => {
                iterations = libc::atoi(optarg);
            }
            c if c == 't' as c_int => {
                tai_time = 1;
            }
            _ => {
                printf(
                    b"Usage: %s [-w] [-i <iterations>]\n\0".as_ptr() as *const c_char,
                    *argv,
                );
                printf(
                    b"\t-w: Set flag and wait for leap second each iteration\0".as_ptr()
                        as *const c_char,
                );
                printf(
                    b"\t    (default sets time to right before leapsecond)\n\0".as_ptr()
                        as *const c_char,
                );
                printf(
                    b"\t-i: Number of iterations (-1 = infinite, default is 10)\n\0".as_ptr()
                        as *const c_char,
                );
                printf(b"\t-t: Print TAI time\n\0".as_ptr() as *const c_char);
                exit(-1);
            }
        }
    }

    /* Make sure TAI support is present if -t was used */
    if tai_time != 0 {
        let mut ts: timespec = std::mem::zeroed();

        if clock_gettime(CLOCK_TAI, &mut ts) != 0 {
            printf(b"System doesn't support CLOCK_TAI\n\0".as_ptr() as *const c_char);
            ksft_exit_fail();
        }
    }

    signal(SIGINT, handler as usize);
    signal(SIGKILL, handler as usize);

    /* Set up timer signal handler: */
    sigfillset(&mut act.sa_mask);
    act.sa_flags = 0;
    act.sa_sigaction = sigalarm as usize;
    sigaction(signum, &act, std::ptr::null_mut());

    if iterations < 0 {
        printf(b"This runs continuously. Press ctrl-c to stop\n\0".as_ptr() as *const c_char);
    } else {
        printf(
            b"Running for %i iterations. Press ctrl-c to stop\n\0".as_ptr() as *const c_char,
            iterations,
        );
    }

    printf(b"\n\0".as_ptr() as *const c_char);
    loop {
        let mut ret: c_int;
        let mut ts: timespec = std::mem::zeroed();
        let mut tx: libc::timex = std::mem::zeroed();
        let mut now: time_t;

        /* Get the current time */
        clock_gettime(CLOCK_REALTIME, &mut ts);

        /* Calculate the next possible leap second 23:59:60 GMT */
        NEXT_LEAP = ts.tv_sec;
        NEXT_LEAP += 86400 - (NEXT_LEAP % 86400);

        if settime != 0 {
            let mut tv: timeval = std::mem::zeroed();

            tv.tv_sec = NEXT_LEAP - 10;
            tv.tv_usec = 0;
            settimeofday(&tv, std::ptr::null());
            printf(
                b"Setting time to %s\0".as_ptr() as *const c_char,
                ctime(&tv.tv_sec),
            );
        }

        /* Reset NTP time state */
        clear_time_state();

        /* Set the leap second insert flag */
        tx.modes = ADJ_STATUS;
        if insert != 0 {
            tx.status = libc::STA_INS;
        } else {
            tx.status = libc::STA_DEL;
        }
        ret = adjtimex(&mut tx);
        if ret < 0 {
            printf(
                b"Error: Problem setting STA_INS/STA_DEL!: %s\n\0".as_ptr() as *const c_char,
                time_state_str(ret),
            );
            ksft_exit_fail();
        }

        /* Validate STA_INS was set */
        tx.modes = 0;
        ret = adjtimex(&mut tx);
        if tx.status != libc::STA_INS && tx.status != libc::STA_DEL {
            printf(
                b"Error: STA_INS/STA_DEL not set!: %s\n\0".as_ptr() as *const c_char,
                time_state_str(ret),
            );
            ksft_exit_fail();
        }

        if tai_time != 0 {
            printf(
                b"Using TAI time, no inconsistencies should be seen!\n\0".as_ptr()
                    as *const c_char,
            );
        }

        printf(
            b"Scheduling leap second for %s\0".as_ptr() as *const c_char,
            ctime(&NEXT_LEAP),
        );

        /* Set up timer */
        printf(
            b"Setting timer for %ld -  %s\0".as_ptr() as *const c_char,
            NEXT_LEAP,
            ctime(&NEXT_LEAP),
        );
        memset(
            &mut se as *mut libc::sigevent as *mut c_void,
            0,
            std::mem::size_of_val(&se) as size_t,
        );
        se.sigev_notify = SIGEV_SIGNAL;
        se.sigev_signo = signum;
        se.sigev_value.sival_int = 0;
        if timer_create(CLOCK_REALTIME, &mut se, &mut tm1) == -1 {
            printf(b"Error: timer_create failed\n\0".as_ptr() as *const c_char);
            ksft_exit_fail();
        }
        its1.it_value.tv_sec = NEXT_LEAP;
        its1.it_value.tv_nsec = 0;
        its1.it_interval.tv_sec = 0;
        its1.it_interval.tv_nsec = 0;
        timer_settime(tm1, TIMER_ABSTIME, &its1, std::ptr::null_mut());

        /* Wake up 3 seconds before leap */
        ts.tv_sec = NEXT_LEAP - 3;
        ts.tv_nsec = 0;

        while clock_nanosleep(CLOCK_REALTIME, TIMER_ABSTIME, &ts, std::ptr::null_mut()) != 0 {
            printf(b"Something woke us up, returning to sleep\n\0".as_ptr() as *const c_char);
        }

        /* Validate STA_INS is still set */
        tx.modes = 0;
        ret = adjtimex(&mut tx);
        if tx.status != libc::STA_INS && tx.status != libc::STA_DEL {
            printf(
                b"Something cleared STA_INS/STA_DEL, setting it again.\n\0".as_ptr()
                    as *const c_char,
            );
            tx.modes = ADJ_STATUS;
            if insert != 0 {
                tx.status = libc::STA_INS;
            } else {
                tx.status = libc::STA_DEL;
            }
            ret = adjtimex(&mut tx);
        }

        /* Check adjtimex output every half second */
        now = tx.time.tv_sec;
        while now < NEXT_LEAP + 2 {
            let mut buf: [c_char; 26] = [0; 26];
            let mut tai: timespec = std::mem::zeroed();
            let ret: c_int;

            tx.modes = 0;
            ret = adjtimex(&mut tx);

            if tai_time != 0 {
                clock_gettime(CLOCK_TAI, &mut tai);
                printf(
                    b"%ld sec, %9ld ns\t%s\n\0".as_ptr() as *const c_char,
                    tai.tv_sec,
                    tai.tv_nsec,
                    time_state_str(ret),
                );
            } else {
                ctime_r(&tx.time.tv_sec, buf.as_mut_ptr());
                buf[strlen(buf.as_ptr()) - 1] = 0; /*remove trailing\n */

                printf(
                    b"%s + %6ld us (%i)\t%s\n\0".as_ptr() as *const c_char,
                    buf.as_ptr(),
                    tx.time.tv_usec,
                    tx.tai,
                    time_state_str(ret),
                );
            }
            now = tx.time.tv_sec;
            /* Sleep for another half second */
            ts.tv_sec = 0;
            ts.tv_nsec = NSEC_PER_SEC / 2;
            clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, std::ptr::null_mut());
        }
        /* Switch to using other mode */
        insert = if insert == 0 { 1 } else { 0 };

        /* Note if kernel has known hrtimer failure */
        test_hrtimer_failure();

        printf(b"Leap complete\n\0".as_ptr() as *const c_char);
        if ERROR_FOUND != 0 {
            printf(b"Errors observed\n\0".as_ptr() as *const c_char);
            clear_time_state();
            ksft_exit_fail();
        }
        printf(b"\n\0".as_ptr() as *const c_char);
        if iterations != -1 {
            iterations -= 1;
            if iterations == 0 {
                break;
            }
        }
    }

    clear_time_state();
    ksft_exit_pass();
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(std::ptr::null_mut());
        main_impl((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
