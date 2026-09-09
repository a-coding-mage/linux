// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency declarations supplied by the corresponding translated headers:
// dml2_internal_shared_types.h
// dml_top_types.h

extern "C" {
    pub fn dml2_pmo_create(
        project_id: dml2_project_id,
        out: *mut dml2_pmo_instance,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
