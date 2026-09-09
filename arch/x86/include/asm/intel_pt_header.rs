/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: _ASM_X86_INTEL_PT_H
 */

pub const PT_CPUID_LEAVES: i32 = 2;
pub const PT_CPUID_REGS_NUM: i32 = 4; /* number of registers (eax, ebx, ecx, edx) */

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pt_capabilities {
    PT_CAP_max_subleaf = 0,
    PT_CAP_cr3_filtering,
    PT_CAP_psb_cyc,
    PT_CAP_ip_filtering,
    PT_CAP_mtc,
    PT_CAP_ptwrite,
    PT_CAP_power_event_trace,
    PT_CAP_event_trace,
    PT_CAP_tnt_disable,
    PT_CAP_topa_output,
    PT_CAP_topa_multiple_entries,
    PT_CAP_single_range_output,
    PT_CAP_output_subsys,
    PT_CAP_payloads_lip,
    PT_CAP_num_address_ranges,
    PT_CAP_mtc_periods,
    PT_CAP_cycle_thresholds,
    PT_CAP_psb_periods,
}

/* C condition: defined(CONFIG_PERF_EVENTS) && defined(CONFIG_CPU_SUP_INTEL) */
#[cfg(all(feature = "CONFIG_PERF_EVENTS", feature = "CONFIG_CPU_SUP_INTEL"))]
extern "C" {
    pub fn cpu_emergency_stop_pt();
    pub fn intel_pt_validate_hw_cap(cap: pt_capabilities) -> u32;
    pub fn intel_pt_validate_cap(caps: *mut u32, cap: pt_capabilities) -> u32;
    pub fn is_intel_pt_event(event: *mut perf_event) -> i32;
}

/* Fallback definitions when CONFIG_PERF_EVENTS or CONFIG_CPU_SUP_INTEL is not enabled. */
#[cfg(not(all(feature = "CONFIG_PERF_EVENTS", feature = "CONFIG_CPU_SUP_INTEL")))]
#[inline]
pub fn cpu_emergency_stop_pt() {}

#[cfg(not(all(feature = "CONFIG_PERF_EVENTS", feature = "CONFIG_CPU_SUP_INTEL")))]
#[inline]
pub fn intel_pt_validate_hw_cap(_cap: pt_capabilities) -> u32 {
    0
}

#[cfg(not(all(feature = "CONFIG_PERF_EVENTS", feature = "CONFIG_CPU_SUP_INTEL")))]
#[inline]
pub fn intel_pt_validate_cap(_caps: *mut u32, _capability: pt_capabilities) -> u32 {
    0
}

#[cfg(not(all(feature = "CONFIG_PERF_EVENTS", feature = "CONFIG_CPU_SUP_INTEL")))]
#[inline]
pub fn is_intel_pt_event(_event: *mut perf_event) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
