// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency: dml2_internal_shared_types.h

unsafe extern "C" {
    pub fn pmo_dcn3_initialize(
        in_out: *mut dml2_pmo_initialize_in_out,
    ) -> bool;

    pub fn pmo_dcn3_optimize_dcc_mcache(
        in_out: *mut dml2_pmo_optimize_dcc_mcache_in_out,
    ) -> bool;

    pub fn pmo_dcn3_init_for_vmin(
        in_out: *mut dml2_pmo_init_for_vmin_in_out,
    ) -> bool;
    pub fn pmo_dcn3_test_for_vmin(
        in_out: *mut dml2_pmo_test_for_vmin_in_out,
    ) -> bool;
    pub fn pmo_dcn3_optimize_for_vmin(
        in_out: *mut dml2_pmo_optimize_for_vmin_in_out,
    ) -> bool;

    pub fn pmo_dcn3_init_for_pstate_support(
        in_out: *mut dml2_pmo_init_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn3_test_for_pstate_support(
        in_out: *mut dml2_pmo_test_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn3_optimize_for_pstate_support(
        in_out: *mut dml2_pmo_optimize_for_pstate_support_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
