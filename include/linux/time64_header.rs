/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left external to this translation.

pub type time64_t = i64;
pub type timeu64_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec64 {
    pub tv_sec: time64_t, /* seconds */
    pub tv_nsec: ::core::ffi::c_long, /* nanoseconds */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerspec64 {
    pub it_interval: timespec64,
    pub it_value: timespec64,
}

/* Parameters used to convert the timespec values: */
pub const PSEC_PER_NSEC: ::core::ffi::c_long = 1000;

/* Located here for timespec[64]_valid_strict */
pub const TIME64_MAX: i64 = !(1u64 << 63) as i64;
pub const TIME64_MIN: i64 = -TIME64_MAX - 1;

pub const KTIME_MAX: i64 = !(1u64 << 63) as i64;
pub const KTIME_MIN: i64 = -KTIME_MAX - 1;
pub const KTIME_SEC_MAX: i64 = KTIME_MAX / NSEC_PER_SEC;
pub const KTIME_SEC_MIN: i64 = KTIME_MIN / NSEC_PER_SEC;

/* Limits for settimeofday(). */
pub const TIME_UPTIME_SEC_MAX: i64 = 30 * 365 * 24 * 3600;
pub const TIME_SETTOD_SEC_MAX: i64 = KTIME_SEC_MAX - TIME_UPTIME_SEC_MAX;

pub unsafe fn timespec64_equal(a: *const timespec64, b: *const timespec64) -> i32 {
    ((*a).tv_sec == (*b).tv_sec && (*a).tv_nsec == (*b).tv_nsec) as i32
}

pub unsafe fn timespec64_is_epoch(ts: *const timespec64) -> bool {
    (*ts).tv_sec == 0 && (*ts).tv_nsec == 0
}

/* lhs < rhs: return <0; lhs == rhs: return 0; lhs > rhs: return >0 */
pub unsafe fn timespec64_compare(lhs: *const timespec64, rhs: *const timespec64) -> i32 {
    if (*lhs).tv_sec < (*rhs).tv_sec {
        return -1;
    }
    if (*lhs).tv_sec > (*rhs).tv_sec {
        return 1;
    }
    (*lhs).tv_nsec.wrapping_sub((*rhs).tv_nsec) as i32
}

extern "C" {
    pub fn set_normalized_timespec64(ts: *mut timespec64, sec: time64_t, nsec: i64);
}

pub unsafe fn timespec64_add(lhs: timespec64, rhs: timespec64) -> timespec64 {
    let mut ts_delta: timespec64 = ::core::mem::zeroed();
    set_normalized_timespec64(
        &mut ts_delta,
        lhs.tv_sec.wrapping_add(rhs.tv_sec),
        (lhs.tv_nsec as i64).wrapping_add(rhs.tv_nsec as i64),
    );
    ts_delta
}

/* sub = lhs - rhs, in normalized form */
pub unsafe fn timespec64_sub(lhs: timespec64, rhs: timespec64) -> timespec64 {
    let mut ts_delta: timespec64 = ::core::mem::zeroed();
    set_normalized_timespec64(
        &mut ts_delta,
        lhs.tv_sec.wrapping_sub(rhs.tv_sec),
        (lhs.tv_nsec as i64).wrapping_sub(rhs.tv_nsec as i64),
    );
    ts_delta
}

/* Returns true if the timespec64 is norm, false if denorm: */
pub unsafe fn timespec64_valid(ts: *const timespec64) -> bool {
    /* Dates before 1970 are bogus */
    if (*ts).tv_sec < 0 {
        return false;
    }
    /* Can't have more nanoseconds then a second */
    if (*ts).tv_nsec as u64 >= NSEC_PER_SEC as u64 {
        return false;
    }
    true
}

pub unsafe fn timespec64_valid_strict(ts: *const timespec64) -> bool {
    if !timespec64_valid(ts) {
        return false;
    }
    /* Disallow values that could overflow ktime_t */
    if (*ts).tv_sec as u64 >= KTIME_SEC_MAX as u64 {
        return false;
    }
    true
}

pub unsafe fn timespec64_valid_settod(ts: *const timespec64) -> bool {
    if !timespec64_valid(ts) {
        return false;
    }
    /* Disallow values which cause overflow issues vs. CLOCK_REALTIME */
    if (*ts).tv_sec as u64 >= TIME_SETTOD_SEC_MAX as u64 {
        return false;
    }
    true
}

pub unsafe fn timespec64_to_ns(ts: *const timespec64) -> i64 {
    /* Prevent multiplication overflow / underflow */
    if (*ts).tv_sec >= KTIME_SEC_MAX {
        return KTIME_MAX;
    }
    if (*ts).tv_sec <= KTIME_SEC_MIN {
        return KTIME_MIN;
    }
    (*ts).tv_sec.wrapping_mul(NSEC_PER_SEC).wrapping_add((*ts).tv_nsec as i64)
}

extern "C" {
    pub fn ns_to_timespec64(nsec: i64) -> timespec64;
    pub fn __iter_div_u64_rem(n: u64, base: u64, rem: *mut u64) -> u64;
}

pub unsafe fn timespec64_add_ns(a: *mut timespec64, mut ns: u64) {
    (*a).tv_sec = (*a).tv_sec.wrapping_add(__iter_div_u64_rem(
        ((*a).tv_nsec as u64).wrapping_add(ns),
        NSEC_PER_SEC as u64,
        &mut ns,
    ) as i64);
    (*a).tv_nsec = ns as ::core::ffi::c_long;
}

/* timespec64_add_safe assumes both values are positive and checks for overflow. */
extern "C" {
    pub fn timespec64_add_safe(lhs: timespec64, rhs: timespec64) -> timespec64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
