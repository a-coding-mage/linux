/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// Dependency intent preserved from: #include "clk_mgr.h"

extern "C" {
    pub fn dcn351_update_bw_bounding_box_fpu(
        dc: *mut dc,
        bw_params: *mut clk_bw_params,
    );

    pub fn dcn351_populate_dml_pipes_from_context_fpu(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        validate_mode: dc_validate_mode,
    ) -> ::core::ffi::c_int;

    pub fn dcn351_decide_zstate_support(
        dc: *mut dc,
        context: *mut dc_state,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
