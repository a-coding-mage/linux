/*
 *  include/linux/ktime.h
 *
 *  ktime_t - nanosecond-resolution time format.
 *
 *  Data type definitions, declarations, prototypes and macros.
 *
 *  For licensing details see kernel-base/COPYING
 */

/* C header dependencies are supplied by other translated headers. */

#[inline]
pub fn ktime_set(secs: s64, nsecs: c_ulong) -> ktime_t {
    if secs >= KTIME_SEC_MAX {
        return KTIME_MAX;
    }
    secs * NSEC_PER_SEC + nsecs as s64
}

#[macro_export]
macro_rules! ktime_sub {
    ($lhs:expr, $rhs:expr) => { ($lhs) - ($rhs) };
}

#[macro_export]
macro_rules! ktime_add {
    ($lhs:expr, $rhs:expr) => { ($lhs) + ($rhs) };
}

#[macro_export]
macro_rules! ktime_add_unsafe {
    ($lhs:expr, $rhs:expr) => { ($lhs as u64).wrapping_add($rhs) };
}

#[macro_export]
macro_rules! ktime_add_ns {
    ($kt:expr, $nsval:expr) => { ($kt) + ($nsval) };
}

#[macro_export]
macro_rules! ktime_sub_ns {
    ($kt:expr, $nsval:expr) => { ($kt) - ($nsval) };
}

#[inline]
pub fn timespec64_to_ktime(ts: timespec64) -> ktime_t {
    ktime_set(ts.tv_sec, ts.tv_nsec)
}

#[macro_export]
macro_rules! ktime_to_timespec64 {
    ($kt:expr) => { ns_to_timespec64($kt) };
}

#[inline]
pub fn ktime_to_ns(kt: ktime_t) -> s64 {
    kt
}

#[inline]
pub fn ktime_compare(cmp1: ktime_t, cmp2: ktime_t) -> c_int {
    if cmp1 < cmp2 { -1 } else if cmp1 > cmp2 { 1 } else { 0 }
}

#[inline]
pub fn ktime_after(cmp1: ktime_t, cmp2: ktime_t) -> bool {
    ktime_compare(cmp1, cmp2) > 0
}

#[inline]
pub fn ktime_before(cmp1: ktime_t, cmp2: ktime_t) -> bool {
    ktime_compare(cmp1, cmp2) < 0
}

/* The original implementation is selected by BITS_PER_LONG. */
#[cfg(target_pointer_width = "32")]
extern "C" {
    pub fn __ktime_divns(kt: ktime_t, div: s64) -> s64;
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub unsafe fn ktime_divns(kt: ktime_t, div: s64) -> s64 {
    /* BUG_ON(div < 0); */
    if div >= 0 && div <= u32::MAX as s64 {
        let ns = kt;
        let tmp = if ns < 0 { (-(ns as i128)) as u64 } else { ns as u64 };
        let result = tmp / div as u64;
        if ns < 0 { -(result as s64) } else { result as s64 }
    } else {
        __ktime_divns(kt, div)
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn ktime_divns(kt: ktime_t, div: s64) -> s64 {
    /* WARN_ON(div < 0); */
    kt / div
}

#[inline]
pub fn ktime_to_us(kt: ktime_t) -> s64 { unsafe { ktime_divns(kt, NSEC_PER_USEC) } }

#[inline]
pub fn ktime_to_ms(kt: ktime_t) -> s64 { unsafe { ktime_divns(kt, NSEC_PER_MSEC) } }

#[inline]
pub fn ktime_us_delta(later: ktime_t, earlier: ktime_t) -> s64 {
    ktime_to_us(later - earlier)
}

#[inline]
pub fn ktime_ms_delta(later: ktime_t, earlier: ktime_t) -> s64 {
    ktime_to_ms(later - earlier)
}

#[inline]
pub fn ktime_add_us(kt: ktime_t, usec: u64) -> ktime_t {
    kt + usec * NSEC_PER_USEC
}

#[inline]
pub fn ktime_add_ms(kt: ktime_t, msec: u64) -> ktime_t {
    kt + msec * NSEC_PER_MSEC
}

#[inline]
pub fn ktime_sub_us(kt: ktime_t, usec: u64) -> ktime_t {
    kt - usec * NSEC_PER_USEC
}

#[inline]
pub fn ktime_sub_ms(kt: ktime_t, msec: u64) -> ktime_t {
    kt - msec * NSEC_PER_MSEC
}

extern "C" {
    pub fn ktime_add_safe(lhs: ktime_t, rhs: ktime_t) -> ktime_t;
}

#[inline]
pub unsafe fn ktime_to_timespec64_cond(kt: ktime_t, ts: *mut timespec64) -> bool {
    if kt != 0 {
        *ts = ns_to_timespec64(kt);
        true
    } else {
        false
    }
}

#[inline]
pub fn ns_to_ktime(ns: u64) -> ktime_t { ns as ktime_t }

#[inline]
pub fn us_to_ktime(us: u64) -> ktime_t { (us * NSEC_PER_USEC) as ktime_t }

#[inline]
pub fn ms_to_ktime(ms: u64) -> ktime_t { (ms * NSEC_PER_MSEC) as ktime_t }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
