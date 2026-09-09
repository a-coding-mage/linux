/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency: ipp.h

// C preprocessor register-list macros are retained as Rust macro declarations.
macro_rules! TO_DCE_IPP { ($ipp:expr) => { container_of!($ipp, dce_ipp, base) }; }

macro_rules! IPP_COMMON_REG_LIST_DCE_BASE {
    ($id:expr) => { SRI!(CUR_UPDATE, DCP, $id), SRI!(CUR_CONTROL, DCP, $id), SRI!(CUR_POSITION, DCP, $id), SRI!(CUR_HOT_SPOT, DCP, $id), SRI!(CUR_COLOR1, DCP, $id), SRI!(CUR_COLOR2, DCP, $id), SRI!(CUR_SIZE, DCP, $id), SRI!(CUR_SURFACE_ADDRESS_HIGH, DCP, $id), SRI!(CUR_SURFACE_ADDRESS, DCP, $id), SRI!(PRESCALE_GRPH_CONTROL, DCP, $id), SRI!(PRESCALE_VALUES_GRPH_R, DCP, $id), SRI!(PRESCALE_VALUES_GRPH_G, DCP, $id), SRI!(PRESCALE_VALUES_GRPH_B, DCP, $id), SRI!(INPUT_GAMMA_CONTROL, DCP, $id), SRI!(DC_LUT_WRITE_EN_MASK, DCP, $id), SRI!(DC_LUT_RW_MODE, DCP, $id), SRI!(DC_LUT_CONTROL, DCP, $id), SRI!(DC_LUT_RW_INDEX, DCP, $id), SRI!(DC_LUT_SEQ_COLOR, DCP, $id), SRI!(DEGAMMA_CONTROL, DCP, $id) };
}
macro_rules! IPP_DCE100_REG_LIST_DCE_BASE { ($id:expr) => { IPP_COMMON_REG_LIST_DCE_BASE!($id), SRI!(DCFE_MEM_PWR_CTRL, CRTC, $id) }; }
macro_rules! IPP_DCE110_REG_LIST_DCE_BASE { ($id:expr) => { IPP_COMMON_REG_LIST_DCE_BASE!($id), SRI!(DCFE_MEM_PWR_CTRL, DCFE, $id) }; }

// The C token-pasting macro IPP_SF expands register/field/postfix names.
macro_rules! IPP_SF { ($reg:ident, $field:ident, $post_fix:ident) => { .$field = concat_idents!($reg, __, $field, $post_fix) }; }

