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

// Dependency supplied by dcn20/dcn20_optc.h.

macro_rules! V_TOTAL_REGS_DCN30_SRI {
    ($inst:expr) => { SRI!(OTG_V_TOTAL_MID, OTG, $inst), SRI!(OTG_DRR_V_TOTAL_REACH_RANGE, OTG, $inst), };
}

macro_rules! OPTC_COMMON_REG_LIST_DCN3_BASE {
    ($inst:expr) => {
        SRI!(OTG_VSTARTUP_PARAM, OTG, $inst), SRI!(OTG_VUPDATE_PARAM, OTG, $inst), SRI!(OTG_VREADY_PARAM, OTG, $inst),
        SRI!(OTG_MASTER_UPDATE_LOCK, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL0, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL1, OTG, $inst),
        SRI!(OTG_GLOBAL_CONTROL2, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL4, OTG, $inst), SRI!(OTG_DOUBLE_BUFFER_CONTROL, OTG, $inst),
        SRI!(OTG_H_TOTAL, OTG, $inst), SRI!(OTG_H_BLANK_START_END, OTG, $inst), SRI!(OTG_H_SYNC_A, OTG, $inst),
        SRI!(OTG_H_SYNC_A_CNTL, OTG, $inst), SRI!(OTG_H_TIMING_CNTL, OTG, $inst), SRI!(OTG_V_TOTAL, OTG, $inst),
        SRI!(OTG_V_BLANK_START_END, OTG, $inst), SRI!(OTG_V_SYNC_A, OTG, $inst), SRI!(OTG_V_SYNC_A_CNTL, OTG, $inst),
        SRI!(OTG_CONTROL, OTG, $inst), SRI!(OTG_STEREO_CONTROL, OTG, $inst), SRI!(OTG_3D_STRUCTURE_CONTROL, OTG, $inst),
        SRI!(OTG_STEREO_STATUS, OTG, $inst), SRI!(OTG_V_TOTAL_MAX, OTG, $inst), SRI!(OTG_V_TOTAL_MIN, OTG, $inst),
        SRI!(OTG_V_TOTAL_CONTROL, OTG, $inst), V_TOTAL_REGS_DCN30_SRI!($inst), SRI!(OTG_TRIGA_CNTL, OTG, $inst),
        SRI!(OTG_FORCE_COUNT_NOW_CNTL, OTG, $inst), SRI!(OTG_STATIC_SCREEN_CONTROL, OTG, $inst), SRI!(OTG_STATUS_FRAME_COUNT, OTG, $inst),
        SRI!(OTG_STATUS, OTG, $inst), SRI!(OTG_STATUS_POSITION, OTG, $inst), SRI!(OTG_NOM_VERT_POSITION, OTG, $inst),
        SRI!(OTG_BLANK_DATA_COLOR, OTG, $inst), SRI!(OTG_BLANK_DATA_COLOR_EXT, OTG, $inst), SRI!(OTG_M_CONST_DTO0, OTG, $inst),
        SRI!(OTG_M_CONST_DTO1, OTG, $inst), SRI!(OTG_CLOCK_CONTROL, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT0_CONTROL, OTG, $inst),
        SRI!(OTG_VERTICAL_INTERRUPT0_POSITION, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT1_CONTROL, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT1_POSITION, OTG, $inst),
        SRI!(OTG_VERTICAL_INTERRUPT2_CONTROL, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT2_POSITION, OTG, $inst), SRI!(OPTC_INPUT_CLOCK_CONTROL, ODM, $inst),
        SRI!(OPTC_DATA_SOURCE_SELECT, ODM, $inst), SRI!(OPTC_INPUT_GLOBAL_CONTROL, ODM, $inst), SRI!(CONTROL, VTG, $inst),
        SRI!(OTG_VERT_SYNC_CONTROL, OTG, $inst), SRI!(OTG_GSL_CONTROL, OTG, $inst), SRI!(OTG_CRC_CNTL, OTG, $inst),
        SRI!(OTG_CRC_CNTL2, OTG, $inst), SRI!(OTG_CRC0_DATA_RG, OTG, $inst), SRI!(OTG_CRC0_DATA_B, OTG, $inst),
        SRI!(OTG_CRC0_WINDOWA_X_CONTROL, OTG, $inst), SRI!(OTG_CRC0_WINDOWA_Y_CONTROL, OTG, $inst), SRI!(OTG_CRC0_WINDOWB_X_CONTROL, OTG, $inst),
        SRI!(OTG_CRC0_WINDOWB_Y_CONTROL, OTG, $inst), SR!(GSL_SOURCE_SELECT), SRI!(OTG_TRIGA_MANUAL_TRIG, OTG, $inst), SRI!(OTG_DRR_CONTROL, OTG, $inst)
    };
}

