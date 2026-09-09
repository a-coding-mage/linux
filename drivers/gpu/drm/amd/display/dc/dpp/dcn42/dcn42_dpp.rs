// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit.

fn get_hist_rgb_luma_coefs(color_space: dc_color_space) -> &'static [u32; 3] {
    // Coefs in s.6.12.
    // Y = 0.2126R + 0.7152G + 0.0722B
    static LUMA_TRANSFORM_BT_709: [u32; 3] = [0x1cb36, 0x1e6e2, 0x1b27b];
    // Y = 0.2627R + 0.678G + 0.0593B
    static LUMA_TRANSFORM_BT_2020: [u32; 3] = [0x1d0d0, 0x1e5b2, 0x1ae5c];

    match color_space {
        dc_color_space::COLOR_SPACE_2020_RGB_FULLRANGE |
        dc_color_space::COLOR_SPACE_2020_RGB_LIMITEDRANGE => &LUMA_TRANSFORM_BT_2020,
        _ => &LUMA_TRANSFORM_BT_709,
    }
}

pub unsafe fn dpp42_dpp_cm_hist_control(
    dpp_base: *mut dpp,
    cntl: cm_hist_control,
    color_space: dc_color_space,
) {
    let dpp = TO_DCN42_DPP(dpp_base);

    REG_UPDATE_10!(dpp, CM_HIST_CNTL,
        CM_HIST_SEL, cntl.tap_point,
        CM_HIST_CH_EN, cntl.channels_enabled,
        CM_HIST_SRC1_SEL, cntl.src_1_select,
        CM_HIST_SRC2_SEL, cntl.src_2_select,
        CM_HIST_SRC3_SEL, cntl.src_3_select,
        CM_HIST_CH1_XBAR, cntl.ch1_src,
        CM_HIST_CH2_XBAR, cntl.ch2_src,
        CM_HIST_CH3_XBAR, cntl.ch3_src,
        CM_HIST_FORMAT, cntl.format,
        CM_HIST_READ_CHANNEL_MASK, cntl.read_channel_mask);

    if cntl.src_2_select == CM_HIST_SRC2_MODE_RGB_TO_Y {
        let luma_transform = get_hist_rgb_luma_coefs(color_space);
        REG_UPDATE!(dpp, CM_HIST_COEFA_SRC2, CM_HIST_COEFA_SRC2, luma_transform[0]);
        REG_UPDATE!(dpp, CM_HIST_COEFB_SRC2, CM_HIST_COEFB_SRC2, luma_transform[1]);
        REG_UPDATE!(dpp, CM_HIST_COEFC_SRC2, CM_HIST_COEFC_SRC2, luma_transform[2]);
    } else {
        REG_UPDATE!(dpp, CM_HIST_COEFA_SRC2, CM_HIST_COEFA_SRC2, 0);
        REG_UPDATE!(dpp, CM_HIST_COEFB_SRC2, CM_HIST_COEFB_SRC2, 0x1f000); // 1 in s.6.12
        REG_UPDATE!(dpp, CM_HIST_COEFC_SRC2, CM_HIST_COEFC_SRC2, 0);
    }
}

