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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::c_void;

// Declarations supplied by the corresponding hardware-sequencer dependencies.
extern "C" {
    fn dcn10_program_gamut_remap(); fn dcn201_init_hw(); fn dce110_apply_ctx_to_hw();
    fn dcn20_program_front_end_for_ctx(); fn dcn10_reset_surface_dcc_and_tiling();
    fn dcn10_wait_for_pending_cleared(); fn dcn10_post_unlock_program_front_end();
    fn dcn201_update_plane_addr(); fn dcn10_update_dchub(); fn dcn10_update_pending_status();
    fn dcn20_program_output_csc(); fn dce110_enable_accelerated_mode();
    fn dcn10_enable_timing_synchronization(); fn dcn10_enable_per_frame_crtc_position_reset();
    fn dce110_update_info_frame(); fn dcn10_send_immediate_sdp_message(); fn dce110_enable_stream();
    fn dce110_disable_stream(); fn dcn201_unblank_stream(); fn dce110_blank_stream();
    fn dce110_enable_audio_stream(); fn dce110_disable_audio_stream(); fn dcn10_disable_plane();
    fn dcn201_pipe_control_lock(); fn dcn10_lock_all_pipes(); fn dcn10_cursor_lock();
    fn dcn20_prepare_bandwidth(); fn dcn20_optimize_bandwidth(); fn dcn20_update_bandwidth();
    fn dcn10_set_drr(); fn dcn10_get_position(); fn dcn10_set_static_screen_control();
    fn dcn10_setup_stereo(); fn dce110_set_avmute(); fn dcn10_log_hw_state(); fn dcn10_get_hw_state();
    fn dcn10_clear_status_bits(); fn dcn10_wait_for_mpcc_disconnect(); fn dce110_edp_backlight_control();
    fn dce110_edp_power_control(); fn dce110_edp_wait_for_hpd_ready(); fn dcn10_setup_periodic_interrupt();
    fn dcn10_set_clock(); fn dcn10_get_clock(); fn dcn20_program_triple_buffer(); fn dcn20_dmdata_status_done();
    fn dcn201_set_dmdata_attributes(); fn dcn10_get_vupdate_offset_from_vsync(); fn dcn10_calc_vupdate_position();
    fn dcn10_set_cursor_position(); fn dcn201_set_cursor_attribute(); fn dcn10_set_cursor_sdr_white_level();
    fn dce110_set_backlight_level(); fn dce110_set_abm_immediate_disable(); fn dce110_set_pipe();
    fn dce110_enable_lvds_link_output(); fn dce110_enable_tmds_link_output(); fn dce110_enable_dp_link_output();
    fn dce110_disable_link_output(); fn dcn20_set_disp_pattern_generator(); fn dcn10_update_visual_confirm_color();
    fn dcn201_plane_atomic_disconnect(); fn dcn10_program_pipe(); fn dcn201_update_mpcc();
    fn dcn20_set_input_transfer_func(); fn dcn20_set_output_transfer_func(); fn dce110_power_down();
    fn dcn10_dummy_display_power_gating(); fn dcn20_blank_pixel_data(); fn dcn10_reset_hw_ctx_wrap();
    fn dcn20_enable_stream_timing(); fn dcn20_setup_vupdate_interrupt(); fn dcn10_did_underflow_occur();
    fn dcn201_init_blank(); fn dcn10_disable_vga(); fn dcn10_bios_golden_init(); fn dcn10_plane_atomic_disable();
    fn dcn10_plane_atomic_power_down(); fn dcn10_enable_power_gating_plane(); fn dcn10_dpp_pg_control();
    fn dcn10_hubp_pg_control(); fn dcn10_set_hdr_multiplier(); fn dcn10_verify_allow_pstate_change_high();
    fn dcn20_wait_for_blank_complete(); fn dcn20_dccg_init(); fn dcn20_set_blend_lut(); fn dcn20_set_shaper_3dlut();
}

