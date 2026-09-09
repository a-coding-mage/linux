// SPDX-License-Identifier: GPL-2.0
/*
 * Dummy stubs used when CONFIG_POSIX_TIMERS=n
 *
 * Created by:  Nicolas Pitre, July 2016
 * Copyright:   (C) 2016 Linaro Limited
 */

/*
 * We preserve minimal support for CLOCK_REALTIME and CLOCK_MONOTONIC
 * as it is easy to remain compatible with little code. CLOCK_BOOTTIME
 * is also included for convenience as at least systemd uses it.
 */

pub unsafe fn clock_settime(which_clock: clockid_t, tp: *const __kernel_timespec) -> c_long {
    let mut new_tp: timespec64 = core::mem::zeroed();

    if which_clock != CLOCK_REALTIME {
        return -EINVAL;
    }
    if get_timespec64(&mut new_tp, tp) != 0 {
        return -EFAULT;
    }

    do_sys_settimeofday64(&new_tp, core::ptr::null_mut())
}

unsafe fn do_clock_gettime(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    match which_clock {
        CLOCK_REALTIME => {
            ktime_get_real_ts64(tp);
        }
        CLOCK_MONOTONIC => {
            ktime_get_ts64(tp);
            timens_add_monotonic(tp);
        }
        CLOCK_BOOTTIME => {
            ktime_get_boottime_ts64(tp);
            timens_add_boottime(tp);
        }
        _ => return -EINVAL,
    }

    0
}

pub unsafe fn clock_gettime(which_clock: clockid_t, tp: *mut __kernel_timespec) -> c_long {
    let mut kernel_tp: timespec64 = core::mem::zeroed();

    let ret = do_clock_gettime(which_clock, &mut kernel_tp);
    if ret != 0 {
        return ret as c_long;
    }

    if put_timespec64(&kernel_tp, tp) != 0 {
        return -EFAULT;
    }
    0
}

pub unsafe fn clock_getres(which_clock: clockid_t, tp: *mut __kernel_timespec) -> c_long {
    let rtn_tp = timespec64 {
        tv_sec: 0,
        tv_nsec: hrtimer_resolution,
    };

    match which_clock {
        CLOCK_REALTIME | CLOCK_MONOTONIC | CLOCK_BOOTTIME => {
            if put_timespec64(&rtn_tp, tp) != 0 {
                return -EFAULT;
            }
            0
        }
        _ => -EINVAL,
    }
}

pub unsafe fn clock_nanosleep(
    which_clock: clockid_t,
    flags: c_int,
    rqtp: *const __kernel_timespec,
    mut rmtp: *mut __kernel_timespec,
) -> c_long {
    let mut t: timespec64 = core::mem::zeroed();
    let mut texp: ktime_t;

    match which_clock {
        CLOCK_REALTIME | CLOCK_MONOTONIC | CLOCK_BOOTTIME => {}
        _ => return -EINVAL,
    }

    if get_timespec64(&mut t, rqtp) != 0 {
        return -EFAULT;
    }
    if !timespec64_valid(&t) {
        return -EINVAL;
    }
    if flags & TIMER_ABSTIME != 0 {
        rmtp = core::ptr::null_mut();
    }
    (*current).restart_block.fn_ = do_no_restart_syscall;
    (*current).restart_block.nanosleep.type_ = if !rmtp.is_null() { TT_NATIVE } else { TT_NONE };
    (*current).restart_block.nanosleep.rmtp = rmtp;
    texp = timespec64_to_ktime(t);
    if flags & TIMER_ABSTIME != 0 {
        texp = timens_ktime_to_host(which_clock, texp);
    }
    hrtimer_nanosleep(
        texp,
        if flags & TIMER_ABSTIME != 0 { HRTIMER_MODE_ABS } else { HRTIMER_MODE_REL },
        which_clock,
    )
}

// Preserved from the source's CONFIG_COMPAT_32BIT_TIME conditional build section.
#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn clock_settime32(which_clock: clockid_t, tp: *mut old_timespec32) -> c_long {
    let mut new_tp: timespec64 = core::mem::zeroed();

    if which_clock != CLOCK_REALTIME {
        return -EINVAL;
    }
    if get_old_timespec32(&mut new_tp, tp) != 0 {
        return -EFAULT;
    }
    do_sys_settimeofday64(&new_tp, core::ptr::null_mut())
}

#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn clock_gettime32(which_clock: clockid_t, tp: *mut old_timespec32) -> c_long {
    let mut kernel_tp: timespec64 = core::mem::zeroed();
    let ret = do_clock_gettime(which_clock, &mut kernel_tp);
    if ret != 0 { return ret as c_long; }
    if put_old_timespec32(&kernel_tp, tp) != 0 { return -EFAULT; }
    0
}

#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn clock_getres_time32(which_clock: clockid_t, tp: *mut old_timespec32) -> c_long {
    let rtn_tp = timespec64 { tv_sec: 0, tv_nsec: hrtimer_resolution };
    match which_clock {
        CLOCK_REALTIME | CLOCK_MONOTONIC | CLOCK_BOOTTIME => {
            if put_old_timespec32(&rtn_tp, tp) != 0 { return -EFAULT; }
            0
        }
        _ => -EINVAL,
    }
}

#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn clock_nanosleep_time32(
    which_clock: clockid_t,
    flags: c_int,
    rqtp: *mut old_timespec32,
    mut rmtp: *mut old_timespec32,
) -> c_long {
    let mut t: timespec64 = core::mem::zeroed();
    let mut texp: ktime_t;

    match which_clock {
        CLOCK_REALTIME | CLOCK_MONOTONIC | CLOCK_BOOTTIME => {}
        _ => return -EINVAL,
    }
    if get_old_timespec32(&mut t, rqtp) != 0 { return -EFAULT; }
    if !timespec64_valid(&t) { return -EINVAL; }
    if flags & TIMER_ABSTIME != 0 { rmtp = core::ptr::null_mut(); }
    (*current).restart_block.fn_ = do_no_restart_syscall;
    (*current).restart_block.nanosleep.type_ = if !rmtp.is_null() { TT_COMPAT } else { TT_NONE };
    (*current).restart_block.nanosleep.compat_rmtp = rmtp;
    texp = timespec64_to_ktime(t);
    if flags & TIMER_ABSTIME != 0 { texp = timens_ktime_to_host(which_clock, texp); }
    hrtimer_nanosleep(
        texp,
        if flags & TIMER_ABSTIME != 0 { HRTIMER_MODE_ABS } else { HRTIMER_MODE_REL },
        which_clock,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
