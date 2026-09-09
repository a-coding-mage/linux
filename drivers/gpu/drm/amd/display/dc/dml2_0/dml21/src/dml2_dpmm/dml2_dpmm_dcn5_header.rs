// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency translated from: dml2_internal_shared_types.h

extern "C" {
    pub fn dpmm_dcn5_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;

    pub fn dpmm_dcn5_map_watermarks(
        in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
    ) -> bool;

    pub fn dcn5_populate_pstate_support_in_programming(
        programming: *mut dml2_display_cfg_programming,
        utm_soc_bb: *const dml2_utm_soc_bb,
        solution: *const dml2_display_solution,
    );

    pub fn dcn5_populate_stutter_support_in_programming(
        programming: *mut dml2_display_cfg_programming,
        utm_soc_bb: *const dml2_utm_soc_bb,
        solution: *const dml2_display_solution,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
