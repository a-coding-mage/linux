// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  This file contains the interface functions for the various time related
 *  system calls: time, stime, gettimeofday, settimeofday, adjtime
 *
 * Modification history:
 *
 * 1993-09-02    Philip Gladstone
 *      Created file with time related functions from sched/core.c and adjtimex()
 * 1993-10-08    Torsten Duwe
 *      adjtime interface update and CMOS clock write code
 * 1995-08-13    Torsten Duwe
 *      kernel PLL updated to 1994-12-13 specs (rfc-1589)
 * 1999-01-16    Ulrich Windl
 *      Introduced error checking for many cases in adjtimex().
 *      Updated NTP code according to technical memorandum Jan '96
 *      "A Kernel Model for Precision Timekeeping" by Dave Mills
 *      Allow time_constant larger than MAXTC(6) for NTP v4 (MAXTC == 10)
 *      (Even though the technical memorandum forbids it)
 * 2004-07-14    Christoph Lameter
 *      Added getnstimeofday to allow the posix timer functions to return
 *      with nanosecond accuracy
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

pub static mut sys_tz: timezone = timezone { tz_minuteswest: 0, tz_dsttime: 0 };

#[cfg(any(target_pointer_width = "64", feature = "compat_32bit_time"))]
pub unsafe fn sys_time(tloc: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    let i = ktime_get_real_seconds() as __kernel_old_time_t;
    if !tloc.is_null() && put_user(i, tloc) != 0 { return -EFAULT; }
    force_successful_syscall_return();
    i
}

#[cfg(any(target_pointer_width = "64", feature = "compat_32bit_time"))]
pub unsafe fn sys_stime(tptr: *const __kernel_old_time_t) -> i32 {
    let mut tv: timespec64 = core::mem::zeroed();
    if get_user(&mut tv.tv_sec, tptr) != 0 { return -EFAULT; }
    tv.tv_nsec = 0;
    let err = security_settime64(&tv, core::ptr::null());
    if err != 0 { return err; }
    do_settimeofday64(&tv);
    0
}

#[cfg(all(feature = "compat_32bit_time", feature = "arch_want_sys_time32"))]
pub unsafe fn sys_time32(tloc: *mut old_time32_t) -> old_time32_t {
    let i = ktime_get_real_seconds() as old_time32_t;
    if !tloc.is_null() && put_user(i, tloc) != 0 { return -EFAULT; }
    force_successful_syscall_return();
    i
}

#[cfg(all(feature = "compat_32bit_time", feature = "arch_want_sys_time32"))]
pub unsafe fn sys_stime32(tptr: *const old_time32_t) -> i32 {
    let mut tv: timespec64 = core::mem::zeroed();
    if get_user(&mut tv.tv_sec, tptr) != 0 { return -EFAULT; }
    tv.tv_nsec = 0;
    let err = security_settime64(&tv, core::ptr::null());
    if err != 0 { return err; }
    do_settimeofday64(&tv);
    0
}

#[cfg(feature = "want_old_time_type_syscall")]
pub unsafe fn sys_gettimeofday(tv: *mut __kernel_old_timeval, tz: *mut timezone) -> i32 {
    if !tv.is_null() {
        let mut ts: timespec64 = core::mem::zeroed();
        ktime_get_real_ts64(&mut ts);
        if put_user(ts.tv_sec, &mut (*tv).tv_sec) != 0 ||
           put_user(ts.tv_nsec / 1000, &mut (*tv).tv_usec) != 0 { return -EFAULT; }
    }
    if !tz.is_null() && copy_to_user(tz, &sys_tz, core::mem::size_of::<timezone>()) != 0 { return -EFAULT; }
    0
}

