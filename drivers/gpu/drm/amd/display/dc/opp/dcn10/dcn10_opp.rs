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
 */

// C dependencies: core_types.h, dm_services.h, dcn10_opp.h, reg_helper.h

/* REG(reg) expands to oppn10->regs->reg in the C implementation. */

unsafe fn opp1_set_truncation(
    oppn10: *mut dcn10_opp,
    params: *const bit_depth_reduction_params,
) {
    REG_UPDATE_3!(oppn10, FMT_BIT_DEPTH_CONTROL,
        FMT_TRUNCATE_EN, (*params).flags.TRUNCATE_ENABLED,
        FMT_TRUNCATE_DEPTH, (*params).flags.TRUNCATE_DEPTH,
        FMT_TRUNCATE_MODE, (*params).flags.TRUNCATE_MODE);
}

unsafe fn opp1_set_spatial_dither(
    oppn10: *mut dcn10_opp,
    params: *const bit_depth_reduction_params,
) {
    /*Disable spatial (random) dithering*/
    REG_UPDATE_7!(oppn10, FMT_BIT_DEPTH_CONTROL,
        FMT_SPATIAL_DITHER_EN, 0,
        FMT_SPATIAL_DITHER_MODE, 0,
        FMT_SPATIAL_DITHER_DEPTH, 0,
        FMT_TEMPORAL_DITHER_EN, 0,
        FMT_HIGHPASS_RANDOM_ENABLE, 0,
        FMT_FRAME_RANDOM_ENABLE, 0,
        FMT_RGB_RANDOM_ENABLE, 0);

    /* only use FRAME_COUNTER_MAX if frameRandom == 1*/
    if (*params).flags.FRAME_RANDOM == 1 {
        if (*params).flags.SPATIAL_DITHER_DEPTH == 0 || (*params).flags.SPATIAL_DITHER_DEPTH == 1 {
            REG_UPDATE_2!(oppn10, FMT_CONTROL,
                FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 15,
                FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 2);
        } else if (*params).flags.SPATIAL_DITHER_DEPTH == 2 {
            REG_UPDATE_2!(oppn10, FMT_CONTROL,
                FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 3,
                FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 1);
        } else {
            return;
        }
    } else {
        REG_UPDATE_2!(oppn10, FMT_CONTROL,
            FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, 0,
            FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, 0);
    }

    /*Set seed for random values for spatial dithering for R,G,B channels*/
    REG_SET!(oppn10, FMT_DITHER_RAND_R_SEED, 0, FMT_RAND_R_SEED, (*params).r_seed_value);
    REG_SET!(oppn10, FMT_DITHER_RAND_G_SEED, 0, FMT_RAND_G_SEED, (*params).g_seed_value);
    REG_SET!(oppn10, FMT_DITHER_RAND_B_SEED, 0, FMT_RAND_B_SEED, (*params).b_seed_value);

    /* FMT_OFFSET_R_Cr, FMT_OFFSET_G_Y and FMT_OFFSET_B_Cb remain at zero. */
    REG_UPDATE_6!(oppn10, FMT_BIT_DEPTH_CONTROL,
        FMT_SPATIAL_DITHER_EN, (*params).flags.SPATIAL_DITHER_ENABLED,
        FMT_SPATIAL_DITHER_MODE, (*params).flags.SPATIAL_DITHER_MODE,
        FMT_SPATIAL_DITHER_DEPTH, (*params).flags.SPATIAL_DITHER_DEPTH,
        FMT_HIGHPASS_RANDOM_ENABLE, (*params).flags.HIGHPASS_RANDOM,
        FMT_FRAME_RANDOM_ENABLE, (*params).flags.FRAME_RANDOM,
        FMT_RGB_RANDOM_ENABLE, (*params).flags.RGB_RANDOM);
}

pub unsafe fn opp1_program_bit_depth_reduction(
    opp: *mut output_pixel_processor,
    params: *const bit_depth_reduction_params,
) {
    let oppn10 = TO_DCN10_OPP!(opp);
    opp1_set_truncation(oppn10, params);
    opp1_set_spatial_dither(oppn10, params);
    /* TODO: set_temporal_dither(oppn10, params); */
}

