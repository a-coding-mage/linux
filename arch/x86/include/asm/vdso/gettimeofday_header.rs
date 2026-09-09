/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Fast user context implementation of clock_gettime, gettimeofday, and time.
 *
 * Copyright (C) 2019 ARM Limited.
 * Copyright 2006 Andi Kleen, SUSE Labs.
 * 32 Bit compat layer by Stefani Seibold <stefani@seibold.net>
 *  sponsored by Rohde & Schwarz GmbH & Co. KG Munich/Germany
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/time.h, asm/vgtod.h, asm/unistd.h, asm/msr.h, asm/pvclock.h,
// clocksource/hyperv_timer.h, and asm/vdso/sys_call.h.

pub const VDSO_HAS_TIME: i32 = 1;
pub const VDSO_HAS_CLOCK_GETRES: i32 = 1;

// These declarations must remain mutable: the backing clock pages change over time.

#[cfg(CONFIG_PARAVIRT_CLOCK)]
extern "C" {
    #[link_name = "pvclock_page"]
    pub static mut pvclock_page: pvclock_vsyscall_time_info;
}

#[cfg(CONFIG_HYPERV_TIMER)]
extern "C" {
    #[link_name = "hvclock_page"]
    pub static mut hvclock_page: ms_hyperv_tsc_page;
}

#[inline(always)]
pub unsafe fn clock_gettime_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> c_long {
    VDSO_SYSCALL2!(clock_gettime, 64, _clkid, _ts)
}

#[inline(always)]
pub unsafe fn gettimeofday_fallback(
    _tv: *mut __kernel_old_timeval,
    _tz: *mut timezone,
) -> c_long {
    VDSO_SYSCALL2!(gettimeofday, _tv, _tz)
}

#[inline(always)]
pub unsafe fn clock_getres_fallback(
    _clkid: clockid_t,
    _ts: *mut __kernel_timespec,
) -> c_long {
    VDSO_SYSCALL2!(clock_getres, _time64, _clkid, _ts)
}

// The following declarations are enabled for 32-bit x86 builds.
#[cfg(not(CONFIG_X86_64))]
#[inline(always)]
pub unsafe fn clock_gettime32_fallback(
    _clkid: clockid_t,
    _ts: *mut old_timespec32,
) -> c_long {
    VDSO_SYSCALL2!(clock_gettime, _clkid, _ts)
}

#[cfg(not(CONFIG_X86_64))]
#[inline(always)]
pub unsafe fn clock_getres32_fallback(
    _clkid: clockid_t,
    _ts: *mut old_timespec32,
) -> c_long {
    VDSO_SYSCALL2!(clock_getres, _clkid, _ts)
}

#[cfg(CONFIG_PARAVIRT_CLOCK)]
pub unsafe fn vread_pvclock() -> u64 {
    let pvti: *const pvclock_vcpu_time_info = &(*core::ptr::addr_of!(pvclock_page)).pvti;
    let mut version: u32;
    let ret: u64;

    loop {
        version = pvclock_read_begin(pvti);

        if unlikely!((*pvti).flags & PVCLOCK_TSC_STABLE_BIT == 0) {
            return U64_MAX;
        }

        ret = __pvclock_read_cycles(pvti, rdtsc_ordered());
        if !pvclock_read_retry(pvti, version) {
            break;
        }
    }

    ret & S64_MAX
}

#[cfg(CONFIG_HYPERV_TIMER)]
pub unsafe fn vread_hvclock() -> u64 {
    let mut tsc: u64 = 0;
    let mut time: u64 = 0;

    if hv_read_tsc_page_tsc(&hvclock_page, &mut tsc, &mut time) {
        return time & S64_MAX;
    }

    U64_MAX
}

#[inline]
pub unsafe fn __arch_get_hw_counter(clock_mode: s32, vd: *const vdso_time_data) -> u64 {
    if likely!(clock_mode == VDSO_CLOCKMODE_TSC) {
        return rdtsc_ordered() as u64 & S64_MAX;
    }

    // Barriers prevent loads from disabled memory-mapped clock pages.
    #[cfg(CONFIG_PARAVIRT_CLOCK)]
    if clock_mode == VDSO_CLOCKMODE_PVCLOCK {
        barrier!();
        return vread_pvclock();
    }

    #[cfg(CONFIG_HYPERV_TIMER)]
    if clock_mode == VDSO_CLOCKMODE_HVCLOCK {
        barrier!();
        return vread_hvclock();
    }

    U64_MAX
}

#[inline]
pub unsafe fn arch_vdso_clocksource_ok(vc: *const vdso_clock) -> bool {
    true
}
pub use arch_vdso_clocksource_ok as vdso_clocksource_ok;

// PV and HyperV clocksources use U64_MAX to indicate asynchronous invalidation.
#[inline]
pub fn arch_vdso_cycles_ok(cycles: u64) -> bool {
    cycles as s64 >= 0
}
pub use arch_vdso_cycles_ok as vdso_cycles_ok;

#[inline(always)]
pub unsafe fn vdso_calc_ns(vc: *const vdso_clock, cycles: u64, base: u64) -> u64 {
    let delta = cycles.wrapping_sub((*vc).cycle_last);

    if unlikely!(delta > (*vc).max_cycles) {
        if delta & (1u64 << 62) != 0 {
            return base >> (*vc).shift;
        }

        return mul_u64_u32_add_u64_shr(
            delta & S64_MAX,
            (*vc).mult,
            base,
            (*vc).shift,
        );
    }

    ((delta.wrapping_mul((*vc).mult as u64)).wrapping_add(base)) >> (*vc).shift
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
