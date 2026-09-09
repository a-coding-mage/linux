/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependency intent: declarations from "dml2_internal_shared_types.h" are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct dml2_pmo_initialize_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pmo_test_for_pstate_support_in_out {
    _private: [u8; 0],
}

extern "C" {
    pub fn pmo_dcn42_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool;
    pub fn pmo_dcn42_init_for_pstate_support(
        in_out: *mut dml2_pmo_init_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn42_fams2_optimize_for_pstate_support(
        in_out: *mut dml2_pmo_optimize_for_pstate_support_in_out,
    ) -> bool;
    pub fn pmo_dcn42_test_for_pstate_support(
        in_out: *mut dml2_pmo_test_for_pstate_support_in_out,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
