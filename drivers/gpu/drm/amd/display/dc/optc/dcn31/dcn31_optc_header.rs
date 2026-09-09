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
 *
 * Authors: AMD
 */

// C dependency: dcn10/dcn10_optc.h

macro_rules! OPTC_COMMON_REG_LIST_DCN3_1 {
    ($inst:expr) => {
        SRI!(OTG_VSTARTUP_PARAM, OTG, $inst), SRI!(OTG_VUPDATE_PARAM, OTG, $inst),
        SRI!(OTG_VREADY_PARAM, OTG, $inst), SRI!(OTG_MASTER_UPDATE_LOCK, OTG, $inst),
        SRI!(OTG_GLOBAL_CONTROL0, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL1, OTG, $inst),
        SRI!(OTG_GLOBAL_CONTROL2, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL4, OTG, $inst),
        SRI!(OTG_DOUBLE_BUFFER_CONTROL, OTG, $inst), SRI!(OTG_H_TOTAL, OTG, $inst),
        SRI!(OTG_H_BLANK_START_END, OTG, $inst), SRI!(OTG_H_SYNC_A, OTG, $inst),
        SRI!(OTG_H_SYNC_A_CNTL, OTG, $inst), SRI!(OTG_H_TIMING_CNTL, OTG, $inst),
        SRI!(OTG_V_TOTAL, OTG, $inst), SRI!(OTG_V_BLANK_START_END, OTG, $inst),
        SRI!(OTG_V_SYNC_A, OTG, $inst), SRI!(OTG_V_SYNC_A_CNTL, OTG, $inst),
        SRI!(OTG_CONTROL, OTG, $inst), SRI!(OTG_STEREO_CONTROL, OTG, $inst),
        SRI!(OTG_3D_STRUCTURE_CONTROL, OTG, $inst), SRI!(OTG_STEREO_STATUS, OTG, $inst),
        SRI!(OTG_V_TOTAL_MAX, OTG, $inst), SRI!(OTG_V_TOTAL_MIN, OTG, $inst),
        SRI!(OTG_V_TOTAL_CONTROL, OTG, $inst), SRI!(OTG_TRIGA_CNTL, OTG, $inst),
        SRI!(OTG_FORCE_COUNT_NOW_CNTL, OTG, $inst), SRI!(OTG_STATIC_SCREEN_CONTROL, OTG, $inst),
        SRI!(OTG_STATUS_FRAME_COUNT, OTG, $inst), SRI!(OTG_STATUS, OTG, $inst),
        SRI!(OTG_STATUS_POSITION, OTG, $inst), SRI!(OTG_NOM_VERT_POSITION, OTG, $inst),
        SRI!(OTG_M_CONST_DTO0, OTG, $inst), SRI!(OTG_M_CONST_DTO1, OTG, $inst),
        SRI!(OTG_CLOCK_CONTROL, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT0_CONTROL, OTG, $inst),
        SRI!(OTG_VERTICAL_INTERRUPT0_POSITION, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT1_CONTROL, OTG, $inst),
        SRI!(OTG_VERTICAL_INTERRUPT1_POSITION, OTG, $inst), SRI!(OTG_VERTICAL_INTERRUPT2_CONTROL, OTG, $inst),
        SRI!(OTG_VERTICAL_INTERRUPT2_POSITION, OTG, $inst), SRI!(OPTC_INPUT_CLOCK_CONTROL, ODM, $inst),
        SRI!(OPTC_DATA_SOURCE_SELECT, ODM, $inst), SRI!(OPTC_INPUT_GLOBAL_CONTROL, ODM, $inst),
        SRI!(CONTROL, VTG, $inst), SRI!(OTG_VERT_SYNC_CONTROL, OTG, $inst),
        SRI!(OTG_GSL_CONTROL, OTG, $inst), SRI!(OTG_CRC_CNTL, OTG, $inst),
        SRI!(OTG_CRC0_DATA_RG, OTG, $inst), SRI!(OTG_CRC0_DATA_B, OTG, $inst),
        SRI!(OTG_CRC0_WINDOWA_X_CONTROL, OTG, $inst), SRI!(OTG_CRC0_WINDOWA_Y_CONTROL, OTG, $inst),
        SRI!(OTG_CRC0_WINDOWB_X_CONTROL, OTG, $inst), SRI!(OTG_CRC0_WINDOWB_Y_CONTROL, OTG, $inst),
        SR!(GSL_SOURCE_SELECT), SRI!(OTG_TRIGA_MANUAL_TRIG, OTG, $inst),
        SRI!(OTG_GLOBAL_CONTROL1, OTG, $inst), SRI!(OTG_GLOBAL_CONTROL2, OTG, $inst),
        SRI!(OTG_GSL_WINDOW_X, OTG, $inst), SRI!(OTG_GSL_WINDOW_Y, OTG, $inst),
        SRI!(OTG_VUPDATE_KEEPOUT, OTG, $inst), SRI!(OTG_DSC_START_POSITION, OTG, $inst),
        SRI!(OTG_DRR_TRIGGER_WINDOW, OTG, $inst), SRI!(OTG_DRR_V_TOTAL_CHANGE, OTG, $inst),
        SRI!(OPTC_DATA_FORMAT_CONTROL, ODM, $inst), SRI!(OPTC_BYTES_PER_PIXEL, ODM, $inst),
        SRI!(OPTC_WIDTH_CONTROL, ODM, $inst), SRI!(OPTC_MEMORY_CONFIG, ODM, $inst),
        SRI!(OTG_CRC_CNTL2, OTG, $inst), SR!(DWB_SOURCE_SELECT), SRI!(OTG_DRR_CONTROL, OTG, $inst),
        SRI!(OTG_PIPE_UPDATE_STATUS, OTG, $inst), SRI!(INTERRUPT_DEST, OTG, $inst)
    };
}

