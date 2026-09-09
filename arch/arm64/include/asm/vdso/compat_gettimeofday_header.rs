// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 ARM Limited
//
// Translated from the non-assembler portion of compat_gettimeofday.h.
// C build-time includes and header guards are intentionally omitted.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;
pub const BUILD_VDSO32: i32 = 1;

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> i32 {
    let tv = _tv;
    let tz = _tz;
    let nr: isize = __NR_compat32_gettimeofday as isize;
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inout("r0") tv => ret,
        in("r1") tz,
        in("r7") nr,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> isize {
    let ts = _ts;
    let clkid = _clkid;
    let nr: isize = __NR_compat32_clock_gettime64 as isize;
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inout("r0") clkid => ret,
        in("r1") ts,
        in("r7") nr,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_gettime32_fallback(
    _clkid: clockid_t,
    _ts: *mut old_timespec32,
) -> isize {
    let ts = _ts;
    let clkid = _clkid;
    let nr: isize = __NR_compat32_clock_gettime as isize;
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inout("r0") clkid => ret,
        in("r1") ts,
        in("r7") nr,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> i32 {
    let ts = _ts;
    let clkid = _clkid;
    let nr: isize = __NR_compat32_clock_getres_time64 as isize;
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inout("r0") clkid => ret,
        in("r1") ts,
        in("r7") nr,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn clock_getres32_fallback(
    _clkid: clockid_t,
    _ts: *mut old_timespec32,
) -> i32 {
    let ts = _ts;
    let clkid = _clkid;
    let nr: isize = __NR_compat32_clock_getres as isize;
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inout("r0") clkid => ret,
        in("r1") ts,
        in("r7") nr,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    clock_mode: s32,
    _vd: *const vdso_time_data,
) -> u64 {
    let mut res: u64;

    /* Core checks for mode already; a raced update returns a value so the
     * core can retry and then observe the mode change and use the syscall. */
    if clock_mode != VDSO_CLOCKMODE_ARCHTIMER {
        return 0;
    }

    /* This isb() prevents the counter value from being speculated. */
    isb();
    core::arch::asm!("mrrc p15, 1, {lo}, {hi}, c14",
        lo = out(reg) res,
        hi = out(reg) _,
        options(nostack));
    /* This isb() prevents the seq lock from being speculated. */
    isb();

    res
}

#[inline(always)]
pub unsafe fn __arch_get_vdso_u_time_data() -> *const vdso_time_data {
    let ret: *const vdso_time_data;
    core::arch::asm!("mov {0}, {1}", out(reg) ret, in(reg) &vdso_u_time_data);
    ret
}

#[inline(always)]
pub fn vdso_clocksource_ok(vc: *const vdso_clock) -> bool {
    unsafe { (*vc).clock_mode == VDSO_CLOCKMODE_ARCHTIMER }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
