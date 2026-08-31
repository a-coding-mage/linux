/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * time definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependencies: ../nolibc.h, ../arch.h, ../sys.h */

unsafe extern "C" {
    fn _sys_clock_gettime(clockid: clockid_t, tp: *mut timespec) -> c_int;
}

/*
 * int gettimeofday(struct timeval *tv, struct timezone *tz);
 */

#[allow(dead_code)]
unsafe fn _sys_gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int {
    let _ = tz; /* Non-NULL tz is undefined behaviour */

    let mut tp: timespec = unsafe { core::mem::zeroed() };
    let ret: c_int;

    ret = unsafe { _sys_clock_gettime(CLOCK_REALTIME, &mut tp) };
    if ret == 0 && !tv.is_null() {
        unsafe {
            (*tv).tv_sec = tp.tv_sec;
            (*tv).tv_usec = (tp.tv_nsec as uint32_t) / 1000;
        }
    }

    ret
}

#[allow(dead_code)]
unsafe fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int {
    unsafe { __sysret(_sys_gettimeofday(tv, tz)) }
}