macro_rules! OPTC_COMMON_REG_LIST_DCN3_0 {
    ($inst:expr) => { OPTC_COMMON_REG_LIST_DCN3_BASE!($inst), SRI!(OTG_GLOBAL_CONTROL1, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL2, OTG, $inst), SRI!(OTG_GSL_WINDOW_X, OTG, $inst), SRI!(OTG_GSL_WINDOW_Y, OTG, $inst), SRI!(OTG_VUPDATE_KEEPOUT, OTG, $inst), SRI!(OTG_DSC_START_POSITION, OTG, $inst), SRI!(OTG_CRC_CNTL2, OTG, $inst), SRI!(OTG_DRR_TRIGGER_WINDOW, OTG, $inst), SRI!(OTG_DRR_V_TOTAL_CHANGE, OTG, $inst), SRI!(OPTC_DATA_FORMAT_CONTROL, ODM, $inst), SRI!(OPTC_BYTES_PER_PIXEL, ODM, $inst), SRI!(OPTC_WIDTH_CONTROL, ODM, $inst), SRI!(OPTC_MEMORY_CONFIG, ODM, $inst), SR!(DWB_SOURCE_SELECT), SRI!(OTG_PIPE_UPDATE_STATUS, OTG, $inst) };
}

macro_rules! DCN30_VTOTAL_REGS_SF {
    ($mask_sh:expr) => { SF!(OTG0_OTG_DRR_V_TOTAL_REACH_RANGE, OTG_DRR_V_TOTAL_REACH_LOWER_RANGE, $mask_sh), SF!(OTG0_OTG_DRR_V_TOTAL_REACH_RANGE, OTG_DRR_V_TOTAL_REACH_UPPER_RANGE, $mask_sh), };
}

