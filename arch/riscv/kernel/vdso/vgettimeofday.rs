// SPDX-License-Identifier: GPL-2.0
/*
 * Copied from arch/arm64/kernel/vdso/vgettimeofday.c
 *
 * Copyright (C) 2018 ARM Ltd.
 * Copyright (C) 2020 SiFive
 */

// Dependencies supplied by the Linux kernel and VDSO headers:
// linux/time.h, linux/types.h, and vdso/gettime.h

unsafe extern "C" {
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> ::core::ffi::c_int;

    fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> ::core::ffi::c_int;

    fn __cvdso_clock_getres(
        clock_id: clockid_t,
        res: *mut __kernel_timespec,
    ) -> ::core::ffi::c_int;
}

pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    unsafe { __cvdso_clock_gettime(clock, ts) }
}

pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> ::core::ffi::c_int {
    unsafe { __cvdso_gettimeofday(tv, tz) }
}

pub unsafe extern "C" fn __vdso_clock_getres(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    unsafe { __cvdso_clock_getres(clock_id, res) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
