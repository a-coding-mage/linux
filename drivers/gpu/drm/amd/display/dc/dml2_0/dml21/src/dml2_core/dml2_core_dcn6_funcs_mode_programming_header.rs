// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

extern "C" {
    pub fn dml2_core_dcn6_funcs_populate_programming(
        core: *mut dml2_core_instance,
        solution: *const dml2_display_solution,
        programming: *mut dml2_display_cfg_programming,
    ) -> dml2_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
