/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// This code is present when CONFIG_GENERIC_GETTIMEOFDAY is enabled.

/// VDSO_HAS_CLOCK_GETRES
pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall 0",
        in("a7") __NR_gettimeofday as isize,
        in("a0") _tv,
        in("a1") _tz,
        lateout("a0") ret,
        out("t0") _,
        out("t1") _,
        out("t2") _,
        out("t3") _,
        out("t4") _,
        out("t5") _,
        out("t6") _,
        out("t7") _,
        out("t8") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall 0",
        in("a7") __NR_clock_gettime as isize,
        in("a0") _clkid,
        in("a1") _ts,
        lateout("a0") ret,
        out("t0") _,
        out("t1") _,
        out("t2") _,
        out("t3") _,
        out("t4") _,
        out("t5") _,
        out("t6") _,
        out("t7") _,
        out("t8") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> i32 {
    let mut ret: isize;
    core::arch::asm!(
        "syscall 0",
        in("a7") __NR_clock_getres as isize,
        in("a0") _clkid,
        in("a1") _ts,
        lateout("a0") ret,
        out("t0") _,
        out("t1") _,
        out("t2") _,
        out("t3") _,
        out("t4") _,
        out("t5") _,
        out("t6") _,
        out("t7") _,
        out("t8") _,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    _clock_mode: i32,
    _vd: *const vdso_time_data,
) -> u64 {
    let mut count: u64;
    core::arch::asm!(
        "rdtime.d {count}, $zero",
        count = lateout(reg) count,
    );
    count
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
