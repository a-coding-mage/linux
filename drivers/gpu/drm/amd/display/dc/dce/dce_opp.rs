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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependencies: dm_services.h, basics/conversion.h, dce_opp.h, reg_helper.h

pub const MAX_PWL_ENTRY: u32 = 128;
pub const MAX_REGIONS_NUMBER: u32 = 16;
pub const MAX_LUT_ENTRY: u32 = 256;
pub const MAX_NUMBER_OF_ENTRIES: u32 = 256;
pub const OUTPUT_CSC_MATRIX_SIZE: u32 = 12;

unsafe fn set_truncation(opp110: *mut dce110_opp, params: *const bit_depth_reduction_params) {
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 0, FMT_TRUNCATE_DEPTH, 0, FMT_TRUNCATE_MODE, 0);
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR422 {
        if (*params).flags.TRUNCATE_DEPTH == 1 { REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, 1, FMT_TRUNCATE_MODE, 0); }
        else if (*params).flags.TRUNCATE_DEPTH == 2 { REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, 2, FMT_TRUNCATE_MODE, 0); }
        return;
    }
    if (*params).flags.TRUNCATE_ENABLED == 0 { return; }
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, (*params).flags.TRUNCATE_DEPTH, FMT_TRUNCATE_MODE, (*params).flags.TRUNCATE_MODE);
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_set_truncation(opp110: *mut dce110_opp, params: *const bit_depth_reduction_params) {
    REG_UPDATE_2!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 0, FMT_TRUNCATE_DEPTH, 0);
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR422 {
        if (*params).flags.TRUNCATE_DEPTH == 1 { REG_UPDATE_2!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, 1); }
        else if (*params).flags.TRUNCATE_DEPTH == 2 { REG_UPDATE_2!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, 2); }
        return;
    }
    if (*params).flags.TRUNCATE_ENABLED == 0 { return; }
    REG_UPDATE_2!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, 1, FMT_TRUNCATE_DEPTH, (*params).flags.TRUNCATE_DEPTH);
}

unsafe fn set_spatial_dither(opp110: *mut dce110_opp, params: *const bit_depth_reduction_params) {
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_SPATIAL_DITHER_EN, 0, FMT_SPATIAL_DITHER_DEPTH, 0, FMT_SPATIAL_DITHER_MODE, 0);
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_HIGHPASS_RANDOM_ENABLE, 0, FMT_FRAME_RANDOM_ENABLE, 0, FMT_RGB_RANDOM_ENABLE, 0);
    REG_UPDATE!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_EN, 0);
    if (*params).flags.SPATIAL_DITHER_ENABLED == 0 { return; }
    if (*(*opp110).opp_mask).FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX != 0 && (*(*opp110).opp_mask).FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP != 0 {
        if (*params).flags.FRAME_RANDOM == 1 {
            if (*params).flags.SPATIAL_DITHER_DEPTH == 0 || (*params).flags.SPATIAL_DITHER_DEPTH == 1 { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 15, FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 2); }
            else if (*params).flags.SPATIAL_DITHER_DEPTH == 2 { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 3, FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 1); }
            else { return; }
        } else { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 0, FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 0); }
    }
    REG_UPDATE!(opp110, FMT_DITHER_RAND_R_SEED, FMT_RAND_R_SEED, (*params).r_seed_value);
    REG_UPDATE!(opp110, FMT_DITHER_RAND_G_SEED, FMT_RAND_G_SEED, (*params).g_seed_value);
    REG_UPDATE!(opp110, FMT_DITHER_RAND_B_SEED, FMT_RAND_B_SEED, (*params).b_seed_value);
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_HIGHPASS_RANDOM_ENABLE, (*params).flags.HIGHPASS_RANDOM, FMT_FRAME_RANDOM_ENABLE, (*params).flags.FRAME_RANDOM, FMT_RGB_RANDOM_ENABLE, (*params).flags.RGB_RANDOM);
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_SPATIAL_DITHER_DEPTH, (*params).flags.SPATIAL_DITHER_DEPTH, FMT_SPATIAL_DITHER_MODE, (*params).flags.SPATIAL_DITHER_MODE, FMT_SPATIAL_DITHER_EN, 1);
}

