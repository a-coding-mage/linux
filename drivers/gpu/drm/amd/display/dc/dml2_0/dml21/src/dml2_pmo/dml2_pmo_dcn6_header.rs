// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

#[repr(C)]
pub struct dml2_pmo_initialize_in_out {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pmo_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pmo_stage_optimizer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_optimization_worksheet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_display_solution {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dml2_pmo_dcn6a_initialize(
        in_out: *mut dml2_pmo_initialize_in_out,
    ) -> bool;

    pub fn dml2_pmo_dcn6a_get_ordered_mandatory_stage_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn6a_get_ordered_optional_stages_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn6b_initialize(
        in_out: *mut dml2_pmo_initialize_in_out,
    ) -> bool;

    pub fn dml2_pmo_dcn6b_get_ordered_mandatory_stage_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn6b_get_ordered_optional_stages_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn6_convert_worksheet_to_solution(
        pmo: *mut dml2_pmo_instance,
        worksheet: *const dml2_optimization_worksheet,
        solution: *mut dml2_display_solution,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
