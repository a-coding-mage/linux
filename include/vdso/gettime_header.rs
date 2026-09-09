/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/types.h>.
use crate::{clockid_t, __kernel_old_time_t};

#[repr(C)]
pub struct __kernel_timespec {
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
pub struct old_timespec32 {
    _private: [u8; 0],
}

// !defined(CONFIG_64BIT) || defined(BUILD_VDSO32_64)
#[cfg(any(
    not(target_pointer_width = "64"),
    feature = "BUILD_VDSO32_64"
))]
extern "C" {
    pub fn __vdso_clock_getres(clock: clockid_t, res: *mut old_timespec32) -> i32;
    pub fn __vdso_clock_gettime(clock: clockid_t, ts: *mut old_timespec32) -> i32;
}

// The 64-bit configuration branch of the original header.
#[cfg(all(
    target_pointer_width = "64",
    not(feature = "BUILD_VDSO32_64")
))]
extern "C" {
    pub fn __vdso_clock_getres(clock: clockid_t, res: *mut __kernel_timespec) -> i32;
    pub fn __vdso_clock_gettime(clock: clockid_t, ts: *mut __kernel_timespec) -> i32;
}

extern "C" {
    pub fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t;
    pub fn __vdso_gettimeofday(tv: *mut __kernel_old_timeval, tz: *mut timezone) -> i32;
    pub fn __vdso_clock_gettime64(clock: clockid_t, ts: *mut __kernel_timespec) -> i32;
    pub fn __vdso_clock_getres_time64(clock: clockid_t, ts: *mut __kernel_timespec) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
