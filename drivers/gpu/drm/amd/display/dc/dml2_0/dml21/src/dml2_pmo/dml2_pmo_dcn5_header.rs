// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by the shared DML2 type definitions:
// #include "dml2_internal_shared_types.h"

extern "C" {
    pub fn dml2_pmo_dcn5_initialize(
        in_out: *mut dml2_pmo_initialize_in_out,
    ) -> bool;

    pub fn dml2_pmo_dcn5_get_ordered_mandatory_stage_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn5_get_ordered_optional_stages_optimizers(
        pmo: *mut dml2_pmo_instance,
        optimers: *mut *mut dml2_pmo_stage_optimizer,
    ) -> i32;

    pub fn dml2_pmo_dcn5_initialize_worksheet(
        pmo: *mut dml2_pmo_instance,
        dispcfg: *const dml2_display_cfg,
        worksheet: *mut dml2_optimization_worksheet,
    );

    pub fn dml2_pmo_dcn5_sanity_check(
        pmo: *mut dml2_pmo_instance,
        worksheet: *const dml2_optimization_worksheet,
    ) -> dml2_status;

    pub fn dml2_pmo_dcn5_convert_worksheet_to_solution(
        pmo: *mut dml2_pmo_instance,
        worksheet: *const dml2_optimization_worksheet,
        solution: *mut dml2_display_solution,
    );

    pub fn dml2_pmo_dcn5_clear_pre_validation_states(
        pmo: *mut dml2_pmo_instance,
        worksheet: *mut dml2_optimization_worksheet,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
