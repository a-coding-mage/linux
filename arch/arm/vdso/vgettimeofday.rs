// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM userspace implementations of gettimeofday() and similar.
 *
 * Copyright 2015 Mentor Graphics Corporation.
 */

// Dependencies supplied by the corresponding Linux and ARM headers:
// linux/time.h, linux/types.h, asm/vdso.h, asm/unwind.h, and vdso/gettime.h.

pub type clockid_t = i32;

#[repr(C)]
pub struct old_timespec32 {
    _private: [u8; 0],
}

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

extern "C" {
    fn __cvdso_clock_gettime32(
        clock: clockid_t,
        ts: *mut old_timespec32,
    ) -> i32;
    fn __cvdso_clock_getres_time32(
        clock_id: clockid_t,
        res: *mut old_timespec32,
    ) -> i32;
    fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> i32;
    fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> i32;
    fn __cvdso_clock_getres(
        clock_id: clockid_t,
        res: *mut __kernel_timespec,
    ) -> i32;
}

// CONFIG_COMPAT_32BIT_TIME is a build-time configuration condition.
#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
pub unsafe fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    __cvdso_clock_gettime32(clock, ts)
}

#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
pub unsafe fn __vdso_clock_getres(
    clock_id: clockid_t,
    res: *mut old_timespec32,
) -> i32 {
    __cvdso_clock_getres_time32(clock_id, res)
}

#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
pub unsafe fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    __cvdso_gettimeofday(tv, tz)
}

pub unsafe fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __cvdso_clock_gettime(clock, ts)
}

pub unsafe fn __vdso_clock_getres_time64(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
) -> i32 {
    __cvdso_clock_getres(clock_id, res)
}

/* Avoid unresolved references emitted by GCC */

pub unsafe fn __aeabi_unwind_cpp_pr0() {}

pub unsafe fn __aeabi_unwind_cpp_pr1() {}

pub unsafe fn __aeabi_unwind_cpp_pr2() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
