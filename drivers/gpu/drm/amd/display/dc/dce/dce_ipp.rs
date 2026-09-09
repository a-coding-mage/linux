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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

// Dependencies supplied by the surrounding translation unit are intentionally
// left external, as in the original C implementation.

unsafe fn dce_ipp_cursor_set_position(
    ipp: *mut input_pixel_processor,
    position: *const dc_cursor_position,
    param: *const dc_cursor_mi_param,
) {
    let _ = param;
    let ipp_dce = TO_DCE_IPP!(ipp);

    REG_UPDATE!(ipp_dce, CUR_UPDATE, CURSOR_UPDATE_LOCK, true);
    REG_UPDATE!(ipp_dce, CUR_CONTROL, CURSOR_EN, (*position).enable);
    REG_SET_2!(ipp_dce, CUR_POSITION, 0, CURSOR_X_POSITION, (*position).x,
        CURSOR_Y_POSITION, (*position).y);
    REG_SET_2!(ipp_dce, CUR_HOT_SPOT, 0, CURSOR_HOT_SPOT_X, (*position).x_hotspot,
        CURSOR_HOT_SPOT_Y, (*position).y_hotspot);
    REG_UPDATE!(ipp_dce, CUR_UPDATE, CURSOR_UPDATE_LOCK, false);
}

unsafe fn dce_ipp_cursor_set_attributes(
    ipp: *mut input_pixel_processor,
    attributes: *const dc_cursor_attributes,
) {
    let ipp_dce = TO_DCE_IPP!(ipp);
    let mode: i32;
    REG_UPDATE!(ipp_dce, CUR_UPDATE, CURSOR_UPDATE_LOCK, true);
    mode = match (*attributes).color_format {
        CURSOR_MODE_MONO => 0,
        CURSOR_MODE_COLOR_1BIT_AND => 1,
        CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA => 2,
        CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA => 3,
        _ => { BREAK_TO_DEBUGGER!(); 0 }
    };
    REG_UPDATE_3!(ipp_dce, CUR_CONTROL, CURSOR_MODE, mode,
        CURSOR_2X_MAGNIFY, (*attributes).attribute_flags.bits.ENABLE_MAGNIFICATION,
        CUR_INV_TRANS_CLAMP, (*attributes).attribute_flags.bits.INVERSE_TRANSPARENT_CLAMPING);
    if (*attributes).color_format == CURSOR_MODE_MONO {
        REG_SET_3!(ipp_dce, CUR_COLOR1, 0, CUR_COLOR1_BLUE, 0,
            CUR_COLOR1_GREEN, 0, CUR_COLOR1_RED, 0);
        REG_SET_3!(ipp_dce, CUR_COLOR2, 0, CUR_COLOR2_BLUE, 0xff,
            CUR_COLOR2_GREEN, 0xff, CUR_COLOR2_RED, 0xff);
    }
    REG_SET_2!(ipp_dce, CUR_SIZE, 0, CURSOR_WIDTH, (*attributes).width - 1,
        CURSOR_HEIGHT, (*attributes).height - 1);
    REG_SET!(ipp_dce, CUR_SURFACE_ADDRESS_HIGH, 0, CURSOR_SURFACE_ADDRESS_HIGH,
        (*attributes).address.high_part);
    REG_SET!(ipp_dce, CUR_SURFACE_ADDRESS, 0, CURSOR_SURFACE_ADDRESS,
        (*attributes).address.low_part);
    REG_UPDATE!(ipp_dce, CUR_UPDATE, CURSOR_UPDATE_LOCK, false);
}

unsafe fn dce_ipp_program_prescale(
    ipp: *mut input_pixel_processor, params: *mut ipp_prescale_params,
) {
    let ipp_dce = TO_DCE_IPP!(ipp);
    REG_UPDATE!(ipp_dce, PRESCALE_GRPH_CONTROL, GRPH_PRESCALE_BYPASS, 1);
    REG_SET_2!(ipp_dce, PRESCALE_VALUES_GRPH_R, 0, GRPH_PRESCALE_SCALE_R, (*params).scale,
        GRPH_PRESCALE_BIAS_R, (*params).bias);
    REG_SET_2!(ipp_dce, PRESCALE_VALUES_GRPH_G, 0, GRPH_PRESCALE_SCALE_G, (*params).scale,
        GRPH_PRESCALE_BIAS_G, (*params).bias);
    REG_SET_2!(ipp_dce, PRESCALE_VALUES_GRPH_B, 0, GRPH_PRESCALE_SCALE_B, (*params).scale,
        GRPH_PRESCALE_BIAS_B, (*params).bias);
    if (*params).mode != IPP_PRESCALE_MODE_BYPASS {
        REG_UPDATE!(ipp_dce, PRESCALE_GRPH_CONTROL, GRPH_PRESCALE_BYPASS, 0);
        REG_UPDATE!(ipp_dce, INPUT_GAMMA_CONTROL, GRPH_INPUT_GAMMA_MODE, 1);
    }
}

