/* SPDX-License-Identifier: GPL-2.0-only */

pub const SPECTRE_UNAFFECTED: i32 = 0;
pub const SPECTRE_MITIGATED: i32 = 1;
pub const SPECTRE_VULNERABLE: i32 = 2;

pub const __SPECTRE_V2_METHOD_BPIALL: u32 = 0;
pub const __SPECTRE_V2_METHOD_ICIALLU: u32 = 1;
pub const __SPECTRE_V2_METHOD_SMC: u32 = 2;
pub const __SPECTRE_V2_METHOD_HVC: u32 = 3;
pub const __SPECTRE_V2_METHOD_LOOP8: u32 = 4;

pub const SPECTRE_V2_METHOD_BPIALL: u32 =
    1u32 << __SPECTRE_V2_METHOD_BPIALL;
pub const SPECTRE_V2_METHOD_ICIALLU: u32 =
    1u32 << __SPECTRE_V2_METHOD_ICIALLU;
pub const SPECTRE_V2_METHOD_SMC: u32 =
    1u32 << __SPECTRE_V2_METHOD_SMC;
pub const SPECTRE_V2_METHOD_HVC: u32 =
    1u32 << __SPECTRE_V2_METHOD_HVC;
pub const SPECTRE_V2_METHOD_LOOP8: u32 =
    1u32 << __SPECTRE_V2_METHOD_LOOP8;

// CONFIG_GENERIC_CPU_VULNERABILITIES selects the external implementation;
// otherwise the C header provides an empty static inline implementation.
#[cfg(feature = "CONFIG_GENERIC_CPU_VULNERABILITIES")]
pub unsafe extern "C" {
    pub fn spectre_v2_update_state(state: u32, methods: u32);
}

#[cfg(not(feature = "CONFIG_GENERIC_CPU_VULNERABILITIES"))]
#[inline]
pub fn spectre_v2_update_state(_state: u32, _methods: u32) {}

pub unsafe extern "C" {
    pub fn spectre_bhb_update_vectors(method: u32) -> i32;

    pub fn cpu_v7_ca8_ibe();
    pub fn cpu_v7_ca15_ibe();
    pub fn cpu_v7_bugs_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
