// SPDX-License-Identifier: GPL-2.0
/*
 * Powerpc userspace implementations of gettimeofday() and similar.
 */

// Declarations supplied by the surrounding kernel/vDSO sources:
// clockid_t, __kernel_timespec, old_timespec32, __kernel_old_timeval,
// timezone, vdso_time_data, and __kernel_old_time_t.

#[cfg(target_arch = "powerpc64")]
pub unsafe fn __c_kernel_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_gettime_data(vd, clock, ts)
}

#[cfg(target_arch = "powerpc64")]
pub unsafe fn __c_kernel_clock_getres(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_getres_data(vd, clock_id, res)
}

#[cfg(not(target_arch = "powerpc64"))]
// CONFIG_COMPAT_32BIT_TIME is represented here by the corresponding Cargo
// feature when the surrounding build system exposes it to Rust.
#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
pub unsafe fn __c_kernel_clock_gettime(
    clock: clockid_t,
    ts: *mut old_timespec32,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_gettime32_data(vd, clock, ts)
}

#[cfg(not(target_arch = "powerpc64"))]
#[cfg(feature = "CONFIG_COMPAT_32BIT_TIME")]
pub unsafe fn __c_kernel_clock_getres(
    clock_id: clockid_t,
    res: *mut old_timespec32,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_getres_time32_data(vd, clock_id, res)
}

#[cfg(not(target_arch = "powerpc64"))]
pub unsafe fn __c_kernel_clock_gettime64(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_gettime_data(vd, clock, ts)
}

#[cfg(not(target_arch = "powerpc64"))]
pub unsafe fn __c_kernel_clock_getres_time64(
    clock_id: clockid_t,
    res: *mut __kernel_timespec,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_clock_getres_data(vd, clock_id, res)
}

#[cfg(any(target_arch = "powerpc64", feature = "CONFIG_COMPAT_32BIT_TIME"))]
pub unsafe fn __c_kernel_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
    vd: *const vdso_time_data,
) -> i32 {
    __cvdso_gettimeofday_data(vd, tv, tz)
}

#[cfg(any(target_arch = "powerpc64", feature = "CONFIG_COMPAT_32BIT_TIME"))]
pub unsafe fn __c_kernel_time(
    time: *mut __kernel_old_time_t,
    vd: *const vdso_time_data,
) -> __kernel_old_time_t {
    __cvdso_time_data(vd, time)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
