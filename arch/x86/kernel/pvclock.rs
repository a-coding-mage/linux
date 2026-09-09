// SPDX-License-Identifier: GPL-2.0-or-later
/*  paravirtual clock -- common code used by kvm/xen

*/

// C header dependencies are supplied by the surrounding kernel translation.

static mut valid_flags: u8 = 0;
static mut pvti_cpu0_va: *mut pvclock_vsyscall_time_info = core::ptr::null_mut();

pub unsafe fn pvclock_set_flags(flags: u8) {
    valid_flags = flags;
}

pub unsafe fn pvclock_tsc_khz(src: *mut pvclock_vcpu_time_info) -> c_ulong {
    let mut pv_tsc_khz: u64 = 1_000_000u64 << 32;

    pv_tsc_khz /= (*src).tsc_to_system_mul as u64;
    if (*src).tsc_shift < 0 {
        pv_tsc_khz <<= (-(*src).tsc_shift) as u32;
    } else {
        pv_tsc_khz >>= (*src).tsc_shift as u32;
    }
    pv_tsc_khz as c_ulong
}

pub unsafe fn pvclock_touch_watchdogs() {
    touch_softlockup_watchdog_sync();
    clocksource_touch_watchdog();
    rcu_cpu_stall_reset();
    reset_hung_task_detector();
}

static mut last_value: atomic64_t = ATOMIC64_INIT(0);

pub unsafe fn pvclock_resume() {
    atomic64_set(&mut last_value, 0);
}

pub unsafe fn pvclock_read_flags(src: *mut pvclock_vcpu_time_info) -> u8 {
    let mut version: c_uint;
    let flags: u8;
    loop {
        version = pvclock_read_begin(src);
        flags = (*src).flags;
        if !pvclock_read_retry(src, version) {
            break;
        }
    }
    flags & valid_flags
}

#[inline(always)]
unsafe fn __pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info, dowd: bool) -> u64 {
    let version: c_uint;
    let ret: u64;
    let flags: u8;
    loop {
        version = pvclock_read_begin(src);
        ret = __pvclock_read_cycles(src, rdtsc_ordered());
        flags = (*src).flags;
        if !pvclock_read_retry(src, version) {
            break;
        }
    }

    if dowd && unlikely((flags & PVCLOCK_GUEST_STOPPED) != 0) {
        (*src).flags &= !PVCLOCK_GUEST_STOPPED;
        pvclock_touch_watchdogs();
    }

    if (valid_flags & PVCLOCK_TSC_STABLE_BIT) != 0
        && (flags & PVCLOCK_TSC_STABLE_BIT) != 0
    {
        return ret;
    }

    let mut last = raw_atomic64_read(&last_value);
    loop {
        if ret <= last {
            return last;
        }
        if !raw_atomic64_try_cmpxchg(&mut last_value, &mut last, ret) {
            break;
        }
    }
    ret
}

pub unsafe fn pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info) -> u64 {
    __pvclock_clocksource_read(src, true)
}

// noinstr
pub unsafe fn pvclock_clocksource_read_nowd(src: *mut pvclock_vcpu_time_info) -> u64 {
    __pvclock_clocksource_read(src, false)
}

pub unsafe fn pvclock_read_wallclock(
    wall_clock: *mut pvclock_wall_clock,
    vcpu_time: *mut pvclock_vcpu_time_info,
    ts: *mut timespec64,
) {
    let version: u32;
    let mut delta: u64;
    let mut now: timespec64;
    loop {
        version = (*wall_clock).version;
        rmb(); // fetch version before time
        // wall_clock->sec is u32; dates beyond 2106 require an extended interface.
        now.tv_sec = (*wall_clock).sec as _;
        now.tv_nsec = (*wall_clock).nsec as _;
        rmb(); // fetch time before checking version
        if ((*wall_clock).version & 1) == 0 && version == (*wall_clock).version {
            break;
        }
    }

    delta = pvclock_clocksource_read(vcpu_time);
    delta = delta.wrapping_add((now.tv_sec as u64).wrapping_mul(NSEC_PER_SEC as u64));
    delta = delta.wrapping_add(now.tv_nsec as u64);

    now.tv_nsec = (delta % NSEC_PER_SEC as u64) as _;
    now.tv_sec = (delta / NSEC_PER_SEC as u64) as _;

    set_normalized_timespec64(ts, now.tv_sec, now.tv_nsec);
}

pub unsafe fn pvclock_set_pvti_cpu0_va(pvti: *mut pvclock_vsyscall_time_info) {
    WARN_ON(vclock_was_used(VDSO_CLOCKMODE_PVCLOCK));
    pvti_cpu0_va = pvti;
}

pub unsafe fn pvclock_get_pvti_cpu0_va() -> *mut pvclock_vsyscall_time_info {
    pvti_cpu0_va
}

// EXPORT_SYMBOL_GPL(pvclock_get_pvti_cpu0_va);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
