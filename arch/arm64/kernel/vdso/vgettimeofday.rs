// SPDX-License-Identifier: GPL-2.0
/*
 * ARM64 userspace implementations of gettimeofday() and similar.
 *
 * Copyright (C) 2018 ARM Limited
 *
 */

unsafe extern "C" {
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> core::ffi::c_int;
    fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> core::ffi::c_int;
    fn __cvdso_clock_getres(
        clock_id: clockid_t,
        res: *mut __kernel_timespec,
    ) -> core::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_gettime(clock, ts)
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> core::ffi::c_int {
    __cvdso_gettimeofday(tv, tz)
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_clock_getres(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_getres(clock_id, res)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
