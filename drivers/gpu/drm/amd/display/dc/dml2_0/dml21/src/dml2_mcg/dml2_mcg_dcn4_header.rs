// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency intent: declarations from "dml2_internal_shared_types.h" are
// supplied by the surrounding Rust translation unit.

unsafe extern "C" {
    pub fn mcg_dcn4_build_min_clock_table(
        in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
    ) -> bool;

    pub fn mcg_dcn4_unit_test() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
