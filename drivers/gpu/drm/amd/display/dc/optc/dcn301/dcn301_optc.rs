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

// Dependencies are supplied by the surrounding driver translation.

pub unsafe fn optc301_set_drr(
    optc: *mut timing_generator,
    params: *const drr_params,
) {
    let optc1: *mut optc = DCN10TG_FROM_TG(optc);

    if !params.is_null()
        && (*params).vertical_total_max > 0
        && (*params).vertical_total_min > 0
    {
        if (*params).vertical_total_mid != 0 {
            REG_SET!(optc1, OTG_V_TOTAL_MID, 0, OTG_V_TOTAL_MID,
                (*params).vertical_total_mid - 1);
            REG_UPDATE_2!(optc1, OTG_V_TOTAL_CONTROL,
                OTG_VTOTAL_MID_REPLACING_MAX_EN, 1,
                OTG_VTOTAL_MID_FRAME_NUM,
                (*params).vertical_total_mid_frame_num as u8);
        }

        ((*(*optc).funcs).set_vtotal_min_max)(
            optc,
            (*params).vertical_total_min - 1,
            (*params).vertical_total_max - 1,
        );
        REG_UPDATE_5!(optc1, OTG_V_TOTAL_CONTROL,
            OTG_V_TOTAL_MIN_SEL, 1,
            OTG_V_TOTAL_MAX_SEL, 1,
            OTG_FORCE_LOCK_ON_EVENT, 0,
            OTG_SET_V_TOTAL_MIN_MASK_EN, 0,
            OTG_SET_V_TOTAL_MIN_MASK, 0);
        // Setup manual flow control for EOF via TRIG_A
        ((*(*optc).funcs).setup_manual_trigger)(optc);
    } else {
        REG_UPDATE_4!(optc1, OTG_V_TOTAL_CONTROL,
            OTG_SET_V_TOTAL_MIN_MASK, 0,
            OTG_V_TOTAL_MIN_SEL, 0,
            OTG_V_TOTAL_MAX_SEL, 0,
            OTG_FORCE_LOCK_ON_EVENT, 0);
        ((*(*optc).funcs).set_vtotal_min_max)(optc, 0, 0);
    }
}

pub unsafe fn optc301_setup_manual_trigger(optc: *mut timing_generator) {
    let optc1: *mut optc = DCN10TG_FROM_TG(optc);
    REG_SET_8!(optc1, OTG_TRIGA_CNTL, 0,
        OTG_TRIGA_SOURCE_SELECT, 21,
        OTG_TRIGA_SOURCE_PIPE_SELECT, (*optc).inst,
        OTG_TRIGA_RISING_EDGE_DETECT_CNTL, 1,
        OTG_TRIGA_FALLING_EDGE_DETECT_CNTL, 0,
        OTG_TRIGA_POLARITY_SELECT, 0,
        OTG_TRIGA_FREQUENCY_SELECT, 0,
        OTG_TRIGA_DELAY, 0,
        OTG_TRIGA_CLEAR, 1);
}

static dcn30_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(optc1_validate_timing),
    program_timing: Some(optc1_program_timing),
    setup_vertical_interrupt0: Some(optc1_setup_vertical_interrupt0),
    setup_vertical_interrupt1: Some(optc1_setup_vertical_interrupt1),
    setup_vertical_interrupt2: Some(optc1_setup_vertical_interrupt2),
    program_global_sync: Some(optc1_program_global_sync),
    enable_crtc: Some(optc2_enable_crtc), disable_crtc: Some(optc1_disable_crtc),
    is_counter_moving: Some(optc1_is_counter_moving), get_position: Some(optc1_get_position),
    get_frame_count: Some(optc1_get_vblank_counter), get_scanoutpos: Some(optc1_get_crtc_scanoutpos),
    get_otg_active_size: Some(optc1_get_otg_active_size), set_early_control: Some(optc1_set_early_control),
    wait_for_state: Some(optc1_wait_for_state), set_blank_color: Some(optc3_program_blank_color),
    did_triggered_reset_occur: Some(optc1_did_triggered_reset_occur), triplebuffer_lock: Some(optc3_triplebuffer_lock),
    triplebuffer_unlock: Some(optc2_triplebuffer_unlock), enable_reset_trigger: Some(optc1_enable_reset_trigger),
    enable_crtc_reset: Some(optc1_enable_crtc_reset), disable_reset_trigger: Some(optc1_disable_reset_trigger),
    lock: Some(optc3_lock), unlock: Some(optc1_unlock), lock_doublebuffer_enable: Some(optc3_lock_doublebuffer_enable),
    lock_doublebuffer_disable: Some(optc3_lock_doublebuffer_disable), enable_optc_clock: Some(optc1_enable_optc_clock),
    set_drr: Some(optc301_set_drr), get_last_used_drr_vtotal: Some(optc2_get_last_used_drr_vtotal),
    set_vtotal_min_max: Some(optc3_set_vtotal_min_max), set_static_screen_control: Some(optc1_set_static_screen_control),
    program_stereo: Some(optc1_program_stereo), is_stereo_left_eye: Some(optc1_is_stereo_left_eye),
    tg_init: Some(optc3_tg_init), is_tg_enabled: Some(optc1_is_tg_enabled),
    is_optc_underflow_occurred: Some(optc1_is_optc_underflow_occurred), clear_optc_underflow: Some(optc1_clear_optc_underflow),
    setup_global_swap_lock: None, get_crc: Some(optc1_get_crc), configure_crc: Some(optc2_configure_crc),
    set_dsc_config: Some(optc3_set_dsc_config), get_dsc_status: Some(optc2_get_dsc_status), set_dwb_source: None,
    set_odm_bypass: Some(optc3_set_odm_bypass), set_odm_combine: Some(optc3_set_odm_combine), get_optc_source: Some(optc2_get_optc_source),
    set_out_mux: Some(optc3_set_out_mux), set_drr_trigger_window: Some(optc3_set_drr_trigger_window), set_vtotal_change_limit: Some(optc3_set_vtotal_change_limit),
    set_gsl: Some(optc2_set_gsl), set_gsl_source_select: Some(optc2_set_gsl_source_select), set_vtg_params: Some(optc1_set_vtg_params),
    program_manual_trigger: Some(optc2_program_manual_trigger), setup_manual_trigger: Some(optc301_setup_manual_trigger), get_hw_timing: Some(optc1_get_hw_timing),
    wait_drr_doublebuffer_pending_clear: Some(optc3_wait_drr_doublebuffer_pending_clear), is_two_pixels_per_container: Some(optc1_is_two_pixels_per_container),
    get_optc_double_buffer_pending: Some(optc3_get_optc_double_buffer_pending), get_otg_double_buffer_pending: Some(optc3_get_otg_update_pending),
    get_pipe_update_pending: Some(optc3_get_pipe_update_pending), read_otg_state: Some(optc1_read_otg_state),
};

pub unsafe fn dcn301_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn30_tg_funcs;
    (*optc1).max_h_total = (*(*optc1).tg_mask).OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*(*optc1).tg_mask).OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