unsafe fn set_temporal_dither(opp110: *mut dce110_opp, params: *const bit_depth_reduction_params) {
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_EN, 0, FMT_TEMPORAL_DITHER_RESET, 0, FMT_TEMPORAL_DITHER_OFFSET, 0);
    REG_UPDATE_2!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_DEPTH, 0, FMT_TEMPORAL_LEVEL, 0);
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_25FRC_SEL, 0, FMT_50FRC_SEL, 0, FMT_75FRC_SEL, 0);
    if (*params).flags.FRAME_MODULATION_ENABLED == 0 || (*params).flags.FRAME_MODULATION_DEPTH == 2 { return; }
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_DEPTH, (*params).flags.FRAME_MODULATION_DEPTH, FMT_TEMPORAL_DITHER_RESET, 0, FMT_TEMPORAL_DITHER_OFFSET, 0);
    if REG!(opp110, FMT_TEMPORAL_DITHER_PATTERN_CONTROL) != 0 {
        REG_WRITE!(opp110, FMT_TEMPORAL_DITHER_PATTERN_CONTROL, 0);
        REG_WRITE!(opp110, FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_S_MATRIX, 0);
        REG_WRITE!(opp110, FMT_TEMPORAL_DITHER_PROGRAMMABLE_PATTERN_T_MATRIX, 0);
    }
    REG_UPDATE!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_LEVEL, (*params).flags.TEMPORAL_LEVEL);
    REG_UPDATE_3!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_25FRC_SEL, (*params).flags.FRC25, FMT_50FRC_SEL, (*params).flags.FRC50, FMT_75FRC_SEL, (*params).flags.FRC75);
    REG_UPDATE!(opp110, FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_EN, 1);
}

pub unsafe fn dce110_opp_set_clamping(opp110: *mut dce110_opp, params: *const clamping_and_pixel_encoding_params) {
    REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 0, FMT_CLAMP_COLOR_FORMAT, 0);
    match (*params).clamping_level {
        CLAMPING_FULL_RANGE => {},
        CLAMPING_LIMITED_RANGE_8BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 1),
        CLAMPING_LIMITED_RANGE_10BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 2),
        CLAMPING_LIMITED_RANGE_12BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 3),
        CLAMPING_LIMITED_RANGE_PROGRAMMABLE => {
            REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 7);
            REG_SET_2!(opp110, FMT_CLAMP_COMPONENT_R, 0, FMT_CLAMP_LOWER_R, 0x10, FMT_CLAMP_UPPER_R, 0xFEF);
            REG_SET_2!(opp110, FMT_CLAMP_COMPONENT_G, 0, FMT_CLAMP_LOWER_G, 0x10, FMT_CLAMP_UPPER_G, 0xFEF);
            REG_SET_2!(opp110, FMT_CLAMP_COMPONENT_B, 0, FMT_CLAMP_LOWER_B, 0x10, FMT_CLAMP_UPPER_B, 0xFEF);
        },
        _ => {}
    }
}

unsafe fn set_pixel_encoding(opp110: *mut dce110_opp, params: *const clamping_and_pixel_encoding_params) {
    if (*(*opp110).opp_mask).FMT_CBCR_BIT_REDUCTION_BYPASS != 0 { REG_UPDATE_3!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 0, FMT_SUBSAMPLING_MODE, 0, FMT_CBCR_BIT_REDUCTION_BYPASS, 0); }
    else { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 0, FMT_SUBSAMPLING_MODE, 0); }
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR422 { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 1, FMT_SUBSAMPLING_ORDER, 0); }
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR420 { REG_UPDATE_3!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 2, FMT_SUBSAMPLING_MODE, 2, FMT_CBCR_BIT_REDUCTION_BYPASS, 1); }
}

