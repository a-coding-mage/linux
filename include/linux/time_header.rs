/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/cache.h, linux/math64.h, linux/time64.h, linux/time32.h, vdso/time.h

extern "C" {
    pub static mut sys_tz: timezone;

    pub fn get_timespec64(
        ts: *mut timespec64,
        uts: *const kernel_timespec,
    ) -> kernel::ffi::c_int;
    pub fn put_timespec64(
        ts: *const timespec64,
        uts: *mut kernel_timespec,
    ) -> kernel::ffi::c_int;
    pub fn get_itimerspec64(
        it: *mut itimerspec64,
        uit: *const kernel_itimerspec,
    ) -> kernel::ffi::c_int;
    pub fn put_itimerspec64(
        it: *const itimerspec64,
        uit: *mut kernel_itimerspec,
    ) -> kernel::ffi::c_int;

    pub fn mktime64(
        year: kernel::ffi::c_uint,
        mon: kernel::ffi::c_uint,
        day: kernel::ffi::c_uint,
        hour: kernel::ffi::c_uint,
        min: kernel::ffi::c_uint,
        sec: kernel::ffi::c_uint,
    ) -> time64_t;

    #[cfg(CONFIG_POSIX_TIMERS)]
    pub fn clear_itimer();

    pub fn do_utimes(
        dfd: kernel::ffi::c_int,
        filename: *const kernel::ffi::c_char,
        times: *mut timespec64,
        flags: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
}

#[cfg(not(CONFIG_POSIX_TIMERS))]
#[inline]
pub fn clear_itimer() {}

/*
 * Similar to the struct tm in userspace <time.h>, but it needs to be here so
 * that the kernel source is self contained.
 */
#[repr(C)]
pub struct tm {
    /* the number of seconds after the minute, normally in the range
     * 0 to 59, but can be up to 60 to allow for leap seconds */
    pub tm_sec: kernel::ffi::c_int,
    /* the number of minutes after the hour, in the range 0 to 59 */
    pub tm_min: kernel::ffi::c_int,
    /* the number of hours past midnight, in the range 0 to 23 */
    pub tm_hour: kernel::ffi::c_int,
    /* the day of the month, in the range 1 to 31 */
    pub tm_mday: kernel::ffi::c_int,
    /* the number of months since January, in the range 0 to 11 */
    pub tm_mon: kernel::ffi::c_int,
    /* the number of years since 1900 */
    pub tm_year: kernel::ffi::c_long,
    /* the number of days since Sunday, in the range 0 to 6 */
    pub tm_wday: kernel::ffi::c_int,
    /* the number of days since January 1, in the range 0 to 365 */
    pub tm_yday: kernel::ffi::c_int,
}

extern "C" {
    pub fn time64_to_tm(totalsecs: time64_t, offset: kernel::ffi::c_int, result: *mut tm);
}

#[inline]
pub unsafe fn itimerspec64_valid(its: *const itimerspec64) -> bool {
    if !timespec64_valid(&(*its).it_interval) || !timespec64_valid(&(*its).it_value) {
        return false;
    }

    true
}

/**
 * time_after32 - compare two 32-bit relative times
 * @a: the time which may be after @b
 * @b: the time which may be before @a
 */
#[macro_export]
macro_rules! time_after32 {
    ($a:expr, $b:expr) => {
        (($b as u32).wrapping_sub($a as u32) as i32) < 0
    };
}

#[macro_export]
macro_rules! time_before32 {
    ($b:expr, $a:expr) => {
        $crate::time_after32!($a, $b)
    };
}

#[macro_export]
macro_rules! time_between32 {
    ($t:expr, $l:expr, $h:expr) => {
        (($h as u32).wrapping_sub($l as u32)) >= (($t as u32).wrapping_sub($l as u32))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
