/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations use the equivalent Rust types and ABI supplied by
// the surrounding kernel translation. The original <linux/types.h> is not included.

#[cfg(CONFIG_INTEL_TDX_GUEST)]
extern "C" {
    pub fn early_tdx_detect();
}

#[cfg(not(CONFIG_INTEL_TDX_GUEST))]
#[inline]
pub fn early_tdx_detect() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
