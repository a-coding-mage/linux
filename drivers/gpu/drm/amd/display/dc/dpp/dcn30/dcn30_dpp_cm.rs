/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// C dependencies and register-access macros are supplied by the surrounding driver.

unsafe fn dpp3_enable_cm_block(dpp_base: *mut dpp) {
    let dpp = TO_DCN30_DPP(dpp_base);
    let mut cm_bypass_mode: u32 = 0;
    if (*(*dpp_base).ctx).dc.debug.cm_in_bypass { cm_bypass_mode = 1; }
    REG_UPDATE!(dpp, CM_CONTROL, CM_BYPASS, cm_bypass_mode);
}

unsafe fn dpp30_get_gamcor_current(dpp_base: *mut dpp) -> dc_lut_mode {
    let dpp = TO_DCN30_DPP(dpp_base);
    let mut state_mode = 0u32;
    let mut lut_mode = 0u32;
    let mut mode = LUT_BYPASS;
    REG_GET!(dpp, CM_GAMCOR_CONTROL, CM_GAMCOR_MODE_CURRENT, &mut state_mode);
    if state_mode == 2 {
        REG_GET!(dpp, CM_GAMCOR_CONTROL, CM_GAMCOR_SELECT_CURRENT, &mut lut_mode);
        mode = if lut_mode == 0 { LUT_RAM_A } else { LUT_RAM_B };
    }
    mode
}

unsafe fn dpp3_program_gammcor_lut(dpp_base: *mut dpp, rgb: *const pwl_result_data, num: u32, is_ram_a: bool) {
    let _ = is_ram_a;
    let dpp = TO_DCN30_DPP(dpp_base);
    let last = *rgb.add((num - 1) as usize);
    let last_r = last.red_reg + last.delta_red_reg;
    let last_g = last.green_reg + last.delta_green_reg;
    let last_b = last.blue_reg + last.delta_blue_reg;
    if is_rgb_equal(rgb, num) {
        for i in 0..num { REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, (*rgb.add(i as usize)).red_reg); }
        REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, last_r);
    } else {
        REG_UPDATE!(dpp, CM_GAMCOR_LUT_CONTROL, CM_GAMCOR_LUT_WRITE_COLOR_MASK, 4);
        for i in 0..num { REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, (*rgb.add(i as usize)).red_reg); }
        REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, last_r);
        REG_SET!(dpp, CM_GAMCOR_LUT_INDEX, 0, CM_GAMCOR_LUT_INDEX, 0);
        REG_UPDATE!(dpp, CM_GAMCOR_LUT_CONTROL, CM_GAMCOR_LUT_WRITE_COLOR_MASK, 2);
        for i in 0..num { REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, (*rgb.add(i as usize)).green_reg); }
        REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, last_g);
        REG_SET!(dpp, CM_GAMCOR_LUT_INDEX, 0, CM_GAMCOR_LUT_INDEX, 0);
        REG_UPDATE!(dpp, CM_GAMCOR_LUT_CONTROL, CM_GAMCOR_LUT_WRITE_COLOR_MASK, 1);
        for i in 0..num { REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, (*rgb.add(i as usize)).blue_reg); }
        REG_SET!(dpp, CM_GAMCOR_LUT_DATA, 0, CM_GAMCOR_LUT_DATA, last_b);
    }
}

unsafe fn dpp3_power_on_gamcor_lut(dpp_base: *mut dpp, power_on: bool) {
    let dpp = TO_DCN30_DPP(dpp_base);
    if (*(*dpp_base).ctx).dc.debug.enable_mem_low_power.bits.cm {
        if power_on {
            REG_UPDATE!(dpp, CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_FORCE, 0);
            if (*(*dpp_base).ctx).dc.caps.ips_v2_support { REG_UPDATE!(dpp, CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_DIS, 1); }
            REG_WAIT!(dpp, CM_MEM_PWR_STATUS, GAMCOR_MEM_PWR_STATE, 0, 1, 5);
            (*dpp_base).deferred_reg_writes.bits.disable_gamcor = false;
        } else {
            (*(*dpp_base).ctx).dc.optimized_required = true;
            (*dpp_base).deferred_reg_writes.bits.disable_gamcor = true;
        }
    } else { REG_SET!(dpp, CM_MEM_PWR_CTRL, 0, GAMCOR_MEM_PWR_DIS, if power_on { 0 } else { 1 }); }
}

pub unsafe fn dpp3_program_cm_dealpha(dpp_base: *mut dpp, enable: u32, additive_blending: u32) {
    let dpp = TO_DCN30_DPP(dpp_base);
    REG_SET_2!(dpp, CM_DEALPHA, 0, CM_DEALPHA_EN, enable, CM_DEALPHA_ABLND, additive_blending);
}

