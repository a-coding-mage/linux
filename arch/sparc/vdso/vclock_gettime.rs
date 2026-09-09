// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006 Andi Kleen, SUSE Labs.
 *
 * Fast user context implementation of clock_gettime, gettimeofday, and time.
 *
 * The code should have no internal unresolved relocations.
 * Check with readelf after changing.
 * Also alternative() doesn't work.
 */
/*
 * Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/vDSO translation unit:
// linux/compiler.h, linux/types.h, vdso/gettime.h, asm/vdso/gettimeofday.h,
// and lib/vdso/gettimeofday.c.

#[cfg(any(CONFIG_SPARC64, CONFIG_COMPAT_32BIT_TIME))]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    __cvdso_gettimeofday(tv, tz)
}

// __weak __alias(__vdso_gettimeofday)
#[cfg(any(CONFIG_SPARC64, CONFIG_COMPAT_32BIT_TIME))]
pub unsafe extern "C" fn gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    __vdso_gettimeofday(tv, tz)
}

#[cfg(CONFIG_SPARC64)]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __cvdso_clock_gettime(clock, ts)
}

// __weak __alias(__vdso_clock_gettime)
#[cfg(CONFIG_SPARC64)]
pub unsafe extern "C" fn clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __vdso_clock_gettime(clock, ts)
}

#[cfg(not(CONFIG_SPARC64))]
#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    __cvdso_clock_gettime32(clock, ts)
}

// __weak __alias(__vdso_clock_gettime)
#[cfg(not(CONFIG_SPARC64))]
#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe extern "C" fn clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    __vdso_clock_gettime(clock, ts)
}

#[cfg(not(CONFIG_SPARC64))]
pub unsafe extern "C" fn __vdso_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __cvdso_clock_gettime(clock, ts)
}

// __weak __alias(__vdso_clock_gettime64)
#[cfg(not(CONFIG_SPARC64))]
pub unsafe extern "C" fn clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    __vdso_clock_gettime64(clock, ts)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
