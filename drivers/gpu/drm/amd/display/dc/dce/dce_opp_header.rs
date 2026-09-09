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
 */

// Dependencies: dc_types.h, opp.h, core_types.h

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dce110_opp_reg_type {
    DCE110_OPP_REG_DCP = 0,
    DCE110_OPP_REG_DCFE,
    DCE110_OPP_REG_FMT,
    DCE110_OPP_REG_MAX,
}

// The C register-list and mask-list macros expand dependency-provided register
// symbols and token-pasted field names; their expansion is intentionally kept
// as conditional source-level intent here.
// FROM_DCE11_OPP(opp) / TO_DCE110_OPP(opp): container_of(opp, dce110_opp, base)
// OPP_COMMON_REG_LIST_BASE, OPP_DCE_{60,80,100,110,112,120}_REG_LIST
// OPP_SF, OPP_COMMON_MASK_SH_LIST_DCE_{60,80,100,110,112,120}

#[repr(C)]
pub struct dce_opp_shift {
    pub FMT_DYNAMIC_EXP_EN: u8,
    pub FMT_DYNAMIC_EXP_MODE: u8,
    pub FMT_TRUNCATE_EN: u8,
    pub FMT_TRUNCATE_DEPTH: u8,
    pub FMT_TRUNCATE_MODE: u8,
    pub FMT_SPATIAL_DITHER_EN: u8,
    pub FMT_SPATIAL_DITHER_DEPTH: u8,
    pub FMT_SPATIAL_DITHER_MODE: u8,
    pub FMT_TEMPORAL_DITHER_EN: u8,
    pub FMT_TEMPORAL_DITHER_RESET: u8,
    pub FMT_TEMPORAL_DITHER_OFFSET: u8,
    pub FMT_TEMPORAL_DITHER_DEPTH: u8,
    pub FMT_TEMPORAL_LEVEL: u8,
    pub FMT_25FRC_SEL: u8,
    pub FMT_50FRC_SEL: u8,
    pub FMT_75FRC_SEL: u8,
    pub FMT_HIGHPASS_RANDOM_ENABLE: u8,
    pub FMT_FRAME_RANDOM_ENABLE: u8,
    pub FMT_RGB_RANDOM_ENABLE: u8,
    pub FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX: u8,
    pub FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP: u8,
    pub FMT_STEREOSYNC_OVERRIDE: u8,
    pub FMT_RAND_R_SEED: u8,
    pub FMT_RAND_G_SEED: u8,
    pub FMT_RAND_B_SEED: u8,
    pub FMT420_MEM0_SOURCE_SEL: u8,
    pub FMT420_MEM0_PWR_FORCE: u8,
    pub FMT_SRC_SELECT: u8,
    pub FMT_420_PIXEL_PHASE_LOCKED_CLEAR: u8,
    pub FMT_420_PIXEL_PHASE_LOCKED: u8,
    pub FMT_CLAMP_DATA_EN: u8,
    pub FMT_CLAMP_COLOR_FORMAT: u8,
    pub FMT_CLAMP_LOWER_R: u8,
    pub FMT_CLAMP_UPPER_R: u8,
    pub FMT_CLAMP_LOWER_G: u8,
    pub FMT_CLAMP_UPPER_G: u8,
    pub FMT_CLAMP_LOWER_B: u8,
    pub FMT_CLAMP_UPPER_B: u8,
    pub FMT_PIXEL_ENCODING: u8,
    pub FMT_SUBSAMPLING_ORDER: u8,
    pub FMT_SUBSAMPLING_MODE: u8,
    pub FMT_CBCR_BIT_REDUCTION_BYPASS: u8,
}