// Mask/shift list macros preserve the original field ordering and external register symbols.
macro_rules! IPP_COMMON_MASK_SH_LIST_DCE_COMMON_BASE {
    ($m:expr) => { IPP_SF!(CUR_UPDATE, CURSOR_UPDATE_LOCK, $m), IPP_SF!(CUR_CONTROL, CURSOR_EN, $m), IPP_SF!(CUR_CONTROL, CURSOR_MODE, $m), IPP_SF!(CUR_CONTROL, CURSOR_2X_MAGNIFY, $m), IPP_SF!(CUR_CONTROL, CUR_INV_TRANS_CLAMP, $m), IPP_SF!(CUR_POSITION, CURSOR_X_POSITION, $m), IPP_SF!(CUR_POSITION, CURSOR_Y_POSITION, $m), IPP_SF!(CUR_HOT_SPOT, CURSOR_HOT_SPOT_X, $m), IPP_SF!(CUR_HOT_SPOT, CURSOR_HOT_SPOT_Y, $m), IPP_SF!(CUR_COLOR1, CUR_COLOR1_BLUE, $m), IPP_SF!(CUR_COLOR1, CUR_COLOR1_GREEN, $m), IPP_SF!(CUR_COLOR1, CUR_COLOR1_RED, $m), IPP_SF!(CUR_COLOR2, CUR_COLOR2_BLUE, $m), IPP_SF!(CUR_COLOR2, CUR_COLOR2_GREEN, $m), IPP_SF!(CUR_COLOR2, CUR_COLOR2_RED, $m), IPP_SF!(CUR_SIZE, CURSOR_WIDTH, $m), IPP_SF!(CUR_SIZE, CURSOR_HEIGHT, $m), IPP_SF!(CUR_SURFACE_ADDRESS_HIGH, CURSOR_SURFACE_ADDRESS_HIGH, $m), IPP_SF!(CUR_SURFACE_ADDRESS, CURSOR_SURFACE_ADDRESS, $m), IPP_SF!(PRESCALE_GRPH_CONTROL, GRPH_PRESCALE_BYPASS, $m), IPP_SF!(PRESCALE_VALUES_GRPH_R, GRPH_PRESCALE_SCALE_R, $m), IPP_SF!(PRESCALE_VALUES_GRPH_R, GRPH_PRESCALE_BIAS_R, $m), IPP_SF!(PRESCALE_VALUES_GRPH_G, GRPH_PRESCALE_SCALE_G, $m), IPP_SF!(PRESCALE_VALUES_GRPH_G, GRPH_PRESCALE_BIAS_G, $m), IPP_SF!(PRESCALE_VALUES_GRPH_B, GRPH_PRESCALE_SCALE_B, $m), IPP_SF!(PRESCALE_VALUES_GRPH_B, GRPH_PRESCALE_BIAS_B, $m), IPP_SF!(INPUT_GAMMA_CONTROL, GRPH_INPUT_GAMMA_MODE, $m), IPP_SF!(DC_LUT_WRITE_EN_MASK, DC_LUT_WRITE_EN_MASK, $m), IPP_SF!(DC_LUT_RW_MODE, DC_LUT_RW_MODE, $m), IPP_SF!(DC_LUT_CONTROL, DC_LUT_DATA_R_FORMAT, $m), IPP_SF!(DC_LUT_CONTROL, DC_LUT_DATA_G_FORMAT, $m), IPP_SF!(DC_LUT_CONTROL, DC_LUT_DATA_B_FORMAT, $m), IPP_SF!(DC_LUT_RW_INDEX, DC_LUT_RW_INDEX, $m), IPP_SF!(DC_LUT_SEQ_COLOR, DC_LUT_SEQ_COLOR, $m), IPP_SF!(DEGAMMA_CONTROL, GRPH_DEGAMMA_MODE, $m), IPP_SF!(DEGAMMA_CONTROL, CURSOR_DEGAMMA_MODE, $m), IPP_SF!(DEGAMMA_CONTROL, CURSOR2_DEGAMMA_MODE, $m) };
}
macro_rules! IPP_DCE100_MASK_SH_LIST_DCE_COMMON_BASE { ($m:expr) => { IPP_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($m), IPP_SF!(DCFE_MEM_PWR_CTRL, DCP_LUT_MEM_PWR_DIS, $m) }; }

#[repr(C)]
pub struct dce_ipp_shift {
    pub CURSOR_UPDATE_LOCK: u8, pub CURSOR_EN: u8, pub CURSOR_X_POSITION: u8, pub CURSOR_Y_POSITION: u8,
    pub CURSOR_HOT_SPOT_X: u8, pub CURSOR_HOT_SPOT_Y: u8, pub CURSOR_MODE: u8, pub CURSOR_2X_MAGNIFY: u8,
    pub CUR_INV_TRANS_CLAMP: u8, pub CUR_COLOR1_BLUE: u8, pub CUR_COLOR1_GREEN: u8, pub CUR_COLOR1_RED: u8,
    pub CUR_COLOR2_BLUE: u8, pub CUR_COLOR2_GREEN: u8, pub CUR_COLOR2_RED: u8, pub CURSOR_WIDTH: u8,
    pub CURSOR_HEIGHT: u8, pub CURSOR_SURFACE_ADDRESS_HIGH: u8, pub CURSOR_SURFACE_ADDRESS: u8,
    pub GRPH_PRESCALE_BYPASS: u8, pub GRPH_PRESCALE_SCALE_R: u8, pub GRPH_PRESCALE_BIAS_R: u8,
    pub GRPH_PRESCALE_SCALE_G: u8, pub GRPH_PRESCALE_BIAS_G: u8, pub GRPH_PRESCALE_SCALE_B: u8,
    pub GRPH_PRESCALE_BIAS_B: u8, pub GRPH_INPUT_GAMMA_MODE: u8, pub DCP_LUT_MEM_PWR_DIS: u8,
    pub DC_LUT_WRITE_EN_MASK: u8, pub DC_LUT_RW_MODE: u8, pub DC_LUT_DATA_R_FORMAT: u8,
    pub DC_LUT_DATA_G_FORMAT: u8, pub DC_LUT_DATA_B_FORMAT: u8, pub DC_LUT_RW_INDEX: u8,
    pub DC_LUT_SEQ_COLOR: u8, pub GRPH_DEGAMMA_MODE: u8, pub CURSOR_DEGAMMA_MODE: u8, pub CURSOR2_DEGAMMA_MODE: u8,
}

