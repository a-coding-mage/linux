// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependency: dml2_internal_shared_types.h

extern "C" {
    pub fn dml2_utm_soc_bb_create(
        project_id: dml2_project_id,
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        qos_model: *const utm_qos_model,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
