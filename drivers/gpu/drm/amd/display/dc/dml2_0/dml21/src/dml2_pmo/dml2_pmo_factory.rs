// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit are intentionally
// referenced here but not reimplemented.

use core::{mem, ptr};

unsafe fn dummy_init_for_stutter(
    in_out: *mut dml2_pmo_init_for_stutter_in_out,
) -> bool {
    let _ = in_out;
    false
}

unsafe fn dummy_test_for_stutter(
    in_out: *mut dml2_pmo_test_for_stutter_in_out,
) -> bool {
    let _ = in_out;
    true
}

unsafe fn dummy_optimize_for_stutter(
    in_out: *mut dml2_pmo_optimize_for_stutter_in_out,
) -> bool {
    let _ = in_out;
    false
}

pub unsafe fn dml2_pmo_create(
    project_id: dml2_project_id,
    out: *mut dml2_pmo_instance,
) -> bool {
    let mut result = false;

    if out.is_null() {
        return false;
    }

    ptr::write_bytes(out as *mut u8, 0, mem::size_of::<dml2_pmo_instance>());

    match project_id {
        dml2_project_id::dml2_project_dcn4x_stage1 => {
            (*out).initialize = pmo_dcn4_fams2_initialize;
            (*out).optimize_dcc_mcache = pmo_dcn4_fams2_optimize_dcc_mcache;
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_stage2 => {
            (*out).initialize = pmo_dcn3_initialize;
            (*out).optimize_dcc_mcache = pmo_dcn3_optimize_dcc_mcache;
            (*out).init_for_vmin = pmo_dcn3_init_for_vmin;
            (*out).test_for_vmin = pmo_dcn3_test_for_vmin;
            (*out).optimize_for_vmin = pmo_dcn3_optimize_for_vmin;
            (*out).init_for_uclk_pstate = pmo_dcn3_init_for_pstate_support;
            (*out).test_for_uclk_pstate = pmo_dcn3_test_for_pstate_support;
            (*out).optimize_for_uclk_pstate = pmo_dcn3_optimize_for_pstate_support;
            (*out).init_for_stutter = dummy_init_for_stutter;
            (*out).test_for_stutter = dummy_test_for_stutter;
            (*out).optimize_for_stutter = dummy_optimize_for_stutter;
            result = true;
        }
        dml2_project_id::dml2_project_dcn42 => {
            (*out).initialize = pmo_dcn42_initialize;
            (*out).init_for_vmin = pmo_dcn4_fams2_init_for_vmin;
            (*out).test_for_vmin = pmo_dcn4_fams2_test_for_vmin;
            (*out).optimize_for_vmin = pmo_dcn4_fams2_optimize_for_vmin;
            (*out).init_for_uclk_pstate = pmo_dcn42_init_for_pstate_support;
            (*out).test_for_uclk_pstate = pmo_dcn42_test_for_pstate_support;
            (*out).optimize_for_uclk_pstate = pmo_dcn42_fams2_optimize_for_pstate_support;
            (*out).init_for_stutter = pmo_dcn4_fams2_init_for_stutter;
            (*out).test_for_stutter = pmo_dcn4_fams2_test_for_stutter;
            (*out).optimize_for_stutter = pmo_dcn4_fams2_optimize_for_stutter;
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_stage2_auto_drr_svp => {
            (*out).initialize = pmo_dcn4_fams2_initialize;
            (*out).optimize_dcc_mcache = pmo_dcn4_fams2_optimize_dcc_mcache;
            (*out).init_for_vmin = pmo_dcn4_fams2_init_for_vmin;
            (*out).test_for_vmin = pmo_dcn4_fams2_test_for_vmin;
            (*out).optimize_for_vmin = pmo_dcn4_fams2_optimize_for_vmin;
            (*out).init_for_uclk_pstate = pmo_dcn4_fams2_init_for_pstate_support;
            (*out).test_for_uclk_pstate = pmo_dcn4_fams2_test_for_pstate_support;
            (*out).optimize_for_uclk_pstate = pmo_dcn4_fams2_optimize_for_pstate_support;
            (*out).init_for_stutter = pmo_dcn4_fams2_init_for_stutter;
            (*out).test_for_stutter = pmo_dcn4_fams2_test_for_stutter;
            (*out).optimize_for_stutter = pmo_dcn4_fams2_optimize_for_stutter;
            result = true;
        }
        dml2_project_id::dml2_project_dcn5x_utm => {
            (*out).initialize = dml2_pmo_dcn5_initialize;
            (*out).get_ordered_mandatory_stage_optimizers = dml2_pmo_dcn5_get_ordered_mandatory_stage_optimizers;
            (*out).get_ordered_optional_stage_optimizers = dml2_pmo_dcn5_get_ordered_optional_stages_optimizers;
            (*out).initialize_worksheet = dml2_pmo_dcn5_initialize_worksheet;
            (*out).optional_sanity_check = dml2_pmo_dcn5_sanity_check;
            (*out).convert_worksheet_to_solution = dml2_pmo_dcn5_convert_worksheet_to_solution;
            (*out).clear_pre_validation_states = dml2_pmo_dcn5_clear_pre_validation_states;
            result = true;
        }
        dml2_project_id::dml2_project_dcn6x_soc_var_a => {
            (*out).initialize = dml2_pmo_dcn6a_initialize;
            (*out).get_ordered_mandatory_stage_optimizers = dml2_pmo_dcn6a_get_ordered_mandatory_stage_optimizers;
            (*out).get_ordered_optional_stage_optimizers = dml2_pmo_dcn6a_get_ordered_optional_stages_optimizers;
            (*out).initialize_worksheet = dml2_pmo_dcn5_initialize_worksheet;
            (*out).optional_sanity_check = dml2_pmo_dcn5_sanity_check;
            (*out).convert_worksheet_to_solution = dml2_pmo_dcn6_convert_worksheet_to_solution;
            (*out).clear_pre_validation_states = dml2_pmo_dcn5_clear_pre_validation_states;
            result = true;
        }
        dml2_project_id::dml2_project_dcn6x_soc_var_b => {
            (*out).initialize = dml2_pmo_dcn6b_initialize;
            (*out).get_ordered_mandatory_stage_optimizers = dml2_pmo_dcn6b_get_ordered_mandatory_stage_optimizers;
            (*out).get_ordered_optional_stage_optimizers = dml2_pmo_dcn6b_get_ordered_optional_stages_optimizers;
            (*out).initialize_worksheet = dml2_pmo_dcn5_initialize_worksheet;
            (*out).optional_sanity_check = dml2_pmo_dcn5_sanity_check;
            (*out).convert_worksheet_to_solution = dml2_pmo_dcn6_convert_worksheet_to_solution;
            (*out).clear_pre_validation_states = dml2_pmo_dcn5_clear_pre_validation_states;
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_utm
        | dml2_project_id::dml2_project_dcn5x
        | dml2_project_id::dml2_project_invalid => {}
    }

    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
