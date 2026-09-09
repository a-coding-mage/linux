// SPDX-License-Identifier: MIT
//
// Copyright 2024-2025 Advanced Micro Devices, Inc.

// Dependency: dml2_internal_shared_types.h

extern "C" {
    pub fn dml2_core_dcn5_funcs_populate_programming(
        core: *mut crate::dml2_core_instance,
        solution: *const crate::dml2_display_solution,
        programming: *mut crate::dml2_display_cfg_programming,
    ) -> crate::dml2_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