pub unsafe fn dpp42_dpp_cm_hist_read(dpp_base: *mut dpp, hist_out: *mut cm_hist) -> bool {
    let dpp = TO_DCN42_DPP(dpp_base);
    if hist_out.is_null() { return false; }

    let mut channel_mask = 0u32;
    let mut rdy_status_a = 0u32;
    let mut rdy_status_b = 0u32;
    REG_GET!(dpp, CM_HIST_CNTL, CM_HIST_READ_CHANNEL_MASK, &mut channel_mask);
    let ch1 = (channel_mask & 1) > 0;
    let ch2 = (channel_mask & 2) > 0;
    let ch3 = (channel_mask & 4) > 0;
    REG_GET!(dpp, CM_HIST_STATUS, CM_HIST_BUFA_RDY_STATUS, &mut rdy_status_a);
    REG_GET!(dpp, CM_HIST_STATUS, CM_HIST_BUFB_RDY_STATUS, &mut rdy_status_b);

    if rdy_status_a != 0 || rdy_status_b != 0 {
        REG_UPDATE!(dpp, CM_HIST_LOCK, CM_HIST_LOCK, 1);
        REG_UPDATE!(dpp, CM_HIST_INDEX, CM_HIST_INDEX, 0);
        for i in 0..256 {
            let mut temp = 0u32;
            if ch1 { REG_GET!(dpp, CM_HIST_DATA, CM_HIST_DATA, &mut temp); (*hist_out).ch1[i] += temp; }
            if ch2 { REG_GET!(dpp, CM_HIST_DATA, CM_HIST_DATA, &mut temp); (*hist_out).ch2[i] += temp; }
            if ch3 { REG_GET!(dpp, CM_HIST_DATA, CM_HIST_DATA, &mut temp); (*hist_out).ch3[i] += temp; }
        }
        REG_UPDATE!(dpp, CM_HIST_LOCK, CM_HIST_LOCK, 0);
        true
    } else { false }
}