unsafe fn opp1_set_pixel_encoding(
    oppn10: *mut dcn10_opp,
    params: *const clamping_and_pixel_encoding_params,
) {
    let force_chroma_subsampling_1tap = (*(*oppn10).base.ctx).dc.debug.force_chroma_subsampling_1tap;
    match (*params).pixel_encoding {
        PIXEL_ENCODING_RGB | PIXEL_ENCODING_YCBCR444 => {
            REG_UPDATE_3!(oppn10, FMT_CONTROL, FMT_PIXEL_ENCODING, 0,
                FMT_SUBSAMPLING_MODE, 0, FMT_CBCR_BIT_REDUCTION_BYPASS, 0);
            REG_UPDATE!(oppn10, FMT_CONTROL, FMT_PIXEL_ENCODING, 0);
        }
        PIXEL_ENCODING_YCBCR422 => REG_UPDATE_3!(oppn10, FMT_CONTROL,
            FMT_PIXEL_ENCODING, 1, FMT_SUBSAMPLING_MODE, 2,
            FMT_CBCR_BIT_REDUCTION_BYPASS, 0),
        PIXEL_ENCODING_YCBCR420 => REG_UPDATE_3!(oppn10, FMT_CONTROL,
            FMT_PIXEL_ENCODING, 2, FMT_SUBSAMPLING_MODE, 2,
            FMT_CBCR_BIT_REDUCTION_BYPASS, 1),
        _ => {}
    }
    if force_chroma_subsampling_1tap { REG_UPDATE!(oppn10, FMT_CONTROL, FMT_SUBSAMPLING_MODE, 0); }
}

unsafe fn opp1_set_clamping(oppn10: *mut dcn10_opp, params: *const clamping_and_pixel_encoding_params) {
    REG_UPDATE_2!(oppn10, FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, 0, FMT_CLAMP_COLOR_FORMAT, 0);
    match (*params).clamping_level {
        CLAMPING_FULL_RANGE => REG_UPDATE_2!(oppn10, FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 0),
        CLAMPING_LIMITED_RANGE_8BPC => REG_UPDATE_2!(oppn10, FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 1),
        CLAMPING_LIMITED_RANGE_10BPC => REG_UPDATE_2!(oppn10, FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 2),
        CLAMPING_LIMITED_RANGE_12BPC => REG_UPDATE_2!(oppn10, FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, 1, FMT_CLAMP_COLOR_FORMAT, 3),
        CLAMPING_LIMITED_RANGE_PROGRAMMABLE => { /* TODO */ }
        _ => {}
    }
}

pub unsafe fn opp1_set_dyn_expansion(opp: *mut output_pixel_processor, color_sp: dc_color_space, color_dpth: dc_color_depth, signal: signal_type) {
    let _ = color_sp;
    let oppn10 = TO_DCN10_OPP!(opp);
    REG_UPDATE_2!(oppn10, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 0, FMT_DYNAMIC_EXP_MODE, 0);
    if (*opp).dyn_expansion == DYN_EXPANSION_DISABLE { return; }
    if signal == SIGNAL_TYPE_HDMI_TYPE_A || signal == SIGNAL_TYPE_HDMI_FRL || signal == SIGNAL_TYPE_DISPLAY_PORT || signal == SIGNAL_TYPE_DISPLAY_PORT_MST || signal == SIGNAL_TYPE_VIRTUAL {
        match color_dpth {
            COLOR_DEPTH_888 => REG_UPDATE_2!(oppn10, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 1, FMT_DYNAMIC_EXP_MODE, 1),
            COLOR_DEPTH_101010 | COLOR_DEPTH_121212 => REG_UPDATE_2!(oppn10, FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, 1, FMT_DYNAMIC_EXP_MODE, 0),
            _ => {}
        }
    }
}

unsafe fn opp1_program_clamping_and_pixel_encoding(opp: *mut output_pixel_processor, params: *const clamping_and_pixel_encoding_params) {
    let oppn10 = TO_DCN10_OPP!(opp);
    opp1_set_clamping(oppn10, params);
    opp1_set_pixel_encoding(oppn10, params);
}

