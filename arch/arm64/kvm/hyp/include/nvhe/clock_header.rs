/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations correspond to the Linux and KVM hyp headers.

#[cfg(feature = "CONFIG_NVHE_EL2_TRACING")]
extern "C" {
    pub fn trace_hyp_clock_update(mult: u32, shift: u32, epoch_ns: u64, epoch_cyc: u64);
    pub fn trace_hyp_clock() -> u64;
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline]
pub fn trace_hyp_clock_update(_mult: u32, _shift: u32, _epoch_ns: u64, _epoch_cyc: u64) {}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline]
pub fn trace_hyp_clock() -> u64 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
