/* Copyright 2012-15 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency: dcn10/dcn10_opp.h

macro_rules! TO_DCN20_OPP { ($opp:expr) => { container_of!($opp, dcn20_opp, base) }; }
macro_rules! OPP_SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { .$field_name = concat_idents!($reg_name, __, $field_name, $post_fix) }; }
macro_rules! OPP_DPG_REG_LIST { ($id:expr) => {
    SRI!(DPG_CONTROL, DPG, $id), SRI!(DPG_DIMENSIONS, DPG, $id),
    SRI!(DPG_OFFSET_SEGMENT, DPG, $id), SRI!(DPG_COLOUR_B_CB, DPG, $id),
    SRI!(DPG_COLOUR_G_Y, DPG, $id), SRI!(DPG_COLOUR_R_CR, DPG, $id),
    SRI!(DPG_RAMP_CONTROL, DPG, $id), SRI!(DPG_STATUS, DPG, $id)
}; }
macro_rules! OPP_REG_LIST_DCN20 { ($id:expr) => { OPP_REG_LIST_DCN10!($id), OPP_DPG_REG_LIST!($id), SRI!(FMT_422_CONTROL, FMT, $id), SRI!(OPPBUF_CONTROL1, OPPBUF, $id) }; }
macro_rules! OPP_REG_VARIABLE_LIST_DCN2_0 { () => { OPP_COMMON_REG_VARIABLE_LIST; FMT_422_CONTROL: u32; DPG_CONTROL: u32; DPG_DIMENSIONS: u32; DPG_OFFSET_SEGMENT: u32; DPG_COLOUR_B_CB: u32; DPG_COLOUR_G_Y: u32; DPG_COLOUR_R_CR: u32; DPG_RAMP_CONTROL: u32; DPG_STATUS: u32; DSCRM_DSC_FORWARD_CONFIG: u32 }; }
macro_rules! OPP_DPG_MASK_SH_LIST { ($mask_sh:ident) => {
    OPP_SF!(DPG0_DPG_CONTROL, DPG_EN, $mask_sh), OPP_SF!(DPG0_DPG_CONTROL, DPG_MODE, $mask_sh), OPP_SF!(DPG0_DPG_CONTROL, DPG_DYNAMIC_RANGE, $mask_sh), OPP_SF!(DPG0_DPG_CONTROL, DPG_BIT_DEPTH, $mask_sh), OPP_SF!(DPG0_DPG_CONTROL, DPG_VRES, $mask_sh), OPP_SF!(DPG0_DPG_CONTROL, DPG_HRES, $mask_sh), OPP_SF!(DPG0_DPG_DIMENSIONS, DPG_ACTIVE_WIDTH, $mask_sh), OPP_SF!(DPG0_DPG_DIMENSIONS, DPG_ACTIVE_HEIGHT, $mask_sh), OPP_SF!(DPG0_DPG_OFFSET_SEGMENT, DPG_X_OFFSET, $mask_sh), OPP_SF!(DPG0_DPG_OFFSET_SEGMENT, DPG_SEGMENT_WIDTH, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_R_CR, DPG_COLOUR0_R_CR, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_R_CR, DPG_COLOUR1_R_CR, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_B_CB, DPG_COLOUR0_B_CB, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_B_CB, DPG_COLOUR1_B_CB, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_G_Y, DPG_COLOUR0_G_Y, $mask_sh), OPP_SF!(DPG0_DPG_COLOUR_G_Y, DPG_COLOUR1_G_Y, $mask_sh), OPP_SF!(DPG0_DPG_RAMP_CONTROL, DPG_RAMP0_OFFSET, $mask_sh), OPP_SF!(DPG0_DPG_RAMP_CONTROL, DPG_INC0, $mask_sh), OPP_SF!(DPG0_DPG_RAMP_CONTROL, DPG_INC1, $mask_sh), OPP_SF!(DPG0_DPG_STATUS, DPG_DOUBLE_BUFFER_PENDING, $mask_sh)
}; }
macro_rules! OPP_MASK_SH_LIST_DCN20 { ($mask_sh:ident) => { OPP_MASK_SH_LIST_DCN!($mask_sh), OPP_DPG_MASK_SH_LIST!($mask_sh), OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_DISPLAY_SEGMENTATION, $mask_sh), OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_OVERLAP_PIXEL_NUM, $mask_sh), OPP_SF!(FMT0_FMT_422_CONTROL, FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT, $mask_sh) }; }
macro_rules! OPP_DCN20_REG_FIELD_LIST { ($type:ty) => { OPP_DCN10_REG_FIELD_LIST!($type); FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT: $type; DPG_EN: $type; DPG_MODE: $type; DPG_DYNAMIC_RANGE: $type; DPG_BIT_DEPTH: $type; DPG_VRES: $type; DPG_HRES: $type; DPG_ACTIVE_WIDTH: $type; DPG_ACTIVE_HEIGHT: $type; DPG_X_OFFSET: $type; DPG_SEGMENT_WIDTH: $type; DPG_COLOUR0_R_CR: $type; DPG_COLOUR1_R_CR: $type; DPG_COLOUR0_B_CB: $type; DPG_COLOUR1_B_CB: $type; DPG_COLOUR0_G_Y: $type; DPG_COLOUR1_G_Y: $type; DPG_RAMP0_OFFSET: $type; DPG_INC0: $type; DPG_INC1: $type; DPG_DOUBLE_BUFFER_PENDING: $type }; }

#[repr(C)]
pub struct dcn20_opp_registers {
    pub common: OPP_COMMON_REG_VARIABLE_LIST,
    pub FMT_422_CONTROL: u32,
    pub DPG_CONTROL: u32,
    pub DPG_DIMENSIONS: u32,
    pub DPG_OFFSET_SEGMENT: u32,
    pub DPG_COLOUR_B_CB: u32,
    pub DPG_COLOUR_G_Y: u32,
    pub DPG_COLOUR_R_CR: u32,
    pub DPG_RAMP_CONTROL: u32,
    pub DPG_STATUS: u32,
    pub DSCRM_DSC_FORWARD_CONFIG: u32,
}

#[repr(C)]
pub struct dcn20_opp_shift {
    pub base: OPP_DCN10_REG_FIELD_LIST!(u8),
    pub FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT: u8,
    pub DPG_EN: u8, pub DPG_MODE: u8, pub DPG_DYNAMIC_RANGE: u8,
    pub DPG_BIT_DEPTH: u8, pub DPG_VRES: u8, pub DPG_HRES: u8,
    pub DPG_ACTIVE_WIDTH: u8, pub DPG_ACTIVE_HEIGHT: u8,
    pub DPG_X_OFFSET: u8, pub DPG_SEGMENT_WIDTH: u8,
    pub DPG_COLOUR0_R_CR: u8, pub DPG_COLOUR1_R_CR: u8,
    pub DPG_COLOUR0_B_CB: u8, pub DPG_COLOUR1_B_CB: u8,
    pub DPG_COLOUR0_G_Y: u8, pub DPG_COLOUR1_G_Y: u8,
    pub DPG_RAMP0_OFFSET: u8, pub DPG_INC0: u8, pub DPG_INC1: u8,
    pub DPG_DOUBLE_BUFFER_PENDING: u8,
}

#[repr(C)]
pub struct dcn20_opp_mask {
    pub base: OPP_DCN10_REG_FIELD_LIST!(u32),
    pub FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT: u32,
    pub DPG_EN: u32, pub DPG_MODE: u32, pub DPG_DYNAMIC_RANGE: u32,
    pub DPG_BIT_DEPTH: u32, pub DPG_VRES: u32, pub DPG_HRES: u32,
    pub DPG_ACTIVE_WIDTH: u32, pub DPG_ACTIVE_HEIGHT: u32,
    pub DPG_X_OFFSET: u32, pub DPG_SEGMENT_WIDTH: u32,
    pub DPG_COLOUR0_R_CR: u32, pub DPG_COLOUR1_R_CR: u32,
    pub DPG_COLOUR0_B_CB: u32, pub DPG_COLOUR1_B_CB: u32,
    pub DPG_COLOUR0_G_Y: u32, pub DPG_COLOUR1_G_Y: u32,
    pub DPG_RAMP0_OFFSET: u32, pub DPG_INC0: u32, pub DPG_INC1: u32,
    pub DPG_DOUBLE_BUFFER_PENDING: u32,
}

#[repr(C)]
pub struct dcn20_opp {
    pub base: output_pixel_processor,
    pub regs: *const dcn20_opp_registers,
    pub opp_shift: *const dcn20_opp_shift,
    pub opp_mask: *const dcn20_opp_mask,
    pub is_write_to_ram_a_safe: bool,
}

extern "C" {
    pub fn dcn20_opp_construct(oppn20: *mut dcn20_opp, ctx: *mut dc_context, inst: u32, regs: *const dcn20_opp_registers, opp_shift: *const dcn20_opp_shift, opp_mask: *const dcn20_opp_mask);
    pub fn opp2_set_disp_pattern_generator(opp: *mut output_pixel_processor, test_pattern: controller_dp_test_pattern, color_space: controller_dp_color_space, color_depth: dc_color_depth, solid_color: *const tg_color, width: i32, height: i32, offset: i32);
    pub fn opp2_program_dpg_dimensions(opp: *mut output_pixel_processor, width: u32, height: u32);
    pub fn opp2_dpg_is_blanked(opp: *mut output_pixel_processor) -> bool;
    pub fn opp2_dpg_is_pending(opp: *mut output_pixel_processor) -> bool;
    pub fn opp2_dpg_set_blank_color(opp: *mut output_pixel_processor, color: *const tg_color);
    pub fn opp2_program_left_edge_extra_pixel(opp: *mut output_pixel_processor, pixel_encoding: dc_pixel_encoding, is_primary: bool);
    pub fn opp2_get_left_edge_extra_pixel_count(opp: *mut output_pixel_processor, pixel_encoding: dc_pixel_encoding, is_primary: bool) -> u32;
    pub fn opp2_read_reg_state(opp: *mut output_pixel_processor, opp_reg_state: *mut dcn_opp_reg_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