pub unsafe fn dpp3_program_cm_bias(dpp_base: *mut dpp, bias_params: *const CM_bias_params) {
    let dpp = TO_DCN30_DPP(dpp_base);
    REG_SET!(dpp, CM_BIAS_CR_R, 0, CM_BIAS_CR_R, (*bias_params).cm_bias_cr_r);
    REG_SET_2!(dpp, CM_BIAS_Y_G_CB_B, 0, CM_BIAS_Y_G, (*bias_params).cm_bias_y_g, CM_BIAS_CB_B, (*bias_params).cm_bias_cb_b);
}

unsafe fn dpp3_configure_gamcor_lut(dpp_base: *mut dpp, is_ram_a: bool) {
    let dpp = TO_DCN30_DPP(dpp_base);
    REG_UPDATE!(dpp, CM_GAMCOR_LUT_CONTROL, CM_GAMCOR_LUT_WRITE_COLOR_MASK, 7);
    REG_UPDATE!(dpp, CM_GAMCOR_LUT_CONTROL, CM_GAMCOR_LUT_HOST_SEL, if is_ram_a { 0 } else { 1 });
    REG_SET!(dpp, CM_GAMCOR_LUT_INDEX, 0, CM_GAMCOR_LUT_INDEX, 0);
}

unsafe fn dpp3_gamcor_reg_field(dpp: *mut dcn3_dpp, reg: *mut dcn3_xfer_func_reg) {
    (*reg).shifts.field_region_start_base = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_START_BASE_B;
    (*reg).masks.field_region_start_base = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_START_BASE_B;
    (*reg).shifts.field_offset = (*dpp).tf_shift.CM_GAMCOR_RAMA_OFFSET_B;
    (*reg).masks.field_offset = (*dpp).tf_mask.CM_GAMCOR_RAMA_OFFSET_B;
    (*reg).shifts.exp_region0_lut_offset = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).masks.exp_region0_lut_offset = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).shifts.exp_region0_num_segments = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).masks.exp_region0_num_segments = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).shifts.exp_region1_lut_offset = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).masks.exp_region1_lut_offset = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).shifts.exp_region1_num_segments = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).masks.exp_region1_num_segments = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).shifts.field_region_end = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_END_B;
    (*reg).masks.field_region_end = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_END_B;
    (*reg).shifts.field_region_end_slope = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).masks.field_region_end_slope = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).shifts.field_region_end_base = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_END_BASE_B;
    (*reg).masks.field_region_end_base = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_END_BASE_B;
    (*reg).shifts.field_region_linear_slope = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).masks.field_region_linear_slope = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).shifts.exp_region_start = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_START_B;
    (*reg).masks.exp_region_start = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_START_B;
    (*reg).shifts.exp_resion_start_segment = (*dpp).tf_shift.CM_GAMCOR_RAMA_EXP_REGION_START_SEGMENT_B;
    (*reg).masks.exp_resion_start_segment = (*dpp).tf_mask.CM_GAMCOR_RAMA_EXP_REGION_START_SEGMENT_B;
}

pub unsafe fn dpp3_program_gamcor_lut(dpp_base: *mut dpp, params: *const pwl_params) -> bool {
    let dpp = TO_DCN30_DPP(dpp_base);
    dpp3_enable_cm_block(dpp_base);
    if params.is_null() {
        REG_SET!(dpp, CM_GAMCOR_CONTROL, 0, CM_GAMCOR_MODE, 0);
        if (*(*dpp_base).ctx).dc.debug.enable_mem_low_power.bits.cm { dpp3_power_on_gamcor_lut(dpp_base, false); }
        return false;
    }
    dpp3_power_on_gamcor_lut(dpp_base, true);
    REG_SET!(dpp, CM_GAMCOR_CONTROL, 0, CM_GAMCOR_MODE, 2);
    let current_mode = dpp30_get_gamcor_current(dpp_base);
    let next_mode = if current_mode == LUT_BYPASS || current_mode == LUT_RAM_A { LUT_RAM_B } else { LUT_RAM_A };
    dpp3_power_on_gamcor_lut(dpp_base, true);
    dpp3_configure_gamcor_lut(dpp_base, next_mode == LUT_RAM_A);
    let mut gam_regs: dcn3_xfer_func_reg = core::mem::zeroed();
    dpp3_gamcor_reg_field(dpp, &mut gam_regs);
    cm_helper_program_gamcor_xfer_func((*dpp_base).ctx, params, &gam_regs);
    dpp3_program_gammcor_lut(dpp_base, (*params).rgb_resulted, (*params).hw_points_num, next_mode == LUT_RAM_A);
    REG_UPDATE!(dpp, CM_GAMCOR_CONTROL, CM_GAMCOR_SELECT, if next_mode == LUT_RAM_A { 0 } else { 1 });
    true
}

