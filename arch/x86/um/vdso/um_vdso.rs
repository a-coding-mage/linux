// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
 *
 * This vDSO turns all calls into a syscall so that UML can trap them.
 */

// DISABLE_BRANCH_PROFILING: profiling is disabled for userspace code.

use core::arch::asm;

// Types supplied by the included kernel headers.
pub type clockid_t = i32;
pub type __kernel_old_time_t = i64;

#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct __kernel_old_timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

// x86 syscall numbers from asm/unistd.h.
const __NR_clock_gettime: usize = 228;
const __NR_gettimeofday: usize = 96;
const __NR_time: usize = 201;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    let ret: i64;

    asm!(
        "syscall",
        inlateout("rax") __NR_clock_gettime as i64 => ret,
        in("rdi") clock as i64,
        in("rsi") ts,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );

    ret as i32
}

// Weak alias of __vdso_clock_gettime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __vdso_clock_gettime(clock, ts)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    let ret: i64;

    asm!(
        "syscall",
        inlateout("rax") __NR_gettimeofday as i64 => ret,
        in("rdi") tv,
        in("rsi") tz,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );

    ret as i32
}

// Weak alias of __vdso_gettimeofday.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    __vdso_gettimeofday(tv, tz)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __vdso_time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    let secs: i64;

    asm!(
        "syscall",
        inlateout("rax") __NR_time as i64 => secs,
        in("rdi") t,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );

    secs
}

// Weak alias of __vdso_time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn time(t: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    __vdso_time(t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
