/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 ARM Limited
 */

// This translation applies to aarch64 non-assembler consumers.
// Dependencies supplied by the surrounding kernel translation are referenced
// but intentionally not defined here.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> i32 {
    let tv = _tv;
    let tz = _tz;
    let nr: c_long = __NR_gettimeofday as c_long;
    let mut ret: c_long;

    core::arch::asm!(
        "svc #0",
        inlateout("x0") tv => ret,
        in("x1") tz,
        in("x8") nr,
        lateout("memory") _,
    );

    ret as i32
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> c_long {
    let clkid = _clkid;
    let ts = _ts;
    let nr: c_long = __NR_clock_gettime as c_long;
    let mut ret: c_long;

    core::arch::asm!(
        "svc #0",
        inlateout("x0") clkid => ret,
        in("x1") ts,
        in("x8") nr,
        lateout("memory") _,
    );

    ret
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> i32 {
    let clkid = _clkid;
    let ts = _ts;
    let nr: c_long = __NR_clock_getres as c_long;
    let mut ret: c_long;

    core::arch::asm!(
        "svc #0",
        inlateout("x0") clkid => ret,
        in("x1") ts,
        in("x8") nr,
        lateout("memory") _,
    );

    ret as i32
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    clock_mode: s32,
    _vd: *const vdso_time_data,
) -> u64 {
    /*
     * Core checks for mode already, so this raced against a concurrent
     * update. Return something. Core will do another round and then
     * see the mode change and fallback to the syscall.
     */
    if clock_mode == VDSO_CLOCKMODE_NONE {
        return 0;
    }

    __arch_counter_get_cntvct()
}

// Preserved build-time condition:
// IS_ENABLED(CONFIG_CC_IS_GCC) && IS_ENABLED(CONFIG_PAGE_SIZE_64KB)
#[inline(always)]
pub unsafe fn __arch_get_vdso_u_time_data() -> *const vdso_time_data {
    let ret: *const vdso_time_data = &vdso_u_time_data;

    /* Work around invalid absolute relocations */
    OPTIMIZER_HIDE_VAR(ret);

    ret
}

// External types, constants, globals, and functions are supplied by the
// corresponding translated kernel headers.
extern "C" {
    static vdso_u_time_data: vdso_time_data;
    fn __arch_counter_get_cntvct() -> u64;
    fn OPTIMIZER_HIDE_VAR<T>(value: T);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
