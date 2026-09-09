/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Definitions for the clocksource provided by the Hyper-V hypervisor to
 * guest VMs, as described in the Hyper-V Top Level Functional Spec (TLFS).
 *
 * Translated from the corresponding C header.
 */

pub const HV_MAX_MAX_DELTA_TICKS: u32 = 0xffff_ffff;
pub const HV_MIN_DELTA_TICKS: u32 = 1;

/* CONFIG_HYPERV_TIMER is a build-time configuration condition. */
#[cfg(feature = "CONFIG_HYPERV_TIMER")]
extern "C" {
    pub fn hv_stimer_alloc(have_percpu_irqs: bool) -> i32;
    pub fn hv_stimer_cleanup(cpu: u32) -> i32;
    pub fn hv_stimer_global_cleanup();

    pub fn hv_init_clocksource();
    pub fn hv_remap_tsc_clocksource();

    pub fn hv_get_tsc_pfn() -> libc::c_ulong;
    pub fn hv_get_tsc_page() -> *mut crate::ms_hyperv_tsc_page;

    pub fn hv_adj_sched_clock_offset(offset: u64);
}

#[cfg(feature = "CONFIG_HYPERV_TIMER")]
#[inline(always)]
pub unsafe fn hv_read_tsc_page_tsc(
    tsc_pg: *const crate::ms_hyperv_tsc_page,
    cur_tsc: *mut u64,
    time: *mut u64,
) -> bool {
    let mut scale: u64;
    let mut offset: u64;
    let sequence: u32;

    /*
     * The protocol for reading the Hyper-V TSC page is specified in the
     * Hypervisor Top-Level Functional Specification ver. 3.0 and above.
     * A zero sequence means that the time source is unreliable.
     */
    loop {
        sequence = core::ptr::read_volatile(core::ptr::addr_of!((*tsc_pg).tsc_sequence));
        if sequence == 0 {
            return false;
        }

        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        scale = core::ptr::read_volatile(core::ptr::addr_of!((*tsc_pg).tsc_scale));
        offset = core::ptr::read_volatile(core::ptr::addr_of!((*tsc_pg).tsc_offset));
        *cur_tsc = hv_get_raw_timer();
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);

        if core::ptr::read_volatile(core::ptr::addr_of!((*tsc_pg).tsc_sequence)) == sequence {
            break;
        }
    }

    *time = crate::mul_u64_u64_shr(*cur_tsc, scale, 64).wrapping_add(offset);
    true
}

#[cfg(not(feature = "CONFIG_HYPERV_TIMER"))]
#[inline]
pub fn hv_get_tsc_pfn() -> libc::c_ulong {
    0
}

#[cfg(not(feature = "CONFIG_HYPERV_TIMER"))]
#[inline]
pub fn hv_get_tsc_page() -> *mut crate::ms_hyperv_tsc_page {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_HYPERV_TIMER"))]
#[inline(always)]
pub unsafe fn hv_read_tsc_page_tsc(
    _tsc_pg: *const crate::ms_hyperv_tsc_page,
    _cur_tsc: *mut u64,
    _time: *mut u64,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_HYPERV_TIMER"))]
#[inline]
pub fn hv_stimer_cleanup(_cpu: u32) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_HYPERV_TIMER"))]
#[inline]
pub fn hv_stimer_global_cleanup() {}

/* Supplied by the Hyper-V timer implementation. */
extern "C" {
    fn hv_get_raw_timer() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
