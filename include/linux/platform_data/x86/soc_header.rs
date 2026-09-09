/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Helpers for Intel SoC model detection
 *
 * Copyright (c) 2019, Intel Corporation.
 */

// The C header is active only when CONFIG_X86 is enabled.  This Rust
// translation uses the corresponding feature as the build-time condition.

#[cfg(feature = "CONFIG_X86")]
mod x86_enabled {
    // Supplied by the architecture/device-id dependencies of the original
    // header.  The X86_MATCH_VFM table entries are likewise external to this
    // file and are intentionally not reimplemented here.
    #[allow(dead_code)]
    pub(crate) enum X86CpuId {}

    // TODO: map each X86_MATCH_VFM(type, NULL) table to the target's native
    // x86_cpu_id representation when the dependent bindings are available.
    #[inline]
    pub(crate) fn soc_intel_is_byt() -> bool {
        unimplemented!("x86_cpu_id/X86_MATCH_VFM dependency")
    }

    #[inline]
    pub(crate) fn soc_intel_is_cht() -> bool {
        unimplemented!("x86_cpu_id/X86_MATCH_VFM dependency")
    }

    #[inline]
    pub(crate) fn soc_intel_is_apl() -> bool {
        unimplemented!("x86_cpu_id/X86_MATCH_VFM dependency")
    }

    #[inline]
    pub(crate) fn soc_intel_is_glk() -> bool {
        unimplemented!("x86_cpu_id/X86_MATCH_VFM dependency")
    }

    #[inline]
    pub(crate) fn soc_intel_is_cml() -> bool {
        unimplemented!("x86_cpu_id/X86_MATCH_VFM dependency")
    }
}

#[cfg(feature = "CONFIG_X86")]
pub(crate) use x86_enabled::{
    soc_intel_is_apl, soc_intel_is_byt, soc_intel_is_cht, soc_intel_is_cml,
    soc_intel_is_glk,
};

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub(crate) fn soc_intel_is_byt() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub(crate) fn soc_intel_is_cht() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub(crate) fn soc_intel_is_apl() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub(crate) fn soc_intel_is_glk() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub(crate) fn soc_intel_is_cml() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
