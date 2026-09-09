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

// Dependencies supplied by the surrounding translated driver modules:
// dce110_hwseq, dcn10_hwseq, dcn20_hwseq, dcn21_hwseq, and dcn21_init.

static dcn21_funcs: hw_sequencer_funcs = hw_sequencer_funcs {
    program_gamut_remap: Some(dcn10_program_gamut_remap),
    init_hw: Some(dcn10_init_hw),
    power_down_on_boot: Some(dcn10_power_down_on_boot),
    apply_ctx_to_hw: Some(dce110_apply_ctx_to_hw),
    apply_ctx_for_surface: None,
    program_front_end_for_ctx: Some(dcn20_program_front_end_for_ctx),
    clear_surface_dcc_and_tiling: Some(dcn10_reset_surface_dcc_and_tiling),
    wait_for_pending_cleared: Some(dcn10_wait_for_pending_cleared),
    post_unlock_program_front_end: Some(dcn20_post_unlock_program_front_end),
    update_plane_addr: Some(dcn20_update_plane_addr),
    update_dchub: Some(dcn10_update_dchub),
    update_pending_status: Some(dcn10_update_pending_status),
    program_output_csc: Some(dcn20_program_output_csc),
    enable_accelerated_mode: Some(dce110_enable_accelerated_mode),
    enable_timing_synchronization: Some(dcn10_enable_timing_synchronization),
    enable_per_frame_crtc_position_reset: Some(dcn10_enable_per_frame_crtc_position_reset),
    update_info_frame: Some(dce110_update_info_frame),
    send_immediate_sdp_message: Some(dcn10_send_immediate_sdp_message),
    enable_stream: Some(dcn20_enable_stream),
    disable_stream: Some(dce110_disable_stream),
    unblank_stream: Some(dcn20_unblank_stream),
    blank_stream: Some(dce110_blank_stream),
    enable_audio_stream: Some(dce110_enable_audio_stream),
    disable_audio_stream: Some(dce110_disable_audio_stream),
    disable_plane: Some(dcn20_disable_plane),
    pipe_control_lock: Some(dcn20_pipe_control_lock),
    interdependent_update_lock: Some(dcn10_lock_all_pipes),
    cursor_lock: Some(dcn10_cursor_lock),
    prepare_bandwidth: Some(dcn20_prepare_bandwidth),
    optimize_bandwidth: Some(dcn20_optimize_bandwidth),
    update_bandwidth: Some(dcn20_update_bandwidth),
    set_drr: Some(dcn10_set_drr),
    get_position: Some(dcn10_get_position),
    set_static_screen_control: Some(dcn10_set_static_screen_control),
    setup_stereo: Some(dcn10_setup_stereo),
    set_avmute: Some(dce110_set_avmute),
    log_hw_state: Some(dcn10_log_hw_state),
    get_hw_state: Some(dcn10_get_hw_state),
    log_color_state: Some(dcn20_log_color_state),
    clear_status_bits: Some(dcn10_clear_status_bits),
    wait_for_mpcc_disconnect: Some(dcn10_wait_for_mpcc_disconnect),
    edp_backlight_control: Some(dce110_edp_backlight_control),
    edp_power_control: Some(dce110_edp_power_control),
    edp_wait_for_hpd_ready: Some(dce110_edp_wait_for_hpd_ready),
    set_cursor_position: Some(dcn10_set_cursor_position),
    set_cursor_attribute: Some(dcn10_set_cursor_attribute),
    set_cursor_sdr_white_level: Some(dcn10_set_cursor_sdr_white_level),
    setup_periodic_interrupt: Some(dcn10_setup_periodic_interrupt),
    set_clock: Some(dcn10_set_clock),
    get_clock: Some(dcn10_get_clock),
    program_triplebuffer: Some(dcn20_program_triple_buffer),
    enable_writeback: Some(dcn20_enable_writeback),
    disable_writeback: Some(dcn20_disable_writeback),
    dmdata_status_done: Some(dcn20_dmdata_status_done),
    program_dmdata_engine: Some(dcn20_program_dmdata_engine),
    set_dmdata_attributes: Some(dcn20_set_dmdata_attributes),
    init_sys_ctx: Some(dcn21_init_sys_ctx),
    init_vm_ctx: Some(dcn20_init_vm_ctx),
    set_flip_control_gsl: Some(dcn20_set_flip_control_gsl),
    optimize_pwr_state: Some(dcn21_optimize_pwr_state),
    exit_optimized_pwr_state: Some(dcn21_exit_optimized_pwr_state),
    get_vupdate_offset_from_vsync: Some(dcn10_get_vupdate_offset_from_vsync),
    calc_vupdate_position: Some(dcn10_calc_vupdate_position),
    set_backlight_level: Some(dcn21_set_backlight_level),
    set_abm_immediate_disable: Some(dcn21_set_abm_immediate_disable),
    set_pipe: Some(dcn21_set_pipe),
    enable_lvds_link_output: Some(dce110_enable_lvds_link_output),
    enable_tmds_link_output: Some(dce110_enable_tmds_link_output),
    enable_dp_link_output: Some(dce110_enable_dp_link_output),
    disable_link_output: Some(dce110_disable_link_output),
    is_abm_supported: Some(dcn21_is_abm_supported),
    set_disp_pattern_generator: Some(dcn20_set_disp_pattern_generator),
    get_dcc_en_bits: Some(dcn10_get_dcc_en_bits),
    update_visual_confirm_color: Some(dcn10_update_visual_confirm_color),
};

