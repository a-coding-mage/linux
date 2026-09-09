// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the included architecture and UAPI headers:
// asm/unistd.h, linux/types.h, and uapi/asm/unistd_32.h.

use core::ffi::c_long;

pub enum timezone {}
pub enum old_timespec32 {}
pub enum __kernel_timespec {}
pub enum __kernel_old_timeval {}

// Forward declarations supplied by other translation units.
extern "C" {
    pub fn syscall2(number: c_long, arg1: c_long, arg2: c_long) -> c_long;
}

pub type clockid_t = i32;

#[no_mangle]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    syscall2(__NR_gettimeofday as c_long, tv as c_long, tz as c_long) as i32
}

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    syscall2(__NR_clock_gettime as c_long, clock as c_long, ts as c_long) as i32
}

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    syscall2(__NR_clock_gettime64 as c_long, clock as c_long, ts as c_long) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
