/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies: core_types.h and hw_sequencer_private.h.

pub enum dc {}
pub enum dc_state {}
pub enum dm_pp_display_configuration {}

extern "C" {
    pub fn dce110_hw_sequencer_construct(dc: *mut dc);

    pub fn dce110_apply_ctx_to_hw(
        dc: *mut dc,
        context: *mut dc_state,
    ) -> dc_status;

    pub fn dce110_apply_single_controller_ctx_to_hw(
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
        dc: *mut dc,
    ) -> dc_status;

    pub fn dce110_enable_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dce110_disable_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dce110_unblank_stream(
        pipe_ctx: *mut pipe_ctx,
        link_settings: *mut dc_link_settings,
    );
    pub fn dce110_blank_stream(pipe_ctx: *mut pipe_ctx);

    pub fn dce110_enable_audio_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dce110_disable_audio_stream(pipe_ctx: *mut pipe_ctx);
    pub fn dce110_update_info_frame(pipe_ctx: *mut pipe_ctx);

    pub fn dce110_set_avmute(pipe_ctx: *mut pipe_ctx, enable: bool);
    pub fn dce110_enable_accelerated_mode(dc: *mut dc, context: *mut dc_state);
    pub fn dce110_power_down(dc: *mut dc);

    pub fn dce110_set_safe_displaymarks(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
    );

    pub fn dce110_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dce110_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);

    pub fn dce110_edp_power_control(link: *mut dc_link, power_up: bool);
    pub fn dce110_edp_backlight_control(link: *mut dc_link, enable: bool);
    pub fn dce110_edp_wait_for_hpd_ready(link: *mut dc_link, power_up: bool);

    pub fn dce110_set_backlight_level(
        pipe_ctx: *mut pipe_ctx,
        params: *mut set_backlight_level_params,
    ) -> bool;
    pub fn dce110_set_abm_immediate_disable(pipe_ctx: *mut pipe_ctx);
    pub fn dce110_set_pipe(pipe_ctx: *mut pipe_ctx);

    pub fn dce110_disable_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );
    pub fn dce110_enable_lvds_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        clock_source: clock_source_id,
        pixel_clock: u32,
    );
    pub fn dce110_enable_tmds_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
        clock_source: clock_source_id,
        color_depth: dc_color_depth,
        pixel_clock: u32,
    );
    pub fn dce110_enable_dp_link_output(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
        clock_source: clock_source_id,
        link_settings: *const dc_link_settings,
    );

    pub fn build_audio_output(
        state: *mut dc_state,
        pipe_ctx: *const pipe_ctx,
        audio_output: *mut audio_output,
    );
    pub fn translate_to_dto_source(crtc_id: controller_id) -> audio_dto_source;
    pub fn populate_audio_dp_link_info(
        pipe_ctx: *const pipe_ctx,
        dp_link_info: *mut audio_dp_link_info,
    );
    pub fn enable_fbc(dc: *mut dc, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
