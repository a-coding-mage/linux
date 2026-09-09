// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Translated from dml2_cga_factory.c. Declarations supplied by the included
// headers are expected to be available from the surrounding crate.

unsafe extern "C" {
    fn cga_dcn6_create(adjuster: *mut dml2_clock_granularity_adjuster);
}

pub unsafe fn dml2_cga_create(
    project_id: dml2_project_id,
    adjuster: *mut dml2_clock_granularity_adjuster,
) -> bool {
    let mut result = false;

    if adjuster.is_null() {
        return false;
    }

    core::ptr::write_bytes(
        adjuster.cast::<u8>(),
        0,
        core::mem::size_of::<dml2_clock_granularity_adjuster>(),
    );

    match project_id {
        dml2_project_dcn4x_stage1
        | dml2_project_dcn42
        | dml2_project_dcn4x_stage2
        | dml2_project_dcn4x_stage2_auto_drr_svp
        | dml2_project_dcn4x_utm
        | dml2_project_dcn5x
        | dml2_project_dcn5x_utm => {
            core::ptr::write_bytes(
                adjuster.cast::<u8>(),
                0,
                core::mem::size_of::<dml2_clock_granularity_adjuster>(),
            );
            result = true;
        }
        dml2_project_dcn6x_soc_var_a | dml2_project_dcn6x_soc_var_b => {
            cga_dcn6_create(adjuster);
            result = true;
        }
        dml2_project_invalid => {}
        _ => {}
    }

    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
