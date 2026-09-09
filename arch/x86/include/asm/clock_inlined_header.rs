/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: definitions supplied by asm/tsc.h.

#[repr(C)]
pub struct clocksource {
    _private: [u8; 0],
}

extern "C" {
    pub fn rdtsc_ordered() -> u64;
    pub fn native_wrmsrq(msr: u32, value: u64);
}

// MSR_IA32_TSC_DEADLINE is supplied by the architecture dependencies.
extern "C" {
    pub static MSR_IA32_TSC_DEADLINE: u32;
}

#[inline(always)]
pub unsafe fn arch_inlined_clocksource_read(_cs: *mut clocksource) -> u64 {
    rdtsc_ordered() as u64
}

#[repr(C)]
pub struct clock_event_device {
    _private: [u8; 0],
}

#[inline(always)]
pub unsafe fn arch_inlined_clockevent_set_next_coupled(
    cycles: u64,
    _evt: *mut clock_event_device,
) {
    native_wrmsrq(MSR_IA32_TSC_DEADLINE, cycles);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
