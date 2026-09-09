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

// C dependencies: reg_helper.h, dcn30_optc.h, dc.h, dcn_calc_math.h,
// dc_dmub_srv.h, dml/dcn30/dcn30_fpu.h, and dc_trace.h.

macro_rules! REG { ($optc1:expr, $reg:ident) => { $optc1.tg_regs.$reg }; }
macro_rules! CTX { ($optc1:expr) => { $optc1.base.ctx }; }
macro_rules! FN { ($optc1:expr, $field:ident) => { ($optc1.tg_shift.$field, $optc1.tg_mask.$field) }; }

pub unsafe fn optc3_triplebuffer_lock(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL2, OTG_MASTER_UPDATE_LOCK_SEL, (*optc).inst);
    REG_SET!(optc1, OTG_VUPDATE_KEEPOUT, 0, OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, 1);
    REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, 1);
    REG_WAIT!(optc1, OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, 1, 1, 10);
    TRACE_OPTC_LOCK_UNLOCK_STATE!(optc1, (*optc).inst, true);
}

pub unsafe fn optc3_lock_doublebuffer_enable(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    let mut v_blank_start: u32 = 0; let mut v_blank_end: u32 = 0;
    let mut h_blank_start: u32 = 0; let mut h_blank_end: u32 = 0;
    REG_GET_2!(optc1, OTG_V_BLANK_START_END, OTG_V_BLANK_START, &mut v_blank_start, OTG_V_BLANK_END, &mut v_blank_end);
    REG_GET_2!(optc1, OTG_H_BLANK_START_END, OTG_H_BLANK_START, &mut h_blank_start, OTG_H_BLANK_END, &mut h_blank_end);
    REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_START_Y, v_blank_start - 1, MASTER_UPDATE_LOCK_DB_END_Y, v_blank_start);
    REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_X, h_blank_start - 180 - 1, DIG_UPDATE_POSITION_Y, v_blank_start - 1);
    // there is a DIG_UPDATE_VCOUNT_MODE and it is 0.
    REG_UPDATE_3!(optc1, OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_START_X, h_blank_start - 200 - 1, MASTER_UPDATE_LOCK_DB_END_X, h_blank_start - 180, MASTER_UPDATE_LOCK_DB_EN, 1);
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL2, GLOBAL_UPDATE_LOCK_EN, 1);
    REG_SET_3!(optc1, OTG_VUPDATE_KEEPOUT, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_START_OFFSET, 0, MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_END_OFFSET, 100, OTG_MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_EN, 1);
    TRACE_OPTC_LOCK_UNLOCK_STATE!(optc1, (*optc).inst, true);
}

pub unsafe fn optc3_lock_doublebuffer_disable(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_START_X, 0, MASTER_UPDATE_LOCK_DB_END_X, 0);
    REG_UPDATE_2!(optc1, OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_START_Y, 0, MASTER_UPDATE_LOCK_DB_END_Y, 0);
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL2, GLOBAL_UPDATE_LOCK_EN, 0);
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_EN, 0);
    TRACE_OPTC_LOCK_UNLOCK_STATE!(optc1, (*optc).inst, true);
}

pub unsafe fn optc3_lock(optc: *mut timing_generator) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_UPDATE!(optc1, OTG_GLOBAL_CONTROL2, OTG_MASTER_UPDATE_LOCK_SEL, (*optc).inst);
    REG_SET!(optc1, OTG_MASTER_UPDATE_LOCK, 0, OTG_MASTER_UPDATE_LOCK, 1);
    REG_WAIT!(optc1, OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, 1, 1, 10);
    TRACE_OPTC_LOCK_UNLOCK_STATE!(optc1, (*optc).inst, true);
}

pub unsafe fn optc3_set_out_mux(optc: *mut timing_generator, dest: otg_out_mux_dest) { let optc1 = DCN10TG_FROM_TG(optc); REG_UPDATE!(optc1, OTG_CONTROL, OTG_OUT_MUX, dest); }

