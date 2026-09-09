// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct timing_generator_funcs {
    pub validate_timing: Option<unsafe extern "C" fn()>,
    pub program_timing: Option<unsafe extern "C" fn()>,
    pub setup_vertical_interrupt0: Option<unsafe extern "C" fn()>,
    pub setup_vertical_interrupt1: Option<unsafe extern "C" fn()>,
    pub setup_vertical_interrupt2: Option<unsafe extern "C" fn()>,
    pub program_global_sync: Option<unsafe extern "C" fn()>,
    pub enable_crtc: Option<unsafe extern "C" fn()>,
    pub disable_crtc: Option<unsafe extern "C" fn()>,
    pub phantom_crtc_post_enable: Option<unsafe extern "C" fn()>,
    pub disable_phantom_crtc: Option<unsafe extern "C" fn()>,
    pub is_counter_moving: Option<unsafe extern "C" fn()>,
    pub get_position: Option<unsafe extern "C" fn()>,
    pub get_frame_count: Option<unsafe extern "C" fn()>,
    pub get_scanoutpos: Option<unsafe extern "C" fn()>,
    pub get_otg_active_size: Option<unsafe extern "C" fn()>,
    pub set_early_control: Option<unsafe extern "C" fn()>,
    pub wait_for_state: Option<unsafe extern "C" fn()>,
    pub did_triggered_reset_occur: Option<unsafe extern "C" fn()>,
    pub triplebuffer_lock: Option<unsafe extern "C" fn()>,
    pub triplebuffer_unlock: Option<unsafe extern "C" fn()>,
    pub enable_reset_trigger: Option<unsafe extern "C" fn()>,
    pub enable_crtc_reset: Option<unsafe extern "C" fn()>,
    pub disable_reset_trigger: Option<unsafe extern "C" fn()>,
    pub lock: Option<unsafe extern "C" fn()>,
    pub unlock: Option<unsafe extern "C" fn()>,
    pub lock_doublebuffer_enable: Option<unsafe extern "C" fn()>,
    pub lock_doublebuffer_disable: Option<unsafe extern "C" fn()>,
    pub enable_optc_clock: Option<unsafe extern "C" fn()>,
    pub set_drr: Option<unsafe extern "C" fn()>,
    pub get_last_used_drr_vtotal: Option<unsafe extern "C" fn()>,
    pub set_vtotal_min_max: Option<unsafe extern "C" fn()>,
    pub set_static_screen_control: Option<unsafe extern "C" fn()>,
    pub program_stereo: Option<unsafe extern "C" fn()>,
    pub is_stereo_left_eye: Option<unsafe extern "C" fn()>,
    pub tg_init: Option<unsafe extern "C" fn()>,
    pub is_tg_enabled: Option<unsafe extern "C" fn()>,
    pub is_optc_underflow_occurred: Option<unsafe extern "C" fn()>,
    pub clear_optc_underflow: Option<unsafe extern "C" fn()>,
    pub get_crc: Option<unsafe extern "C" fn()>,
    pub configure_crc: Option<unsafe extern "C" fn()>,
    pub set_dsc_config: Option<unsafe extern "C" fn()>,
    pub get_dsc_status: Option<unsafe extern "C" fn()>,
    pub set_odm_bypass: Option<unsafe extern "C" fn()>,
    pub set_odm_combine: Option<unsafe extern "C" fn()>,
    pub wait_odm_doublebuffer_pending_clear: Option<unsafe extern "C" fn()>,
    pub set_h_timing_div_manual_mode: Option<unsafe extern "C" fn()>,
    pub get_optc_source: Option<unsafe extern "C" fn()>,
    pub set_out_mux: Option<unsafe extern "C" fn()>,
    pub set_drr_trigger_window: Option<unsafe extern "C" fn()>,
    pub set_vtotal_change_limit: Option<unsafe extern "C" fn()>,
    pub set_gsl: Option<unsafe extern "C" fn()>,
    pub set_gsl_source_select: Option<unsafe extern "C" fn()>,
    pub set_vtg_params: Option<unsafe extern "C" fn()>,
    pub program_manual_trigger: Option<unsafe extern "C" fn()>,
    pub setup_manual_trigger: Option<unsafe extern "C" fn()>,
    pub get_hw_timing: Option<unsafe extern "C" fn()>,
    pub is_two_pixels_per_container: Option<unsafe extern "C" fn()>,
    pub get_optc_double_buffer_pending: Option<unsafe extern "C" fn()>,
    pub get_otg_double_buffer_pending: Option<unsafe extern "C" fn()>,
    pub get_pipe_update_pending: Option<unsafe extern "C" fn()>,
    pub set_vupdate_keepout: Option<unsafe extern "C" fn()>,
    pub wait_update_lock_status: Option<unsafe extern "C" fn()>,
    pub read_otg_state: Option<unsafe extern "C" fn()>,
    pub optc_read_reg_state: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn optc1_validate_timing(); fn optc1_program_timing();
    fn optc1_setup_vertical_interrupt0(); fn optc1_setup_vertical_interrupt1();
    fn optc1_setup_vertical_interrupt2(); fn optc401_program_global_sync();
    fn optc401_enable_crtc(); fn optc401_disable_crtc();
    fn optc401_phantom_crtc_post_enable(); fn optc401_disable_phantom_otg();
    fn optc1_is_counter_moving(); fn optc1_get_position(); fn optc1_get_vblank_counter();
    fn optc1_get_crtc_scanoutpos(); fn optc1_get_otg_active_size(); fn optc1_set_early_control();
    fn optc1_wait_for_state(); fn optc1_did_triggered_reset_occur(); fn optc3_triplebuffer_lock();
    fn optc2_triplebuffer_unlock(); fn optc1_enable_reset_trigger(); fn optc1_enable_crtc_reset();
    fn optc1_disable_reset_trigger(); fn optc3_lock(); fn optc1_unlock();
    fn optc3_lock_doublebuffer_enable(); fn optc3_lock_doublebuffer_disable(); fn optc1_enable_optc_clock();
    fn optc401_set_drr(); fn optc2_get_last_used_drr_vtotal(); fn optc401_set_vtotal_min_max();
    fn optc1_set_static_screen_control(); fn optc1_program_stereo(); fn optc1_is_stereo_left_eye();
    fn optc3_tg_init(); fn optc1_is_tg_enabled(); fn optc1_is_optc_underflow_occurred(); fn optc1_clear_optc_underflow();
    fn optc42_get_crc(); fn optc1_configure_crc(); fn optc3_set_dsc_config(); fn optc2_get_dsc_status();
    fn optc401_set_odm_bypass(); fn optc401_set_odm_combine(); fn optc32_wait_odm_doublebuffer_pending_clear();
    fn optc401_set_h_timing_div_manual_mode(); fn optc2_get_optc_source(); fn optc401_set_out_mux();
    fn optc3_set_drr_trigger_window(); fn optc3_set_vtotal_change_limit(); fn optc2_set_gsl();
    fn optc1_set_vtg_params(); fn optc2_program_manual_trigger(); fn optc2_setup_manual_trigger();
    fn optc1_get_hw_timing(); fn optc1_is_two_pixels_per_container(); fn optc3_get_optc_double_buffer_pending();
    fn optc3_get_otg_update_pending(); fn optc3_get_pipe_update_pending(); fn optc401_set_vupdate_keepout();
    fn optc401_wait_update_lock_status(); fn optc31_read_otg_state(); fn optc31_read_reg_state();
}