unsafe fn dpp42_dpp_setup(
    dpp_base: *mut dpp, format: surface_pixel_format, mode: expansion_mode,
    input_csc_color_matrix: dc_csc_transform, input_color_space: dc_color_space,
    alpha_2bit_lut: *mut cnv_alpha_2bit_lut,
) {
    let dpp = TO_DCN401_DPP(dpp_base);
    let mut pixel_format = 0u32;
    let mut alpha_en = 1u32;
    let mut color_space = COLOR_SPACE_SRGB;
    let mut select = INPUT_CSC_SELECT_BYPASS;
    let mut is_2bit = 0u32;
    let mut alpha_plane_enable = 0u32;
    let (mut dealpha_en, mut dealpha_ablnd_en, mut realpha_en, mut realpha_ablnd_en) = (0u32, 0, 0, 0);

    REG_SET_2!(dpp, FORMAT_CONTROL, 0, CNVC_BYPASS, 0, FORMAT_EXPANSION_MODE, mode);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CNV16, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CNVC_BYPASS_MSB_ALIGN, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CLAMP_POSITIVE, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CLAMP_POSITIVE_C, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CROSSBAR_R, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CROSSBAR_G, 1);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CROSSBAR_B, 2);

    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => pixel_format = 1,
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => { pixel_format = 3; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => pixel_format = 8,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 => { pixel_format = 10; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb => { pixel_format = if format == SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr { 65 } else { 64 }; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr | SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb => { pixel_format = if format == SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr { 67 } else { 66 }; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => pixel_format = 26,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F => pixel_format = 24,
        SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => pixel_format = 25,
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => { pixel_format = 12; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => { pixel_format = 112; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => { pixel_format = 113; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => { pixel_format = 114; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_VIDEO_CrYCbA1010102 => { pixel_format = 115; color_space = COLOR_SPACE_YCBCR709; select = INPUT_CSC_SELECT_ICSC; is_2bit = 1; },
        SURFACE_PIXEL_FORMAT_GRPH_RGBE => { pixel_format = 116; alpha_plane_enable = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => { pixel_format = 116; alpha_plane_enable = 1; },
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => { pixel_format = 118; alpha_en = 0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => { pixel_format = 119; alpha_en = 0; },
        _ => {}
    }
    color_space = if input_color_space != 0 { input_color_space } else { color_space };
    if is_2bit == 1 && !alpha_2bit_lut.is_null() {
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT01, ALPHA_2BIT_LUT0, (*alpha_2bit_lut).lut0);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT01, ALPHA_2BIT_LUT1, (*alpha_2bit_lut).lut1);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT23, ALPHA_2BIT_LUT2, (*alpha_2bit_lut).lut2);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT23, ALPHA_2BIT_LUT3, (*alpha_2bit_lut).lut3);
    }
    REG_SET_2!(dpp, CNVC_SURFACE_PIXEL_FORMAT, 0, CNVC_SURFACE_PIXEL_FORMAT, pixel_format, CNVC_ALPHA_PLANE_ENABLE, alpha_plane_enable);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CONTROL__ALPHA_EN, alpha_en);
    REG_SET_2!(dpp, PRE_DEALPHA, 0, PRE_DEALPHA_EN, dealpha_en, PRE_DEALPHA_ABLND_EN, dealpha_ablnd_en);
    REG_SET_2!(dpp, PRE_REALPHA, 0, PRE_REALPHA_EN, realpha_en, PRE_REALPHA_ABLND_EN, realpha_ablnd_en);
    if input_csc_color_matrix.enable_adjustment == true {
        let mut tbl_entry = out_csc_color_matrix::default();
        for i in 0..12 { tbl_entry.regval[i] = input_csc_color_matrix.matrix[i]; }
        tbl_entry.color_space = input_color_space;
        select = if dpp3_should_bypass_post_csc_for_colorspace(color_space) { INPUT_CSC_SELECT_BYPASS } else { INPUT_CSC_SELECT_ICSC };
        dpp3_program_post_csc(dpp_base, color_space, select, &mut tbl_entry);
    } else { dpp3_program_post_csc(dpp_base, color_space, select, std::ptr::null_mut()); }
}

unsafe fn dcn42_dpp_force_disable_cursor(dpp_base: *mut dpp) {
    let dpp = TO_DCN401_DPP(dpp_base);
    REG_UPDATE!(dpp, CURSOR0_CONTROL, CUR0_ENABLE, 0);
    (*dpp_base).pos.cur0_ctl.bits.cur0_enable = 0;
}

static mut dcn42_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_program_gamcor_lut: dpp3_program_gamcor_lut, dpp_read_state: dpp401_read_state,
    dpp_reset, dpp_set_scaler: dpp401_dscl_set_scaler_manual_scale,
    dpp_get_optimal_number_of_taps: dpp3_get_optimal_number_of_taps, dpp_set_pre_degam: dpp3_set_pre_degam,
    dpp_setup: dpp42_dpp_setup, dpp_program_cm_dealpha: dpp3_program_cm_dealpha,
    dpp_program_cm_bias: dpp3_program_cm_bias, dpp_program_bias_and_scale: dpp35_program_bias_and_scale_fcnv,
    dpp_cnv_set_alpha_keyer: dpp2_cnv_set_alpha_keyer, set_cursor_attributes: dpp401_set_cursor_attributes,
    set_cursor_position: dpp401_set_cursor_position, set_optional_cursor_attributes: dpp401_set_optional_cursor_attributes,
    dpp_dppclk_control: dpp35_dppclk_control, dpp_set_hdr_multiplier: dpp3_set_hdr_multiplier,
    set_cursor_matrix: dpp401_set_cursor_matrix, dpp_cm_hist_control: dpp42_dpp_cm_hist_control,
    dpp_cm_hist_read: dpp42_dpp_cm_hist_read, dpp_read_reg_state: dpp30_read_reg_state,
    dpp_force_disable_cursor: dcn42_dpp_force_disable_cursor,
};

static mut dcn42_dpp_cap: dpp_caps = dpp_caps {
    dscl_data_proc_format: DSCL_DATA_PRCESSING_FLOAT_FORMAT,
    max_lb_partitions: 63,
    dscl_calc_lb_num_partitions: dscl401_calc_lb_num_partitions,
};

pub unsafe fn dpp42_construct(dpp: *mut dcn42_dpp, ctx: *mut dc_context, inst: u32,
    tf_regs: *const dcn42_dpp_registers, tf_shift: *const dcn42_dpp_shift,
    tf_mask: *const dcn42_dpp_mask) -> bool {
    (*dpp).base.ctx = ctx;
    (*dpp).base.inst = inst;
    (*dpp).base.funcs = &raw mut dcn42_dpp_funcs;
    (*dpp).base.caps = &raw mut dcn42_dpp_cap;
    (*dpp).tf_regs = tf_regs;
    (*dpp).tf_shift = tf_shift;
    (*dpp).tf_mask = tf_mask;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
