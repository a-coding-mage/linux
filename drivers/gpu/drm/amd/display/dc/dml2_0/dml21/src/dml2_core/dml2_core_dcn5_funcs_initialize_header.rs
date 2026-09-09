// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency corresponding to: #include "dml2_internal_shared_types.h"

extern "C" {
    pub fn dml2_core_dcn5_funcs_initialize(
        in_out: *mut dml2_core_initialize_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