pub unsafe fn do_sys_settimeofday64(tv: *const timespec64, tz: *const timezone) -> i32 {
    static mut firsttime: i32 = 1;
    if !tv.is_null() && !timespec64_valid_settod(tv) { return -EINVAL; }
    let error = security_settime64(tv, tz);
    if error != 0 { return error; }
    if !tz.is_null() {
        if (*tz).tz_minuteswest > 15 * 60 || (*tz).tz_minuteswest < -15 * 60 { return -EINVAL; }
        sys_tz = *tz;
        update_vsyscall_tz();
        if firsttime != 0 {
            firsttime = 0;
            if tv.is_null() { timekeeping_warp_clock(); }
        }
    }
    if !tv.is_null() { return do_settimeofday64(tv); }
    0
}

pub unsafe fn sys_settimeofday(tv: *const __kernel_old_timeval, tz: *const timezone) -> i32 {
    let mut new_ts: timespec64 = core::mem::zeroed();
    let mut new_tz: timezone = core::mem::zeroed();
    if !tv.is_null() {
        if !cfg!(feature = "want_old_time_type_syscall") { return -EINVAL; }
        if get_user(&mut new_ts.tv_sec, &(*tv).tv_sec) != 0 || get_user(&mut new_ts.tv_nsec, &(*tv).tv_usec) != 0 { return -EFAULT; }
        if new_ts.tv_nsec >= USEC_PER_SEC || new_ts.tv_nsec < 0 { return -EINVAL; }
        new_ts.tv_nsec *= NSEC_PER_USEC;
    }
    if !tz.is_null() && copy_from_user(&mut new_tz, tz, core::mem::size_of::<timezone>()) != 0 { return -EFAULT; }
    do_sys_settimeofday64(if tv.is_null() { core::ptr::null() } else { &new_ts }, if tz.is_null() { core::ptr::null() } else { &new_tz })
}

#[cfg(feature = "compat_32bit_time")]
pub unsafe fn compat_sys_gettimeofday(tv: *mut old_timeval32, tz: *mut timezone) -> i32 {
    if !tv.is_null() {
        let mut ts: timespec64 = core::mem::zeroed();
        ktime_get_real_ts64(&mut ts);
        if put_user(ts.tv_sec, &mut (*tv).tv_sec) != 0 || put_user(ts.tv_nsec / 1000, &mut (*tv).tv_usec) != 0 { return -EFAULT; }
    }
    if !tz.is_null() && copy_to_user(tz, &sys_tz, core::mem::size_of::<timezone>()) != 0 { return -EFAULT; }
    0
}

