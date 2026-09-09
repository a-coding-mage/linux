/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 */

// C dependencies and register helper macros are supplied by the surrounding build.

pub const NUM_PHASES: u32 = 64;
pub const HORZ_MAX_TAPS: u32 = 8;
pub const VERT_MAX_TAPS: u32 = 8;
pub const BLACK_OFFSET_RGB_Y: u32 = 0x0;
pub const BLACK_OFFSET_CBCR: u32 = 0x8000;

pub unsafe fn dpp20_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state) {
    let dpp = TO_DCN20_DPP(dpp_base);
    REG_GET!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, &mut (*s).is_enabled);
    // Degamma LUT (RAM)
    REG_GET!(dpp, CM_DGAM_CONTROL, CM_DGAM_LUT_MODE, &mut (*s).dgam_lut_mode);
    // Shaper LUT (RAM), 3D LUT (mode, bit-depth, size)
    REG_GET!(dpp, CM_SHAPER_CONTROL, CM_SHAPER_LUT_MODE, &mut (*s).shaper_lut_mode);
    REG_GET_2!(dpp, CM_3DLUT_READ_WRITE_CONTROL,
        CM_3DLUT_CONFIG_STATUS, &mut (*s).lut3d_mode,
        CM_3DLUT_30BIT_EN, &mut (*s).lut3d_bit_depth);
    REG_GET!(dpp, CM_3DLUT_MODE, CM_3DLUT_SIZE, &mut (*s).lut3d_size);
    // Blend/Out Gamma (RAM)
    REG_GET!(dpp, CM_BLNDGAM_LUT_WRITE_EN_MASK,
        CM_BLNDGAM_CONFIG_STATUS, &mut (*s).rgam_lut_mode);
}

pub unsafe fn dpp2_power_on_obuf(dpp_base: *mut dpp, power_on: bool) {
    let dpp = TO_DCN20_DPP(dpp_base);
    REG_UPDATE!(dpp, CM_MEM_PWR_CTRL, SHARED_MEM_PWR_DIS, if power_on { 1 } else { 0 });
    REG_UPDATE!(dpp, OBUF_MEM_PWR_CTRL, OBUF_MEM_PWR_FORCE, if power_on { 0 } else { 1 });
    REG_UPDATE!(dpp, DSCL_MEM_PWR_CTRL, LUT_MEM_PWR_FORCE, if power_on { 0 } else { 1 });
}

pub unsafe fn dpp2_dummy_program_input_lut(dpp_base: *mut dpp, gamma: *const dc_gamma) {
    let _ = dpp_base;
    let _ = gamma;
}

unsafe fn dpp2_cnv_setup(dpp_base: *mut dpp, format: surface_pixel_format,
    mode: expansion_mode, input_csc_color_matrix: dc_csc_transform,
    input_color_space: dc_color_space, alpha_2bit_lut: *mut cnv_alpha_2bit_lut) {
    let dpp = TO_DCN20_DPP(dpp_base);
    let mut pixel_format: u32 = 0;
    let mut alpha_en: u32 = 1;
    let mut color_space = COLOR_SPACE_SRGB;
    let mut select = DCN2_ICSC_SELECT_BYPASS;
    let mut force_disable_cursor = false;
    let mut tbl_entry: out_csc_color_matrix = core::mem::zeroed();
    let mut is_2bit: u32 = 0;
    let mut i: i32 = 0;

    REG_SET_2!(dpp, FORMAT_CONTROL, 0, CNVC_BYPASS, 0, FORMAT_EXPANSION_MODE, mode);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CNV16, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CNVC_BYPASS_MSB_ALIGN, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CLAMP_POSITIVE, 0);
    REG_UPDATE!(dpp, FORMAT_CONTROL, CLAMP_POSITIVE_C, 0);

    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => pixel_format = 1,
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => { pixel_format = 3; alpha_en = 0; }
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => pixel_format = 8,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 => { pixel_format = 10; is_2bit = 1; }
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr => { pixel_format = 65; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; }
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb => { force_disable_cursor = true; pixel_format = 64; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; }
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr => { force_disable_cursor = true; pixel_format = 67; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; }
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb => { force_disable_cursor = true; pixel_format = 66; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; }
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => pixel_format = 26,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F => pixel_format = 24,
        SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => pixel_format = 25,
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => { pixel_format = 12; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; }
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => { pixel_format = 112; alpha_en = 0; }
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => { pixel_format = 113; alpha_en = 0; }
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => { pixel_format = 114; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; is_2bit = 1; }
        SURFACE_PIXEL_FORMAT_VIDEO_CrYCbA1010102 => { pixel_format = 115; color_space = COLOR_SPACE_YCBCR709; select = DCN2_ICSC_SELECT_ICSC_A; is_2bit = 1; }
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => { pixel_format = 118; alpha_en = 0; }
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => { pixel_format = 119; alpha_en = 0; }
        _ => {}
    }
    color_space = if input_color_space != 0 { input_color_space } else { color_space };
    if is_2bit == 1 && !alpha_2bit_lut.is_null() {
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT0, (*alpha_2bit_lut).lut0);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT1, (*alpha_2bit_lut).lut1);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT2, (*alpha_2bit_lut).lut2);
        REG_UPDATE!(dpp, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT3, (*alpha_2bit_lut).lut3);
    }
    REG_SET!(dpp, CNVC_SURFACE_PIXEL_FORMAT, 0, CNVC_SURFACE_PIXEL_FORMAT, pixel_format);
    REG_UPDATE!(dpp, FORMAT_CONTROL, FORMAT_CONTROL__ALPHA_EN, alpha_en);
    if input_csc_color_matrix.enable_adjustment == true {
        while i < 12 { tbl_entry.regval[i as usize] = input_csc_color_matrix.matrix[i as usize]; i += 1; }
        tbl_entry.color_space = input_color_space;
        select = if color_space >= COLOR_SPACE_YCBCR601 { DCN2_ICSC_SELECT_ICSC_A } else { DCN2_ICSC_SELECT_BYPASS };
        dpp2_program_input_csc(dpp_base, color_space, select, &mut tbl_entry);
    } else { dpp2_program_input_csc(dpp_base, color_space, select, core::ptr::null_mut()); }
    if force_disable_cursor {
        REG_UPDATE!(dpp, CURSOR_CONTROL, CURSOR_ENABLE, 0);
        REG_UPDATE!(dpp, CURSOR0_CONTROL, CUR0_ENABLE, 0);
    }
    dpp2_power_on_obuf(dpp_base, true);
}

