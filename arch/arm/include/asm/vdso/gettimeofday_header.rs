/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 ARM Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/barrier.h, asm/errno.h, asm/unistd.h, asm/vdso/cp15.h,
// vdso/clocksource.h, vdso/time32.h, and uapi/linux/time.h.

pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inlateout("r0") tv as isize => ret,
        in("r1") tz as isize,
        in("r7") __NR_gettimeofday as isize,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    clkid: clockid_t,
    ts: *mut __kernel_timespec,
) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inlateout("r0") clkid as isize => ret,
        in("r1") ts as isize,
        in("r7") __NR_clock_gettime64 as isize,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_gettime32_fallback(
    clkid: clockid_t,
    ts: *mut old_timespec32,
) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inlateout("r0") clkid as isize => ret,
        in("r1") ts as isize,
        in("r7") __NR_clock_gettime as isize,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    clkid: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inlateout("r0") clkid as isize => ret,
        in("r1") ts as isize,
        in("r7") __NR_clock_getres_time64 as isize,
        options(nostack)
    );
    ret as i32
}

#[inline(always)]
pub unsafe fn clock_getres32_fallback(
    clkid: clockid_t,
    ts: *mut old_timespec32,
) -> i32 {
    let mut ret: isize;
    core::arch::asm!(
        "swi #0",
        inlateout("r0") clkid as isize => ret,
        in("r1") ts as isize,
        in("r7") __NR_clock_getres as isize,
        options(nostack)
    );
    ret as i32
}

#[inline]
pub fn arm_vdso_hres_capable() -> bool {
    // Equivalent build-time condition for CONFIG_ARM_ARCH_TIMER.
    cfg!(feature = "CONFIG_ARM_ARCH_TIMER")
}

pub use arm_vdso_hres_capable as __arch_vdso_hres_capable;

#[inline(always)]
pub unsafe fn __arch_get_hw_counter(
    clock_mode: i32,
    vd: *const vdso_time_data,
) -> u64 {
    let _ = vd;
    // CONFIG_ARM_ARCH_TIMER conditionally includes the hardware-counter path.
    #[cfg(feature = "CONFIG_ARM_ARCH_TIMER")]
    {
        if clock_mode == VDSO_CLOCKMODE_NONE {
            return 0;
        }

        isb();
        return read_sysreg(CNTVCT);
    }

    #[cfg(not(feature = "CONFIG_ARM_ARCH_TIMER"))]
    {
        let _ = clock_mode;
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
