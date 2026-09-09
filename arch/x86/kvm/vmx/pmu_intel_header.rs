/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/kvm_host.h>
// #include "cpuid.h"

/// Equivalent to the C `static inline` helper.
#[inline]
pub unsafe fn vcpu_get_perf_capabilities(vcpu: *mut kvm_vcpu) -> u64 {
    if !guest_cpu_cap_has(vcpu, X86_FEATURE_PDCM) {
        return 0;
    }

    (*vcpu).arch.perf_capabilities
}

/// Equivalent to the C `static inline` helper.
#[inline]
pub unsafe fn fw_writes_is_enabled(vcpu: *mut kvm_vcpu) -> bool {
    (vcpu_get_perf_capabilities(vcpu) & PERF_CAP_FW_WRITES) != 0
}

extern "C" {
    pub fn intel_pmu_lbr_is_enabled(vcpu: *mut kvm_vcpu) -> bool;
    pub fn intel_pmu_create_guest_lbr_event(vcpu: *mut kvm_vcpu) -> i32;
}

#[repr(C)]
pub struct lbr_desc {
    /* Basic info about guest LBR records. */
    pub records: x86_pmu_lbr,

    /*
     * Emulate LBR feature via passthrough LBR registers when the
     * per-vcpu guest LBR event is scheduled on the current pcpu.
     *
     * The records may be inaccurate if the host reclaims the LBR.
     */
    pub event: *mut perf_event,

    /* True if LBRs are marked as not intercepted in the MSR bitmap */
    pub msr_passthrough: bool,
}

extern "C" {
    pub static mut vmx_lbr_caps: x86_pmu_lbr;
}

// External types, functions, and constants referenced above are supplied by
// the translated kernel dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
