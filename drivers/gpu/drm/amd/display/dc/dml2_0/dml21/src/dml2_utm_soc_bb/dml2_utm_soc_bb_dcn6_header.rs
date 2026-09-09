// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency supplied by the corresponding shared types header:
// dml2_internal_shared_types.h

extern "C" {
    pub fn dml2_utm_soc_bb_dcn6a_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        explicit_qos_model: *const utm_qos_model,
    ) -> bool;

    pub fn dml2_utm_soc_bb_dcn6b_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        explicit_qos_model: *const utm_qos_model,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