pub unsafe fn dce110_opp_program_bit_depth_reduction(opp: *mut output_pixel_processor, params: *const bit_depth_reduction_params) {
    let opp110 = TO_DCE110_OPP!(opp); set_truncation(opp110, params); set_spatial_dither(opp110, params); set_temporal_dither(opp110, params);
}

pub unsafe fn dce110_opp_program_clamping_and_pixel_encoding(opp: *mut output_pixel_processor, params: *const clamping_and_pixel_encoding_params) {
    let opp110 = TO_DCE110_OPP!(opp); dce110_opp_set_clamping(opp110, params); set_pixel_encoding(opp110, params);
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_opp_program_bit_depth_reduction(opp: *mut output_pixel_processor, params: *const bit_depth_reduction_params) {
    let opp110 = TO_DCE110_OPP!(opp); dce60_set_truncation(opp110, params); set_spatial_dither(opp110, params); set_temporal_dither(opp110, params);
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_opp_set_clamping(opp110: *mut dce110_opp, params: *const clamping_and_pixel_encoding_params) {
    REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 0, FMT_CLAMP_COLOR_FORMAT, 0);
    match (*params).clamping_level {
        CLAMPING_FULL_RANGE => {},
        CLAMPING_LIMITED_RANGE_8BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 1),
        CLAMPING_LIMITED_RANGE_10BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 2),
        CLAMPING_LIMITED_RANGE_12BPC => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 3),
        CLAMPING_LIMITED_RANGE_PROGRAMMABLE => REG_SET_2!(opp110, FMT_CLAMP_CNTL, 0, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 7),
        _ => {}
    }
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_set_pixel_encoding(opp110: *mut dce110_opp, params: *const clamping_and_pixel_encoding_params) {
    if (*(*opp110).opp_mask).FMT_CBCR_BIT_REDUCTION_BYPASS != 0 { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 0, FMT_CBCR_BIT_REDUCTION_BYPASS, 0); } else { REG_UPDATE!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 0); }
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR422 { REG_UPDATE!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 1); }
    if (*params).pixel_encoding == PIXEL_ENCODING_YCBCR420 { REG_UPDATE_2!(opp110, FMT_CONTROL, FMT_PIXEL_ENCODING, 2, FMT_CBCR_BIT_REDUCTION_BYPASS, 1); }
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_opp_program_clamping_and_pixel_encoding(opp: *mut output_pixel_processor, params: *const clamping_and_pixel_encoding_params) {
    let opp110 = TO_DCE110_OPP!(opp); dce60_opp_set_clamping(opp110, params); dce60_set_pixel_encoding(opp110, params);
}

unsafe fn program_formatter_420_memory(opp: *mut output_pixel_processor) {
    let opp110 = TO_DCE110_OPP!(opp); let mut fmt_mem_cntl_value: u32 = 0;
    REG_GET!(opp110, CONTROL, FMT420_MEM0_SOURCE_SEL, &mut fmt_mem_cntl_value); REG_UPDATE!(opp110, FMT_CONTROL, FMT_SRC_SELECT, fmt_mem_cntl_value); REG_UPDATE!(opp110, CONTROL, FMT420_MEM0_PWR_FORCE, 0);
}

pub unsafe fn dce110_opp_set_dyn_expansion(opp: *mut output_pixel_processor, _color_sp: dc_color_space, color_dpth: dc_color_depth, signal: signal_type) {
    let opp110 = TO_DCE110_OPP!(opp); REG_UPDATE_2!(opp110, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 0, FMT_DYNAMIC_EXP_MODE, 0);
    if signal == SIGNAL_TYPE_HDMI_TYPE_A || signal == SIGNAL_TYPE_DISPLAY_PORT || signal == SIGNAL_TYPE_DISPLAY_PORT_MST {
        match color_dpth { COLOR_DEPTH_888 => REG_UPDATE_2!(opp110, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 1, FMT_DYNAMIC_EXP_MODE, 1), COLOR_DEPTH_101010 | COLOR_DEPTH_121212 => REG_UPDATE_2!(opp110, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 1, FMT_DYNAMIC_EXP_MODE, 0), _ => {} }
    }
}

