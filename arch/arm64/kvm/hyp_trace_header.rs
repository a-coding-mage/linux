/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

// Corresponds to CONFIG_NVHE_EL2_TRACING.
#[cfg(CONFIG_NVHE_EL2_TRACING)]
unsafe extern "C" {
    pub fn kvm_hyp_trace_init() -> c_int;
}

#[cfg(not(CONFIG_NVHE_EL2_TRACING))]
pub(crate) fn kvm_hyp_trace_init() -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
