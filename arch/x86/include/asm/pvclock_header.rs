/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by asm/clocksource.h and asm/pvclock-abi.h remain
// external dependencies of this translation.

#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}

extern "C" {
    pub fn pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info) -> u64;
    pub fn pvclock_clocksource_read_nowd(src: *mut pvclock_vcpu_time_info) -> u64;
    pub fn pvclock_read_flags(src: *mut pvclock_vcpu_time_info) -> u8;
    pub fn pvclock_set_flags(flags: u8);
    pub fn pvclock_tsc_khz(src: *mut pvclock_vcpu_time_info) -> usize;
    pub fn pvclock_read_wallclock(
        wall: *mut pvclock_wall_clock,
        vcpu: *mut pvclock_vcpu_time_info,
        ts: *mut timespec64,
    );
    pub fn pvclock_resume();
    pub fn pvclock_touch_watchdogs();
}

#[inline(always)]
pub unsafe fn pvclock_read_begin(src: *const pvclock_vcpu_time_info) -> u32 {
    let version = (*src).version & !1;
    // Make sure that the version is read before the data.
    virt_rmb();
    version
}

#[inline(always)]
pub unsafe fn pvclock_read_retry(
    src: *const pvclock_vcpu_time_info,
    version: u32,
) -> bool {
    // Make sure that the version is re-read after the data.
    virt_rmb();
    unlikely(version != (*src).version)
}

/*
 * Scale a 64-bit delta by scaling and multiplying by a 32-bit fraction,
 * yielding a 64-bit result.
 */
#[inline(always)]
pub fn pvclock_scale_delta(mut delta: u64, mul_frac: u32, shift: i32) -> u64 {
    if shift < 0 {
        delta >>= (-shift) as u32;
    } else {
        delta = delta.wrapping_shl(shift as u32);
    }

    // This is the 32-bit-fraction product computed by the architecture-
    // specific inline assembly in the original header.
    (((delta as u128) * (mul_frac as u128)) >> 32) as u64
}

#[inline(always)]
pub unsafe fn __pvclock_read_cycles(
    src: *const pvclock_vcpu_time_info,
    tsc: u64,
) -> u64 {
    let delta = tsc.wrapping_sub((*src).tsc_timestamp);
    let offset = pvclock_scale_delta(delta, (*src).tsc_to_system_mul, (*src).tsc_shift);
    (*src).system_time.wrapping_add(offset)
}

#[repr(C, align(64))]
pub struct pvclock_vsyscall_time_info {
    pub pvti: pvclock_vcpu_time_info,
}

pub const PVTI_SIZE: usize = core::mem::size_of::<pvclock_vsyscall_time_info>();

#[cfg(feature = "CONFIG_PARAVIRT_CLOCK")]
extern "C" {
    pub fn pvclock_set_pvti_cpu0_va(pvti: *mut pvclock_vsyscall_time_info);
    pub fn pvclock_get_pvti_cpu0_va() -> *mut pvclock_vsyscall_time_info;
}

#[cfg(not(feature = "CONFIG_PARAVIRT_CLOCK"))]
#[inline]
pub fn pvclock_get_pvti_cpu0_va() -> *mut pvclock_vsyscall_time_info {
    core::ptr::null_mut()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