pub unsafe fn dscl2_calc_lb_num_partitions(scl_data: *const scaler_data, lb_config: lb_memory_config,
    num_part_y: *mut i32, num_part_c: *mut i32) {
    dscl2_calc_lb_num_partitions_inner((*scl_data).viewport.width, (*scl_data).recout.width,
        (*scl_data).viewport_c.width, (*scl_data).lb_params.alpha_en, lb_config, num_part_y, num_part_c);
}

unsafe fn dscl2_calc_lb_num_partitions_inner(viewport_width: i32, recout_width: i32, viewport_c_width: i32,
    alpha_en: bool, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32) {
    let mut line_size = if viewport_width < recout_width { viewport_width } else { recout_width };
    let mut line_size_c = if viewport_c_width < recout_width { viewport_c_width } else { recout_width };
    if line_size == 0 { line_size = 1; } if line_size_c == 0 { line_size_c = 1; }
    let memory_line_size_y = (line_size + 5) / 6; let memory_line_size_c = (line_size_c + 5) / 6; let memory_line_size_a = (line_size + 5) / 6;
    let (lb_memory_size, lb_memory_size_c, lb_memory_size_a) = match lb_config {
        LB_MEMORY_CONFIG_1 => (970, 970, 970), LB_MEMORY_CONFIG_2 => (1290, 1290, 1290),
        LB_MEMORY_CONFIG_3 => (970 + 1290 + 484 + 484 + 484, 970 + 1290, 970 + 1290 + 484),
        _ => (970 + 1290 + 484, 970 + 1290 + 484, 970 + 1290 + 484),
    };
    *num_part_y = lb_memory_size / memory_line_size_y; *num_part_c = lb_memory_size_c / memory_line_size_c;
    let num_partitions_a = lb_memory_size_a / memory_line_size_a;
    if alpha_en && num_partitions_a < *num_part_y { *num_part_y = num_partitions_a; }
    if *num_part_y > 64 { *num_part_y = 64; } if *num_part_c > 64 { *num_part_c = 64; }
}

pub unsafe fn dpp2_cnv_set_alpha_keyer(dpp_base: *mut dpp, color_keyer: *const cnv_color_keyer_params) {
    let dpp = TO_DCN20_DPP(dpp_base);
    REG_UPDATE!(dpp, COLOR_KEYER_CONTROL, COLOR_KEYER_EN, (*color_keyer).color_keyer_en);
    REG_UPDATE!(dpp, COLOR_KEYER_CONTROL, COLOR_KEYER_MODE, (*color_keyer).color_keyer_mode);
    REG_UPDATE!(dpp, COLOR_KEYER_ALPHA, COLOR_KEYER_ALPHA_LOW, (*color_keyer).color_keyer_alpha_low);
    REG_UPDATE!(dpp, COLOR_KEYER_ALPHA, COLOR_KEYER_ALPHA_HIGH, (*color_keyer).color_keyer_alpha_high);
    REG_UPDATE!(dpp, COLOR_KEYER_RED, COLOR_KEYER_RED_LOW, (*color_keyer).color_keyer_red_low);
    REG_UPDATE!(dpp, COLOR_KEYER_RED, COLOR_KEYER_RED_HIGH, (*color_keyer).color_keyer_red_high);
    REG_UPDATE!(dpp, COLOR_KEYER_GREEN, COLOR_KEYER_GREEN_LOW, (*color_keyer).color_keyer_green_low);
    REG_UPDATE!(dpp, COLOR_KEYER_GREEN, COLOR_KEYER_GREEN_HIGH, (*color_keyer).color_keyer_green_high);
    REG_UPDATE!(dpp, COLOR_KEYER_BLUE, COLOR_KEYER_BLUE_LOW, (*color_keyer).color_keyer_blue_low);
    REG_UPDATE!(dpp, COLOR_KEYER_BLUE, COLOR_KEYER_BLUE_HIGH, (*color_keyer).color_keyer_blue_high);
}