#[repr(C)]
pub struct dce_ipp_mask { pub CURSOR_UPDATE_LOCK: u32, pub CURSOR_EN: u32, pub CURSOR_X_POSITION: u32, pub CURSOR_Y_POSITION: u32, pub CURSOR_HOT_SPOT_X: u32, pub CURSOR_HOT_SPOT_Y: u32, pub CURSOR_MODE: u32, pub CURSOR_2X_MAGNIFY: u32, pub CUR_INV_TRANS_CLAMP: u32, pub CUR_COLOR1_BLUE: u32, pub CUR_COLOR1_GREEN: u32, pub CUR_COLOR1_RED: u32, pub CUR_COLOR2_BLUE: u32, pub CUR_COLOR2_GREEN: u32, pub CUR_COLOR2_RED: u32, pub CURSOR_WIDTH: u32, pub CURSOR_HEIGHT: u32, pub CURSOR_SURFACE_ADDRESS_HIGH: u32, pub CURSOR_SURFACE_ADDRESS: u32, pub GRPH_PRESCALE_BYPASS: u32, pub GRPH_PRESCALE_SCALE_R: u32, pub GRPH_PRESCALE_BIAS_R: u32, pub GRPH_PRESCALE_SCALE_G: u32, pub GRPH_PRESCALE_BIAS_G: u32, pub GRPH_PRESCALE_SCALE_B: u32, pub GRPH_PRESCALE_BIAS_B: u32, pub GRPH_INPUT_GAMMA_MODE: u32, pub DCP_LUT_MEM_PWR_DIS: u32, pub DC_LUT_WRITE_EN_MASK: u32, pub DC_LUT_RW_MODE: u32, pub DC_LUT_DATA_R_FORMAT: u32, pub DC_LUT_DATA_G_FORMAT: u32, pub DC_LUT_DATA_B_FORMAT: u32, pub DC_LUT_RW_INDEX: u32, pub DC_LUT_SEQ_COLOR: u32, pub GRPH_DEGAMMA_MODE: u32, pub CURSOR_DEGAMMA_MODE: u32, pub CURSOR2_DEGAMMA_MODE: u32 }

#[repr(C)]
pub struct dce_ipp_registers { pub CUR_UPDATE: u32, pub CUR_CONTROL: u32, pub CUR_POSITION: u32, pub CUR_HOT_SPOT: u32, pub CUR_COLOR1: u32, pub CUR_COLOR2: u32, pub CUR_SIZE: u32, pub CUR_SURFACE_ADDRESS_HIGH: u32, pub CUR_SURFACE_ADDRESS: u32, pub PRESCALE_GRPH_CONTROL: u32, pub PRESCALE_VALUES_GRPH_R: u32, pub PRESCALE_VALUES_GRPH_G: u32, pub PRESCALE_VALUES_GRPH_B: u32, pub INPUT_GAMMA_CONTROL: u32, pub DCFE_MEM_PWR_CTRL: u32, pub DC_LUT_WRITE_EN_MASK: u32, pub DC_LUT_RW_MODE: u32, pub DC_LUT_CONTROL: u32, pub DC_LUT_RW_INDEX: u32, pub DC_LUT_SEQ_COLOR: u32, pub DEGAMMA_CONTROL: u32 }

#[repr(C)]
pub struct dce_ipp { pub base: input_pixel_processor, pub regs: *const dce_ipp_registers, pub ipp_shift: *const dce_ipp_shift, pub ipp_mask: *const dce_ipp_mask }

extern "C" {
    pub fn dce_ipp_construct(ipp_dce: *mut dce_ipp, ctx: *mut dc_context, inst: ::core::ffi::c_int, regs: *const dce_ipp_registers, ipp_shift: *const dce_ipp_shift, ipp_mask: *const dce_ipp_mask);
    pub fn dce_ipp_destroy(ipp: *mut *mut input_pixel_processor);
    // Enabled when CONFIG_DRM_AMD_DC_SI is defined in the C build.
    pub fn dce60_ipp_construct(ipp_dce: *mut dce_ipp, ctx: *mut dc_context, inst: ::core::ffi::c_int, regs: *const dce_ipp_registers, ipp_shift: *const dce_ipp_shift, ipp_mask: *const dce_ipp_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
