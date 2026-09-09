/*
 * Copyright 2016-2020 Advanced Micro Devices, Inc.
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

// Translated from dcn10_hwseq.h. Dependencies are supplied by other modules.

extern "C" {
    pub fn dcn10_hw_sequencer_construct(dc: *mut crate::dc);
    pub fn dcn10_get_vupdate_offset_from_vsync(pipe_ctx: *mut crate::pipe_ctx) -> ::core::ffi::c_int;
    pub fn dcn10_calc_vupdate_position(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, start_line: *mut u32, end_line: *mut u32);
    pub fn dcn10_setup_vupdate_interrupt(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_enable_stream_timing(pipe_ctx: *mut crate::pipe_ctx, context: *mut crate::dc_state, dc: *mut crate::dc) -> crate::dc_status;
    pub fn dcn10_optimize_bandwidth(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_prepare_bandwidth(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_wait_for_pipe_update_if_needed(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, is_surface_update_only: bool);
    pub fn dcn10_set_wait_for_update_needed_for_pipe(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_pipe_control_lock(dc: *mut crate::dc, pipe: *mut crate::pipe_ctx, lock: bool);
    pub fn dcn10_cursor_lock(dc: *mut crate::dc, pipe: *mut crate::pipe_ctx, lock: bool);
    pub fn dcn10_blank_pixel_data(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, blank: bool);
    pub fn dcn10_unblank_stream(pipe_ctx: *mut crate::pipe_ctx, link_settings: *mut crate::dc_link_settings);
    pub fn dcn10_program_output_csc(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, colorspace: crate::dc_color_space, matrix: *mut u16, opp_id: ::core::ffi::c_int);
    pub fn dcn10_set_output_transfer_func(params: *mut crate::set_output_transfer_func_params) -> bool;
    pub fn dcn10_set_input_transfer_func(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, plane_state: *const crate::dc_plane_state) -> bool;
    pub fn dcn10_update_plane_addr(dc: *const crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_update_mpcc(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_reset_hw_ctx_wrap(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_disable_plane(dc: *mut crate::dc, state: *mut crate::dc_state, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_lock_all_pipes(dc: *mut crate::dc, context: *mut crate::dc_state, lock: bool);
    pub fn dcn10_post_unlock_program_front_end(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_hubp_pg_control(hws: *mut crate::dce_hwseq, hubp_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn dcn10_dpp_pg_control(hws: *mut crate::dce_hwseq, dpp_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn dcn10_enable_power_gating_plane(hws: *mut crate::dce_hwseq, enable: bool);
    pub fn dcn10_plane_atomic_disable(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_disable_vga(hws: *mut crate::dce_hwseq);
    pub fn dcn10_program_pipe(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, context: *mut crate::dc_state);
    pub fn dcn10_program_gamut_remap(params: *mut crate::program_gamut_remap_params);
    pub fn dcn10_init_hw(dc: *mut crate::dc);
    pub fn dcn10_init_pipes(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_power_down_on_boot(dc: *mut crate::dc);
    pub fn dce110_apply_ctx_to_hw(dc: *mut crate::dc, context: *mut crate::dc_state) -> crate::dc_status;
    pub fn dcn10_plane_atomic_disconnect(dc: *mut crate::dc, state: *mut crate::dc_state, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_update_dchub(hws: *mut crate::dce_hwseq, dh_data: *mut crate::dchub_init_data);
    pub fn dcn10_update_pending_status(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dce110_power_down(dc: *mut crate::dc);
    pub fn dce110_enable_accelerated_mode(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_enable_timing_synchronization(dc: *mut crate::dc, state: *mut crate::dc_state, group_index: ::core::ffi::c_int, group_size: ::core::ffi::c_int, grouped_pipes: *mut *mut crate::pipe_ctx);
    pub fn dcn10_enable_vblanks_synchronization(dc: *mut crate::dc, group_index: ::core::ffi::c_int, group_size: ::core::ffi::c_int, grouped_pipes: *mut *mut crate::pipe_ctx);
    pub fn dcn10_enable_per_frame_crtc_position_reset(dc: *mut crate::dc, group_size: ::core::ffi::c_int, grouped_pipes: *mut *mut crate::pipe_ctx);
    pub fn dce110_update_info_frame(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_send_immediate_sdp_message(pipe_ctx: *mut crate::pipe_ctx, custom_sdp_message: *const u8, sdp_message_size: ::core::ffi::c_uint);
    pub fn dce110_blank_stream(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dce110_enable_audio_stream(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dce110_disable_audio_stream(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_dummy_display_power_gating(dc: *mut crate::dc, controller_id: u8, dcb: *mut crate::dc_bios, power_gating: crate::pipe_gating_control) -> bool;
    pub fn dcn10_set_drr(pipe_ctx: *mut *mut crate::pipe_ctx, num_pipes: ::core::ffi::c_int, adjust: crate::dc_crtc_timing_adjust);
    pub fn dcn10_get_position(pipe_ctx: *mut *mut crate::pipe_ctx, num_pipes: ::core::ffi::c_int, position: *mut crate::crtc_position);
    pub fn dcn10_set_static_screen_control(pipe_ctx: *mut *mut crate::pipe_ctx, num_pipes: ::core::ffi::c_int, params: *const crate::dc_static_screen_params);
    pub fn dcn10_setup_stereo(pipe_ctx: *mut crate::pipe_ctx, dc: *mut crate::dc);
    pub fn dce110_set_avmute(pipe_ctx: *mut crate::pipe_ctx, enable: bool);
    pub fn dcn10_log_hw_state(dc: *mut crate::dc, log_ctx: *mut crate::dc_log_buffer_ctx);
    pub fn dcn10_get_hw_state(dc: *mut crate::dc, pBuf: *mut ::core::ffi::c_char, bufSize: ::core::ffi::c_uint, mask: ::core::ffi::c_uint);
    pub fn dcn10_clear_status_bits(dc: *mut crate::dc, mask: ::core::ffi::c_uint);
    pub fn dcn10_wait_for_mpcc_disconnect(dc: *mut crate::dc, res_pool: *mut crate::resource_pool, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dce110_edp_backlight_control(link: *mut crate::dc_link, enable: bool);
    pub fn dce110_edp_wait_for_T12(link: *mut crate::dc_link);
    pub fn dce110_edp_power_control(link: *mut crate::dc_link, power_up: bool);
    pub fn dce110_edp_wait_for_hpd_ready(link: *mut crate::dc_link, power_up: bool);
    pub fn dcn10_set_cursor_position(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_set_cursor_attribute(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_set_cursor_sdr_white_level(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_setup_periodic_interrupt(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_set_clock(dc: *mut crate::dc, clock_type: crate::dc_clock_type, clk_khz: u32, stepping: u32) -> crate::dc_status;
    pub fn dcn10_get_clock(dc: *mut crate::dc, clock_type: crate::dc_clock_type, clock_cfg: *mut crate::dc_clock_config);
    pub fn dcn10_did_underflow_occur(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx) -> bool;
    pub fn dcn10_bios_golden_init(dc: *mut crate::dc);
    pub fn dcn10_plane_atomic_power_down(dc: *mut crate::dc, dpp: *mut crate::dpp, hubp: *mut crate::hubp);
    pub fn dcn10_disconnect_pipes(dc: *mut crate::dc, context: *mut crate::dc_state) -> bool;
    pub fn dcn10_wait_for_pending_cleared(dc: *mut crate::dc, context: *mut crate::dc_state);
    pub fn dcn10_set_hdr_multiplier(pipe_ctx: *mut crate::pipe_ctx);
    pub fn dcn10_verify_allow_pstate_change_high(dc: *mut crate::dc);
    pub fn dcn10_get_dcc_en_bits(dc: *mut crate::dc, dcc_en_bits: *mut ::core::ffi::c_int);
    pub fn dcn10_update_visual_confirm_color(dc: *mut crate::dc, pipe_ctx: *mut crate::pipe_ctx, mpcc_id: ::core::ffi::c_int);
    pub fn dcn10_reset_surface_dcc_and_tiling(pipe_ctx: *mut crate::pipe_ctx, plane_state: *mut crate::dc_plane_state, clear_tiling: bool);
    pub fn dcn10_config_stereo_parameters(stream: *mut crate::dc_stream_state, flags: *mut crate::crtc_stereo_flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
