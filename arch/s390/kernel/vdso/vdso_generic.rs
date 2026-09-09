// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "../../../../lib/vdso/gettimeofday.c"
// #include "vdso.h"

use core::ffi::c_int;

// Types supplied by the included kernel headers.
#[repr(C)]
pub struct __kernel_old_timeval {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timezone {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __kernel_timespec {
    _private: [u8; 0],
}

// `clockid_t` is supplied by the included kernel headers.
pub type clockid_t = i32;

extern "C" {
    fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> c_int;
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> c_int;
    fn __cvdso_clock_getres(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> c_int;
}

pub unsafe fn __s390_vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> c_int {
    unsafe { __cvdso_gettimeofday(tv, tz) }
}

pub unsafe fn __s390_vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> c_int {
    unsafe { __cvdso_clock_gettime(clock, ts) }
}

pub unsafe fn __s390_vdso_clock_getres(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> c_int {
    unsafe { __cvdso_clock_getres(clock, ts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
