// SPDX-License-Identifier: GPL-2.0
/*
 * test for timerfd functions used by perf-kvm-stat-live
 */

use core::ffi::c_int;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

const CLOCK_MONOTONIC: c_int = 1;
const TFD_NONBLOCK: c_int = 0o0004000;

unsafe extern "C" {
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(
        fd: c_int,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
}

fn main() -> c_int {
    let new_value = MaybeUninit::<itimerspec>::uninit();

    let fd = unsafe { timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK) };
    if fd < 0 {
        return 1;
    }

    if unsafe { timerfd_settime(fd, 0, new_value.as_ptr(), core::ptr::null_mut()) } != 0 {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
