// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fast user context implementation of clock_gettime, gettimeofday, and time.
 *
 * Copyright 2006 Andi Kleen, SUSE Labs.
 * Copyright 2019 ARM Limited
 *
 * 32 Bit compat layer by Stefani Seibold <stefani@seibold.net>
 *  sponsored by Rohde & Schwarz GmbH & Co. KG Munich/Germany
 */

// Dependencies supplied by the Linux kernel and vDSO support headers.
// The included implementation is provided by lib/vdso/gettimeofday.c.

#[cfg(any(target_arch = "x86_64", feature = "config_compat_32bit_time"))]
pub unsafe fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> ::core::ffi::c_int {
    __cvdso_gettimeofday(tv, tz)
}

#[cfg(any(target_arch = "x86_64", feature = "config_compat_32bit_time"))]
#[no_mangle]
pub unsafe extern "C" fn gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> ::core::ffi::c_int {
    __vdso_gettimeofday(tv, tz)
}

#[cfg(any(target_arch = "x86_64", feature = "config_compat_32bit_time"))]
pub unsafe fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    __cvdso_time(t)
}

#[cfg(any(target_arch = "x86_64", feature = "config_compat_32bit_time"))]
#[no_mangle]
pub unsafe extern "C" fn time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    __vdso_time(t)
}

#[cfg(all(target_arch = "x86_64", not(feature = "build_vdso32_64")))]
// Both 64-bit and x32 use these.
pub unsafe fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __cvdso_clock_gettime(clock, ts)
}

#[cfg(all(target_arch = "x86_64", not(feature = "build_vdso32_64")))]
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __vdso_clock_gettime(clock, ts)
}

#[cfg(all(target_arch = "x86_64", not(feature = "build_vdso32_64")))]
pub unsafe fn __vdso_clock_getres(
    clock: clockid_t,
    res: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __cvdso_clock_getres(clock, res)
}

#[cfg(all(target_arch = "x86_64", not(feature = "build_vdso32_64")))]
#[no_mangle]
pub unsafe extern "C" fn clock_getres(
    clock: clockid_t,
    res: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __vdso_clock_getres(clock, res)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
// i386 only.
#[cfg(feature = "config_compat_32bit_time")]
pub unsafe fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> ::core::ffi::c_int {
    __cvdso_clock_gettime32(clock, ts)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
#[cfg(feature = "config_compat_32bit_time")]
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> ::core::ffi::c_int {
    __vdso_clock_gettime(clock, ts)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
#[cfg(feature = "config_compat_32bit_time")]
pub unsafe fn __vdso_clock_getres(
    clock: clockid_t,
    res: *mut old_timespec32,
) -> ::core::ffi::c_int {
    __cvdso_clock_getres_time32(clock, res)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
#[cfg(feature = "config_compat_32bit_time")]
#[no_mangle]
pub unsafe extern "C" fn clock_getres(
    clock: clockid_t,
    res: *mut old_timespec32,
) -> ::core::ffi::c_int {
    __vdso_clock_getres(clock, res)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
pub unsafe fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __cvdso_clock_gettime(clock, ts)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
#[no_mangle]
pub unsafe extern "C" fn clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __vdso_clock_gettime64(clock, ts)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
pub unsafe fn __vdso_clock_getres_time64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __cvdso_clock_getres(clock, ts)
}

#[cfg(not(all(target_arch = "x86_64", not(feature = "build_vdso32_64"))))]
#[no_mangle]
pub unsafe extern "C" fn clock_getres_time64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> ::core::ffi::c_int {
    __vdso_clock_getres_time64(clock, ts)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