pub unsafe fn dpp2_set_cursor_attributes(dpp_base: *mut dpp, cursor_attributes: *const dc_cursor_attributes) {
    let dpp = TO_DCN20_DPP(dpp_base); let color_format = (*cursor_attributes).color_format; let mut cur_rom_en = 0;
    if color_format == CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA || color_format == CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA {
        if (*cursor_attributes).attribute_flags.bits.ENABLE_CURSOR_DEGAMMA { cur_rom_en = 1; }
    }
    REG_UPDATE_3!(dpp, CURSOR0_CONTROL, CUR0_MODE, color_format, CUR0_EXPANSION_MODE, 0, CUR0_ROM_EN, cur_rom_en);
    if color_format == CURSOR_MODE_MONO { REG_UPDATE!(dpp, CURSOR0_COLOR0, CUR0_COLOR0, 0x00000000); REG_UPDATE!(dpp, CURSOR0_COLOR1, CUR0_COLOR1, 0xFFFFFFFF); }
}

pub unsafe fn oppn20_dummy_program_regamma_pwl(dpp: *mut dpp, params: *const pwl_params, mode: opp_regamma) { let _ = (dpp, params, mode); }

// Function tables and capability tables retain the C layout and external callbacks.
pub static mut dcn20_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_read_state: Some(dpp20_read_state), dpp_reset: Some(dpp_reset), dpp_set_scaler: Some(dpp1_dscl_set_scaler_manual_scale),
    dpp_get_optimal_number_of_taps: Some(dpp1_get_optimal_number_of_taps), dpp_set_gamut_remap: Some(dpp2_cm_set_gamut_remap),
    dpp_set_csc_adjustment: None, dpp_set_csc_default: None, dpp_program_regamma_pwl: Some(oppn20_dummy_program_regamma_pwl),
    dpp_set_degamma: Some(dpp2_set_degamma), dpp_program_input_lut: Some(dpp2_dummy_program_input_lut), dpp_full_bypass: Some(dpp1_full_bypass),
    dpp_setup: Some(dpp2_cnv_setup), dpp_program_degamma_pwl: Some(dpp2_set_degamma_pwl), dpp_program_blnd_lut: Some(dpp20_program_blnd_lut),
    dpp_program_shaper_lut: Some(dpp20_program_shaper), dpp_program_3dlut: Some(dpp20_program_3dlut), dpp_program_bias_and_scale: None,
    dpp_cnv_set_alpha_keyer: Some(dpp2_cnv_set_alpha_keyer), set_cursor_attributes: Some(dpp2_set_cursor_attributes),
    set_cursor_position: Some(dpp1_set_cursor_position), set_optional_cursor_attributes: Some(dpp1_cnv_set_optional_cursor_attributes),
    dpp_dppclk_control: Some(dpp1_dppclk_control), dpp_set_hdr_multiplier: Some(dpp2_set_hdr_multiplier), dpp_get_gamut_remap: Some(dpp2_cm_get_gamut_remap),
};

pub static mut dcn20_dpp_cap: dpp_caps = dpp_caps { dscl_data_proc_format: DSCL_DATA_PRCESSING_FLOAT_FORMAT, dscl_calc_lb_num_partitions: Some(dscl2_calc_lb_num_partitions) };

pub unsafe fn dpp2_construct(dpp: *mut dcn20_dpp, ctx: *mut dc_context, inst: u32,
    tf_regs: *const dcn2_dpp_registers, tf_shift: *const dcn2_dpp_shift, tf_mask: *const dcn2_dpp_mask) -> bool {
    (*dpp).base.ctx = ctx; (*dpp).base.inst = inst; (*dpp).base.funcs = &mut dcn20_dpp_funcs; (*dpp).base.caps = &mut dcn20_dpp_cap;
    (*dpp).tf_regs = tf_regs; (*dpp).tf_shift = tf_shift; (*dpp).tf_mask = tf_mask;
    (*dpp).lb_pixel_depth_supported = LB_PIXEL_DEPTH_18BPP | LB_PIXEL_DEPTH_24BPP | LB_PIXEL_DEPTH_30BPP | LB_PIXEL_DEPTH_36BPP;
    (*dpp).lb_bits_per_entry = LB_BITS_PER_ENTRY; (*dpp).lb_memory_size = LB_TOTAL_NUMBER_OF_ENTRIES; /*0x1404*/ true
}

pub unsafe fn dscl2_spl_calc_lb_num_partitions(alpha_en: bool, scl_data: *const spl_scaler_data, lb_config: lb_memory_config,
    num_part_y: *mut i32, num_part_c: *mut i32) {
    dscl2_calc_lb_num_partitions_inner((*scl_data).viewport.width, (*scl_data).recout.width, (*scl_data).viewport_c.width,
        alpha_en, lb_config, num_part_y, num_part_c);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
