// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml2_core_factory.h.
//
// Dependencies supplied by the original included headers:
// - dml2_internal_shared_types.h
// - dml_top_types.h

extern "C" {
    pub fn dml2_core_create(
        project_id: dml2_project_id,
        out: *mut dml2_core_instance,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
