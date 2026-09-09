// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Declarations supplied by the corresponding C headers are intentionally left
// as external dependencies of this translation unit.

pub unsafe extern "C" fn dml2_core_create(
    project_id: dml2_project_id,
    out: *mut dml2_core_instance,
) -> bool {
    let mut result = false;

    if out.is_null() {
        return false;
    }

    core::ptr::write_bytes(
        out as *mut u8,
        0,
        core::mem::size_of::<dml2_core_instance>(),
    );

    match project_id {
        dml2_project_id::dml2_project_dcn4x_stage1 => {
            result = false;
        }
        dml2_project_id::dml2_project_dcn4x_stage2
        | dml2_project_id::dml2_project_dcn4x_stage2_auto_drr_svp => {
            (*out).initialize = Some(core_dcn4_initialize);
            (*out).mode_support = Some(core_dcn4_mode_support);
            (*out).mode_programming = Some(core_dcn4_mode_programming);
            (*out).populate_informative = Some(core_dcn4_populate_informative);
            (*out).calculate_mcache_allocation = Some(core_dcn4_calculate_mcache_allocation);
            result = true;
        }
        dml2_project_id::dml2_project_dcn42 => {
            (*out).initialize = Some(core_dcn42_initialize);
            (*out).mode_support = Some(core_dcn4_mode_support);
            (*out).mode_programming = Some(core_dcn4_mode_programming);
            (*out).populate_informative = Some(core_dcn4_populate_informative);
            (*out).calculate_mcache_allocation = Some(core_dcn4_calculate_mcache_allocation);
            result = true;
        }
        dml2_project_id::dml2_project_dcn5x_utm => {
            (*out).initialize = Some(dml2_core_dcn5_funcs_initialize);
            (*out).validate_solution = Some(dml2_core_dcn5_funcs_validate_solution);
            (*out).populate_programming = Some(dml2_core_dcn5_funcs_populate_programming);
            (*out).populate_informative = Some(core_dcn4_populate_informative);
            (*out).calculate_mcache_allocation = Some(core_dcn4_calculate_mcache_allocation);
            result = true;
        }
        dml2_project_id::dml2_project_dcn6x_soc_var_a
        | dml2_project_id::dml2_project_dcn6x_soc_var_b => {
            (*out).initialize = Some(dml2_core_dcn6_funcs_initialize);
            (*out).validate_solution = Some(dml2_core_dcn6_funcs_validate_solution);
            (*out).populate_programming = Some(dml2_core_dcn6_funcs_populate_programming);
            (*out).populate_informative = Some(core_dcn4_populate_informative);
            (*out).calculate_mcache_allocation = Some(core_dcn4_calculate_mcache_allocation);
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_utm
        | dml2_project_id::dml2_project_dcn5x
        | dml2_project_id::dml2_project_invalid => {}
        _ => {}
    }

    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
