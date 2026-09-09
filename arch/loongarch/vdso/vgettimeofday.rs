// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LoongArch userspace implementations of gettimeofday() and similar.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Types and functions supplied by the corresponding Linux VDSO dependencies
// are referenced here but intentionally not implemented in this translation.

unsafe extern "C" {
    fn __cvdso_clock_gettime(
        clock: crate::clockid_t,
        ts: *mut crate::__kernel_timespec,
    ) -> core::ffi::c_int;

    fn __cvdso_gettimeofday(
        tv: *mut crate::__kernel_old_timeval,
        tz: *mut crate::timezone,
    ) -> core::ffi::c_int;

    fn __cvdso_clock_getres(
        clock_id: crate::clockid_t,
        res: *mut crate::__kernel_timespec,
    ) -> core::ffi::c_int;
}

pub unsafe fn __vdso_clock_gettime(
    clock: crate::clockid_t,
    ts: *mut crate::__kernel_timespec,
) -> core::ffi::c_int {
    unsafe { __cvdso_clock_gettime(clock, ts) }
}

pub unsafe fn __vdso_gettimeofday(
    tv: *mut crate::__kernel_old_timeval,
    tz: *mut crate::timezone,
) -> core::ffi::c_int {
    unsafe { __cvdso_gettimeofday(tv, tz) }
}

pub unsafe fn __vdso_clock_getres(
    clock_id: crate::clockid_t,
    res: *mut crate::__kernel_timespec,
) -> core::ffi::c_int {
    unsafe { __cvdso_clock_getres(clock_id, res) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
