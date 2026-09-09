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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies supplied by the surrounding translation unit.

unsafe fn optc32_set_odm_combine(optc: *mut timing_generator, opp_id: *mut i32,
    opp_cnt: i32, segment_width: i32, _last_segment_width: i32) {
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
            memory_mask = (0x1 << (*opp_id.add(0) * 2)) | (0x1 << (*opp_id.add(1) * 2));
        } else if odm_mem_count <= 4 {
            memory_mask = (0x3 << (*opp_id.add(0) * 2)) | (0x3 << (*opp_id.add(1) * 2));
        } else { memory_mask = 0x77; }
    }

    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask);
    if opp_cnt == 2 {
        REG_SET_3!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1));
    } else if opp_cnt == 4 {
        REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 3,
            OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1),
            OPTC_SEG2_SRC_SEL, *opp_id.add(2), OPTC_SEG3_SRC_SEL, *opp_id.add(3));
    }
    REG_UPDATE!(optc1, OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, opp_cnt - 1);
    (*optc1).opp_count = opp_cnt;
}

pub unsafe fn optc32_get_odm_combine_segments(tg: *mut timing_generator, out: *mut i32) {
    let optc1 = DCN10TG_FROM_TG(tg);
    let mut segments: u32 = 0;
    REG_GET!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_NUM_OF_INPUT_SEGMENT, &mut segments);
    *out = match segments { 0 => 1, 1 => 2, 3 => 4, _ => -1 };
}

pub unsafe fn optc32_wait_odm_doublebuffer_pending_clear(tg: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(tg);
    REG_WAIT!(optc1, OTG_DOUBLE_BUFFER_CONTROL, OTG_H_TIMING_DIV_MODE_DB_UPDATE_PENDING, 0, 2, 50000);
}

pub unsafe fn optc32_set_h_timing_div_manual_mode(optc: *mut timing_generator, manual_mode: bool) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE_MANUAL, if manual_mode { 1 } else { 0 });
}

unsafe fn optc32_enable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, (*optc).inst);
    REG_UPDATE!(optc1, CONTROL, VTG0_ENABLE, 1);
    REG_UPDATE_2!(optc1, OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 2, OTG_MASTER_EN, 1);
    true
}

unsafe fn optc32_disable_crtc(optc: *mut timing_generator) -> bool {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf,
        OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE!(optc1, OPTC_MEMORY_CONFIG, OPTC_MEM_SEL, 0);
    REG_UPDATE!(optc1, OTG_CONTROL, OTG_MASTER_EN, 0);
    REG_UPDATE!(optc1, CONTROL, VTG0_ENABLE, 0);
    REG_WAIT!(optc1, OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 150000);
    true
}

unsafe fn optc32_phantom_crtc_post_enable(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2!(optc1, OTG_CONTROL, OTG_DISABLE_POINT_CNTL, 0, OTG_MASTER_EN, 0);
    REG_WAIT!(optc1, OTG_CLOCK_CONTROL, OTG_BUSY, 0, 1, 100000);
}

unsafe fn optc32_disable_phantom_otg(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_5!(optc1, OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, 0xf, OPTC_SEG1_SRC_SEL, 0xf,
        OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf, OPTC_NUM_OF_INPUT_SEGMENT, 0);
    REG_UPDATE!(optc1, OTG_CONTROL, OTG_MASTER_EN, 0);
}

pub unsafe fn optc32_set_odm_bypass(optc: *mut timing_generator, timing: *const dc_crtc_timing) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let h_div = (*optc).funcs.is_two_pixels_per_container(optc, timing);
    REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 0,
        OPTC_SEG0_SRC_SEL, (*optc).inst, OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf);
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, h_div);
    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0);
    (*optc1).opp_count = 1;
}

unsafe fn optc32_setup_manual_trigger(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let dc = (*optc).ctx.dc;
    if (*dc).caps.dmub_caps.mclk_sw && !(*dc).debug.disable_fams {
        dc_dmub_srv_set_drr_manual_trigger_cmd(dc, (*optc).inst);
    } else {
        REG_UPDATE_4!(optc1, OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, 1, OTG_V_TOTAL_MAX_SEL, 1,
            OTG_FORCE_LOCK_ON_EVENT, 0, OTG_SET_V_TOTAL_MIN_MASK, 1 << 1);
    }
}

unsafe fn optc32_set_drr(optc: *mut timing_generator, params: *const drr_params) {
    let optc1 = DCN10TG_FROM_TG(optc);
    if !params.is_null() && (*params).vertical_total_max > 0 && (*params).vertical_total_min > 0 {
        if (*params).vertical_total_mid != 0 {
            REG_SET!(optc1, OTG_V_TOTAL_MID, 0, OTG_V_TOTAL_MID, (*params).vertical_total_mid - 1);
            REG_UPDATE_2!(optc1, OTG_V_TOTAL_CONTROL, OTG_VTOTAL_MID_REPLACING_MAX_EN, 1,
                OTG_VTOTAL_MID_FRAME_NUM, (*params).vertical_total_mid_frame_num as u8);
        }
        (*optc).funcs.set_vtotal_min_max(optc, (*params).vertical_total_min - 1, (*params).vertical_total_max - 1);
    }
    optc32_setup_manual_trigger(optc);
}

