// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

// Dependencies supplied by the surrounding UML implementation and libc.
use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

extern "C" {
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
    fn __errno_location() -> *mut i32;
    fn timerfd_create(clockid: i32, flags: i32) -> i32;
    fn timerfd_settime(
        fd: i32,
        flags: i32,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> i32;
    fn close(fd: i32) -> i32;

    fn os_pipe(fds: *mut i32, close_on_exec: i32, nonblock: i32) -> i32;
    fn sigio_broken();
    fn add_sigio_fd(fd: i32) -> i32;
    fn ignore_sigio_fd(fd: i32);
    fn os_close_file(fd: i32);
    fn uml_rtc_stop(timetravel: bool);
}

const CLOCK_REALTIME: i32 = 0;
const TFD_CLOEXEC: i32 = 0o2000000;

static mut uml_rtc_irq_fds: [i32; 2] = [0; 2];

pub unsafe fn uml_rtc_send_timetravel_alarm() {
    let c: u64 = 1;

    // CATCH_EINTR(write(...)) is supplied by the surrounding implementation.
    write(
        uml_rtc_irq_fds[1],
        (&c as *const u64).cast::<c_void>(),
        core::mem::size_of::<u64>(),
    );
}

pub unsafe fn uml_rtc_start(timetravel: bool) -> i32 {
    let mut err: i32;

    if timetravel {
        err = os_pipe(uml_rtc_irq_fds.as_mut_ptr(), 1, 1);
        if err != 0 {
            uml_rtc_stop(timetravel);
            return err;
        }
    } else {
        uml_rtc_irq_fds[0] = timerfd_create(CLOCK_REALTIME, TFD_CLOEXEC);
        if uml_rtc_irq_fds[0] < 0 {
            err = -(*__errno_location());
            uml_rtc_stop(timetravel);
            return err;
        }

        /* apparently timerfd won't send SIGIO, use workaround */
        sigio_broken();
        err = add_sigio_fd(uml_rtc_irq_fds[0]);
        if err < 0 {
            close(uml_rtc_irq_fds[0]);
            uml_rtc_stop(timetravel);
            return err;
        }
    }

    uml_rtc_irq_fds[0]
}

pub unsafe fn uml_rtc_enable_alarm(delta_seconds: u64) -> i32 {
    let it = itimerspec {
        it_interval: timespec { tv_sec: 0, tv_nsec: 0 },
        it_value: timespec {
            tv_sec: delta_seconds as i64,
            tv_nsec: 0,
        },
    };

    if timerfd_settime(uml_rtc_irq_fds[0], 0, &it, core::ptr::null_mut()) != 0 {
        return -(*__errno_location());
    }
    0
}

pub unsafe fn uml_rtc_disable_alarm() {
    uml_rtc_enable_alarm(0);
}

pub unsafe fn uml_rtc_stop(timetravel: bool) {
    if timetravel {
        os_close_file(uml_rtc_irq_fds[1]);
    } else {
        ignore_sigio_fd(uml_rtc_irq_fds[0]);
    }
    os_close_file(uml_rtc_irq_fds[0]);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
