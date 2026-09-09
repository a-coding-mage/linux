/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit.

pub const __DML2_WRAPPER_MAX_STREAMS_PLANES__: usize = 6;

#[repr(C)]
pub struct dml2_wrapper_optimize_configuration_params {
    pub dml_core_ctx: *mut display_mode_lib_st,
    pub config: *mut dml2_configuration_options,
    pub ip_params: *mut ip_params_st,
    pub cur_display_config: *mut dml_display_cfg_st,
    pub new_display_config: *mut dml_display_cfg_st,
    pub cur_mode_support_info: *const dml_mode_support_info_st,
    pub cur_policy: *mut dml_mode_eval_policy_st,
    pub new_policy: *mut dml_mode_eval_policy_st,
}

#[repr(C)]
pub struct dml2_calculate_lowest_supported_state_for_temp_read_scratch {
    pub evaluation_info: dml_mode_support_info_st,
    pub uclk_change_latencies: [dml_float_t; __DML_MAX_STATE_ARRAY_SIZE__],
    pub cur_display_config: dml_display_cfg_st,
    pub new_display_config: dml_display_cfg_st,
    pub new_policy: dml_mode_eval_policy_st,
    pub cur_policy: dml_mode_eval_policy_st,
}

#[repr(C)]
pub struct dml2_create_scratch {
    pub build_synthetic_socbb_scratch: dml2_policy_build_synthetic_soc_states_scratch,
    pub in_states: soc_states_st,
}

#[repr(C)]
pub struct dml2_calculate_rq_and_dlg_params_scratch {
    pub rq_regs: _vcs_dpi_dml_display_rq_regs_st,
    pub disp_dlg_regs: _vcs_dpi_dml_display_dlg_regs_st,
    pub disp_ttu_regs: _vcs_dpi_dml_display_ttu_regs_st,
}

#[repr(C)]
pub struct dml2_dml_to_dc_pipe_mapping {
    pub disp_cfg_to_stream_id: [u32; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_stream_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_plane_id: [u32; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_plane_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_stream_id: [u32; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_stream_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_id: [u32; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_index: [u32; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_index_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
}

#[repr(C)]
pub struct dml2_wrapper_scratch {
    pub cur_display_config: dml_display_cfg_st,
    pub new_display_config: dml_display_cfg_st,
    pub new_policy: dml_mode_eval_policy_st,
    pub cur_policy: dml_mode_eval_policy_st,
    pub mode_support_info: dml_mode_support_info_st,
    pub mode_support_params: dml_mode_support_ex_params_st,
    pub dummy_pstate_table: [dummy_pstate_entry; 4],
    pub create_scratch: dml2_create_scratch,
    pub dml2_calculate_lowest_supported_state_for_temp_read_scratch: dml2_calculate_lowest_supported_state_for_temp_read_scratch,
    pub calculate_rq_and_dlg_params_scratch: dml2_calculate_rq_and_dlg_params_scratch,
    pub optimize_configuration_params: dml2_wrapper_optimize_configuration_params,
    pub build_synthetic_socbb_params: dml2_policy_build_synthetic_soc_states_params,
    pub dml_to_dc_pipe_mapping: dml2_dml_to_dc_pipe_mapping,
    pub enable_flexible_pipe_mapping: bool,
    pub plane_duplicate_exists: bool,
    pub hpo_stream_to_link_encoder_mapping: [i32; MAX_HPO_DP2_ENCODERS],
}

#[repr(C)]
pub struct dml2_helper_det_policy_scratch {
    pub dpps_per_surface: [i32; MAX_PLANES],
}

#[repr(i32)]
pub enum dml2_architecture {
    dml2_architecture_20,
    dml2_architecture_21,
}

#[repr(C)]
pub struct prepare_mcache_programming_locals {
    pub build_mcache_programming_params: dml2_build_mcache_programming_in_out,
}

#[repr(C)]
pub struct dml21_wrapper_scratch {
    pub prepare_mcache_locals: prepare_mcache_programming_locals,
    pub temp_pipe: pipe_ctx,
}

#[repr(C)]
pub struct dml2_pipe_combine_factor {
    pub source: u32,
    pub target: u32,
}

#[repr(C)]
pub struct dml2_pipe_combine_scratch {
    pub odm_factors: [dml2_pipe_combine_factor; MAX_PIPES],
    pub mpc_factors: [[dml2_pipe_combine_factor; MAX_PIPES]; MAX_PIPES],
}

#[repr(C)]
pub struct dml2_context_v20 {
    pub dml_core_ctx: display_mode_lib_st,
    pub scratch: dml2_wrapper_scratch,
    pub g6_temp_read_watermark_set: dcn_watermarks,
}

#[repr(C)]
pub struct dml2_context_v21 {
    pub scratch: dml21_wrapper_scratch,
    pub dml_init: dml2_initialize_instance_in_out,
    pub display_config: dml2_display_cfg,
    pub mode_support: dml2_check_mode_supported_in_out,
    pub mode_programming: dml2_build_mode_programming_in_out,
    pub dml_to_dc_pipe_mapping: dml2_dml_to_dc_pipe_mapping,
}

#[repr(C)]
pub union dml2_context_variant {
    pub v20: dml2_context_v20,
    pub v21: dml2_context_v21,
}

#[repr(C)]
pub struct dml2_context {
    pub architecture: dml2_architecture,
    pub config: dml2_configuration_options,
    pub det_helper_scratch: dml2_helper_det_policy_scratch,
    pub pipe_combine_scratch: dml2_pipe_combine_scratch,
    pub variant: dml2_context_variant,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
