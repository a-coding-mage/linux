/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "hw_sequencer_private.h"
use crate::{
    controller_dp_color_space, controller_dp_test_pattern, dc, dc_color_depth,
    dc_crtc_timing, dc_crtc_timing_adjust, dc_cursor_attributes, dc_link,
    dc_log_buffer_ctx, dc_plane_state, dc_state, dc_status, dc_stream_state,
    dc_underflow_debug_data, dc_writeback_info, enum_clock_source_id, fva_adj,
    pipe_ctx, program_gamut_remap_params, set_output_transfer_func_params,
    surface_pixel_format, tg_color, timing_generator,
};

extern "C" {
    pub fn dcn30_init_hw(dc: *mut dc);

    pub fn dcn30_program_all_writeback_pipes_in_tree(
        dc: *mut dc,
        stream: *const dc_stream_state,
        context: *mut dc_state,
    );

    pub fn dcn30_update_writeback(
        dc: *mut dc,
        wb_info: *mut dc_writeback_info,
        context: *mut dc_state,
    );

    pub fn dcn30_enable_writeback(
        dc: *mut dc,
        wb_info: *mut dc_writeback_info,
        context: *mut dc_state,
    );

    pub fn dcn30_disable_writeback(dc: *mut dc, dwb_pipe_inst: ::core::ffi::c_uint);

    pub fn dcn30_mmhubbub_warmup(
        dc: *mut dc,
        num_dwb: ::core::ffi::c_uint,
        wb_info: *mut dc_writeback_info,
    ) -> bool;

    pub fn dcn30_log_color_state(dc: *mut dc, log_ctx: *mut dc_log_buffer_ctx);

    pub fn dcn30_set_blend_lut(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;

    pub fn dcn30_set_input_transfer_func(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;

    pub fn dcn30_program_gamut_remap(params: *mut program_gamut_remap_params);

    pub fn dcn30_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
    pub fn dcn30_set_avmute(pipe_ctx: *mut pipe_ctx, enable: bool);
    pub fn dcn30_update_info_frame(pipe_ctx: *mut pipe_ctx);
    pub fn dcn30_program_dmdata_engine(pipe_ctx: *mut pipe_ctx);

    pub fn dcn30_setup_hdmi_frl_link(
        link: *mut dc_link,
        hpo_inst: ::core::ffi::c_int,
        frl_phy_clock_source_id: enum_clock_source_id,
    ) -> dc_status;

    pub fn dcn30_hw_set_fva_vrr_adj(
        dc: *mut dc,
        pipe_ctx: *mut *mut pipe_ctx,
        num_pipes: ::core::ffi::c_int,
        fva_adj: *mut fva_adj,
        vrr_adj: *mut dc_crtc_timing_adjust,
    );

    pub fn dcn30_hw_get_max_fva_factor(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        timing: *mut dc_crtc_timing,
        max_pixel_clock: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn dcn30_does_plane_fit_in_mall(
        dc: *mut dc,
        pitch: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
        format: surface_pixel_format,
        cursor_attr: *mut dc_cursor_attributes,
    ) -> bool;

    pub fn dcn30_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;

    pub fn dcn30_hardware_release(dc: *mut dc);

    pub fn dcn30_set_disp_pattern_generator(
        dc: *const dc,
        pipe_ctx: *mut pipe_ctx,
        test_pattern: controller_dp_test_pattern,
        color_space: controller_dp_color_space,
        color_depth: dc_color_depth,
        solid_color: *const tg_color,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );

    pub fn dcn30_set_hubp_blank(
        dc: *const dc,
        pipe_ctx: *mut pipe_ctx,
        blank_enable: bool,
    );

    pub fn dcn30_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dcn30_wait_for_all_pending_updates(pipe_ctx: *const pipe_ctx);

    pub fn dcn30_get_underflow_debug_data(
        dc: *const dc,
        tg: *mut timing_generator,
        out_data: *mut dc_underflow_debug_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