pub unsafe fn optc3_program_blank_color(optc: *mut timing_generator, blank_color: *const tg_color) {
    let optc1 = DCN10TG_FROM_TG(optc);
    REG_SET_3!(optc1, OTG_BLANK_DATA_COLOR, 0, OTG_BLANK_DATA_COLOR_BLUE_CB, (*blank_color).color_b_cb, OTG_BLANK_DATA_COLOR_GREEN_Y, (*blank_color).color_g_y, OTG_BLANK_DATA_COLOR_RED_CR, (*blank_color).color_r_cr);
    REG_SET_3!(optc1, OTG_BLANK_DATA_COLOR_EXT, 0, OTG_BLANK_DATA_COLOR_BLUE_CB_EXT, (*blank_color).color_b_cb >> 10, OTG_BLANK_DATA_COLOR_GREEN_Y_EXT, (*blank_color).color_g_y >> 10, OTG_BLANK_DATA_COLOR_RED_CR_EXT, (*blank_color).color_r_cr >> 10);
}

pub unsafe fn optc3_set_drr_trigger_window(optc: *mut timing_generator, window_start: u32, window_end: u32) { let optc1 = DCN10TG_FROM_TG(optc); REG_SET_2!(optc1, OTG_DRR_TRIGGER_WINDOW, 0, OTG_DRR_TRIGGER_WINDOW_START_X, window_start, OTG_DRR_TRIGGER_WINDOW_END_X, window_end); }
pub unsafe fn optc3_set_vtotal_change_limit(optc: *mut timing_generator, limit: u32) { let optc1 = DCN10TG_FROM_TG(optc); REG_SET!(optc1, OTG_DRR_V_TOTAL_CHANGE, 0, OTG_DRR_V_TOTAL_CHANGE_LIMIT, limit); }

pub unsafe fn optc3_set_dsc_config(optc: *mut timing_generator, dsc_mode: optc_dsc_mode, dsc_bytes_per_pixel: u32, dsc_slice_width: u32) {
    let optc1 = DCN10TG_FROM_TG(optc);
    optc2_set_dsc_config(optc, dsc_mode, dsc_bytes_per_pixel, dsc_slice_width);
    if dsc_mode != OPTC_DSC_DISABLED && (*optc1).signal == SIGNAL_TYPE_HDMI_FRL { REG_UPDATE!(optc1, OTG_V_SYNC_A_CNTL, OTG_V_SYNC_MODE, 1); } else { REG_UPDATE!(optc1, OTG_V_SYNC_A_CNTL, OTG_V_SYNC_MODE, 0); }
}

pub unsafe fn optc3_set_odm_bypass(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing) {
    let optc1 = DCN10TG_FROM_TG(optc); let mut h_div = H_TIMING_NO_DIV;
    REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 0, OPTC_SEG0_SRC_SEL, (*optc).inst, OPTC_SEG1_SRC_SEL, 0xf, OPTC_SEG2_SRC_SEL, 0xf, OPTC_SEG3_SRC_SEL, 0xf);
    h_div = (*(*optc).funcs).is_two_pixels_per_container((*dc_crtc_timing));
    REG_UPDATE!(optc1, OTG_H_TIMING_CNTL, OTG_H_TIMING_DIV_MODE, h_div);
    REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, 0); (*optc1).opp_count = 1;
}

