// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019 Madhavan Srinivasan, IBM Corporation.

// The C __init annotation is a build/link-time initialization attribute.
unsafe extern "C" {
    pub fn init_ppc970_pmu() -> ::core::ffi::c_int;
    pub fn init_power5_pmu() -> ::core::ffi::c_int;
    pub fn init_power5p_pmu() -> ::core::ffi::c_int;
    pub fn init_power6_pmu() -> ::core::ffi::c_int;
    pub fn init_power7_pmu() -> ::core::ffi::c_int;
    pub fn init_power8_pmu() -> ::core::ffi::c_int;
    pub fn init_power9_pmu() -> ::core::ffi::c_int;
    pub fn init_power10_pmu() -> ::core::ffi::c_int;
    pub fn init_power11_pmu() -> ::core::ffi::c_int;
    pub fn init_power12_pmu() -> ::core::ffi::c_int;
    pub fn init_generic_compat_pmu() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
