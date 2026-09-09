// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

extern "C" {
    pub fn dml2_top_utm_initialize_instance(
        in_out: *mut crate::dml2_initialize_instance_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
