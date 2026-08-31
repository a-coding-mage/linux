/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * time function definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency intent: #include "nolibc.h" */

/* C header guard omitted: _NOLIBC_TIME_H */

/*
 * C dependency intent:
 * #include "std.h"
 * #include "arch.h"
 * #include "types.h"
 * #include "sys.h"
 * #include <linux/signal.h>
 * #include <linux/time.h>
 */

macro_rules! __nolibc_assert_time64_type {
    ($t:expr) => {{
        let _ = ::core::mem::transmute::<[u8; ::core::mem::size_of_val(&$t)], [u8; 8]>;
    }};
}

macro_rules! __nolibc_assert_native_time64 {
    () => {{
        let _ = ::core::mem::transmute::<[u8; ::core::mem::size_of::<__kernel_old_time_t>()], [u8; 8]>;
    }};
}

/*
 * int clock_getres(clockid_t clockid, struct timespec *res);
 * int clock_gettime(clockid_t clockid, struct timespec *tp);
 * int clock_settime(clockid_t clockid, const struct timespec *tp);
 * int clock_nanosleep(clockid_t clockid, int flags, const struct timespec *rqtp,
 *                     struct timespec *rmtp)
 */

pub unsafe fn _sys_clock_getres(clockid: clockid_t, res: *mut timespec) -> i32 {
    #[cfg(__NR_clock_getres_time64)]
    {
        __nolibc_assert_time64_type!((*res).tv_sec);
        return __nolibc_syscall2(__NR_clock_getres_time64, clockid, res) as i32;
    }
    #[cfg(not(__NR_clock_getres_time64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall2(__NR_clock_getres, clockid, res) as i32;
    }
}

pub unsafe fn clock_getres(clockid: clockid_t, res: *mut timespec) -> i32 {
    return __sysret(_sys_clock_getres(clockid, res));
}

pub unsafe fn _sys_clock_gettime(clockid: clockid_t, tp: *mut timespec) -> i32 {
    #[cfg(__NR_clock_gettime64)]
    {
        __nolibc_assert_time64_type!((*tp).tv_sec);
        return __nolibc_syscall2(__NR_clock_gettime64, clockid, tp) as i32;
    }
    #[cfg(not(__NR_clock_gettime64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall2(__NR_clock_gettime, clockid, tp) as i32;
    }
}

pub unsafe fn clock_gettime(clockid: clockid_t, tp: *mut timespec) -> i32 {
    return __sysret(_sys_clock_gettime(clockid, tp));
}

pub unsafe fn _sys_clock_settime(clockid: clockid_t, tp: *mut timespec) -> i32 {
    #[cfg(__NR_clock_settime64)]
    {
        __nolibc_assert_time64_type!((*tp).tv_sec);
        return __nolibc_syscall2(__NR_clock_settime64, clockid, tp) as i32;
    }
    #[cfg(not(__NR_clock_settime64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall2(__NR_clock_settime, clockid, tp) as i32;
    }
}

pub unsafe fn clock_settime(clockid: clockid_t, tp: *mut timespec) -> i32 {
    return __sysret(_sys_clock_settime(clockid, tp));
}

pub unsafe fn _sys_clock_nanosleep(
    clockid: clockid_t,
    flags: i32,
    rqtp: *const timespec,
    rmtp: *mut timespec,
) -> i32 {
    #[cfg(__NR_clock_nanosleep_time64)]
    {
        __nolibc_assert_time64_type!((*rqtp).tv_sec);
        return __nolibc_syscall4(__NR_clock_nanosleep_time64, clockid, flags, rqtp, rmtp) as i32;
    }
    #[cfg(not(__NR_clock_nanosleep_time64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall4(__NR_clock_nanosleep, clockid, flags, rqtp, rmtp) as i32;
    }
}

pub unsafe fn clock_nanosleep(
    clockid: clockid_t,
    flags: i32,
    rqtp: *const timespec,
    rmtp: *mut timespec,
) -> i32 {
    /* Directly return a positive error number */
    return -_sys_clock_nanosleep(clockid, flags, rqtp, rmtp);
}

#[inline]
pub unsafe fn difftime(time1: time_t, time2: time_t) -> f64 {
    return (time1 - time2) as f64;
}

#[inline]
pub unsafe fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> i32 {
    return __sysret(_sys_clock_nanosleep(CLOCK_REALTIME, 0, rqtp, rmtp));
}

pub unsafe fn time(tptr: *mut time_t) -> time_t {
    let mut tv: timeval = ::core::mem::zeroed();

    /* note, cannot fail here */
    _sys_gettimeofday(&mut tv, ::core::ptr::null_mut());

    if !tptr.is_null() {
        *tptr = tv.tv_sec;
    }
    return tv.tv_sec;
}

/*
 * int timer_create(clockid_t clockid, struct sigevent *evp, timer_t *timerid);
 * int timer_gettime(timer_t timerid, struct itimerspec *curr_value);
 * int timer_settime(timer_t timerid, int flags, const struct itimerspec *new_value, struct itimerspec *old_value);
 */

pub unsafe fn _sys_timer_create(clockid: clockid_t, evp: *mut sigevent, timerid: *mut timer_t) -> i32 {
    return __nolibc_syscall3(__NR_timer_create, clockid, evp, timerid) as i32;
}

pub unsafe fn timer_create(clockid: clockid_t, evp: *mut sigevent, timerid: *mut timer_t) -> i32 {
    return __sysret(_sys_timer_create(clockid, evp, timerid));
}

pub unsafe fn _sys_timer_delete(timerid: timer_t) -> i32 {
    return __nolibc_syscall1(__NR_timer_delete, timerid) as i32;
}

pub unsafe fn timer_delete(timerid: timer_t) -> i32 {
    return __sysret(_sys_timer_delete(timerid));
}

pub unsafe fn _sys_timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> i32 {
    #[cfg(__NR_timer_gettime64)]
    {
        __nolibc_assert_time64_type!((*curr_value).it_value.tv_sec);
        return __nolibc_syscall2(__NR_timer_gettime64, timerid, curr_value) as i32;
    }
    #[cfg(not(__NR_timer_gettime64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall2(__NR_timer_gettime, timerid, curr_value) as i32;
    }
}

pub unsafe fn timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> i32 {
    return __sysret(_sys_timer_gettime(timerid, curr_value));
}

pub unsafe fn _sys_timer_settime(
    timerid: timer_t,
    flags: i32,
    new_value: *const itimerspec,
    old_value: *mut itimerspec,
) -> i32 {
    #[cfg(__NR_timer_settime64)]
    {
        __nolibc_assert_time64_type!((*new_value).it_value.tv_sec);
        return __nolibc_syscall4(__NR_timer_settime64, timerid, flags, new_value, old_value) as i32;
    }
    #[cfg(not(__NR_timer_settime64))]
    {
        __nolibc_assert_native_time64!();
        return __nolibc_syscall4(__NR_timer_settime, timerid, flags, new_value, old_value) as i32;
    }
}

pub unsafe fn timer_settime(
    timerid: timer_t,
    flags: i32,
    new_value: *const itimerspec,
    old_value: *mut itimerspec,
) -> i32 {
    return __sysret(_sys_timer_settime(timerid, flags, new_value, old_value));
}
