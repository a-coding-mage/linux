// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

unsafe fn optc314_set_odm_combine(
    optc: *mut timing_generator,
    opp_id: *mut i32,
    opp_cnt: i32,
    segment_width: i32,
    _last_segment_width: i32,
) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let mut memory_mask: u32 = 0;
    let h_active = segment_width * opp_cnt;
    let odm_mem_count = (h_active + 2047) / 2048;

    if opp_cnt == 4 {
        if odm_mem_count <= 2 { memory_mask = 0x3; }
        else if odm_mem_count <= 4 { memory_mask = 0xf; }
        else { memory_mask = 0x3f; }
    } else {
        if odm_mem_count <= 2 {
            memory_mask = (0x1 << ((*opp_id.add(0) * 2) as u32)) |
                (0x1 << ((*opp_id.add(1) * 2) as u32));
        } else if odm_mem_count <= 4 {
            memory_mask = (0x3 << ((*opp_id.add(0) * 2) as u32)) |
                (0x3 << ((*opp_id.add(1) * 2) as u32));
        } else { memory_mask = 0x77; }
    }

    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask);
    if opp_cnt == 2 {
        REG_SET_3!(optc1, OPTC_DATA_SOURCE_SELECT, 0,
            OPTC_NUM_OF_INPUT_SEGMENT, 1,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0),
            OPTC_SEG1_SRC_SEL, *opp_id.add(1));
    } else if opp_cnt == 4 {
        REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0,
            OPTC_NUM_OF_INPUT_SEGMENT, 3,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0),
            OPTC_SEG1_SRC_SEL, *opp_id.add(1),
            OPTC_SEG2_SRC_SEL, *opp_id.add(2),
            OPTC_SEG3_SRC_SEL, *opp_id.add(3));
    }
    REG_UPDATE!(optc1, OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, opp_cnt - 1);
    (*optc1).opp_count = opp_cnt;
}

unsafe fn optc314_enable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE!(optc1, CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2!(optc1, OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 2, OTG_MASTER_EN, 1);
    true
}

unsafe fn optc314_disable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OTG_CONTROL, OTG_MASTER_EN, 0);
    REG_UPDATE!(optc1, CONTROL, VTG0_ENABLE, 0);
    REG_WAIT!(optc1, OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
    true
}

unsafe fn optc314_phantom_crtc_post_enable(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2!(optc1, OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 0, OTG_MASTER_EN, 0);
    REG_WAIT!(optc1, OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
}

unsafe fn optc314_set_odm_bypass(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let h_div = (*optc).funcs.is_two_pixels_per_container(dc_crtc_timing);
    REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0,
        OPTC_NUM_OF_INPUT_SEGMENT, 0, OPTC_SEG0_SRC_SEL, (*optc).inst,
        OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, h_div);
    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0);
    (*optc1).opp_count = 1;
}

unsafe fn optc314_set_h_timing_div_manual_mode(optc: *mut timing_generator, manual_mode: bool) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE_MANUAL,
        if manual_mode { 1 } else { 0 });
}

// Function table translated from dcn314_tg_funcs; referenced functions and types are external.
static dcn314_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: optc1_validate_timing,
    program_timing: optc1_program_timing,
    setup_vertical_interrupt0: optc1_setup_vertical_interrupt0,
    setup_vertical_interrupt1: optc1_setup_vertical_interrupt1,
    setup_vertical_interrupt2: optc1_setup_vertical_interrupt2,
    program_global_sync: optc1_program_global_sync,
    enable_crtc: optc314_enable_crtc,
    disable_crtc: optc314_disable_crtc,
    immediate_disable_crtc: optc31_immediate_disable_crtc,
    phantom_crtc_post_enable: optc314_phantom_crtc_post_enable,
    is_counter_moving: optc1_is_counter_moving,
    get_position: optc1_get_position,
    get_frame_count: optc1_get_vblank_counter,
    get_scanoutpos: optc1_get_crtc_scanoutpos,
    get_otg_active_size: optc1_get_otg_active_size,
    set_early_control: optc1_set_early_control,
    wait_for_state: optc1_wait_for_state,
    set_blank_color: optc3_program_blank_color,
    did_triggered_reset_occur: optc1_did_triggered_reset_occur,
    triplebuffer_lock: optc3_triplebuffer_lock,
    triplebuffer_unlock: optc2_triplebuffer_unlock,
    enable_reset_trigger: optc1_enable_reset_trigger,
    enable_crtc_reset: optc1_enable_crtc_reset,
    disable_reset_trigger: optc1_disable_reset_trigger,
    lock: optc3_lock,
    unlock: optc1_unlock,
    lock_doublebuffer_enable: optc3_lock_doublebuffer_enable,
    lock_doublebuffer_disable: optc3_lock_doublebuffer_disable,
    enable_optc_clock: optc1_enable_optc_clock,
    set_drr: optc31_set_drr,
    get_last_used_drr_vtotal: optc2_get_last_used_drr_vtotal,
    set_vtotal_min_max: optc1_set_vtotal_min_max,
    set_static_screen_control: optc1_set_static_screen_control,
    program_stereo: optc1_program_stereo,
    is_stereo_left_eye: optc1_is_stereo_left_eye,
    tg_init: optc3_tg_init,
    is_tg_enabled: optc1_is_tg_enabled,
    is_optc_underflow_occurred: optc1_is_optc_underflow_occurred,
    clear_optc_underflow: optc1_clear_optc_underflow,
    setup_global_swap_lock: None,
    get_crc: optc1_get_crc,
    configure_crc: optc1_configure_crc,
    set_dsc_config: optc3_set_dsc_config,
    get_dsc_status: optc2_get_dsc_status,
    set_dwb_source: None,
    get_optc_source: optc2_get_optc_source,
    set_out_mux: optc3_set_out_mux,
    set_drr_trigger_window: optc3_set_drr_trigger_window,
    set_vtotal_change_limit: optc3_set_vtotal_change_limit,
    set_gsl: optc2_set_gsl,
    set_gsl_source_select: optc2_set_gsl_source_select,
    set_vtg_params: optc1_set_vtg_params,
    program_manual_trigger: optc2_program_manual_trigger,
    setup_manual_trigger: optc2_setup_manual_trigger,
    get_hw_timing: optc1_get_hw_timing,
    init_odm: optc3_init_odm,
    set_odm_bypass: optc314_set_odm_bypass,
    set_odm_combine: optc314_set_odm_combine,
    set_h_timing_div_manual_mode: optc314_set_h_timing_div_manual_mode,
    is_two_pixels_per_container: optc1_is_two_pixels_per_container,
    read_otg_state: optc31_read_otg_state,
    optc_read_reg_state: optc31_read_reg_state,
    ..timing_generator_funcs::default()
};

pub unsafe fn dcn314_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn314_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