unsafe fn program_formatter_reset_dig_resync_fifo(opp: *mut output_pixel_processor) { let opp110 = TO_DCE110_OPP!(opp); REG_UPDATE!(opp110, FMT_CONTROL, FMT_420_PIXEL_PHASE_LOCKED_CLEAR, 1); REG_WAIT!(opp110, FMT_CONTROL, FMT_420_PIXEL_PHASE_LOCKED, 1, 10, 10); }

pub unsafe fn dce110_opp_program_fmt(opp: *mut output_pixel_processor, fmt_bit_depth: *mut bit_depth_reduction_params, clamping: *mut clamping_and_pixel_encoding_params) {
    if (*clamping).pixel_encoding == PIXEL_ENCODING_YCBCR420 { program_formatter_420_memory(opp); }
    dce110_opp_program_bit_depth_reduction(opp, fmt_bit_depth); dce110_opp_program_clamping_and_pixel_encoding(opp, clamping);
    if (*clamping).pixel_encoding == PIXEL_ENCODING_YCBCR420 { program_formatter_reset_dig_resync_fifo(opp); }
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
unsafe fn dce60_opp_program_fmt(opp: *mut output_pixel_processor, fmt_bit_depth: *mut bit_depth_reduction_params, clamping: *mut clamping_and_pixel_encoding_params) {
    if (*clamping).pixel_encoding == PIXEL_ENCODING_YCBCR420 { program_formatter_420_memory(opp); }
    dce60_opp_program_bit_depth_reduction(opp, fmt_bit_depth); dce60_opp_program_clamping_and_pixel_encoding(opp, clamping);
    if (*clamping).pixel_encoding == PIXEL_ENCODING_YCBCR420 { program_formatter_reset_dig_resync_fifo(opp); }
}

pub unsafe fn dce110_opp_construct(opp110: *mut dce110_opp, ctx: *mut dc_context, inst: u32, regs: *const dce_opp_registers, opp_shift: *const dce_opp_shift, opp_mask: *const dce_opp_mask) { (*opp110).base.funcs = &funcs; (*opp110).base.ctx = ctx; (*opp110).base.inst = inst; (*opp110).regs = regs; (*opp110).opp_shift = opp_shift; (*opp110).opp_mask = opp_mask; }

#[cfg(CONFIG_DRM_AMD_DC_SI)]
pub unsafe fn dce60_opp_construct(opp110: *mut dce110_opp, ctx: *mut dc_context, inst: u32, regs: *const dce_opp_registers, opp_shift: *const dce_opp_shift, opp_mask: *const dce_opp_mask) { (*opp110).base.funcs = &dce60_opp_funcs; (*opp110).base.ctx = ctx; (*opp110).base.inst = inst; (*opp110).regs = regs; (*opp110).opp_shift = opp_shift; (*opp110).opp_mask = opp_mask; }

pub unsafe fn dce110_opp_destroy(opp: *mut *mut output_pixel_processor) { if !(*opp).is_null() { kfree!(FROM_DCE11_OPP!(*opp)); } *opp = core::ptr::null_mut(); }

static funcs: opp_funcs = opp_funcs { opp_set_dyn_expansion: Some(dce110_opp_set_dyn_expansion), opp_destroy: Some(dce110_opp_destroy), opp_program_fmt: Some(dce110_opp_program_fmt), opp_program_bit_depth_reduction: Some(dce110_opp_program_bit_depth_reduction) };

#[cfg(CONFIG_DRM_AMD_DC_SI)]
static dce60_opp_funcs: opp_funcs = opp_funcs { opp_set_dyn_expansion: Some(dce110_opp_set_dyn_expansion), opp_destroy: Some(dce110_opp_destroy), opp_program_fmt: Some(dce60_opp_program_fmt), opp_program_bit_depth_reduction: Some(dce60_opp_program_bit_depth_reduction) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
