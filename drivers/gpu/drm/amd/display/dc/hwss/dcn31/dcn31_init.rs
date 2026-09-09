/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Declarations supplied by the corresponding hardware-sequencer modules.
extern "C" {
    fn dcn30_program_gamut_remap(); fn dcn31_init_hw(); fn dcn10_power_down_on_boot();
    fn dce110_apply_ctx_to_hw(); fn dcn20_program_front_end_for_ctx();
    fn dcn10_reset_surface_dcc_and_tiling(); fn dcn10_wait_for_pending_cleared();
    fn dcn20_post_unlock_program_front_end(); fn dcn20_update_plane_addr(); fn dcn10_update_dchub();
    fn dcn10_update_pending_status(); fn dcn20_program_output_csc(); fn dce110_enable_accelerated_mode();
    fn dcn10_enable_timing_synchronization(); fn dcn10_enable_per_frame_crtc_position_reset();
    fn dcn31_update_info_frame(); fn dcn10_send_immediate_sdp_message(); fn dcn20_enable_stream();
    fn dce110_disable_stream(); fn dcn20_unblank_stream(); fn dce110_blank_stream();
    fn dce110_enable_audio_stream(); fn dce110_disable_audio_stream(); fn dcn20_disable_plane();
    fn dcn20_disable_pixel_data(); fn dcn20_pipe_control_lock(); fn dcn10_lock_all_pipes();
    fn dcn10_cursor_lock(); fn dcn20_prepare_bandwidth(); fn dcn20_optimize_bandwidth();
    fn dcn20_update_bandwidth(); fn dcn10_set_drr(); fn dcn10_get_position(); fn dcn31_set_static_screen_control();
    fn dcn10_setup_stereo(); fn dcn30_set_avmute(); fn dcn10_log_hw_state(); fn dcn30_log_color_state();
    fn dcn10_get_hw_state(); fn dcn10_clear_status_bits(); fn dcn10_wait_for_mpcc_disconnect();
    fn dce110_edp_backlight_control(); fn dce110_edp_power_control(); fn dce110_edp_wait_for_T12();
    fn dce110_edp_wait_for_hpd_ready(); fn dcn10_set_cursor_position(); fn dcn10_set_cursor_attribute();
    fn dcn10_set_cursor_sdr_white_level(); fn dcn10_setup_periodic_interrupt(); fn dcn10_set_clock();
    fn dcn10_get_clock(); fn dcn20_program_triple_buffer(); fn dcn30_enable_writeback(); fn dcn30_disable_writeback();
    fn dcn30_update_writeback(); fn dcn20_dmdata_status_done(); fn dcn30_program_dmdata_engine();
    fn dcn20_set_dmdata_attributes(); fn dcn31_init_sys_ctx(); fn dcn20_init_vm_ctx(); fn dcn20_set_flip_control_gsl();
    fn dcn10_get_vupdate_offset_from_vsync(); fn dcn10_calc_vupdate_position(); fn dcn30_setup_hdmi_frl_link();
    fn dcn21_set_backlight_level(); fn dcn21_set_abm_immediate_disable(); fn dcn21_set_pipe();
    fn dce110_enable_lvds_link_output(); fn dce110_enable_tmds_link_output(); fn dce110_enable_dp_link_output();
    fn dce110_disable_link_output(); fn dcn31_z10_restore(); fn dcn31_z10_save_init(); fn dcn30_set_disp_pattern_generator();
    fn dcn21_optimize_pwr_state(); fn dcn21_exit_optimized_pwr_state(); fn dcn10_update_visual_confirm_color();
    fn dcn31_setup_hpo_hw_control(); fn dcn30_get_underflow_debug_data(); fn dcn10_init_pipes();
    fn dcn10_plane_atomic_disconnect(); fn dcn20_update_mpcc(); fn dcn30_set_input_transfer_func();
    fn dcn30_set_output_transfer_func(); fn dce110_power_down(); fn dcn10_dummy_display_power_gating();
    fn dcn20_blank_pixel_data(); fn dcn31_reset_hw_ctx_wrap(); fn dcn20_enable_stream_timing();
    fn dcn20_disable_stream_gating(); fn dcn20_enable_stream_gating(); fn dcn20_setup_vupdate_interrupt();
    fn dcn10_did_underflow_occur(); fn dcn20_init_blank(); fn dcn20_disable_vga(); fn dcn10_bios_golden_init();
    fn dcn20_plane_atomic_disable(); fn dcn10_plane_atomic_power_down(); fn dcn31_enable_power_gating_plane();
    fn dcn31_hubp_pg_control(); fn dcn30_program_all_writeback_pipes_in_tree(); fn dcn20_update_odm();
    fn dcn31_dsc_pg_control(); fn dcn10_set_hdr_multiplier(); fn dcn10_verify_allow_pstate_change_high();
    fn dcn20_wait_for_blank_complete(); fn dcn30_set_blend_lut(); fn dcn20_set_shaper_3dlut();
    fn dcn10_wait_for_pipe_update_if_needed(); fn dcn10_set_wait_for_update_needed_for_pipe();
}

// Function-table field types and imported members are supplied by the shared
// DC headers; the opaque representations retain their ABI-level role here.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_sequencer_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwseq_private_funcs;

static dcn31_funcs: hw_sequencer_funcs = hw_sequencer_funcs;
static dcn31_private_funcs: hwseq_private_funcs = hwseq_private_funcs;
#[repr(C)]
pub struct dc {
    pub hwss: hw_sequencer_funcs,
    pub hwseq: *mut hwseq,
}
#[repr(C)]
pub struct hwseq { pub funcs: hwseq_private_funcs }

pub unsafe fn dcn31_hw_sequencer_construct(dc: *mut dc) {
    (*dc).hwss = dcn31_funcs;
    (*(*dc).hwseq).funcs = dcn31_private_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