unsafe fn dce_ipp_program_input_lut(
    ipp: *mut input_pixel_processor, gamma: *const dc_gamma,
) {
    let ipp_dce = TO_DCE_IPP!(ipp);
    if REG!(ipp_dce, DCFE_MEM_PWR_CTRL) != 0 {
        REG_SET!(ipp_dce, DCFE_MEM_PWR_CTRL, 0, DCP_LUT_MEM_PWR_DIS, 1);
    }
    REG_SET!(ipp_dce, DC_LUT_WRITE_EN_MASK, 0, DC_LUT_WRITE_EN_MASK, 0x7);
    REG_UPDATE!(ipp_dce, DC_LUT_RW_MODE, DC_LUT_RW_MODE, 0);
    REG_SET_3!(ipp_dce, DC_LUT_CONTROL, 0, DC_LUT_DATA_R_FORMAT, 3,
        DC_LUT_DATA_G_FORMAT, 3, DC_LUT_DATA_B_FORMAT, 3);
    REG_SET!(ipp_dce, DC_LUT_RW_INDEX, 0, DC_LUT_RW_INDEX, 0);
    for i in 0..(*gamma).num_entries {
        REG_SET!(ipp_dce, DC_LUT_SEQ_COLOR, 0, DC_LUT_SEQ_COLOR,
            dc_fixpt_round!((*gamma).entries.red[i]));
        REG_SET!(ipp_dce, DC_LUT_SEQ_COLOR, 0, DC_LUT_SEQ_COLOR,
            dc_fixpt_round!((*gamma).entries.green[i]));
        REG_SET!(ipp_dce, DC_LUT_SEQ_COLOR, 0, DC_LUT_SEQ_COLOR,
            dc_fixpt_round!((*gamma).entries.blue[i]));
    }
    if REG!(ipp_dce, DCFE_MEM_PWR_CTRL) != 0 {
        REG_SET!(ipp_dce, DCFE_MEM_PWR_CTRL, 0, DCP_LUT_MEM_PWR_DIS, 0);
    }
    REG_UPDATE!(ipp_dce, PRESCALE_GRPH_CONTROL, GRPH_PRESCALE_BYPASS, 1);
    REG_UPDATE!(ipp_dce, INPUT_GAMMA_CONTROL, GRPH_INPUT_GAMMA_MODE, 0);
}

unsafe fn dce_ipp_set_degamma(ipp: *mut input_pixel_processor, mode: ipp_degamma_mode) {
    let ipp_dce = TO_DCE_IPP!(ipp);
    let degamma_type: u32 = if mode == IPP_DEGAMMA_MODE_HW_sRGB { 1 } else { 0 };
    ASSERT!(mode == IPP_DEGAMMA_MODE_BYPASS || mode == IPP_DEGAMMA_MODE_HW_sRGB);
    REG_SET_3!(ipp_dce, DEGAMMA_CONTROL, 0, GRPH_DEGAMMA_MODE, degamma_type,
        CURSOR_DEGAMMA_MODE, degamma_type, CURSOR2_DEGAMMA_MODE, degamma_type);
}

// CONFIG_DRM_AMD_DC_SI provides the DCE6 variant, whose register lacks the
// CURSOR2_DEGAMMA_MODE field; its function table and constructor are supplied
// by the corresponding build configuration.

static dce_ipp_funcs: ipp_funcs = ipp_funcs {
    ipp_cursor_set_attributes: Some(dce_ipp_cursor_set_attributes),
    ipp_cursor_set_position: Some(dce_ipp_cursor_set_position),
    ipp_program_prescale: Some(dce_ipp_program_prescale),
    ipp_program_input_lut: Some(dce_ipp_program_input_lut),
    ipp_set_degamma: Some(dce_ipp_set_degamma),
};

pub unsafe fn dce_ipp_construct(
    ipp_dce: *mut dce_ipp, ctx: *mut dc_context, inst: i32,
    regs: *const dce_ipp_registers, ipp_shift: *const dce_ipp_shift,
    ipp_mask: *const dce_ipp_mask,
) {
    (*ipp_dce).base.ctx = ctx;
    (*ipp_dce).base.inst = inst;
    (*ipp_dce).base.funcs = &dce_ipp_funcs;
    (*ipp_dce).regs = regs;
    (*ipp_dce).ipp_shift = ipp_shift;
    (*ipp_dce).ipp_mask = ipp_mask;
}

pub unsafe fn dce_ipp_destroy(ipp: *mut *mut input_pixel_processor) {
    kfree!(TO_DCE_IPP!(*ipp));
    *ipp = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
