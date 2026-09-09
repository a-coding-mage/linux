// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

#[repr(C)]
pub struct dml2_policy_parameters {
    pub odm_combine_dispclk_threshold_khz: ::core::ffi::c_ulong,
    pub max_immediate_flip_latency: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