pub unsafe fn opp1_program_fmt(opp: *mut output_pixel_processor, fmt_bit_depth: *mut bit_depth_reduction_params, clamping: *mut clamping_and_pixel_encoding_params) {
    let oppn10 = TO_DCN10_OPP!(opp);
    if (*clamping).pixel_encoding == PIXEL_ENCODING_YCBCR420 { REG_UPDATE!(oppn10, FMT_MAP420_MEMORY_CONTROL, FMT_MAP420MEM_PWR_FORCE, 0); }
    opp1_program_bit_depth_reduction(opp, fmt_bit_depth);
    opp1_program_clamping_and_pixel_encoding(opp, clamping);
}

pub unsafe fn opp1_program_stereo(opp: *mut output_pixel_processor, enable: bool, timing: *const dc_crtc_timing) {
    let oppn10 = TO_DCN10_OPP!(opp);
    let mut active_width = (*timing).h_addressable - (*timing).h_border_right - (*timing).h_border_right;
    let mut space1_size = (*timing).v_total - (*timing).v_addressable;
    let mut space2_size = (*timing).v_total - (*timing).v_addressable;
    if !enable { active_width = 0; space1_size = 0; space2_size = 0; }
    REG_UPDATE!(oppn10, FMT_CONTROL, FMT_STEREOSYNC_OVERRIDE, 0);
    REG_UPDATE!(oppn10, OPPBUF_CONTROL, OPPBUF_ACTIVE_WIDTH, active_width);
    if (*timing).timing_3d_format == TIMING_3D_FORMAT_FRAME_ALTERNATE { REG_UPDATE!(oppn10, OPPBUF_3D_PARAMETERS_0, OPPBUF_3D_VACT_SPACE2_SIZE, space2_size); }
    else { REG_UPDATE!(oppn10, OPPBUF_3D_PARAMETERS_0, OPPBUF_3D_VACT_SPACE1_SIZE, space1_size); }
}

pub unsafe fn opp1_pipe_clock_control(opp: *mut output_pixel_processor, enable: bool) {
    let oppn10 = TO_DCN10_OPP!(opp);
    let regval = if enable { 1 } else { 0 };
    REG_UPDATE!(oppn10, OPP_PIPE_CONTROL, OPP_PIPE_CLOCK_EN, regval);
}

pub unsafe fn opp1_read_reg_state(opp: *mut output_pixel_processor, state: *mut dcn_opp_reg_state) {
    let oppn10 = TO_DCN10_OPP!(opp);
    (*state).fmt_control = REG_READ!(oppn10, FMT_CONTROL);
    (*state).opp_pipe_control = REG_READ!(oppn10, OPP_PIPE_CONTROL);
    (*state).opp_pipe_crc_control = REG_READ!(oppn10, OPP_PIPE_CRC_CONTROL);
    (*state).oppbuf_control = REG_READ!(oppn10, OPPBUF_CONTROL);
}

pub unsafe fn opp1_destroy(opp: *mut *mut output_pixel_processor) {
    kfree!(TO_DCN10_OPP!(*opp));
    *opp = core::ptr::null_mut();
}

static DCN10_OPP_FUNCS: opp_funcs = opp_funcs {
    opp_set_dyn_expansion: Some(opp1_set_dyn_expansion),
    opp_program_fmt: Some(opp1_program_fmt),
    opp_program_bit_depth_reduction: Some(opp1_program_bit_depth_reduction),
    opp_program_stereo: Some(opp1_program_stereo),
    opp_pipe_clock_control: Some(opp1_pipe_clock_control),
    opp_set_disp_pattern_generator: None,
    opp_program_dpg_dimensions: None,
    dpg_is_blanked: None,
    dpg_is_pending: None,
    opp_destroy: Some(opp1_destroy),
    opp_read_reg_state: Some(opp1_read_reg_state),
};

pub unsafe fn dcn10_opp_construct(oppn10: *mut dcn10_opp, ctx: *mut dc_context, inst: u32, regs: *const dcn10_opp_registers, opp_shift: *const dcn10_opp_shift, opp_mask: *const dcn10_opp_mask) {
    (*oppn10).base.ctx = ctx;
    (*oppn10).base.inst = inst;
    (*oppn10).base.funcs = &DCN10_OPP_FUNCS;
    (*oppn10).regs = regs;
    (*oppn10).opp_shift = opp_shift;
    (*oppn10).opp_mask = opp_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
