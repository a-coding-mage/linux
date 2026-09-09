// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translation unit.

pub unsafe fn dpp60_dpp_setup(
    dpp_base: *mut dpp,
    format: surface_pixel_format,
    mode: expansion_mode,
    input_csc_color_matrix: dc_csc_transform,
    input_color_space: dc_color_space,
    alpha_2bit_lut: *mut cnv_alpha_2bit_lut,
) {
    let dpp: *mut dcn60_dpp = TO_DCN60_DPP(dpp_base);
    let mut pixel_format: u32 = 8;
    let mut alpha_en: u32 = 1;
    let mut color_space: dc_color_space = COLOR_SPACE_SRGB;
    let mut select: dcn10_input_csc_select = INPUT_CSC_SELECT_BYPASS;
    let mut is_2bit: u32 = 0;
    let mut dealpha_en: u32 = 0;
    let mut dealpha_ablnd_en: u32 = 0;
    let mut realpha_en: u32 = 0;
    let mut realpha_ablnd_en: u32 = 0;
    let mut tbl_entry: out_csc_color_matrix = core::mem::zeroed();

    REG_SET_2!((*dpp).tf_regs, FORMAT_CONTROL, 0,
        CNVC_BYPASS, 0, FORMAT_EXPANSION_MODE, mode);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CNV16, 0);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, CNVC_BYPASS_MSB_ALIGN, 0);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, CLAMP_POSITIVE, 0);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, CLAMP_POSITIVE_C, 0);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_R, 0);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_G, 1);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_B, 2);

    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => pixel_format = 1,
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => { pixel_format = 3; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => pixel_format = 8,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 => { pixel_format = 10; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb | SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P208 => { pixel_format = 64; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P208 => { pixel_format = 65; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb | SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P210 => { pixel_format = 66; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P210 => { pixel_format = 67; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P212 => { pixel_format = 68; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P212 => { pixel_format = 69; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_422_YCrYCb | SURFACE_PIXEL_FORMAT_VIDEO_422_YCbYCr | SURFACE_PIXEL_FORMAT_VIDEO_422_CrYCbY | SURFACE_PIXEL_FORMAT_VIDEO_422_CbYCrY => { pixel_format = (format as u32) - (SURFACE_PIXEL_FORMAT_VIDEO_422_YCrYCb as u32) + 72; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCrYCb | SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCbYCr | SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CrYCbY | SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CbYCrY => { pixel_format = (format as u32) - (SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCrYCb as u32) + 76; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCrYCb | SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCbYCr | SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CrYCbY | SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CbYCrY => { pixel_format = (format as u32) - (SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCrYCb as u32) + 80; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => pixel_format = 26,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F => pixel_format = 24,
        SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => pixel_format = 25,
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => { pixel_format = 12; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => { pixel_format = 112; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => { pixel_format = 113; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => { pixel_format = 114; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_VIDEO_CrYCbA1010102 => { pixel_format = 115; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_GRPH_RGBE | SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => pixel_format = 116,
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => { pixel_format = 118; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => { pixel_format = 119; alpha_en = 0; },
        _ => {}
    }

    color_space = if input_color_space != 0 { input_color_space } else { color_space };
    if is_2bit == 1 && !alpha_2bit_lut.is_null() {
        REG_UPDATE!((*dpp).tf_regs, ALPHA_2BIT_LUT01, ALPHA_2BIT_LUT0, (*alpha_2bit_lut).lut0);
        REG_UPDATE!((*dpp).tf_regs, ALPHA_2BIT_LUT01, ALPHA_2BIT_LUT1, (*alpha_2bit_lut).lut1);
        REG_UPDATE!((*dpp).tf_regs, ALPHA_2BIT_LUT23, ALPHA_2BIT_LUT2, (*alpha_2bit_lut).lut2);
        REG_UPDATE!((*dpp).tf_regs, ALPHA_2BIT_LUT23, ALPHA_2BIT_LUT3, (*alpha_2bit_lut).lut3);
    }
    REG_SET!((*dpp).tf_regs, CNVC_SURFACE_PIXEL_FORMAT, 0, CNVC_SURFACE_PIXEL_FORMAT, pixel_format);
    REG_UPDATE!((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CONTROL__ALPHA_EN, alpha_en);
    REG_SET_2!((*dpp).tf_regs, PRE_DEALPHA, 0, PRE_DEALPHA_EN, dealpha_en, PRE_DEALPHA_ABLND_EN, dealpha_ablnd_en);
    REG_SET_2!((*dpp).tf_regs, PRE_REALPHA, 0, PRE_REALPHA_EN, realpha_en, PRE_REALPHA_ABLND_EN, realpha_ablnd_en);

    if (format as u32) < (SURFACE_PIXEL_FORMAT_VIDEO_BEGIN as u32) ||
       (format as u32) >= (SURFACE_PIXEL_FORMAT_SUBSAMPLE_END as u32) || color_space != COLOR_SPACE_SRGB {
        if input_csc_color_matrix.enable_adjustment == true {
            for i in 0..12 { tbl_entry.regval[i] = input_csc_color_matrix.matrix[i]; }
            tbl_entry.color_space = input_color_space;
            select = if dpp3_should_bypass_post_csc_for_colorspace(color_space) { INPUT_CSC_SELECT_BYPASS } else { INPUT_CSC_SELECT_ICSC };
            dpp3_program_post_csc(dpp_base, color_space, select, &mut tbl_entry);
        } else { dpp3_program_post_csc(dpp_base, color_space, select, core::ptr::null_mut()); }
    }
}

pub unsafe fn dpp60_full_bypass(dpp_base: *mut dpp) {
    let dpp = TO_DCN60_DPP(dpp_base);
    REG_SET!((*dpp).tf_regs, CNVC_SURFACE_PIXEL_FORMAT, 0, CNVC_SURFACE_PIXEL_FORMAT, 0x8);
    REG_SET_3!((*dpp).tf_regs, FORMAT_CONTROL, 0, CNVC_BYPASS, 0, FORMAT_CONTROL__ALPHA_EN, 0, FORMAT_EXPANSION_MODE, 0);
    if (*dpp).tf_mask.CM_BYPASS_EN != 0 { REG_SET!((*dpp).tf_regs, CM_CONTROL, 0, CM_BYPASS_EN, 1); }
    else { REG_SET!((*dpp).tf_regs, CM_CONTROL, 0, CM_BYPASS, 1); }
}

static mut dcn60_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_program_gamcor_lut: Some(dpp3_program_gamcor_lut), dpp_read_state: Some(dpp401_read_state), dpp_reset: Some(dpp_reset),
    dpp_set_scaler: Some(dpp60_dscl_set_scaler_manual_scale), dpp_get_optimal_number_of_taps: Some(dpp3_get_optimal_number_of_taps), dpp_set_pre_degam: None,
    dpp_full_bypass: Some(dpp60_full_bypass), dpp_setup: Some(dpp60_dpp_setup), dpp_program_cm_dealpha: Some(dpp3_program_cm_dealpha), dpp_program_cm_bias: Some(dpp3_program_cm_bias),
    dpp_program_bias_and_scale: Some(dpp35_program_bias_and_scale_fcnv), dpp_cnv_set_alpha_keyer: Some(dpp2_cnv_set_alpha_keyer), set_cursor_attributes: Some(dpp401_set_cursor_attributes),
    set_cursor_position: Some(dpp401_set_cursor_position), set_optional_cursor_attributes: Some(dpp401_set_optional_cursor_attributes), dpp_dppclk_control: Some(dpp1_dppclk_control),
    dpp_set_hdr_multiplier: Some(dpp3_set_hdr_multiplier), set_cursor_matrix: Some(dpp401_set_cursor_matrix), dpp_cm_hist_control: Some(dpp42_dpp_cm_hist_control),
    dpp_cm_hist_read: Some(dpp42_dpp_cm_hist_read), dpp_read_reg_state: Some(dpp30_read_reg_state), dpp_set_pregam_state: Some(dpp50_set_pregam_state),
};

static mut dcn60_dpp_cap: dpp_caps = dpp_caps { dscl_data_proc_format: DSCL_DATA_PRCESSING_FLOAT_FORMAT, max_lb_partitions: 63, dscl_calc_lb_num_partitions: Some(dscl401_calc_lb_num_partitions) };

pub unsafe fn dpp60_construct(dpp: *mut dcn60_dpp, ctx: *mut dc_context, inst: u32, tf_regs: *const dcn60_dpp_registers, tf_shift: *const dcn60_dpp_shift, tf_mask: *const dcn60_dpp_mask) -> bool {
    (*dpp).base.ctx = ctx; (*dpp).base.inst = inst; (*dpp).base.funcs = &raw const dcn60_dpp_funcs; (*dpp).base.caps = &raw const dcn60_dpp_cap;
    (*dpp).tf_regs = tf_regs; (*dpp).tf_shift = tf_shift; (*dpp).tf_mask = tf_mask; true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