// Function table corresponding to the C `dcn32_tg_funcs` initializer.  The
// referenced implementations and types are provided by other translation units.
#[allow(non_upper_case_globals)]
static dcn32_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: Some(optc1_validate_timing),
    program_timing: Some(optc1_program_timing),
    setup_vertical_interrupt0: Some(optc1_setup_vertical_interrupt0),
    setup_vertical_interrupt1: Some(optc1_setup_vertical_interrupt1),
    setup_vertical_interrupt2: Some(optc1_setup_vertical_interrupt2),
    program_global_sync: Some(optc1_program_global_sync),
    enable_crtc: Some(optc32_enable_crtc),
    disable_crtc: Some(optc32_disable_crtc),
    phantom_crtc_post_enable: Some(optc32_phantom_crtc_post_enable),
    disable_phantom_crtc: Some(optc32_disable_phantom_otg),
    is_counter_moving: Some(optc1_is_counter_moving),
    get_position: Some(optc1_get_position),
    get_frame_count: Some(optc1_get_vblank_counter),
    get_scanoutpos: Some(optc1_get_crtc_scanoutpos),
    get_otg_active_size: Some(optc1_get_otg_active_size),
    set_early_control: Some(optc1_set_early_control),
    wait_for_state: Some(optc1_wait_for_state),
    set_blank_color: Some(optc3_program_blank_color),
    did_triggered_reset_occur: Some(optc1_did_triggered_reset_occur),
    triplebuffer_lock: Some(optc3_triplebuffer_lock),
    triplebuffer_unlock: Some(optc2_triplebuffer_unlock),
    enable_reset_trigger: Some(optc1_enable_reset_trigger),
    enable_crtc_reset: Some(optc1_enable_crtc_reset),
    disable_reset_trigger: Some(optc1_disable_reset_trigger),
    lock: Some(optc3_lock),
    unlock: Some(optc1_unlock),
    lock_doublebuffer_enable: Some(optc3_lock_doublebuffer_enable),
    lock_doublebuffer_disable: Some(optc3_lock_doublebuffer_disable),
    enable_optc_clock: Some(optc1_enable_optc_clock),
    set_drr: Some(optc32_set_drr),
    get_last_used_drr_vtotal: Some(optc2_get_last_used_drr_vtotal),
    set_vtotal_min_max: Some(optc3_set_vtotal_min_max),
    set_static_screen_control: Some(optc1_set_static_screen_control),
    program_stereo: Some(optc1_program_stereo),
    is_stereo_left_eye: Some(optc1_is_stereo_left_eye),
    tg_init: Some(optc3_tg_init),
    is_tg_enabled: Some(optc1_is_tg_enabled),
    is_optc_underflow_occurred: Some(optc1_is_optc_underflow_occurred),
    clear_optc_underflow: Some(optc1_clear_optc_underflow),
    setup_global_swap_lock: None,
    get_crc: Some(optc1_get_crc),
    configure_crc: Some(optc1_configure_crc),
    set_dsc_config: Some(optc3_set_dsc_config),
    get_dsc_status: Some(optc2_get_dsc_status),
    set_dwb_source: None,
    set_odm_bypass: Some(optc32_set_odm_bypass),
    set_odm_combine: Some(optc32_set_odm_combine),
    get_odm_combine_segments: Some(optc32_get_odm_combine_segments),
    wait_odm_doublebuffer_pending_clear: Some(optc32_wait_odm_doublebuffer_pending_clear),
    set_h_timing_div_manual_mode: Some(optc32_set_h_timing_div_manual_mode),
    get_optc_source: Some(optc2_get_optc_source),
    set_out_mux: Some(optc3_set_out_mux),
    set_drr_trigger_window: Some(optc3_set_drr_trigger_window),
    set_vtotal_change_limit: Some(optc3_set_vtotal_change_limit),
    set_gsl: Some(optc2_set_gsl),
    set_gsl_source_select: Some(optc2_set_gsl_source_select),
    set_vtg_params: Some(optc1_set_vtg_params),
    program_manual_trigger: Some(optc2_program_manual_trigger),
    setup_manual_trigger: Some(optc2_setup_manual_trigger),
    get_hw_timing: Some(optc1_get_hw_timing),
    is_two_pixels_per_container: Some(optc1_is_two_pixels_per_container),
    get_optc_double_buffer_pending: Some(optc3_get_optc_double_buffer_pending),
    get_otg_double_buffer_pending: Some(optc3_get_otg_update_pending),
    get_pipe_update_pending: Some(optc3_get_pipe_update_pending),
    read_otg_state: Some(optc31_read_otg_state),
    optc_read_reg_state: Some(optc31_read_reg_state),
};

pub unsafe fn dcn32_timing_generator_init(optc1: *mut optc) {
    (*optc1).base.funcs = &dcn32_tg_funcs;
    (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1;
    (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1;
    (*optc1).min_h_blank = 32;
    (*optc1).min_v_blank = 3;
    (*optc1).min_v_blank_interlace = 5;
    (*optc1).min_h_sync_width = 4;
    (*optc1).min_v_sync_width = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
