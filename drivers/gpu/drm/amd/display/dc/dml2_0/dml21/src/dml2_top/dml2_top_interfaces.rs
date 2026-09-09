// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies: dml_top.h, dml2_internal_shared_types.h,
// dml2_top_soc15.h, and dml2_top_utm.h.

unsafe extern "C" {
    fn dml2_top_soc15_initialize_instance(
        in_out: *mut dml2_initialize_instance_in_out,
    ) -> bool;
    fn dml2_top_utm_initialize_instance(
        in_out: *mut dml2_initialize_instance_in_out,
    ) -> bool;
}

pub unsafe fn dml2_get_instance_size_bytes() -> ::core::ffi::c_uint {
    ::core::mem::size_of::<dml2_instance>() as ::core::ffi::c_uint
}

pub unsafe fn dml2_initialize_instance(
    in_out: *mut dml2_initialize_instance_in_out,
) -> bool {
    match (*in_out).options.project_id {
        dml2_project_dcn4x_stage1
        | dml2_project_dcn4x_stage2
        | dml2_project_dcn4x_stage2_auto_drr_svp
        | dml2_project_dcn42 => dml2_top_soc15_initialize_instance(in_out),
        dml2_project_dcn5x_utm
        | dml2_project_dcn6x_soc_var_a
        | dml2_project_dcn6x_soc_var_b => dml2_top_utm_initialize_instance(in_out),
        dml2_project_dcn4x_utm
        | dml2_project_dcn5x
        | dml2_project_invalid => false,
        _ => false,
    }
}

pub unsafe fn dml2_check_mode_supported(
    in_out: *mut dml2_check_mode_supported_in_out,
) -> bool {
    let instance = (*in_out).dml2_instance;
    let function = (*instance).funcs.check_mode_supported;
    if function.is_none() {
        return false;
    }

    function.unwrap()(in_out)
}

pub unsafe fn dml2_build_mode_programming(
    in_out: *mut dml2_build_mode_programming_in_out,
) -> bool {
    let instance = (*in_out).dml2_instance;
    let function = (*instance).funcs.build_mode_programming;
    if function.is_none() {
        return false;
    }

    function.unwrap()(in_out)
}

pub unsafe fn dml2_build_mcache_programming(
    in_out: *mut dml2_build_mcache_programming_in_out,
) -> bool {
    let instance = (*in_out).dml2_instance;
    let function = (*instance).funcs.build_mcache_programming;
    if function.is_none() {
        return false;
    }

    function.unwrap()(in_out)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
