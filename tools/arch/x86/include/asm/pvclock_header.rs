// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of arch/x86/include/asm/pvclock.h.
// Dependencies from <asm/barrier.h> and <asm/pvclock-abi.h> are expected to be
// supplied by surrounding translated code.

unsafe extern "C" {
    /* some helper functions for xen and kvm pv clock sources */
    pub fn pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info) -> u64;
    pub fn pvclock_read_flags(src: *mut pvclock_vcpu_time_info) -> u8;
    pub fn pvclock_set_flags(flags: u8);
    pub fn pvclock_tsc_khz(src: *mut pvclock_vcpu_time_info) -> c_ulong;
    pub fn pvclock_resume();

    pub fn pvclock_touch_watchdogs();
}

#[inline(always)]
pub unsafe fn pvclock_read_begin(src: *const pvclock_vcpu_time_info) -> c_uint {
    let version: c_uint = unsafe { (*src).version & !1 };
    /* Make sure that the version is read before the data. */
    unsafe { rmb() };
    version
}

#[inline(always)]
pub unsafe fn pvclock_read_retry(
    src: *const pvclock_vcpu_time_info,
    version: c_uint,
) -> bool {
    /* Make sure that the version is re-read after the data. */
    unsafe { rmb() };
    unsafe { version != (*src).version }
}

/*
 * Scale a 64-bit delta by scaling and multiplying by a 32-bit fraction,
 * yielding a 64-bit result.
 */
#[inline]
pub unsafe fn pvclock_scale_delta(mut delta: u64, mul_frac: u32, shift: c_int) -> u64 {
    if shift < 0 {
        delta >>= (-shift) as u32;
    } else {
        delta <<= shift as u32;
    }

    /*
     * The C header uses x86 inline assembly for __i386__ and __x86_64__ and
     * emits #error for other architectures. The assembly computes the high
     * 64-bit scaled result, equivalent to (delta * mul_frac) >> 32.
     */
    (((delta as u128) * (mul_frac as u128)) >> 32) as u64
}

#[inline(always)]
pub unsafe fn __pvclock_read_cycles(
    src: *const pvclock_vcpu_time_info,
    tsc: u64,
) -> u64 {
    let delta: u64 = unsafe { tsc.wrapping_sub((*src).tsc_timestamp) };
    let offset: u64 = unsafe {
        pvclock_scale_delta(delta, (*src).tsc_to_system_mul, (*src).tsc_shift)
    };
    unsafe { (*src).system_time.wrapping_add(offset) }
}

#[repr(C, align(64))]
pub struct pvclock_vsyscall_time_info {
    pub pvti: pvclock_vcpu_time_info,
}

pub const PVTI_SIZE: usize = core::mem::size_of::<pvclock_vsyscall_time_info>();

// CONFIG_PARAVIRT_CLOCK: external declarations are present when enabled.
#[cfg(CONFIG_PARAVIRT_CLOCK)]
unsafe extern "C" {
    pub fn pvclock_set_pvti_cpu0_va(pvti: *mut pvclock_vsyscall_time_info);
    pub fn pvclock_get_pvti_cpu0_va() -> *mut pvclock_vsyscall_time_info;
}

#[cfg(not(CONFIG_PARAVIRT_CLOCK))]
#[inline]
pub unsafe fn pvclock_get_pvti_cpu0_va() -> *mut pvclock_vsyscall_time_info {
    core::ptr::null_mut()
}
