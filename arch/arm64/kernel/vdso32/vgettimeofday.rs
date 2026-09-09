// SPDX-License-Identifier: GPL-2.0
/*
 * ARM64 compat userspace implementations of gettimeofday() and similar.
 *
 * Copyright (C) 2018 ARM Limited
 *
 */

// BUILD_VDSO32_64
// Dependency intent: declarations are supplied by <vdso/gettime.h>.

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

unsafe extern "C" {
    pub fn __cvdso_clock_gettime32(
        clock: clockid_t,
        ts: *mut old_timespec32,
    ) -> i32;
    pub fn __cvdso_clock_getres_time32(
        clock_id: clockid_t,
        res: *mut old_timespec32,
    ) -> i32;
    pub fn __cvdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> i32;
    pub fn __cvdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> i32;
    pub fn __cvdso_clock_getres(
        clock_id: clockid_t,
        res: *mut __kernel_timespec,
    ) -> i32;
}

// CONFIG_COMPAT_32BIT_TIME
// The following definitions are included when CONFIG_COMPAT_32BIT_TIME is enabled.
#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    unsafe { __cvdso_clock_gettime32(clock, ts) }
}

#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_clock_getres(
    clock_id: clockid_t,
    res: *mut old_timespec32,
) -> i32 {
    unsafe { __cvdso_clock_getres_time32(clock_id, res) }
}

#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    unsafe { __cvdso_gettimeofday(tv, tz) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    unsafe { __cvdso_clock_gettime(clock, ts) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_clock_getres_time64(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
) -> i32 {
    unsafe { __cvdso_clock_getres(clock_id, res) }
}

/* Avoid unresolved references emitted by GCC */

#[unsafe(no_mangle)]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[unsafe(no_mangle)]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}

#[unsafe(no_mangle)]
pub extern "C" fn __aeabi_unwind_cpp_pr2() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
