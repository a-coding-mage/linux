/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to linux/clocksource.h, linux/spinlock.h, and
// linux/time.h are supplied by other translation units.

use core::ffi::c_ulong;

#[repr(C)]
pub struct timekeeper {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub static mut timekeeping_mg_floor_swaps: c_ulong;
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[inline]
pub unsafe fn timekeeping_inc_mg_floor_swaps() {
    // `this_cpu_inc` is a kernel-provided per-CPU primitive.
    this_cpu_inc!(timekeeping_mg_floor_swaps);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn tk_debug_account_sleep_time(t: *const timespec64);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn tk_debug_account_sleep_time(_x: *const timespec64) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn timekeeping_inc_mg_floor_swaps() {}

#[inline]
pub const fn clocksource_delta(now: u64, last: u64, mask: u64, max_delta: u64) -> u64 {
    let ret = now.wrapping_sub(last) & mask;

    /*
     * Prevent time going backwards by checking the result against
     * @max_delta. If greater, return 0.
     */
    if ret > max_delta { 0 } else { ret }
}

/* Semi public for serialization of non timekeeper VDSO updates. */
extern "C" {
    pub fn timekeeper_lock_irqsave() -> c_ulong;
    pub fn timekeeper_unlock_irqrestore(flags: c_ulong);
}

/* NTP specific interface to access the current seconds value */
extern "C" {
    pub fn ktime_get_ntp_seconds(id: u32) -> isize;
}

#[cfg(feature = "CONFIG_GENERIC_GETTIMEOFDAY")]
extern "C" {
    pub fn update_vsyscall(tk: *mut timekeeper);
    pub fn update_vsyscall_tz();
    pub fn vdso_time_update_aux(tk: *mut timekeeper);
}

#[cfg(not(feature = "CONFIG_GENERIC_GETTIMEOFDAY"))]
#[inline]
pub fn update_vsyscall(_tk: *mut timekeeper) {}

#[cfg(not(feature = "CONFIG_GENERIC_GETTIMEOFDAY"))]
#[inline]
pub fn update_vsyscall_tz() {}

#[cfg(not(feature = "CONFIG_GENERIC_GETTIMEOFDAY"))]
#[inline]
pub fn vdso_time_update_aux(_tk: *mut timekeeper) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
