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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// C dependency: opp.h

macro_rules! TO_DCN10_OPP { ($opp:expr) => { container_of!($opp, dcn10_opp, base) }; }
macro_rules! OPP_SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { .$field_name = concat_idents!($reg_name, __, $field_name, $post_fix) }; }
macro_rules! OPP_REG_LIST_DCN {
    ($id:ident) => {
        SRI!(FMT_BIT_DEPTH_CONTROL, FMT, $id), SRI!(FMT_CONTROL, FMT, $id),
        SRI!(FMT_DITHER_RAND_R_SEED, FMT, $id), SRI!(FMT_DITHER_RAND_G_SEED, FMT, $id),
        SRI!(FMT_DITHER_RAND_B_SEED, FMT, $id), SRI!(FMT_CLAMP_CNTL, FMT, $id),
        SRI!(FMT_DYNAMIC_EXP_CNTL, FMT, $id), SRI!(FMT_MAP420_MEMORY_CONTROL, FMT, $id),
        SRI!(OPPBUF_CONTROL, OPPBUF, $id), SRI!(OPPBUF_3D_PARAMETERS_0, OPPBUF, $id),
        SRI!(OPPBUF_3D_PARAMETERS_1, OPPBUF, $id), SRI!(OPP_PIPE_CONTROL, OPP_PIPE, $id)
    };
}
macro_rules! OPP_REG_LIST_DCN10 { ($id:ident) => { OPP_REG_LIST_DCN!($id) }; }

#[repr(C)]
pub struct dcn10_opp_registers {
    pub FMT_BIT_DEPTH_CONTROL: u32, pub FMT_CONTROL: u32,
    pub FMT_DITHER_RAND_R_SEED: u32, pub FMT_DITHER_RAND_G_SEED: u32,
    pub FMT_DITHER_RAND_B_SEED: u32, pub FMT_CLAMP_CNTL: u32,
    pub FMT_DYNAMIC_EXP_CNTL: u32, pub FMT_MAP420_MEMORY_CONTROL: u32,
    pub OPPBUF_CONTROL: u32, pub OPPBUF_CONTROL1: u32,
    pub OPPBUF_3D_PARAMETERS_0: u32, pub OPPBUF_3D_PARAMETERS_1: u32,
    pub OPP_PIPE_CONTROL: u32, pub OPP_PIPE_CRC_CONTROL: u32,
}

#[repr(C)]
pub struct dcn10_opp_shift {
    pub FMT_TRUNCATE_EN: u8, pub FMT_TRUNCATE_DEPTH: u8, pub FMT_TRUNCATE_MODE: u8,
    pub FMT_SPATIAL_DITHER_EN: u8, pub FMT_SPATIAL_DITHER_MODE: u8, pub FMT_SPATIAL_DITHER_DEPTH: u8,
    pub FMT_TEMPORAL_DITHER_EN: u8, pub FMT_HIGHPASS_RANDOM_ENABLE: u8,
    pub FMT_FRAME_RANDOM_ENABLE: u8, pub FMT_RGB_RANDOM_ENABLE: u8,
    pub FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX: u8, pub FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP: u8,
    pub FMT_RAND_R_SEED: u8, pub FMT_RAND_G_SEED: u8, pub FMT_RAND_B_SEED: u8,
    pub FMT_PIXEL_ENCODING: u8, pub FMT_SUBSAMPLING_MODE: u8,
    pub FMT_CBCR_BIT_REDUCTION_BYPASS: u8, pub FMT_CLAMP_DATA_EN: u8,
    pub FMT_CLAMP_COLOR_FORMAT: u8, pub FMT_DYNAMIC_EXP_EN: u8, pub FMT_DYNAMIC_EXP_MODE: u8,
    pub FMT_MAP420MEM_PWR_FORCE: u8, pub FMT_STEREOSYNC_OVERRIDE: u8,
    pub OPPBUF_ACTIVE_WIDTH: u8, pub OPPBUF_PIXEL_REPETITION: u8,
    pub OPPBUF_DISPLAY_SEGMENTATION: u8, pub OPPBUF_OVERLAP_PIXEL_NUM: u8,
    pub OPPBUF_NUM_SEGMENT_PADDED_PIXELS: u8, pub OPPBUF_3D_VACT_SPACE1_SIZE: u8,
    pub OPPBUF_3D_VACT_SPACE2_SIZE: u8, pub OPP_PIPE_CLOCK_EN: u8,
}

