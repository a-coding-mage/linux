/* SPDX-License-Identifier: MIT */
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependency equivalent of: #include "dml2_internal_shared_types.h"

extern "C" {
    pub fn mcg_dcn42_build_min_clock_table(
        in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
