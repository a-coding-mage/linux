/* SPDX-License-Identifier: LGPL-2.1+ */
/* Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org> */
/* C dependencies: <sys/sysinfo.h>, <sys/time.h> */

unsafe extern "C" {
    pub fn uptimeofday_init() -> ::std::os::raw::c_int;
    pub fn getuptimeofday_ms() -> ::std::os::raw::c_ulong;
    pub fn msec_to_timespec(msec: ::std::os::raw::c_int) -> libc::timespec;
}