pub unsafe fn dpp3_set_hdr_multiplier(dpp_base: *mut dpp, multiplier: u32) {
    let dpp = TO_DCN30_DPP(dpp_base);
    REG_UPDATE!(dpp, CM_HDR_MULT_COEF, CM_HDR_MULT_COEF, multiplier);
}

unsafe fn program_gamut_remap(dpp: *mut dcn3_dpp, regval: *const u16, select: u32) {
    if regval.is_null() || select == GAMUT_REMAP_BYPASS { REG_SET!(dpp, CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, 0); return; }
    let selection = match select { GAMUT_REMAP_COEFF => 1, GAMUT_REMAP_COMA_COEFF => 2, _ => 0 };
    let mut regs: color_matrices_reg = core::mem::zeroed();
    regs.shifts.csc_c11 = (*dpp).tf_shift.CM_GAMUT_REMAP_C11;
    regs.masks.csc_c11 = (*dpp).tf_mask.CM_GAMUT_REMAP_C11;
    regs.shifts.csc_c12 = (*dpp).tf_shift.CM_GAMUT_REMAP_C12;
    regs.masks.csc_c12 = (*dpp).tf_mask.CM_GAMUT_REMAP_C12;
    if select == GAMUT_REMAP_COEFF { regs.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_C11_C12); regs.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_C33_C34); }
    else if select == GAMUT_REMAP_COMA_COEFF { regs.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_B_C11_C12); regs.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_B_C33_C34); }
    cm_helper_program_color_matrices((*dpp).base.ctx, regval, &regs);
    REG_SET!(dpp, CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, selection);
}

pub unsafe fn dpp3_cm_set_gamut_remap(dpp_base: *mut dpp, adjust: *const dpp_grph_csc_adjustment) {
    let dpp = TO_DCN30_DPP(dpp_base);
    if (*adjust).gamut_adjust_type != GRAPHICS_GAMUT_ADJUST_TYPE_SW { program_gamut_remap(dpp, core::ptr::null(), GAMUT_REMAP_BYPASS); }
    else {
        let mut matrix: [fixed31_32; 12] = core::mem::zeroed();
        let mut regval = [0u16; 12];
        for i in 0..12 { matrix[i] = (*adjust).temperature_matrix[i]; }
        convert_float_matrix(regval.as_mut_ptr(), matrix.as_ptr(), CM_GAMUT_REMAP_COEF_FORMAT_S2_13, 12);
        let mut mode = 0u32;
        REG_GET!(dpp, CM_GAMUT_REMAP_CONTROL, CM_GAMUT_REMAP_MODE_CURRENT, &mut mode);
        mode = if mode == 0 { 1 } else if mode == 1 { 2 } else { 1 };
        program_gamut_remap(dpp, regval.as_ptr(), mode);
    }
}

unsafe fn read_gamut_remap(dpp: *mut dcn3_dpp, regval: *mut u16, select: *mut i32) {
    let mut selection = 0u32;
    REG_GET!(dpp, CM_GAMUT_REMAP_CONTROL, CM_GAMUT_REMAP_MODE_CURRENT, &mut selection);
    *select = selection as i32;
    let mut regs: color_matrices_reg = core::mem::zeroed();
    regs.shifts.csc_c11 = (*dpp).tf_shift.CM_GAMUT_REMAP_C11; regs.masks.csc_c11 = (*dpp).tf_mask.CM_GAMUT_REMAP_C11;
    regs.shifts.csc_c12 = (*dpp).tf_shift.CM_GAMUT_REMAP_C12; regs.masks.csc_c12 = (*dpp).tf_mask.CM_GAMUT_REMAP_C12;
    if *select == GAMUT_REMAP_COEFF { regs.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_C11_C12); regs.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_C33_C34); cm_helper_read_color_matrices((*dpp).base.ctx, regval, &regs); }
    else if *select == GAMUT_REMAP_COMA_COEFF { regs.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_B_C11_C12); regs.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_B_C33_C34); cm_helper_read_color_matrices((*dpp).base.ctx, regval, &regs); }
}

pub unsafe fn dpp3_cm_get_gamut_remap(dpp_base: *mut dpp, adjust: *mut dpp_grph_csc_adjustment) {
    let dpp = TO_DCN30_DPP(dpp_base); let mut vals = [0u16; 12]; let mut select = 0i32;
    read_gamut_remap(dpp, vals.as_mut_ptr(), &mut select);
    if select == GAMUT_REMAP_BYPASS { (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS; return; }
    (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_SW;
    convert_hw_matrix((*adjust).temperature_matrix.as_mut_ptr(), vals.as_ptr(), CM_GAMUT_REMAP_COEF_FORMAT_S2_13, vals.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