static dcn21_private_funcs: hwseq_private_funcs = hwseq_private_funcs {
    init_pipes: Some(dcn10_init_pipes),
    plane_atomic_disconnect: Some(dcn10_plane_atomic_disconnect),
    update_mpcc: Some(dcn20_update_mpcc),
    set_input_transfer_func: Some(dcn20_set_input_transfer_func),
    set_output_transfer_func: Some(dcn20_set_output_transfer_func),
    power_down: Some(dce110_power_down),
    enable_display_power_gating: Some(dcn10_dummy_display_power_gating),
    blank_pixel_data: Some(dcn20_blank_pixel_data),
    reset_hw_ctx_wrap: Some(dcn20_reset_hw_ctx_wrap),
    enable_stream_timing: Some(dcn20_enable_stream_timing),
    edp_backlight_control: Some(dce110_edp_backlight_control),
    disable_stream_gating: Some(dcn20_disable_stream_gating),
    enable_stream_gating: Some(dcn20_enable_stream_gating),
    setup_vupdate_interrupt: Some(dcn20_setup_vupdate_interrupt),
    did_underflow_occur: Some(dcn10_did_underflow_occur),
    init_blank: Some(dcn20_init_blank),
    disable_vga: Some(dcn20_disable_vga),
    bios_golden_init: Some(dcn10_bios_golden_init),
    plane_atomic_disable: Some(dcn20_plane_atomic_disable),
    plane_atomic_power_down: Some(dcn10_plane_atomic_power_down),
    enable_power_gating_plane: Some(dcn20_enable_power_gating_plane),
    dpp_pg_control: Some(dcn20_dpp_pg_control),
    hubp_pg_control: Some(dcn20_hubp_pg_control),
    update_odm: Some(dcn20_update_odm),
    dsc_pg_control: Some(dcn20_dsc_pg_control),
    set_hdr_multiplier: Some(dcn10_set_hdr_multiplier),
    verify_allow_pstate_change_high: Some(dcn10_verify_allow_pstate_change_high),
    s0i3_golden_init_wa: Some(dcn21_s0i3_golden_init_wa),
    wait_for_blank_complete: Some(dcn20_wait_for_blank_complete),
    dccg_init: Some(dcn20_dccg_init),
    set_blend_lut: Some(dcn20_set_blend_lut),
    set_shaper_3dlut: Some(dcn20_set_shaper_3dlut),
    PLAT_58856_wa: Some(dcn21_PLAT_58856_wa),
};

pub unsafe fn dcn21_hw_sequencer_construct(dc: *mut dc) {
    (*dc).hwss = dcn21_funcs;
    (*(*dc).hwseq).funcs = dcn21_private_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
