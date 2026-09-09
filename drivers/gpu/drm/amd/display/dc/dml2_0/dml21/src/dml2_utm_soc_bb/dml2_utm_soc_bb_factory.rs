// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by:
// #include "dml2_utm_soc_bb_factory.h"
// #include "dml2_utm_soc_bb_dcn5.h"
// #include "dml2_utm_soc_bb_dcn6.h"

unsafe extern "C" {
    fn dml2_utm_soc_bb_dcn5_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        qos_model: *const utm_qos_model,
    ) -> bool;
    fn dml2_utm_soc_bb_dcn6a_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        qos_model: *const utm_qos_model,
    ) -> bool;
    fn dml2_utm_soc_bb_dcn6b_create(
        utm_soc_bb: *mut dml2_utm_soc_bb,
        soc_bb: *const dml2_soc_bb,
        qos_model: *const utm_qos_model,
    ) -> bool;
}

pub unsafe fn dml2_utm_soc_bb_create(
    project_id: dml2_project_id,
    utm_soc_bb: *mut dml2_utm_soc_bb,
    soc_bb: *const dml2_soc_bb,
    qos_model: *const utm_qos_model,
) -> bool {
    match project_id {
        dml2_project_id::dml2_project_dcn5x_utm => unsafe {
            dml2_utm_soc_bb_dcn5_create(utm_soc_bb, soc_bb, qos_model)
        },
        dml2_project_id::dml2_project_dcn6x_soc_var_a => unsafe {
            dml2_utm_soc_bb_dcn6a_create(utm_soc_bb, soc_bb, qos_model)
        },
        dml2_project_id::dml2_project_dcn6x_soc_var_b => unsafe {
            dml2_utm_soc_bb_dcn6b_create(utm_soc_bb, soc_bb, qos_model)
        },
        dml2_project_id::dml2_project_dcn4x_utm
        | dml2_project_id::dml2_project_dcn5x
        | dml2_project_id::dml2_project_dcn4x_stage1
        | dml2_project_id::dml2_project_dcn42
        | dml2_project_id::dml2_project_dcn4x_stage2
        | dml2_project_id::dml2_project_dcn4x_stage2_auto_drr_svp
        | dml2_project_id::dml2_project_invalid => false,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