#[cfg(feature = "compat")]
pub unsafe fn compat_sys_settimeofday(tv: *const old_timeval32, tz: *const timezone) -> i32 {
    let mut new_ts: timespec64 = core::mem::zeroed();
    let mut new_tz: timezone = core::mem::zeroed();
    if !tv.is_null() {
        if !cfg!(feature = "compat_32bit_time") { return -EINVAL; }
        if get_user(&mut new_ts.tv_sec, &(*tv).tv_sec) != 0 || get_user(&mut new_ts.tv_nsec, &(*tv).tv_usec) != 0 { return -EFAULT; }
        if new_ts.tv_nsec >= USEC_PER_SEC || new_ts.tv_nsec < 0 { return -EINVAL; }
        new_ts.tv_nsec *= NSEC_PER_USEC;
    }
    if !tz.is_null() && copy_from_user(&mut new_tz, tz, core::mem::size_of::<timezone>()) != 0 { return -EFAULT; }
    do_sys_settimeofday64(if tv.is_null() { core::ptr::null() } else { &new_ts }, if tz.is_null() { core::ptr::null() } else { &new_tz })
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn sys_adjtimex(txc_p: *mut __kernel_timex) -> i32 {
    let mut txc: __kernel_timex = core::mem::zeroed();
    if copy_from_user(&mut txc, txc_p, core::mem::size_of::<__kernel_timex>()) != 0 { return -EFAULT; }
    let ret = do_adjtimex(&mut txc);
    if copy_to_user(txc_p, &txc, core::mem::size_of::<__kernel_timex>()) != 0 { -EFAULT } else { ret }
}

#[cfg(feature = "compat_32bit_time")]
pub unsafe fn get_old_timex32(txc: *mut __kernel_timex, utp: *const old_timex32) -> i32 {
    let mut tx32: old_timex32 = core::mem::zeroed();
    *txc = core::mem::zeroed();
    if copy_from_user(&mut tx32, utp, core::mem::size_of::<old_timex32>()) != 0 { return -EFAULT; }
    (*txc).modes=tx32.modes; (*txc).offset=tx32.offset; (*txc).freq=tx32.freq;
    (*txc).maxerror=tx32.maxerror; (*txc).esterror=tx32.esterror; (*txc).status=tx32.status;
    (*txc).constant=tx32.constant; (*txc).precision=tx32.precision; (*txc).tolerance=tx32.tolerance;
    (*txc).time.tv_sec=tx32.time.tv_sec; (*txc).time.tv_usec=tx32.time.tv_usec;
    (*txc).tick=tx32.tick; (*txc).ppsfreq=tx32.ppsfreq; (*txc).jitter=tx32.jitter;
    (*txc).shift=tx32.shift; (*txc).stabil=tx32.stabil; (*txc).jitcnt=tx32.jitcnt;
    (*txc).calcnt=tx32.calcnt; (*txc).errcnt=tx32.errcnt; (*txc).stbcnt=tx32.stbcnt;
    0
}

#[cfg(feature = "compat_32bit_time")]
pub unsafe fn put_old_timex32(utp: *mut old_timex32, txc: *const __kernel_timex) -> i32 {
    let mut tx32: old_timex32 = core::mem::zeroed();
    tx32.modes=(*txc).modes; tx32.offset=(*txc).offset; tx32.freq=(*txc).freq;
    tx32.maxerror=(*txc).maxerror; tx32.esterror=(*txc).esterror; tx32.status=(*txc).status;
    tx32.constant=(*txc).constant; tx32.precision=(*txc).precision; tx32.tolerance=(*txc).tolerance;
    tx32.time.tv_sec=(*txc).time.tv_sec; tx32.time.tv_usec=(*txc).time.tv_usec;
    tx32.tick=(*txc).tick; tx32.ppsfreq=(*txc).ppsfreq; tx32.jitter=(*txc).jitter;
    tx32.shift=(*txc).shift; tx32.stabil=(*txc).stabil; tx32.jitcnt=(*txc).jitcnt;
    tx32.calcnt=(*txc).calcnt; tx32.errcnt=(*txc).errcnt; tx32.stbcnt=(*txc).stbcnt; tx32.tai=(*txc).tai;
    if copy_to_user(utp, &tx32, core::mem::size_of::<old_timex32>()) != 0 { -EFAULT } else { 0 }
}

#[cfg(feature = "compat_32bit_time")]
pub unsafe fn sys_adjtimex_time32(utp: *mut old_timex32) -> i32 {
    let mut txc: __kernel_timex = core::mem::zeroed();
    let err = get_old_timex32(&mut txc, utp); if err != 0 { return err; }
    let ret = do_adjtimex(&mut txc);
    let err = put_old_timex32(utp, &txc); if err != 0 { return err; }
    ret
}

#[cfg(any())]
pub fn jiffies_to_msecs(j: usize) -> u32 { j as u32 }
#[cfg(any())]
pub fn jiffies_to_usecs(j: usize) -> u32 { j as u32 }

pub fn mktime64(year0: u32, mon0: u32, day: u32, hour: u32, min: u32, sec: u32) -> time64_t {
    let mut mon = mon0;
    let mut year = year0;
    if 0 >= (mon as i32 - 2) { mon += 12; year -= 1; }
    ((((year as time64_t / 4 - year as time64_t / 100 + year as time64_t / 400 + 367 * mon as time64_t / 12 + day as time64_t) + year as time64_t * 365 - 719499) * 24 + hour as time64_t) * 60 + min as time64_t) * 60 + sec as time64_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
