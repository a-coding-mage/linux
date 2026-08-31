/* Demo leapsecond deadlock
 *              by: John Stultz (john.stultz@linaro.org)
 *              (C) Copyright IBM 2012
 *              (C) Copyright 2013, 2015 Linaro Limited
 *              Licensed under the GPL
 *
 * This test demonstrates leapsecond deadlock that is possible
 * on kernels from 2.6.26 to 3.3.
 *
 * WARNING: THIS WILL LIKELY HARD HANG SYSTEMS AND MAY LOSE DATA
 * RUN AT YOUR OWN RISK!
 *  To build:
 *	$ gcc leapcrash.c -o leapcrash -lrt
 */

/* Dependencies from C source:
 * stdio.h, stdlib.h, time.h, sys/time.h, sys/timex.h, string.h, signal.h,
 * and "kselftest.h".
 */

use core::ffi::{c_int, c_void};

extern "C" {
    static mut stdout: *mut c_void;

    fn setbuf(stream: *mut c_void, buf: *mut i8);
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn printf(format: *const i8, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn settimeofday(tv: *const timeval, tz: *const c_void) -> c_int;
    fn adjtimex(buf: *mut timex) -> c_int;

    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

type time_t = i64;
type suseconds_t = i64;

const SIGINT: c_int = 2;
const SIGKILL: c_int = 9;
const CLOCK_REALTIME: c_int = 0;

const ADJ_STATUS: u32 = 0x0010;
const STA_PLL: i32 = 0x0001;
const STA_INS: i32 = 0x0010;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timex {
    modes: u32,
    offset: i64,
    freq: i64,
    maxerror: i64,
    esterror: i64,
    status: c_int,
    constant: i64,
    precision: i64,
    tolerance: i64,
    time: timeval,
    tick: i64,
    ppsfreq: i64,
    jitter: i64,
    shift: c_int,
    stabil: i64,
    jitcnt: i64,
    calcnt: i64,
    errcnt: i64,
    stbcnt: i64,
    tai: c_int,
    __padding: [c_int; 11],
}

/* clear NTP time_status & time_state */
fn clear_time_state() -> c_int {
    let mut tx: timex = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    /*
     * We have to call adjtime twice here, as kernels
     * prior to 6b1859dba01c7 (included in 3.5 and
     * -stable), had an issue with the state machine
     * and wouldn't clear the STA_INS/DEL flag directly.
     */
    tx.modes = ADJ_STATUS;
    tx.status = STA_PLL;
    ret = unsafe { adjtimex(&mut tx) };

    tx.modes = ADJ_STATUS;
    tx.status = 0;
    ret = unsafe { adjtimex(&mut tx) };

    ret
}

/* Make sure we cleanup on ctrl-c */
extern "C" fn handler(_unused: c_int) {
    clear_time_state();
    unsafe {
        exit(0);
    }
}

fn main() {
    let mut tx: timex = unsafe { core::mem::zeroed() };
    let mut ts: timespec = unsafe { core::mem::zeroed() };
    let mut next_leap: time_t;
    let mut count: c_int = 0;

    unsafe {
        setbuf(stdout, core::ptr::null_mut());

        signal(SIGINT, handler);
        signal(SIGKILL, handler);
        printf(c"This runs for a few minutes. Press ctrl-c to stop\n".as_ptr());
    }

    clear_time_state();

    /* Get the current time */
    unsafe {
        clock_gettime(CLOCK_REALTIME, &mut ts);
    }

    /* Calculate the next possible leap second 23:59:60 GMT */
    next_leap = ts.tv_sec;
    next_leap += 86400 - (next_leap % 86400);

    while count < 20 {
        let mut tv: timeval = unsafe { core::mem::zeroed() };

        /* set the time to 2 seconds before the leap */
        tv.tv_sec = next_leap - 2;
        tv.tv_usec = 0;
        if unsafe { settimeofday(&tv, core::ptr::null()) } != 0 {
            unsafe {
                printf(c"Error: You're likely not running with proper (ie: root) permissions\n".as_ptr());
                ksft_exit_fail();
            }
        }
        tx.modes = 0;
        unsafe {
            adjtimex(&mut tx);
        }

        /* hammer on adjtime w/ STA_INS */
        while tx.time.tv_sec < next_leap + 1 {
            /* Set the leap second insert flag */
            tx.modes = ADJ_STATUS;
            tx.status = STA_INS;
            unsafe {
                adjtimex(&mut tx);
            }
        }
        clear_time_state();
        unsafe {
            printf(c".".as_ptr());
            fflush(stdout);
        }

        count += 1;
    }
    unsafe {
        printf(c"[OK]\n".as_ptr());
        ksft_exit_pass();
    }
}