pub unsafe fn optc3_set_odm_combine(optc: *mut timing_generator, opp_id: *mut i32, opp_cnt: i32, segment_width: i32, last_segment_width: i32) {
    let _ = last_segment_width; let optc1 = DCN10TG_FROM_TG(optc); let mut memory_mask: u32 = 0;
    ASSERT!(opp_cnt == 2 || opp_cnt == 4);
    if opp_cnt == 2 { memory_mask = 0x3 << ((*opp_id.add(0)) * 2) | 0x3 << ((*opp_id.add(1)) * 2); } else if opp_cnt == 4 { memory_mask = 0x1 << ((*opp_id.add(0)) * 2) | 0x1 << ((*opp_id.add(1)) * 2) | 0x1 << ((*opp_id.add(2)) * 2) | 0x1 << ((*opp_id.add(3)) * 2); }
    if REG!(optc1, OPTC_MEMORY_CONFIG) != 0 { REG_SET!(optc1, OPTC_MEMORY_CONFIG, 0, OPTC_MEM_SEL, memory_mask); }
    if opp_cnt == 2 { REG_SET_3!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 1, OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1)); } else if opp_cnt == 4 { REG_SET_5!(optc1, OPTC_DATA_SOURCE_SELECT, 0, OPTC_NUM_OF_INPUT_SEGMENT, 3, OPTC_SEG0_SRC_SEL, *opp_id.add(0), OPTC_SEG1_SRC_SEL, *opp_id.add(1), OPTC_SEG2_SRC_SEL, *opp_id.add(2), OPTC_SEG3_SRC_SEL, *opp_id.add(3)); }
    REG_UPDATE!(optc1, OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, segment_width); REG_SET!(optc1, OTG_H_TIMING_CNTL, 0, OTG_H_TIMING_DIV_MODE, opp_cnt - 1); (*optc1).opp_count = opp_cnt;
}

pub unsafe fn optc3_get_optc_double_buffer_pending(optc: *mut timing_generator) -> bool { let optc1 = DCN10TG_FROM_TG(optc); let mut update_pending=0; REG_GET!(optc1, OPTC_INPUT_GLOBAL_CONTROL, OPTC_DOUBLE_BUFFER_PENDING, &mut update_pending); update_pending == 1 }
pub unsafe fn optc3_get_otg_update_pending(optc: *mut timing_generator) -> bool { let optc1 = DCN10TG_FROM_TG(optc); let mut update_pending=0; REG_GET!(optc1, OTG_DOUBLE_BUFFER_CONTROL, OTG_UPDATE_PENDING, &mut update_pending); update_pending == 1 }
pub unsafe fn optc3_get_pipe_update_pending(optc: *mut timing_generator) -> bool { let optc1 = DCN10TG_FROM_TG(optc); let mut flip_pending=0; let mut dc_update_pending=0; REG_GET_2!(optc1, OTG_PIPE_UPDATE_STATUS, OTG_FLIP_PENDING, &mut flip_pending, OTG_DC_REG_UPDATE_PENDING, &mut dc_update_pending); flip_pending == 1 || dc_update_pending == 1 }

/** optc3_set_timing_double_buffer() - DRR double buffering control */
unsafe fn optc3_set_timing_double_buffer(optc: *mut timing_generator, enable: bool) { let optc1=DCN10TG_FROM_TG(optc); let mode = if enable {2} else {0}; REG_UPDATE!(optc1, OTG_DOUBLE_BUFFER_CONTROL, OTG_DRR_TIMING_DBUF_UPDATE_MODE, mode); }
pub unsafe fn optc3_wait_drr_doublebuffer_pending_clear(optc: *mut timing_generator) { let optc1=DCN10TG_FROM_TG(optc); REG_WAIT!(optc1, OTG_DOUBLE_BUFFER_CONTROL, OTG_DRR_TIMING_DBUF_UPDATE_PENDING, 0, 2, 100000); }
pub unsafe fn optc3_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: i32, vtotal_max: i32) { let dc=(*optc).ctx.dc; if (*dc).caps.dmub_caps.mclk_sw && !(*dc).debug.disable_fams { dc_dmub_srv_drr_update_cmd(dc, (*optc).inst, vtotal_min, vtotal_max); } else { optc1_set_vtotal_min_max(optc, vtotal_min, vtotal_max); } }
pub unsafe fn optc3_tg_init(optc: *mut timing_generator) { optc3_set_timing_double_buffer(optc, true); optc1_clear_optc_underflow(optc); }

