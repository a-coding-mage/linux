// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

#[repr(C)]
pub struct dml2_pmo_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pmo_stage_optimizer {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dml2_pmo_dcn6_stage_optimizer_uclk_pstate_create(
        pmo: *mut dml2_pmo_instance,
        stage: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn6_stage_optimizer_vmin_dcfclk_create(
        pmo: *mut dml2_pmo_instance,
        stage: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn6_stage_optimizer_mcache_create(
        pmo: *mut dml2_pmo_instance,
        stage: *mut dml2_pmo_stage_optimizer,
    );
    pub fn dml2_pmo_dcn6_stage_optimizer_fclk_ppt_temp_read_pstate_create(
        pmo: *mut dml2_pmo_instance,
        stage: *mut dml2_pmo_stage_optimizer,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