#[repr(C)]
pub struct dcn10_opp_mask {
    pub FMT_TRUNCATE_EN: u32, pub FMT_TRUNCATE_DEPTH: u32, pub FMT_TRUNCATE_MODE: u32,
    pub FMT_SPATIAL_DITHER_EN: u32, pub FMT_SPATIAL_DITHER_MODE: u32, pub FMT_SPATIAL_DITHER_DEPTH: u32,
    pub FMT_TEMPORAL_DITHER_EN: u32, pub FMT_HIGHPASS_RANDOM_ENABLE: u32,
    pub FMT_FRAME_RANDOM_ENABLE: u32, pub FMT_RGB_RANDOM_ENABLE: u32,
    pub FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX: u32, pub FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP: u32,
    pub FMT_RAND_R_SEED: u32, pub FMT_RAND_G_SEED: u32, pub FMT_RAND_B_SEED: u32,
    pub FMT_PIXEL_ENCODING: u32, pub FMT_SUBSAMPLING_MODE: u32, pub FMT_CBCR_BIT_REDUCTION_BYPASS: u32,
    pub FMT_CLAMP_DATA_EN: u32, pub FMT_CLAMP_COLOR_FORMAT: u32, pub FMT_DYNAMIC_EXP_EN: u32,
    pub FMT_DYNAMIC_EXP_MODE: u32, pub FMT_MAP420MEM_PWR_FORCE: u32, pub FMT_STEREOSYNC_OVERRIDE: u32,
    pub OPPBUF_ACTIVE_WIDTH: u32, pub OPPBUF_PIXEL_REPETITION: u32, pub OPPBUF_DISPLAY_SEGMENTATION: u32,
    pub OPPBUF_OVERLAP_PIXEL_NUM: u32, pub OPPBUF_NUM_SEGMENT_PADDED_PIXELS: u32,
    pub OPPBUF_3D_VACT_SPACE1_SIZE: u32, pub OPPBUF_3D_VACT_SPACE2_SIZE: u32, pub OPP_PIPE_CLOCK_EN: u32,
}

#[repr(C)]
pub struct dcn10_opp {
    pub base: output_pixel_processor,
    pub regs: *const dcn10_opp_registers,
    pub opp_shift: *const dcn10_opp_shift,
    pub opp_mask: *const dcn10_opp_mask,
    pub is_write_to_ram_a_safe: bool,
}

extern "C" {
    pub fn dcn10_opp_construct(oppn10: *mut dcn10_opp, ctx: *mut dc_context, inst: u32,
        regs: *const dcn10_opp_registers, opp_shift: *const dcn10_opp_shift, opp_mask: *const dcn10_opp_mask);
    pub fn opp1_set_dyn_expansion(opp: *mut output_pixel_processor, color_sp: dc_color_space,
        color_dpth: dc_color_depth, signal: signal_type);
    pub fn opp1_program_fmt(opp: *mut output_pixel_processor, fmt_bit_depth: *mut bit_depth_reduction_params,
        clamping: *mut clamping_and_pixel_encoding_params);
    pub fn opp1_program_bit_depth_reduction(opp: *mut output_pixel_processor, params: *const bit_depth_reduction_params);
    pub fn opp1_program_stereo(opp: *mut output_pixel_processor, enable: bool, timing: *const dc_crtc_timing);
    pub fn opp1_pipe_clock_control(opp: *mut output_pixel_processor, enable: bool);
    pub fn opp1_destroy(opp: *mut *mut output_pixel_processor);
    pub fn opp1_read_reg_state(opp: *mut output_pixel_processor, opp_reg_state: *mut dcn_opp_reg_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
