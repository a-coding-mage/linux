// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency provided by dml2_internal_shared_types.h.

extern "C" {
    pub fn dml2_cga_create(
        project_id: dml2_project_id,
        adjuster: *mut dml2_clock_granularity_adjuster,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