// The field-list macro is retained as a direct token-level Rust macro.
macro_rules! OPTC_COMMON_MASK_SH_LIST_DCN3_1 {
    ($mask_sh:expr) => {
        SF!(OTG0_OTG_VSTARTUP_PARAM, VSTARTUP_START, $mask_sh),
        SF!(OTG0_OTG_VUPDATE_PARAM, VUPDATE_OFFSET, $mask_sh), SF!(OTG0_OTG_VUPDATE_PARAM, VUPDATE_WIDTH, $mask_sh),
        SF!(OTG0_OTG_VREADY_PARAM, VREADY_OFFSET, $mask_sh), SF!(OTG0_OTG_MASTER_UPDATE_LOCK, OTG_MASTER_UPDATE_LOCK, $mask_sh),
        SF!(OTG0_OTG_MASTER_UPDATE_LOCK, UPDATE_LOCK_STATUS, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_START_X, $mask_sh),
        SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_END_X, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL0, MASTER_UPDATE_LOCK_DB_EN, $mask_sh),
        SF!(OTG0_OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_START_Y, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL1, MASTER_UPDATE_LOCK_DB_END_Y, $mask_sh),
        SF!(OTG0_OTG_GLOBAL_CONTROL2, OTG_MASTER_UPDATE_LOCK_SEL, $mask_sh), SF!(OTG0_OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_X, $mask_sh),
        SF!(OTG0_OTG_GLOBAL_CONTROL4, DIG_UPDATE_POSITION_Y, $mask_sh), SF!(OTG0_OTG_DOUBLE_BUFFER_CONTROL, OTG_UPDATE_PENDING, $mask_sh),
        SF!(OTG0_OTG_H_TOTAL, OTG_H_TOTAL, $mask_sh), SF!(OTG0_OTG_H_BLANK_START_END, OTG_H_BLANK_START, $mask_sh),
        SF!(OTG0_OTG_H_BLANK_START_END, OTG_H_BLANK_END, $mask_sh), SF!(OTG0_OTG_H_SYNC_A, OTG_H_SYNC_A_START, $mask_sh),
        SF!(OTG0_OTG_H_SYNC_A, OTG_H_SYNC_A_END, $mask_sh), SF!(OTG0_OTG_H_SYNC_A_CNTL, OTG_H_SYNC_A_POL, $mask_sh),
        SF!(OTG0_OTG_V_TOTAL, OTG_V_TOTAL, $mask_sh), SF!(OTG0_OTG_V_BLANK_START_END, OTG_V_BLANK_START, $mask_sh),
        SF!(OTG0_OTG_V_BLANK_START_END, OTG_V_BLANK_END, $mask_sh), SF!(OTG0_OTG_V_SYNC_A, OTG_V_SYNC_A_START, $mask_sh),
        SF!(OTG0_OTG_V_SYNC_A, OTG_V_SYNC_A_END, $mask_sh), SF!(OTG0_OTG_V_SYNC_A_CNTL, OTG_V_SYNC_A_POL, $mask_sh),
        SF!(OTG0_OTG_V_SYNC_A_CNTL, OTG_V_SYNC_MODE, $mask_sh), SF!(OTG0_OTG_CONTROL, OTG_MASTER_EN, $mask_sh),
        SF!(OTG0_OTG_CONTROL, OTG_START_POINT_CNTL, $mask_sh), SF!(OTG0_OTG_CONTROL, OTG_DISABLE_POINT_CNTL, $mask_sh),
        SF!(OTG0_OTG_CONTROL, OTG_FIELD_NUMBER_CNTL, $mask_sh), SF!(OTG0_OTG_CONTROL, OTG_OUT_MUX, $mask_sh),
        SF!(OTG0_OTG_STEREO_CONTROL, OTG_STEREO_EN, $mask_sh), SF!(OTG0_OTG_STEREO_STATUS, OTG_STEREO_CURRENT_EYE, $mask_sh),
        SF!(OTG0_OTG_3D_STRUCTURE_CONTROL, OTG_3D_STRUCTURE_EN, $mask_sh), SF!(OTG0_OTG_V_TOTAL_MAX, OTG_V_TOTAL_MAX, $mask_sh),
        SF!(OTG0_OTG_V_TOTAL_MIN, OTG_V_TOTAL_MIN, $mask_sh), SF!(OTG0_OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MIN_SEL, $mask_sh),
        SF!(OTG0_OTG_V_TOTAL_CONTROL, OTG_V_TOTAL_MAX_SEL, $mask_sh), SF!(OTG0_OTG_FORCE_COUNT_NOW_CNTL, OTG_FORCE_COUNT_NOW_CLEAR, $mask_sh),
        SF!(OTG0_OTG_TRIGA_CNTL, OTG_TRIGA_SOURCE_SELECT, $mask_sh), SF!(OTG0_OTG_STATUS_FRAME_COUNT, OTG_FRAME_COUNT, $mask_sh),
        SF!(OTG0_OTG_STATUS, OTG_V_BLANK, $mask_sh), SF!(OTG0_OTG_STATUS_POSITION, OTG_HORZ_COUNT, $mask_sh),
        SF!(OTG0_OTG_STATUS_POSITION, OTG_VERT_COUNT, $mask_sh), SF!(ODM0_OPTC_INPUT_CLOCK_CONTROL, OPTC_INPUT_CLK_EN, $mask_sh),
        SF!(ODM0_OPTC_INPUT_CLOCK_CONTROL, OPTC_INPUT_CLK_ON, $mask_sh), SF!(ODM0_OPTC_INPUT_GLOBAL_CONTROL, OPTC_UNDERFLOW_CLEAR, $mask_sh),
        SF!(VTG0_CONTROL, VTG0_ENABLE, $mask_sh), SF!(OTG0_OTG_GSL_CONTROL, OTG_GSL0_EN, $mask_sh),
        SF!(OTG0_OTG_GSL_CONTROL, OTG_GSL1_EN, $mask_sh), SF!(OTG0_OTG_GSL_CONTROL, OTG_GSL2_EN, $mask_sh),
        SF!(OTG0_OTG_GSL_CONTROL, OTG_GSL_MASTER_EN, $mask_sh), SF!(OTG0_OTG_CRC_CNTL, OTG_CRC_CONT_EN, $mask_sh),
        SF!(OTG0_OTG_CRC_CNTL, OTG_CRC0_SELECT, $mask_sh), SF!(OTG0_OTG_CRC_CNTL, OTG_CRC_EN, $mask_sh),
        SF!(OTG0_OTG_TRIGA_MANUAL_TRIG, OTG_TRIGA_MANUAL_TRIG, $mask_sh), SF!(GSL_SOURCE_SELECT, GSL0_READY_SOURCE_SEL, $mask_sh),
        SF!(GSL_SOURCE_SELECT, GSL1_READY_SOURCE_SEL, $mask_sh), SF!(GSL_SOURCE_SELECT, GSL2_READY_SOURCE_SEL, $mask_sh),
        SF!(ODM0_OPTC_DATA_SOURCE_SELECT, OPTC_SEG0_SRC_SEL, $mask_sh), SF!(ODM0_OPTC_DATA_SOURCE_SELECT, OPTC_SEG1_SRC_SEL, $mask_sh),
        SF!(ODM0_OPTC_DATA_SOURCE_SELECT, OPTC_SEG2_SRC_SEL, $mask_sh), SF!(ODM0_OPTC_DATA_SOURCE_SELECT, OPTC_SEG3_SRC_SEL, $mask_sh),
        SF!(ODM0_OPTC_DATA_SOURCE_SELECT, OPTC_NUM_OF_INPUT_SEGMENT, $mask_sh), SF!(ODM0_OPTC_MEMORY_CONFIG, OPTC_MEM_SEL, $mask_sh),
        SF!(ODM0_OPTC_DATA_FORMAT_CONTROL, OPTC_DATA_FORMAT, $mask_sh), SF!(ODM0_OPTC_DATA_FORMAT_CONTROL, OPTC_DSC_MODE, $mask_sh),
        SF!(ODM0_OPTC_BYTES_PER_PIXEL, OPTC_DSC_BYTES_PER_PIXEL, $mask_sh), SF!(ODM0_OPTC_WIDTH_CONTROL, OPTC_DSC_SLICE_WIDTH, $mask_sh),
        SF!(ODM0_OPTC_WIDTH_CONTROL, OPTC_SEGMENT_WIDTH, $mask_sh), SF!(DWB_SOURCE_SELECT, OPTC_DWB0_SOURCE_SELECT, $mask_sh),
        SF!(DWB_SOURCE_SELECT, OPTC_DWB1_SOURCE_SELECT, $mask_sh), SF!(OTG0_OTG_DRR_CONTROL, OTG_V_TOTAL_LAST_USED_BY_DRR, $mask_sh),
        SF!(OTG0_OTG_PIPE_UPDATE_STATUS, OTG_FLIP_PENDING, $mask_sh), SF!(OTG0_INTERRUPT_DEST, OTG0_IHC_OTG_VERTICAL_INTERRUPT2_DEST, $mask_sh)
    };
}

macro_rules! OPTC_COMMON_MASK_SH_LIST_DCN31X {
    ($mask_sh:expr) => { OPTC_COMMON_MASK_SH_LIST_DCN3_1!($mask_sh), SF!(OTG0_OTG_CRC_CNTL, OTG_CRC1_EN, $mask_sh) };
}

extern "C" {
    pub fn dcn31_timing_generator_init(optc1: *mut optc);
    pub fn optc31_immediate_disable_crtc(optc: *mut timing_generator) -> bool;
    pub fn optc31_set_drr(optc: *mut timing_generator, params: *const drr_params);
    pub fn optc3_init_odm(optc: *mut timing_generator);
    pub fn optc31_read_otg_state(optc: *mut timing_generator, s: *mut dcn_otg_state);
    pub fn optc31_read_reg_state(optc: *mut timing_generator, optc_reg_state: *mut dcn_optc_reg_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
