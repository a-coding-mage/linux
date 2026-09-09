// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by the corresponding internal shared types header:
// #include "dml2_internal_shared_types.h"

unsafe extern "C" {
    pub fn dml2_utm_soc_bb_dcn5_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        explicit_qos_model: *const utm_qos_model,
    ) -> bool;

    pub fn dml2_utm_soc_bb_dcn5_build_sop_table(
        table: *mut dml2_sop_table,
        utm_soc_bb: *const dml2_utm_soc_bb,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