static dcn30_tg_funcs: timing_generator_funcs = timing_generator_funcs {
    validate_timing: optc1_validate_timing, program_timing: optc1_program_timing,
    setup_vertical_interrupt0: optc1_setup_vertical_interrupt0, setup_vertical_interrupt1: optc1_setup_vertical_interrupt1, setup_vertical_interrupt2: optc1_setup_vertical_interrupt2,
    program_global_sync: optc1_program_global_sync, enable_crtc: optc2_enable_crtc, disable_crtc: optc1_disable_crtc,
    is_counter_moving: optc1_is_counter_moving, get_position: optc1_get_position, get_frame_count: optc1_get_vblank_counter, get_scanoutpos: optc1_get_crtc_scanoutpos, get_otg_active_size: optc1_get_otg_active_size, set_early_control: optc1_set_early_control, wait_for_state: optc1_wait_for_state,
    set_blank_color: optc3_program_blank_color, did_triggered_reset_occur: optc1_did_triggered_reset_occur, triplebuffer_lock: optc3_triplebuffer_lock, triplebuffer_unlock: optc2_triplebuffer_unlock, enable_reset_trigger: optc1_enable_reset_trigger, enable_crtc_reset: optc1_enable_crtc_reset, disable_reset_trigger: optc1_disable_reset_trigger, lock: optc3_lock, unlock: optc1_unlock, lock_doublebuffer_enable: optc3_lock_doublebuffer_enable, lock_doublebuffer_disable: optc3_lock_doublebuffer_disable, enable_optc_clock: optc1_enable_optc_clock, set_drr: optc1_set_drr, get_last_used_drr_vtotal: optc2_get_last_used_drr_vtotal, set_vtotal_min_max: optc3_set_vtotal_min_max, set_static_screen_control: optc1_set_static_screen_control, program_stereo: optc1_program_stereo, is_stereo_left_eye: optc1_is_stereo_left_eye, tg_init: optc3_tg_init, is_tg_enabled: optc1_is_tg_enabled, is_optc_underflow_occurred: optc1_is_optc_underflow_occurred, clear_optc_underflow: optc1_clear_optc_underflow, setup_global_swap_lock: None, get_crc: optc1_get_crc, configure_crc: optc2_configure_crc, set_dsc_config: optc3_set_dsc_config, get_dsc_status: optc2_get_dsc_status, set_dwb_source: None, set_odm_bypass: optc3_set_odm_bypass, set_odm_combine: optc3_set_odm_combine, get_optc_source: optc2_get_optc_source, set_out_mux: optc3_set_out_mux, set_drr_trigger_window: optc3_set_drr_trigger_window, set_vtotal_change_limit: optc3_set_vtotal_change_limit, set_gsl: optc2_set_gsl, set_gsl_source_select: optc2_set_gsl_source_select, set_vtg_params: optc1_set_vtg_params, program_manual_trigger: optc2_program_manual_trigger, setup_manual_trigger: optc2_setup_manual_trigger, get_hw_timing: optc1_get_hw_timing, wait_drr_doublebuffer_pending_clear: optc3_wait_drr_doublebuffer_pending_clear, is_two_pixels_per_container: optc1_is_two_pixels_per_container, get_optc_double_buffer_pending: optc3_get_optc_double_buffer_pending, get_otg_double_buffer_pending: optc3_get_otg_update_pending, get_pipe_update_pending: optc3_get_pipe_update_pending, read_otg_state: optc1_read_otg_state,
};

pub unsafe fn dcn30_timing_generator_init(optc1: *mut optc) { (*optc1).base.funcs = &dcn30_tg_funcs; (*optc1).max_h_total = (*optc1).tg_mask.OTG_H_TOTAL + 1; (*optc1).max_v_total = (*optc1).tg_mask.OTG_V_TOTAL + 1; (*optc1).min_h_blank=32; (*optc1).min_v_blank=3; (*optc1).min_v_blank_interlace=5; (*optc1).min_h_sync_width=4; (*optc1).min_v_sync_width=1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
