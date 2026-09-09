/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the non-assembler portion of the PowerPC VDSO header.
// Dependencies supplied by the surrounding kernel translation are referenced
// by name and are intentionally not defined here.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;
pub const VDSO_HAS_TIME: i32 = 1;

/*
 * powerpc specific delta calculation.
 *
 * This variant removes the masking of the subtraction because the
 * clocksource mask of all VDSO capable clocksources on powerpc is U64_MAX
 * which would result in a pointless operation. The compiler cannot
 * optimize it away as the mask comes from the vdso data and is not compile
 * time constant.
 */
pub const VDSO_DELTA_NOMASK: i32 = 1;

#[inline(always)]
pub unsafe fn do_syscall_2(_r0: u64, _r3: u64, _r4: u64) -> i32 {
    let mut r0 = _r0;
    let mut r3 = _r3;
    let mut r4 = _r4;
    let mut ret: i32;
    core::arch::asm!(
        "sc",
        "bns+ 1f",
        "neg {ret}, {ret}",
        "1:",
        ret = lateout(reg) ret,
        inout("r0") r0,
        inout("r4") r4,
        in("r3") r3,
        lateout("r5") _, lateout("r6") _, lateout("r7") _,
        lateout("r8") _, lateout("r9") _, lateout("r10") _,
        lateout("r11") _, lateout("r12") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut crate::__kernel_old_timeval,
    _tz: *mut crate::timezone,
) -> i32 {
    do_syscall_2(crate::__NR_gettimeofday as u64, _tv as u64, _tz as u64)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::__kernel_timespec,
) -> i32 {
    do_syscall_2(crate::__NR_clock_gettime as u64, _clkid as u64, _ts as u64)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::__kernel_timespec,
) -> i32 {
    do_syscall_2(crate::__NR_clock_getres as u64, _clkid as u64, _ts as u64)
}

// The following items correspond to the 32-bit (__powerpc64__ absent) branch.
#[cfg(target_pointer_width = "32")]
pub const BUILD_VDSO32: i32 = 1;

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::__kernel_timespec,
) -> i32 {
    do_syscall_2(crate::__NR_clock_gettime64 as u64, _clkid as u64, _ts as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::__kernel_timespec,
) -> i32 {
    do_syscall_2(crate::__NR_clock_getres_time64 as u64, _clkid as u64, _ts as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn clock_gettime32_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::old_timespec32,
) -> i32 {
    do_syscall_2(crate::__NR_clock_gettime as u64, _clkid as u64, _ts as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub unsafe fn clock_getres32_fallback(
    _clkid: crate::clockid_t,
    _ts: *mut crate::old_timespec32,
) -> i32 {
    do_syscall_2(crate::__NR_clock_getres as u64, _clkid as u64, _ts as u64)
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    _clock_mode: crate::s32,
    _vd: *const crate::vdso_time_data,
) -> crate::u64 {
    crate::get_tb()
}

#[inline]
pub fn vdso_clocksource_ok(_vc: *const crate::vdso_clock) -> bool {
    true
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn vdso_shift_ns(ns: crate::u64, shift: usize) -> crate::u64 {
    let mut hi = (ns >> 32) as crate::u32;
    let mut lo = ns as crate::u32;
    lo >>= shift;
    lo |= hi << (32 - shift);
    hi >>= shift;
    if hi == 0 { return lo as crate::u64; }
    ((hi as crate::u64) << 32) | lo as crate::u64
}

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub fn __c_kernel_clock_gettime(clock: crate::clockid_t, ts: *mut crate::__kernel_timespec, vd: *const crate::vdso_time_data) -> i32;
    pub fn __c_kernel_clock_getres(clock_id: crate::clockid_t, res: *mut crate::__kernel_timespec, vd: *const crate::vdso_time_data) -> i32;
}

#[cfg(target_pointer_width = "32")]
extern "C" {
    pub fn __c_kernel_clock_gettime(clock: crate::clockid_t, ts: *mut crate::old_timespec32, vd: *const crate::vdso_time_data) -> i32;
    pub fn __c_kernel_clock_gettime64(clock: crate::clockid_t, ts: *mut crate::__kernel_timespec, vd: *const crate::vdso_time_data) -> i32;
    pub fn __c_kernel_clock_getres(clock_id: crate::clockid_t, res: *mut crate::old_timespec32, vd: *const crate::vdso_time_data) -> i32;
    pub fn __c_kernel_clock_getres_time64(clock_id: crate::clockid_t, res: *mut crate::__kernel_timespec, vd: *const crate::vdso_time_data) -> i32;
}

extern "C" {
    pub fn __c_kernel_gettimeofday(tv: *mut crate::__kernel_old_timeval, tz: *mut crate::timezone, vd: *const crate::vdso_time_data) -> i32;
    pub fn __c_kernel_time(time: *mut crate::__kernel_old_time_t, vd: *const crate::vdso_time_data) -> crate::__kernel_old_time_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
