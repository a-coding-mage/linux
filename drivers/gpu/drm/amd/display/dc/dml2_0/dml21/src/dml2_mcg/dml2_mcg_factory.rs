// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from:
// #include "dml2_mcg_factory.h"
// #include "dml2_mcg_dcn4.h"
// #include "dml2_mcg_dcn42.h"
// #include "dml2_external_lib_deps.h"

extern "C" {
    fn mcg_dcn4_build_min_clock_table(
        in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
    ) -> bool;
    fn mcg_dcn42_build_min_clock_table(
        in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
    ) -> bool;
}

unsafe extern "C" fn dummy_build_min_clock_table(
    in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
) -> bool {
    let _ = in_out;
    true
}

pub unsafe extern "C" fn dml2_mcg_create(
    project_id: dml2_project_id,
    out: *mut dml2_mcg_instance,
) -> bool {
    let mut result = false;

    if out.is_null() {
        return false;
    }

    core::ptr::write_bytes(
        out.cast::<u8>(),
        0,
        core::mem::size_of::<dml2_mcg_instance>(),
    );

    match project_id {
        dml2_project_id::dml2_project_dcn4x_stage1 => {
            (*out).build_min_clock_table = Some(dummy_build_min_clock_table);
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_stage2
        | dml2_project_id::dml2_project_dcn4x_stage2_auto_drr_svp => {
            (*out).build_min_clock_table = Some(mcg_dcn4_build_min_clock_table);
            result = true;
        }
        dml2_project_id::dml2_project_dcn42 => {
            (*out).build_min_clock_table = Some(mcg_dcn42_build_min_clock_table);
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_utm
        | dml2_project_id::dml2_project_dcn5x
        | dml2_project_id::dml2_project_dcn5x_utm
        | dml2_project_id::dml2_project_dcn6x_soc_var_a
        | dml2_project_id::dml2_project_dcn6x_soc_var_b
        | dml2_project_id::dml2_project_invalid => {}
        _ => {}
    }

    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