#[repr(C)]
pub struct hw_sequencer_funcs { pub entries: [*const c_void; 56] }
#[repr(C)] pub struct hwseq_private_funcs { pub entries: [*const c_void; 35] }
#[repr(C)] pub struct hwseq { pub funcs: hwseq_private_funcs }
#[repr(C)] pub struct dc { pub hwss: hw_sequencer_funcs, pub hwseq: *mut hwseq }

static mut DCN201_FUNCS: hw_sequencer_funcs = hw_sequencer_funcs { entries: [
    dcn10_program_gamut_remap as *const c_void, dcn201_init_hw as *const c_void, core::ptr::null(),
    dce110_apply_ctx_to_hw as *const c_void, core::ptr::null(), dcn20_program_front_end_for_ctx as *const c_void,
    dcn10_reset_surface_dcc_and_tiling as *const c_void, dcn10_wait_for_pending_cleared as *const c_void,
    dcn10_post_unlock_program_front_end as *const c_void, dcn201_update_plane_addr as *const c_void,
    dcn10_update_dchub as *const c_void, dcn10_update_pending_status as *const c_void, dcn20_program_output_csc as *const c_void,
    dce110_enable_accelerated_mode as *const c_void, dcn10_enable_timing_synchronization as *const c_void,
    dcn10_enable_per_frame_crtc_position_reset as *const c_void, dce110_update_info_frame as *const c_void,
    dcn10_send_immediate_sdp_message as *const c_void, dce110_enable_stream as *const c_void, dce110_disable_stream as *const c_void,
    dcn201_unblank_stream as *const c_void, dce110_blank_stream as *const c_void, dce110_enable_audio_stream as *const c_void,
    dce110_disable_audio_stream as *const c_void, dcn10_disable_plane as *const c_void, dcn201_pipe_control_lock as *const c_void,
    dcn10_lock_all_pipes as *const c_void, dcn10_cursor_lock as *const c_void, dcn20_prepare_bandwidth as *const c_void,
    dcn20_optimize_bandwidth as *const c_void, dcn20_update_bandwidth as *const c_void, dcn10_set_drr as *const c_void,
    dcn10_get_position as *const c_void, dcn10_set_static_screen_control as *const c_void, dcn10_setup_stereo as *const c_void,
    dce110_set_avmute as *const c_void, dcn10_log_hw_state as *const c_void, dcn10_get_hw_state as *const c_void,
    dcn10_clear_status_bits as *const c_void, dcn10_wait_for_mpcc_disconnect as *const c_void, dce110_edp_backlight_control as *const c_void,
    dce110_edp_power_control as *const c_void, dce110_edp_wait_for_hpd_ready as *const c_void, dcn10_setup_periodic_interrupt as *const c_void,
    dcn10_set_clock as *const c_void, dcn10_get_clock as *const c_void, dcn20_program_triple_buffer as *const c_void,
    dcn20_dmdata_status_done as *const c_void, dcn201_set_dmdata_attributes as *const c_void, dcn10_get_vupdate_offset_from_vsync as *const c_void,
    dcn10_calc_vupdate_position as *const c_void, dcn10_set_cursor_position as *const c_void, dcn201_set_cursor_attribute as *const c_void,
    dcn10_set_cursor_sdr_white_level as *const c_void, dce110_set_backlight_level as *const c_void, dce110_set_abm_immediate_disable as *const c_void,
    dce110_set_pipe as *const c_void, dce110_enable_lvds_link_output as *const c_void, dce110_enable_tmds_link_output as *const c_void,
    dce110_enable_dp_link_output as *const c_void, dce110_disable_link_output as *const c_void, dcn20_set_disp_pattern_generator as *const c_void,
    dcn10_update_visual_confirm_color as *const c_void,
] };
static mut DCN201_PRIVATE_FUNCS: hwseq_private_funcs = hwseq_private_funcs { entries: [core::ptr::null(); 35] };

pub unsafe extern "C" fn dcn201_hw_sequencer_construct(dc: *mut dc) {
    (*dc).hwss = DCN201_FUNCS;
    (*(*dc).hwseq).funcs = DCN201_PRIVATE_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
