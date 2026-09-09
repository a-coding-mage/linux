// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MIPS64 and compat userspace implementations of gettimeofday()
 * and similar.
 *
 * Copyright (C) 2015 Imagination Technologies
 * Copyright (C) 2018 ARM Limited
 *
 */

// Dependency declarations supplied by the surrounding kernel/VDSO sources.
// The C condition `_MIPS_SIM != _MIPS_SIM_ABI64` is represented by the
// `mips_abi64` configuration feature.

unsafe extern "C" {
    #[cfg(all(not(feature = "mips_abi64"), feature = "compat_32bit_time"))]
    fn __cvdso_clock_gettime32(
        clock: clockid_t,
        ts: *mut old_timespec32,
    ) -> core::ffi::c_int;

    #[cfg(any(not(feature = "mips_abi64"), feature = "mips_abi64"))]
    fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> core::ffi::c_int;

    #[cfg(all(not(feature = "mips_abi64"), feature = "compat_32bit_time"))]
    fn __cvdso_clock_getres_time32(
        clock_id: clockid_t,
        res: *mut old_timespec32,
    ) -> core::ffi::c_int;

    #[cfg(not(feature = "mips_abi64"))]
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> core::ffi::c_int;

    #[cfg(feature = "mips_abi64")]
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> core::ffi::c_int;

    #[cfg(not(feature = "mips_abi64"))]
    fn __cvdso_clock_getres(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> core::ffi::c_int;

    #[cfg(feature = "mips_abi64")]
    fn __cvdso_clock_getres(
        clock: clockid_t,
        res: *mut __kernel_timespec,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "mips_abi64"))]
#[cfg(feature = "compat_32bit_time")]
pub unsafe fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> core::ffi::c_int {
    __cvdso_clock_gettime32(clock, ts)
}

#[cfg(not(feature = "mips_abi64"))]
#[cfg(feature = "compat_32bit_time")]
pub unsafe fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> core::ffi::c_int {
    __cvdso_gettimeofday(tv, tz)
}

#[cfg(not(feature = "mips_abi64"))]
#[cfg(feature = "compat_32bit_time")]
pub unsafe fn __vdso_clock_getres(
    clock_id: clockid_t,
    res: *mut old_timespec32,
) -> core::ffi::c_int {
    __cvdso_clock_getres_time32(clock_id, res)
}

#[cfg(not(feature = "mips_abi64"))]
pub unsafe fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_gettime(clock, ts)
}

#[cfg(not(feature = "mips_abi64"))]
pub unsafe fn __vdso_clock_getres_time64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_getres(clock, ts)
}

#[cfg(feature = "mips_abi64")]
pub unsafe fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_gettime(clock, ts)
}

#[cfg(feature = "mips_abi64")]
pub unsafe fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> core::ffi::c_int {
    __cvdso_gettimeofday(tv, tz)
}

#[cfg(feature = "mips_abi64")]
pub unsafe fn __vdso_clock_getres(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
) -> core::ffi::c_int {
    __cvdso_clock_getres(clock_id, res)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
