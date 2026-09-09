/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/jiffies.h. Included C headers provide the referenced
// types and constants; build-time HZ/BITS_PER_LONG conditions are retained.

/* SHIFT_HZ is selected by the HZ range in the original preprocessor code. */
// #if HZ >= 12 && HZ < 24: const SHIFT_HZ: u32 = 4;
// #elif HZ >= 24 && HZ < 48: const SHIFT_HZ: u32 = 5;
// ... through HZ < 12288, with SHIFT_HZ = 13.

#[inline]
pub const fn sh_div(nom: u64, den: u64, lsh: u32) -> u64 {
    ((nom / den) << lsh) + ((((nom % den) << lsh) + den / 2) / den)
}

pub const LATCH: u64 = (CLOCK_TICK_RATE + HZ / 2) / HZ;
pub const TICK_USEC: u64 = (USEC_PER_SEC + HZ / 2) / HZ;
pub const USER_TICK_USEC: u64 = (1_000_000u64 + USER_HZ / 2) / USER_HZ;

extern "C" {
    pub static mut jiffies_64: u64;
    pub static mut jiffies: usize;
    pub fn register_refined_jiffies(clock_tick_rate: i64);
}

#[inline]
pub unsafe fn get_jiffies_64() -> u64 {
    // On 32-bit builds the C declaration is an external sequence-number-safe
    // implementation; on 64-bit builds this is the inline cast below.
    jiffies as u64
}

#[inline]
pub fn time_after(a: usize, b: usize) -> bool {
    (b.wrapping_sub(a) as isize) < 0
}
#[inline]
pub fn time_before(a: usize, b: usize) -> bool { time_after(b, a) }
#[inline]
pub fn time_after_eq(a: usize, b: usize) -> bool {
    (a.wrapping_sub(b) as isize) >= 0
}
#[inline]
pub fn time_before_eq(a: usize, b: usize) -> bool { time_after_eq(b, a) }
#[inline]
pub fn time_in_range(a: usize, b: usize, c: usize) -> bool {
    time_after_eq(a, b) && time_before_eq(a, c)
}
#[inline]
pub fn time_in_range_open(a: usize, b: usize, c: usize) -> bool {
    time_after_eq(a, b) && time_before(a, c)
}

#[inline]
pub fn time_after64(a: u64, b: u64) -> bool { (b.wrapping_sub(a) as i64) < 0 }
#[inline]
pub fn time_before64(a: u64, b: u64) -> bool { time_after64(b, a) }
#[inline]
pub fn time_after_eq64(a: u64, b: u64) -> bool { (a.wrapping_sub(b) as i64) >= 0 }
#[inline]
pub fn time_before_eq64(a: u64, b: u64) -> bool { time_after_eq64(b, a) }
#[inline]
pub fn time_in_range64(a: u64, b: u64, c: u64) -> bool {
    time_after_eq64(a, b) && time_before_eq64(a, c)
}

#[inline] pub unsafe fn time_is_before_jiffies(a: usize) -> bool { time_after(jiffies, a) }
#[inline] pub unsafe fn time_is_before_jiffies64(a: u64) -> bool { time_after64(get_jiffies_64(), a) }
#[inline] pub unsafe fn time_is_after_jiffies(a: usize) -> bool { time_before(jiffies, a) }
#[inline] pub unsafe fn time_is_after_jiffies64(a: u64) -> bool { time_before64(get_jiffies_64(), a) }
#[inline] pub unsafe fn time_is_before_eq_jiffies(a: usize) -> bool { time_after_eq(jiffies, a) }
#[inline] pub unsafe fn time_is_before_eq_jiffies64(a: u64) -> bool { time_after_eq64(get_jiffies_64(), a) }
#[inline] pub unsafe fn time_is_after_eq_jiffies(a: usize) -> bool { time_before_eq(jiffies, a) }
#[inline] pub unsafe fn time_is_after_eq_jiffies64(a: u64) -> bool { time_before_eq64(get_jiffies_64(), a) }

