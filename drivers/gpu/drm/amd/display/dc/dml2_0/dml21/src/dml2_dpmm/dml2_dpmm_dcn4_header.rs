// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

extern "C" {
    pub fn dpmm_dcn3_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;
    pub fn dpmm_dcn4_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;
    pub fn dpmm_dcn4_map_watermarks(
        in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
    ) -> bool;
    pub fn dpmm_dcn42_map_watermarks(
        in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
