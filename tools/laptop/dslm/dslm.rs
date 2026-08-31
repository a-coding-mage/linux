/*
 * dslm.c
 * Simple Disk Sleep Monitor
 *  by Bartek Kania
 * Licensed under the GPL
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_ulong};

type time_t = i64;

const WIN_CHECKPOWERMODE1: u8 = 0xe5;
const WIN_CHECKPOWERMODE2: u8 = 0x98;
const HDIO_DRIVE_CMD: c_ulong = 0x031f;
const O_RDONLY: c_int = 0;
const O_NONBLOCK: c_int = 0o4000;
const EIO: c_int = 5;
const SIGINT: c_int = 2;

unsafe extern "C" {
    fn sleep(seconds: c_uint) -> c_uint;
    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn ctime(timep: *const time_t) -> *mut c_char;
    fn time(tloc: *mut time_t) -> time_t;
    fn strlen(s: *const c_char) -> usize;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn ioctl(fd: c_int, request: c_ulong, argp: *mut c_void) -> c_int;
    fn __errno_location() -> *mut c_int;
}

static mut endit: c_int = 0;

/* Check if the disk is in powersave-mode
 * Most of the code is stolen from hdparm.
 * 1 = active, 0 = standby/sleep, -1 = unknown */
unsafe fn check_powermode(fd: c_int) -> c_int {
    let mut args: [u8; 4] = [WIN_CHECKPOWERMODE1, 0, 0, 0];
    let state: c_int;

    if ioctl(fd, HDIO_DRIVE_CMD, args.as_mut_ptr() as *mut c_void) != 0
        && {
            args[0] = WIN_CHECKPOWERMODE2;
            args[0] != 0
        } /* try again with 0x98 */
        && ioctl(fd, HDIO_DRIVE_CMD, args.as_mut_ptr() as *mut c_void) != 0
    {
        if *__errno_location() != EIO || args[0] != 0 || args[1] != 0 {
            state = -1; /* "unknown"; */
        } else {
            state = 0; /* "sleeping"; */
        }
    } else {
        state = if args[2] == 255 { 1 } else { 0 };
    }
    /* DEBUG macro D(printf(" drive state is:  %d\n", state)); */

    state
}

unsafe fn state_name(i: c_int) -> *const c_char {
    if i == -1 {
        return c"unknown".as_ptr();
    }
    if i == 0 {
        return c"sleeping".as_ptr();
    }
    if i == 1 {
        return c"active".as_ptr();
    }

    c"internal error".as_ptr()
}

unsafe fn myctime(time_arg: time_t) -> *mut c_char {
    let ts = ctime(&time_arg as *const time_t);
    *ts.add(strlen(ts as *const c_char) - 1) = 0;

    ts
}

unsafe fn measure(fd: c_int) {
    let start_time: time_t;
    let mut last_state: c_int;
    let mut last_time: time_t;
    let mut curr_state: c_int;
    let mut curr_time: time_t = 0;
    let mut time_diff: time_t;
    let mut active_time: time_t = 0;
    let mut sleep_time: time_t = 0;
    let mut unknown_time: time_t = 0;
    let mut total_time: time_t;
    let mut changes: c_int = 0;
    let mut tmp: c_float;

    printf(c"Starting measurements\n".as_ptr());

    last_state = check_powermode(fd);
    last_time = time(std::ptr::null_mut());
    start_time = last_time;
    printf(
        c"  System is in state %s\n\n".as_ptr(),
        state_name(last_state),
    );

    while endit == 0 {
        sleep(1);
        curr_state = check_powermode(fd);

        if curr_state != last_state || endit != 0 {
            changes += 1;
            curr_time = time(std::ptr::null_mut());
            time_diff = curr_time - last_time;

            if last_state == 1 {
                active_time += time_diff;
            } else if last_state == 0 {
                sleep_time += time_diff;
            } else {
                unknown_time += time_diff;
            }

            last_state = curr_state;
            last_time = curr_time;

            printf(
                c"%s: State-change to %s\n".as_ptr(),
                myctime(curr_time),
                state_name(curr_state),
            );
        }
    }
    changes -= 1; /* Compensate for SIGINT */

    total_time = time(std::ptr::null_mut()) - start_time;
    printf(
        c"\nTotal running time:  %lus\n".as_ptr(),
        (curr_time - start_time) as c_ulong,
    );
    printf(c" State changed %d times\n".as_ptr(), changes);

    tmp = sleep_time as c_float / total_time as c_float * 100.0;
    printf(
        c" Time in sleep state:   %lus (%.2f%%)\n".as_ptr(),
        sleep_time as c_ulong,
        tmp as f64,
    );
    tmp = active_time as c_float / total_time as c_float * 100.0;
    printf(
        c" Time in active state:  %lus (%.2f%%)\n".as_ptr(),
        active_time as c_ulong,
        tmp as f64,
    );
    tmp = unknown_time as c_float / total_time as c_float * 100.0;
    printf(
        c" Time in unknown state: %lus (%.2f%%)\n".as_ptr(),
        unknown_time as c_ulong,
        tmp as f64,
    );
}

unsafe extern "C" fn ender(_s: c_int) {
    endit = 1;
}

unsafe fn usage() -> ! {
    puts(c"usage: dslm [-w <time>] <disk>".as_ptr());
    exit(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd: c_int;
    let mut disk: *mut c_char = std::ptr::null_mut();
    let mut settle_time: c_int = 60;

    /* Parse the simple command-line */
    if argc == 2 {
        disk = *argv.add(1);
    } else if argc == 4 {
        settle_time = atoi(*argv.add(2));
        disk = *argv.add(3);
    } else {
        usage();
    }

    fd = open(disk, O_RDONLY | O_NONBLOCK);
    if fd == 0 {
        printf(
            c"Can't open %s, because: %s\n".as_ptr(),
            disk,
            strerror(*__errno_location()),
        );
        exit(-1);
    }

    if settle_time != 0 {
        printf(
            c"Waiting %d seconds for the system to settle down to 'normal'\n".as_ptr(),
            settle_time,
        );
        sleep(settle_time as c_uint);
    } else {
        puts(c"Not waiting for system to settle down".as_ptr());
    }

    signal(SIGINT, ender);

    measure(fd);

    close(fd);

    0
}
