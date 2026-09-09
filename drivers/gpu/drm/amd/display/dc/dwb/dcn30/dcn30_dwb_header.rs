/* Copyright 2020 Advanced Micro Devices, Inc.
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

// C preprocessor register-list macros are represented as Rust token macros.
// Their arguments are supplied by the surrounding register-generation code.
#[macro_export]
macro_rules! TO_DCN30_DWBC { ($dwbc_base:expr) => { container_of!($dwbc_base, dcn30_dwbc, base) }; }

#[macro_export]
macro_rules! DWBC_COMMON_REG_LIST_DCN30 {
    ($inst:ident) => {
        SR!(DWB_ENABLE_CLK_CTRL); SR!(DWB_MEM_PWR_CTRL); SR!(FC_MODE_CTRL);
        SR!(FC_FLOW_CTRL); SR!(FC_WINDOW_START); SR!(FC_WINDOW_SIZE);
        SR!(FC_SOURCE_SIZE); SR!(DWB_UPDATE_CTRL); SR!(DWB_CRC_CTRL);
        SR!(DWB_CRC_MASK_R_G); SR!(DWB_CRC_MASK_B_A); SR!(DWB_CRC_VAL_R_G);
        SR!(DWB_CRC_VAL_B_A); SR!(DWB_OUT_CTRL);
        SR!(DWB_MMHUBBUB_BACKPRESSURE_CNT_EN); SR!(DWB_MMHUBBUB_BACKPRESSURE_CNT);
        SR!(DWB_HOST_READ_CONTROL); SR!(DWB_SOFT_RESET); SR!(DWB_HDR_MULT_COEF);
        SR!(DWB_GAMUT_REMAP_MODE); SR!(DWB_GAMUT_REMAP_COEF_FORMAT);
        SR!(DWB_GAMUT_REMAPA_C11_C12); SR!(DWB_GAMUT_REMAPA_C13_C14);
        SR!(DWB_GAMUT_REMAPA_C21_C22); SR!(DWB_GAMUT_REMAPA_C23_C24);
        SR!(DWB_GAMUT_REMAPA_C31_C32); SR!(DWB_GAMUT_REMAPA_C33_C34);
        SR!(DWB_GAMUT_REMAPB_C11_C12); SR!(DWB_GAMUT_REMAPB_C13_C14);
        SR!(DWB_GAMUT_REMAPB_C21_C22); SR!(DWB_GAMUT_REMAPB_C23_C24);
        SR!(DWB_GAMUT_REMAPB_C31_C32); SR!(DWB_GAMUT_REMAPB_C33_C34);
        SR!(DWB_OGAM_CONTROL); SR!(DWB_OGAM_LUT_INDEX); SR!(DWB_OGAM_LUT_DATA);
        SR!(DWB_OGAM_LUT_CONTROL);
    };
}

#[repr(C)]
pub struct dcn30_dwbc_registers {
    pub DWB_ENABLE_CLK_CTRL: u32, pub DWB_MEM_PWR_CTRL: u32,
    pub FC_MODE_CTRL: u32, pub FC_FLOW_CTRL: u32,
    pub FC_WINDOW_START: u32, pub FC_WINDOW_SIZE: u32,
    pub FC_SOURCE_SIZE: u32, pub DWB_UPDATE_CTRL: u32,
    pub DWB_CRC_CTRL: u32, pub DWB_CRC_MASK_R_G: u32,
    pub DWB_CRC_MASK_B_A: u32, pub DWB_CRC_VAL_R_G: u32,
    pub DWB_CRC_VAL_B_A: u32, pub DWB_OUT_CTRL: u32,
    pub DWB_MMHUBBUB_BACKPRESSURE_CNT_EN: u32,
    pub DWB_MMHUBBUB_BACKPRESSURE_CNT: u32, pub DWB_HOST_READ_CONTROL: u32,
    pub DWB_SOFT_RESET: u32, pub DWB_DEBUG_CTRL: u32, pub DWB_DEBUG: u32,
    pub DWB_TEST_DEBUG_INDEX: u32, pub DWB_TEST_DEBUG_DATA: u32,
    pub DWBSCL_COEF_RAM_TAP_SELECT: u32, pub DWBSCL_COEF_RAM_TAP_DATA: u32,
    pub DWBSCL_MODE: u32, pub DWBSCL_TAP_CONTROL: u32,
    pub DWBSCL_HORZ_FILTER_SCALE_RATIO: u32, pub DWBSCL_HORZ_FILTER_INIT: u32,
    pub DWBSCL_VERT_FILTER_SCALE_RATIO: u32, pub DWBSCL_VERT_FILTER_INIT: u32,
    pub DWBSCL_BOUNDARY_CTRL: u32, pub DWBSCL_DEST_SIZE: u32,
    pub DWBSCL_OVERFLOW_STATUS: u32, pub DWBSCL_OVERFLOW_COUNTER: u32,
    pub DWBSCL_DEBUG: u32, pub DWBSCL_TEST_DEBUG_INDEX: u32,
    pub DWBSCL_TEST_DEBUG_DATA: u32,
    pub DWB_HDR_MULT_COEF: u32, pub DWB_GAMUT_REMAP_MODE: u32,
    pub DWB_GAMUT_REMAP_COEF_FORMAT: u32,
    pub DWB_GAMUT_REMAPA_C11_C12: u32, pub DWB_GAMUT_REMAPA_C13_C14: u32,
    pub DWB_GAMUT_REMAPA_C21_C22: u32, pub DWB_GAMUT_REMAPA_C23_C24: u32,
    pub DWB_GAMUT_REMAPA_C31_C32: u32, pub DWB_GAMUT_REMAPA_C33_C34: u32,
    pub DWB_GAMUT_REMAPB_C11_C12: u32, pub DWB_GAMUT_REMAPB_C13_C14: u32,
    pub DWB_GAMUT_REMAPB_C21_C22: u32, pub DWB_GAMUT_REMAPB_C23_C24: u32,
    pub DWB_GAMUT_REMAPB_C31_C32: u32, pub DWB_GAMUT_REMAPB_C33_C34: u32,
    pub DWB_OGAM_CONTROL: u32, pub DWB_OGAM_LUT_INDEX: u32,
    pub DWB_OGAM_LUT_DATA: u32, pub DWB_OGAM_LUT_CONTROL: u32,
    pub DWB_OGAM_RAMA_START_CNTL_B: u32, pub DWB_OGAM_RAMA_START_CNTL_G: u32,
    pub DWB_OGAM_RAMA_START_CNTL_R: u32,
    pub DWB_OGAM_RAMB_START_CNTL_B: u32, pub DWB_OGAM_RAMB_START_CNTL_G: u32,
    pub DWB_OGAM_RAMB_START_CNTL_R: u32,
    pub DWBCP_DEBUG: u32, pub DWBCP_TEST_DEBUG_INDEX: u32,
    pub DWBCP_TEST_DEBUG_DATA: u32,
}

#[repr(i32)]
pub enum dwbscl_coef_filter_type_sel {
    DWBSCL_COEF_RAM_FILTER_TYPE_VERT_RGB = 0,
    DWBSCL_COEF_RAM_FILTER_TYPE_HORZ_RGB = 1,
}

#[repr(C)]
pub struct dcn30_dwbc_mask { pub fields: [u32; 256] }
#[repr(C)]
pub struct dcn30_dwbc_shift { pub fields: [u8; 256] }
#[repr(C)]
pub struct dcn30_dwbc {
    pub base: dwbc,
    pub dwbc_regs: *const dcn30_dwbc_registers,
    pub dwbc_shift: *const dcn30_dwbc_shift,
    pub dwbc_mask: *const dcn30_dwbc_mask,
}

extern "C" {
    pub fn dcn30_dwbc_construct(dwbc30: *mut dcn30_dwbc, ctx: *mut dc_context,
        dwbc_regs: *const dcn30_dwbc_registers, dwbc_shift: *const dcn30_dwbc_shift,
        dwbc_mask: *const dcn30_dwbc_mask, inst: i32);
    pub fn dwb3_enable(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool;
    pub fn dwb3_disable(dwbc: *mut dwbc) -> bool;
    pub fn dwb3_update(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool;
    pub fn dwb3_is_enabled(dwbc: *mut dwbc) -> bool;
    pub fn dwb3_set_fc_enable(dwbc: *mut dwbc, enable: dwb_frame_capture_enable);
    pub fn dwb3_set_stereo(dwbc: *mut dwbc, stereo_params: *mut dwb_stereo_params);
    pub fn dwb3_set_new_content(dwbc: *mut dwbc, is_new_content: bool);
    pub fn dwb3_config_fc(dwbc: *mut dwbc, params: *mut dc_dwb_params);
    pub fn dwb3_set_denorm(dwbc: *mut dwbc, params: *mut dc_dwb_params);
    pub fn dwb3_program_hdr_mult(dwbc: *mut dwbc, params: *const dc_dwb_params);
    pub fn dwb3_set_gamut_remap(dwbc: *mut dwbc, params: *const dc_dwb_params);
    pub fn dwb3_ogam_set_input_transfer_func(dwbc: *mut dwbc,
        in_transfer_func_dwb_ogam: *const dc_transfer_func) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