// Field-list macros retain the complete source-level register/field tuples.
macro_rules! OPTC_COMMON_MASK_SH_LIST_DCN3_BASE { ($mask_sh:expr) => {
    SF!(OTG0_OTG_VSTARTUP_PARAM, VSTARTUP_START, $mask_sh), SF!(OTG0_OTG_VUPDATE_PARAM, VUPDATE_OFFSET, $mask_sh), SF!(OTG0_OTG_VUPDATE_PARAM, VUPDATE_WIDTH, $mask_sh), SF!(OTG0_OTG_VREADY_PARAM, VREADY_OFFSET, $mask_sh),
    SF!(OTG0_OTG_MASTER_UPDATE_LOCK, OTG_MASTER_UPDATE_LOCK, $mask_sh), SF!(OTG0_OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_START_X, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_END_X, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_EN, $mask_sh),
    SF!(OTG0_OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_START_Y, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_END_Y, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL2, OTG_MASTER_UPDATE_LOCK_SEL, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_X, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_Y, $mask_sh), SF!(OTG0_OTG_DOUBLE_BUFFER_CONTROL, OTG_UPDATE_PENDING, $mask_sh),
    DCN30_VTOTAL_REGS_SF!($mask_sh), SF!(OTG0_OTG_FORCE_COUNT_NOW_CNTL, OTG_FORCE_COUNT_NOW_CLEAR, $mask_sh), SF!(OTG0_OTG_FORCE_COUNT_NOW_CNTL, OTG_FORCE_COUNT_NOW_MODE, $mask_sh), SF!(OTG0_OTG_FORCE_COUNT_NOW_CNTL, OTG_FORCE_COUNT_NOW_OCCURRED, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_SOURCE_SELECT, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_SOURCE_PIPE_SELECT, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_RISING_EDGE_DETECT_CNTL, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_FALLING_EDGE_DETECT_CNTL, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_POLARITY_SELECT, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_FREQUENCY_SELECT, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_DELAY, $mask_sh), SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_CLEAR, $mask_sh),
    SF!(OTG0_OTG_TRIGA_MANUAL_TRIG, OTG_TRIGA_MANUAL_TRIG, $mask_sh), SF!(GSL_SOURCE_SELECT, GSL0_READY_SOURCE_SEL, $mask_sh), SF!(GSL_SOURCE_SELECT, GSL1_READY_SOURCE_SEL, $mask_sh), SF!(GSL_SOURCE_SELECT, GSL2_READY_SOURCE_SEL, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL2, MANUAL_FLOW_CONTROL_SEL, $mask_sh), SF!(OTG0_OTG_DRR_CONTROL, OTG_V_TOTAL_LAST_USED_BY_DRR, $mask_sh)
}; }

macro_rules! OPTC_COMMON_MASK_SH_LIST_DCN3_0 { ($mask_sh:expr) => { OPTC_COMMON_MASK_SH_LIST_DCN3_BASE!($mask_sh), }; }
macro_rules! OPTC_COMMON_MASK_SH_LIST_DCN30 { ($mask_sh:expr) => { OPTC_COMMON_MASK_SH_LIST_DCN3_BASE!($mask_sh), }; }

extern "C" {
    pub fn dcn30_timing_generator_init(optc1: *mut optc);
    pub fn optc3_set_out_mux(optc: *mut timing_generator, dest: otg_out_mux_dest);
    pub fn optc3_lock(optc: *mut timing_generator);
    pub fn optc3_lock_doublebuffer_enable(optc: *mut timing_generator);
    pub fn optc3_lock_doublebuffer_disable(optc: *mut timing_generator);
    pub fn optc3_set_drr_trigger_window(optc: *mut timing_generator, window_start: u32, window_end: u32);
    pub fn optc3_triplebuffer_lock(optc: *mut timing_generator);
    pub fn optc3_program_blank_color(optc: *mut timing_generator, blank_color: *const tg_color);
    pub fn optc3_set_vtotal_change_limit(optc: *mut timing_generator, limit: u32);
    pub fn optc3_set_dsc_config(optc: *mut timing_generator, dsc_mode: optc_dsc_mode, dsc_bytes_per_pixel: u32, dsc_slice_width: u32);
    pub fn optc3_set_timing_db_mode(optc: *mut timing_generator, enable: bool);
    pub fn optc3_set_odm_bypass(optc: *mut timing_generator, dc_crtc_timing: *const dc_crtc_timing);
    pub fn optc3_set_odm_combine(optc: *mut timing_generator, opp_id: *mut i32, opp_cnt: i32, segment_width: i32, last_segment_width: i32);
    pub fn optc3_wait_drr_doublebuffer_pending_clear(optc: *mut timing_generator);
    pub fn optc3_tg_init(optc: *mut timing_generator);
    pub fn optc3_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: i32, vtotal_max: i32);
    pub fn optc3_get_optc_double_buffer_pending(optc: *mut timing_generator) -> bool;
    pub fn optc3_get_otg_update_pending(optc: *mut timing_generator) -> bool;
    pub fn optc3_get_pipe_update_pending(optc: *mut timing_generator) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