pub const INITIAL_JIFFIES: usize = (-300i64 * HZ as i64) as usize;
pub const MAX_JIFFY_OFFSET: usize = ((LONG_MAX as usize) >> 1) - 1;
pub const TIMESTAMP_SIZE: usize = 30;

pub static mut preset_lpj: usize = 0;

// Scaling constants are compile-time expressions in the C header.
pub const SEC_JIFFIE_SC: u32 = 31 - SHIFT_HZ;
pub const NSEC_JIFFIE_SC: u32 = SEC_JIFFIE_SC + 29;
pub const SEC_CONVERSION: usize = (((NSEC_PER_SEC as u64) << SEC_JIFFIE_SC)
    + TICK_NSEC as u64 - 1) as usize / TICK_NSEC as usize;
pub const NSEC_CONVERSION: usize = (((1u64 << NSEC_JIFFIE_SC)
    + TICK_NSEC as u64 - 1) / TICK_NSEC as u64) as usize;
// MAX_SEC_IN_JIFFIES follows the BITS_PER_LONG < 64 / >= 64 branches.

extern "C" {
    pub fn jiffies_to_msecs(j: usize) -> u32;
    pub fn jiffies_to_usecs(j: usize) -> u32;
    pub fn jiffies64_to_nsecs(j: u64) -> u64;
    pub fn jiffies64_to_msecs(j: u64) -> u64;
    pub fn __msecs_to_jiffies(m: u32) -> usize;
    pub fn __usecs_to_jiffies(u: u32) -> usize;
    pub fn timespec64_to_jiffies(value: *const timespec64) -> usize;
    pub fn jiffies_to_timespec64(jiffies: usize, value: *mut timespec64);
    pub fn jiffies_to_clock_t(x: usize) -> clock_t;
    pub fn clock_t_to_jiffies(x: usize) -> usize;
    pub fn jiffies_64_to_clock_t(x: u64) -> u64;
    pub fn nsec_to_clock_t(x: u64) -> u64;
    pub fn nsecs_to_jiffies64(n: u64) -> u64;
    pub fn nsecs_to_jiffies(n: u64) -> usize;
}

#[inline]
pub fn jiffies_to_nsecs(j: usize) -> u64 {
    unsafe { jiffies_to_usecs(j) as u64 * NSEC_PER_USEC as u64 }
}

#[inline] pub fn _msecs_to_jiffies(m: u32) -> usize { unsafe { __msecs_to_jiffies(m) } }
#[inline] pub fn msecs_to_jiffies(m: u32) -> usize { unsafe { __msecs_to_jiffies(m) } }
#[inline] pub fn secs_to_jiffies(secs: usize) -> usize { secs * HZ as usize }
#[inline] pub fn _usecs_to_jiffies(u: u32) -> usize { unsafe { __usecs_to_jiffies(u) } }
#[inline] pub fn usecs_to_jiffies(u: u32) -> usize { unsafe { __usecs_to_jiffies(u) } }

#[inline]
pub unsafe fn jiffies_delta_to_clock_t(delta: i64) -> clock_t {
    jiffies_to_clock_t(delta.max(0) as usize)
}
#[inline]
pub unsafe fn jiffies_delta_to_msecs(delta: i64) -> u32 {
    jiffies_to_msecs(delta.max(0) as usize)
}

#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct ctl_table { _private: [u8; 0] }

extern "C" {
    pub fn proc_dointvec_jiffies(table: *const ctl_table, dir: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
    pub fn proc_dointvec_ms_jiffies_minmax(table: *const ctl_table, dir: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
    pub fn proc_dointvec_userhz_jiffies(table: *const ctl_table, dir: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
    pub fn proc_dointvec_ms_jiffies(table: *const ctl_table, dir: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
    pub fn proc_doulongvec_ms_jiffies_minmax(table: *const ctl_table, dir: i32, buffer: *mut core::ffi::c_void, lenp: *mut usize, ppos: *mut i64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
