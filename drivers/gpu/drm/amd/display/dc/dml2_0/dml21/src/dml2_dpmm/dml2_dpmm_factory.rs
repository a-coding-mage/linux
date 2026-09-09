// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Declarations supplied by the corresponding DML2 headers are intentionally
// left as external dependencies.

extern "C" {
    fn dpmm_dcn3_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;
    fn dpmm_dcn4_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;
    fn dpmm_dcn4_map_watermarks(
        in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
    ) -> bool;
    fn dpmm_dcn42_map_watermarks(
        in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
    ) -> bool;
    fn dpmm_dcn5_map_mode_to_soc_dpm(
        in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
    ) -> bool;
}

unsafe extern "C" fn dummy_map_mode_to_soc_dpm(
    _in_out: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out,
) -> bool {
    true
}

unsafe extern "C" fn dummy_map_watermarks(
    _in_out: *mut dml2_dpmm_map_watermarks_params_in_out,
) -> bool {
    true
}

pub unsafe extern "C" fn dml2_dpmm_create(
    project_id: dml2_project_id,
    out: *mut dml2_dpmm_instance,
) -> bool {
    let mut result = false;

    if out.is_null() {
        return false;
    }

    core::ptr::write_bytes(
        out as *mut u8,
        0,
        core::mem::size_of::<dml2_dpmm_instance>(),
    );

    match project_id {
        dml2_project_id::dml2_project_dcn4x_stage1 => {
            (*out).map_mode_to_soc_dpm = Some(dummy_map_mode_to_soc_dpm);
            (*out).map_watermarks = Some(dummy_map_watermarks);
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_stage2 => {
            (*out).map_mode_to_soc_dpm = Some(dpmm_dcn3_map_mode_to_soc_dpm);
            (*out).map_watermarks = Some(dummy_map_watermarks);
            result = true;
        }
        dml2_project_id::dml2_project_dcn4x_stage2_auto_drr_svp => {
            (*out).map_mode_to_soc_dpm = Some(dpmm_dcn4_map_mode_to_soc_dpm);
            (*out).map_watermarks = Some(dpmm_dcn4_map_watermarks);
            result = true;
        }
        dml2_project_id::dml2_project_dcn42 => {
            (*out).map_mode_to_soc_dpm = Some(dpmm_dcn4_map_mode_to_soc_dpm);
            (*out).map_watermarks = Some(dpmm_dcn42_map_watermarks);
            result = true;
        }
        dml2_project_id::dml2_project_dcn5x_utm => {
            (*out).map_mode_to_soc_dpm = Some(dpmm_dcn5_map_mode_to_soc_dpm);
            result = true;
        }
        dml2_project_id::dml2_project_dcn6x_soc_var_a
        | dml2_project_id::dml2_project_dcn6x_soc_var_b => {
            // dpmm is deprecated
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
