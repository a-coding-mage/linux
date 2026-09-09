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
 */

// Dependencies supplied by the surrounding translation unit:
// os_types.h, dml2_dc_types.h

#[repr(C)]
pub struct dc { _private: [u8; 0] }
#[repr(C)]
pub struct dml_timing_cfg_st { _private: [u8; 0] }
#[repr(C)]
pub struct dml2_dcn_clocks { _private: [u8; 0] }
#[repr(C)]
pub struct dc_state { _private: [u8; 0] }
#[repr(C)]
pub struct dml_plane_cfg_st { _private: [u8; 0] }
#[repr(C)]
pub struct dml_surface_cfg_st { _private: [u8; 0] }
#[repr(C)]
pub struct dml_output_cfg_st { _private: [u8; 0] }
#[repr(C)]
pub struct dcn_watermarks { _private: [u8; 0] }
#[repr(C)]
pub struct display_mode_lib_st { _private: [u8; 0] }
#[repr(C)]
pub struct dml2_context { _private: [u8; 0] }
#[repr(C)]
pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)]
pub struct dml_mode_support_info_st { _private: [u8; 0] }
#[repr(C)]
pub struct resource_context { _private: [u8; 0] }
#[repr(C)]
pub struct dml2_helper_det_policy_scratch { _private: [u8; 0] }
#[repr(C)]
pub struct dml_display_cfg_st { _private: [u8; 0] }

// External enum and type definitions are supplied by dml2_dc_types.h.
pub type dml_output_encoder_class = i32;
pub type mmhubbub_wbif_mode = i32;
pub type dc_status = i32;
pub type display_pipe_params_st = u8;

extern "C" {
    pub fn dml2_util_copy_dml_timing(
        dml_timing_array: *mut dml_timing_cfg_st,
        dst_index: u32,
        src_index: u32,
    );
    pub fn dml2_util_copy_dml_plane(
        dml_plane_array: *mut dml_plane_cfg_st,
        dst_index: u32,
        src_index: u32,
    );
    pub fn dml2_util_copy_dml_surface(
        dml_surface_array: *mut dml_surface_cfg_st,
        dst_index: u32,
        src_index: u32,
    );
    pub fn dml2_util_copy_dml_output(
        dml_output_array: *mut dml_output_cfg_st,
        dst_index: u32,
        src_index: u32,
    );
    pub fn dml2_util_get_maximum_odm_combine_for_output(
        force_odm_4to1: bool,
        encoder: dml_output_encoder_class,
        dsc_enabled: bool,
    ) -> u32;
    pub fn dml2_copy_clocks_to_dc_state(out_clks: *mut dml2_dcn_clocks, context: *mut dc_state);
    pub fn dml2_extract_watermark_set(watermark: *mut dcn_watermarks, dml_core_ctx: *mut display_mode_lib_st);
    pub fn dml2_extract_writeback_wm(context: *mut dc_state, dml_core_ctx: *mut display_mode_lib_st);
    pub fn dml2_helper_find_dml_pipe_idx_by_stream_id(ctx: *mut dml2_context, stream_id: u32) -> i32;
    pub fn is_dtbclk_required(dc: *const dc, context: *mut dc_state) -> bool;
    pub fn dml2_is_stereo_timing(stream: *const dc_stream_state) -> bool;
    pub fn dml2_calc_max_scaled_time(time_per_pixel: u32, mode: mmhubbub_wbif_mode, urgent_watermark: u32) -> u32;

    pub fn dml2_dc_construct_pipes(
        context: *mut dc_state,
        dml_mode_support_st: *mut dml_mode_support_info_st,
        out_hw_context: *mut resource_context,
    );
    pub fn dml2_predict_pipe_split(context: *mut dc_state, pipe: display_pipe_params_st, index: i32) -> bool;
    pub fn dml2_build_mapped_resource(
        dc: *const dc,
        context: *mut dc_state,
        stream: *mut dc_stream_state,
    ) -> dc_status;
    pub fn dml2_calculate_rq_and_dlg_params(
        dc: *const dc,
        context: *mut dc_state,
        out_new_hw_state: *mut resource_context,
        in_ctx: *mut dml2_context,
        pipe_cnt: u32,
    );
    pub fn dml2_apply_det_buffer_allocation_policy(in_ctx: *mut dml2_context, dml_dispcfg: *mut dml_display_cfg_st);
    pub fn dml2_verify_det_buffer_configuration(
        in_ctx: *mut dml2_context,
        display_state: *mut dc_state,
        det_scratch: *mut dml2_helper_det_policy_scratch,
    ) -> bool;
    pub fn dml2_initialize_det_scratch(in_ctx: *mut dml2_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
