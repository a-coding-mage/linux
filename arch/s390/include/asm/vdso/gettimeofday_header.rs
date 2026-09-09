/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: ASM_VDSO_GETTIMEOFDAY_H

pub const VDSO_HAS_TIME: i32 = 1;
pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;
pub const VDSO_DELTA_NOMASK: i32 = 1;

// Dependencies supplied by the corresponding architecture and kernel bindings
// are intentionally referenced here rather than reimplemented.

#[inline]
pub unsafe fn __arch_get_hw_counter(
    _clock_mode: i32,
    vd: *const vdso_time_data,
) -> u64 {
    get_tod_clock().wrapping_sub((*vd).arch_data.tod_delta)
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    clkid: clockid_t,
    ts: *mut __kernel_timespec,
) -> i64 {
    syscall2(__NR_clock_gettime, clkid as i64, ts as i64)
}

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i64 {
    syscall2(__NR_gettimeofday, tv as i64, tz as i64)
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    clkid: clockid_t,
    ts: *mut __kernel_timespec,
) -> i64 {
    syscall2(__NR_clock_getres, clkid as i64, ts as i64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
