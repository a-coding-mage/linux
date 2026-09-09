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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies and register-access macros are supplied by the surrounding
// translation unit.

unsafe fn optc201_triplebuffer_lock(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);

    REG_SET((*optc1).tg_regs, OTG_GLOBAL_CONTROL0, 0,
        OTG_MASTER_UPDATE_LOCK_SEL, (*optc).inst);
    REG_SET((*optc1).tg_regs, OTG_VUPDATE_KEEPOUT, 0,
        OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, 1);
    REG_SET((*optc1).tg_regs, OTG_MASTER_UPDATE_LOCK, 0,
        OTG_MASTER_UPDATE_LOCK, 1);

    REG_WAIT((*optc1).tg_regs, OTG_MASTER_UPDATE_LOCK,
        UPDATE_LOCK_STATUS, 1, 1, 10);
}

unsafe fn optc201_triplebuffer_unlock(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);

    REG_SET((*optc1).tg_regs, OTG_MASTER_UPDATE_LOCK, 0,
        OTG_MASTER_UPDATE_LOCK, 0);
    REG_SET((*optc1).tg_regs, OTG_VUPDATE_KEEPOUT, 0,
        OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, 0);
}

unsafe fn optc201_validate_timing(
    optc: *mut timing_generator,
    timing: *const dc_crtc_timing,
) -> bool {
    let timing = &*timing;
    let optc1 = DCN10TG_FROM_TG(optc);

    let v_blank = timing.v_total - timing.v_addressable
        - timing.v_border_top - timing.v_border_bottom;
    let h_blank = timing.h_total - timing.h_addressable
        - timing.h_border_right - timing.h_border_left;

    if timing.timing_3d_format != TIMING_3D_FORMAT_NONE
        && timing.timing_3d_format != TIMING_3D_FORMAT_HW_FRAME_PACKING
        && timing.timing_3d_format != TIMING_3D_FORMAT_TOP_AND_BOTTOM
        && timing.timing_3d_format != TIMING_3D_FORMAT_SIDE_BY_SIDE
        && timing.timing_3d_format != TIMING_3D_FORMAT_FRAME_ALTERNATE
        && timing.timing_3d_format != TIMING_3D_FORMAT_INBAND_FA
    {
        return false;
    }

    if timing.h_total > (*optc1).max_h_total || timing.v_total > (*optc1).max_v_total {
        return false;
    }
    if h_blank < (*optc1).min_h_blank {
        return false;
    }
    if timing.h_sync_width < (*optc1).min_h_sync_width
        || timing.v_sync_width < (*optc1).min_v_sync_width
    {
        return false;
    }

    let min_v_blank = if timing.flags.INTERLACE {
        (*optc1).min_v_blank_interlace
    } else {
        (*optc1).min_v_blank
    };
    if v_blank < min_v_blank {
        return false;
    }
    true
}

unsafe fn optc201_get_optc_source(
    optc: *mut timing_generator,
    num_of_src_opp: *mut u32,
    src_opp_id_0: *mut u32,
    _src_opp_id_1: *mut u32,
) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_GET((*optc1).tg_regs, OPTC_DATA_SOURCE_SELECT,
        OPTC_SEG0_SRC_SEL, src_opp_id_0);
    *num_of_src_opp = 1;
}

static dcn201_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(optc201_validate_timing),
    program_timing: Some(optc1_program_timing),
    setup_vertical_interrupt0: Some(optc1_setup_vertical_interrupt0),
    setup_vertical_interrupt1: Some(optc1_setup_vertical_interrupt1),
    setup_vertical_interrupt2: Some(optc1_setup_vertical_interrupt2),
    program_global_sync: Some(optc1_program_global_sync),
    enable_crtc: Some(optc2_enable_crtc),
    disable_crtc: Some(optc1_disable_crtc),
    is_counter_moving: Some(optc1_is_counter_moving),
    get_position: Some(optc1_get_position),
    get_frame_count: Some(optc1_get_vblank_counter),
    get_scanoutpos: Some(optc1_get_crtc_scanoutpos),
    get_otg_active_size: Some(optc1_get_otg_active_size),
    set_early_control: Some(optc1_set_early_control),
    wait_for_state: Some(optc1_wait_for_state),
    set_blank: Some(optc1_set_blank),
    is_blanked: Some(optc1_is_blanked),
    set_blank_color: Some(optc1_program_blank_color),
    did_triggered_reset_occur: Some(optc1_did_triggered_reset_occur),
    enable_reset_trigger: Some(optc1_enable_reset_trigger),
    enable_crtc_reset: Some(optc1_enable_crtc_reset),
    disable_reset_trigger: Some(optc1_disable_reset_trigger),
    triplebuffer_lock: Some(optc201_triplebuffer_lock),
    triplebuffer_unlock: Some(optc201_triplebuffer_unlock),
    lock: Some(optc1_lock),
    unlock: Some(optc1_unlock),
    enable_optc_clock: Some(optc1_enable_optc_clock),
    set_drr: Some(optc1_set_drr),
    get_last_used_drr_vtotal: None,
    set_vtotal_min_max: Some(optc1_set_vtotal_min_max),
    set_static_screen_control: Some(optc1_set_static_screen_control),
    program_stereo: Some(optc1_program_stereo),
    is_stereo_left_eye: Some(optc1_is_stereo_left_eye),
    set_blank_data_double_buffer: Some(optc1_set_blank_data_double_buffer),
    tg_init: Some(optc1_tg_init),
    is_tg_enabled: Some(optc1_is_tg_enabled),
    is_optc_underflow_occurred: Some(optc1_is_optc_underflow_occurred),
    clear_optc_underflow: Some(optc1_clear_optc_underflow),
    get_crc: Some(optc1_get_crc),
    configure_crc: Some(optc2_configure_crc),
    set_dsc_config: Some(optc2_set_dsc_config),
    set_dwb_source: None,
    get_optc_source: Some(optc201_get_optc_source),
    set_vtg_params: Some(optc1_set_vtg_params),
    program_manual_trigger: Some(optc2_program_manual_trigger),
    setup_manual_trigger: Some(optc2_setup_manual_trigger),
    get_hw_timing: Some(optc1_get_hw_timing),
    is_two_pixels_per_container: Some(optc1_is_two_pixels_per_container),
    read_otg_state: Some(optc1_read_otg_state),
};

unsafe fn dcn201_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn201_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 8;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