#[repr(C)]
pub struct dce_opp_mask {
    pub FMT_DYNAMIC_EXP_EN: u32, pub FMT_DYNAMIC_EXP_MODE: u32,
    pub FMT_TRUNCATE_EN: u32, pub FMT_TRUNCATE_DEPTH: u32, pub FMT_TRUNCATE_MODE: u32,
    pub FMT_SPATIAL_DITHER_EN: u32, pub FMT_SPATIAL_DITHER_DEPTH: u32, pub FMT_SPATIAL_DITHER_MODE: u32,
    pub FMT_TEMPORAL_DITHER_EN: u32, pub FMT_TEMPORAL_DITHER_RESET: u32, pub FMT_TEMPORAL_DITHER_OFFSET: u32,
    pub FMT_TEMPORAL_DITHER_DEPTH: u32, pub FMT_TEMPORAL_LEVEL: u32, pub FMT_25FRC_SEL: u32,
    pub FMT_50FRC_SEL: u32, pub FMT_75FRC_SEL: u32, pub FMT_HIGHPASS_RANDOM_ENABLE: u32,
    pub FMT_FRAME_RANDOM_ENABLE: u32, pub FMT_RGB_RANDOM_ENABLE: u32,
    pub FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX: u32, pub FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP: u32,
    pub FMT_STEREOSYNC_OVERRIDE: u32, pub FMT_RAND_R_SEED: u32, pub FMT_RAND_G_SEED: u32, pub FMT_RAND_B_SEED: u32,
    pub FMT420_MEM0_SOURCE_SEL: u32, pub FMT420_MEM0_PWR_FORCE: u32, pub FMT_SRC_SELECT: u32,
    pub FMT_420_PIXEL_PHASE_LOCKED_CLEAR: u32, pub FMT_420_PIXEL_PHASE_LOCKED: u32,
    pub FMT_CLAMP_DATA_EN: u32, pub FMT_CLAMP_COLOR_FORMAT: u32, pub FMT_CLAMP_LOWER_R: u32, pub FMT_CLAMP_UPPER_R: u32,
    pub FMT_CLAMP_LOWER_G: u32, pub FMT_CLAMP_UPPER_G: u32, pub FMT_CLAMP_LOWER_B: u32, pub FMT_CLAMP_UPPER_B: u32,
    pub FMT_PIXEL_ENCODING: u32, pub FMT_SUBSAMPLING_ORDER: u32, pub FMT_SUBSAMPLING_MODE: u32,
    pub FMT_CBCR_BIT_REDUCTION_BYPASS: u32,
}

#[repr(C)]
pub struct dce_opp_registers {
    pub FMT_DYNAMIC_EXP_CNTL: u32, pub FMT_BIT_DEPTH_CONTROL: u32, pub FMT_CONTROL: u32,
    pub FMT_DITHER_RAND_R_SEED: u32, pub FMT_DITHER_RAND_G_SEED: u32, pub FMT_DITHER_RAND_B_SEED: u32,
    pub FMT_TEMPORAL_DITHER_PATTERN_CONTROL: u32,
    pub FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_S_MATRIX: u32,
    pub FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_T_MATRIX: u32,
    pub CONTROL: u32, pub FMT_CLAMP_CNTL: u32, pub FMT_CLAMP_COMPONENT_R: u32,
    pub FMT_CLAMP_COMPONENT_G: u32, pub FMT_CLAMP_COMPONENT_B: u32,
}

#[repr(C)]
pub struct dce110_opp {
    pub base: output_pixel_processor,
    pub regs: *const dce_opp_registers,
    pub opp_shift: *const dce_opp_shift,
    pub opp_mask: *const dce_opp_mask,
}

extern "C" {
    pub fn dce110_opp_construct(opp110: *mut dce110_opp, ctx: *mut dc_context, inst: u32,
        regs: *const dce_opp_registers, opp_shift: *const dce_opp_shift, opp_mask: *const dce_opp_mask);
    #[cfg(CONFIG_DRM_AMD_DC_SI)]
    pub fn dce60_opp_construct(opp110: *mut dce110_opp, ctx: *mut dc_context, inst: u32,
        regs: *const dce_opp_registers, opp_shift: *const dce_opp_shift, opp_mask: *const dce_opp_mask);
    pub fn dce110_opp_destroy(opp: *mut *mut output_pixel_processor);
    pub fn dce110_opp_program_bit_depth_reduction(opp: *mut output_pixel_processor, params: *const bit_depth_reduction_params);
    pub fn dce110_opp_program_clamping_and_pixel_encoding(opp: *mut output_pixel_processor, params: *const clamping_and_pixel_encoding_params);
    pub fn dce110_opp_set_dyn_expansion(opp: *mut output_pixel_processor, color_sp: dc_color_space, color_dpth: dc_color_depth, signal: signal_type);
    pub fn dce110_opp_program_fmt(opp: *mut output_pixel_processor, fmt_bit_depth: *mut bit_depth_reduction_params, clamping: *mut clamping_and_pixel_encoding_params);
    pub fn dce110_opp_set_clamping(opp110: *mut dce110_opp, params: *const clamping_and_pixel_encoding_params);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