pub static dcn60_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(optc1_validate_timing), program_timing: Some(optc1_program_timing),
    setup_vertical_interrupt0: Some(optc1_setup_vertical_interrupt0), setup_vertical_interrupt1: Some(optc1_setup_vertical_interrupt1), setup_vertical_interrupt2: Some(optc1_setup_vertical_interrupt2),
    program_global_sync: Some(optc401_program_global_sync), enable_crtc: Some(optc401_enable_crtc), disable_crtc: Some(optc401_disable_crtc), phantom_crtc_post_enable: Some(optc401_phantom_crtc_post_enable), disable_phantom_crtc: Some(optc401_disable_phantom_otg),
    is_counter_moving: Some(optc1_is_counter_moving), get_position: Some(optc1_get_position), get_frame_count: Some(optc1_get_vblank_counter), get_scanoutpos: Some(optc1_get_crtc_scanoutpos), get_otg_active_size: Some(optc1_get_otg_active_size), set_early_control: Some(optc1_set_early_control), wait_for_state: Some(optc1_wait_for_state), did_triggered_reset_occur: Some(optc1_did_triggered_reset_occur), triplebuffer_lock: Some(optc3_triplebuffer_lock), triplebuffer_unlock: Some(optc2_triplebuffer_unlock), enable_reset_trigger: Some(optc1_enable_reset_trigger), enable_crtc_reset: Some(optc1_enable_crtc_reset), disable_reset_trigger: Some(optc1_disable_reset_trigger), lock: Some(optc3_lock), unlock: Some(optc1_unlock), lock_doublebuffer_enable: Some(optc3_lock_doublebuffer_enable), lock_doublebuffer_disable: Some(optc3_lock_doublebuffer_disable), enable_optc_clock: Some(optc1_enable_optc_clock), set_drr: Some(optc401_set_drr), get_last_used_drr_vtotal: Some(optc2_get_last_used_drr_vtotal), set_vtotal_min_max: Some(optc401_set_vtotal_min_max), set_static_screen_control: Some(optc1_set_static_screen_control), program_stereo: Some(optc1_program_stereo), is_stereo_left_eye: Some(optc1_is_stereo_left_eye), tg_init: Some(optc3_tg_init), is_tg_enabled: Some(optc1_is_tg_enabled), is_optc_underflow_occurred: Some(optc1_is_optc_underflow_occurred), clear_optc_underflow: Some(optc1_clear_optc_underflow), get_crc: Some(optc42_get_crc), configure_crc: Some(optc1_configure_crc), set_dsc_config: Some(optc3_set_dsc_config), get_dsc_status: Some(optc2_get_dsc_status), set_odm_bypass: Some(optc401_set_odm_bypass), set_odm_combine: Some(optc401_set_odm_combine), wait_odm_doublebuffer_pending_clear: Some(optc32_wait_odm_doublebuffer_pending_clear), set_h_timing_div_manual_mode: Some(optc401_set_h_timing_div_manual_mode), get_optc_source: Some(optc2_get_optc_source), set_out_mux: Some(optc401_set_out_mux), set_drr_trigger_window: Some(optc3_set_drr_trigger_window), set_vtotal_change_limit: Some(optc3_set_vtotal_change_limit), set_gsl: Some(optc2_set_gsl), set_gsl_source_select: None, set_vtg_params: Some(optc1_set_vtg_params), program_manual_trigger: Some(optc2_program_manual_trigger), setup_manual_trigger: Some(optc2_setup_manual_trigger), get_hw_timing: Some(optc1_get_hw_timing), is_two_pixels_per_container: Some(optc1_is_two_pixels_per_container), get_optc_double_buffer_pending: Some(optc3_get_optc_double_buffer_pending), get_otg_double_buffer_pending: Some(optc3_get_otg_update_pending), get_pipe_update_pending: Some(optc3_get_pipe_update_pending), set_vupdate_keepout: Some(optc401_set_vupdate_keepout), wait_update_lock_status: Some(optc401_wait_update_lock_status), read_otg_state: Some(optc31_read_otg_state), optc_read_reg_state: Some(optc31_read_reg_state),
};

// The containing `struct optc` and its nested fields are supplied by dcn60_optc.h.
pub unsafe fn dcn60_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn60_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
