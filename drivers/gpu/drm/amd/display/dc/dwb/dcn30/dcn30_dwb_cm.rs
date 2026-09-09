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
 */

// Dependencies supplied by the surrounding driver translation.

unsafe fn dwb3_get_reg_field_ogam(dwbc30: *mut dcn30_dwbc, reg: *mut dcn3_xfer_func_reg) {
    (*reg).shifts.field_region_start_base = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_START_BASE_B;
    (*reg).masks.field_region_start_base = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_START_BASE_B;
    (*reg).shifts.field_offset = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_OFFSET_B;
    (*reg).masks.field_offset = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_OFFSET_B;
    (*reg).shifts.exp_region0_lut_offset = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).masks.exp_region0_lut_offset = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).shifts.exp_region0_num_segments = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).masks.exp_region0_num_segments = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).shifts.exp_region1_lut_offset = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).masks.exp_region1_lut_offset = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).shifts.exp_region1_num_segments = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).masks.exp_region1_num_segments = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).shifts.field_region_end = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_END_B;
    (*reg).masks.field_region_end = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_END_B;
    (*reg).shifts.field_region_end_slope = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).masks.field_region_end_slope = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).shifts.field_region_end_base = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_END_BASE_B;
    (*reg).masks.field_region_end_base = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_END_BASE_B;
    (*reg).shifts.field_region_linear_slope = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).masks.field_region_linear_slope = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).shifts.exp_region_start = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_START_B;
    (*reg).masks.exp_region_start = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_START_B;
    (*reg).shifts.exp_resion_start_segment = (*dwbc30).dwbc_shift.DWB_OGAM_RAMA_EXP_REGION_START_SEGMENT_B;
    (*reg).masks.exp_resion_start_segment = (*dwbc30).dwbc_mask.DWB_OGAM_RAMA_EXP_REGION_START_SEGMENT_B;
}

unsafe fn dwb3_program_ogam_luta_settings(dwbc30: *mut dcn30_dwbc, params: *const pwl_params) {
    let mut gam_regs: dcn3_xfer_func_reg = core::mem::zeroed();
    dwb3_get_reg_field_ogam(dwbc30, &mut gam_regs);
    gam_regs.start_cntl_b = REG!(dwbc30, DWB_OGAM_RAMA_START_CNTL_B);
    gam_regs.start_cntl_g = REG!(dwbc30, DWB_OGAM_RAMA_START_CNTL_G);
    gam_regs.start_cntl_r = REG!(dwbc30, DWB_OGAM_RAMA_START_CNTL_R);
    gam_regs.start_base_cntl_b = REG!(dwbc30, DWB_OGAM_RAMA_START_BASE_CNTL_B);
    gam_regs.start_base_cntl_g = REG!(dwbc30, DWB_OGAM_RAMA_START_BASE_CNTL_G);
    gam_regs.start_base_cntl_r = REG!(dwbc30, DWB_OGAM_RAMA_START_BASE_CNTL_R);
    gam_regs.start_slope_cntl_b = REG!(dwbc30, DWB_OGAM_RAMA_START_SLOPE_CNTL_B);
    gam_regs.start_slope_cntl_g = REG!(dwbc30, DWB_OGAM_RAMA_START_SLOPE_CNTL_G);
    gam_regs.start_slope_cntl_r = REG!(dwbc30, DWB_OGAM_RAMA_START_SLOPE_CNTL_R);
    gam_regs.start_end_cntl1_b = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL1_B);
    gam_regs.start_end_cntl2_b = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL2_B);
    gam_regs.start_end_cntl1_g = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL1_G);
    gam_regs.start_end_cntl2_g = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL2_G);
    gam_regs.start_end_cntl1_r = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL1_R);
    gam_regs.start_end_cntl2_r = REG!(dwbc30, DWB_OGAM_RAMA_END_CNTL2_R);
    gam_regs.offset_b = REG!(dwbc30, DWB_OGAM_RAMA_OFFSET_B);
    gam_regs.offset_g = REG!(dwbc30, DWB_OGAM_RAMA_OFFSET_G);
    gam_regs.offset_r = REG!(dwbc30, DWB_OGAM_RAMA_OFFSET_R);
    gam_regs.region_start = REG!(dwbc30, DWB_OGAM_RAMA_REGION_0_1);
    gam_regs.region_end = REG!(dwbc30, DWB_OGAM_RAMA_REGION_32_33);
    cm_helper_program_gamcor_xfer_func((*dwbc30).base.ctx, params, &gam_regs);
}

unsafe fn dwb3_program_ogam_lutb_settings(dwbc30: *mut dcn30_dwbc, params: *const pwl_params) {
    let mut gam_regs: dcn3_xfer_func_reg = core::mem::zeroed();
    dwb3_get_reg_field_ogam(dwbc30, &mut gam_regs);
    gam_regs.start_cntl_b = REG!(dwbc30, DWB_OGAM_RAMB_START_CNTL_B);
    gam_regs.start_cntl_g = REG!(dwbc30, DWB_OGAM_RAMB_START_CNTL_G);
    gam_regs.start_cntl_r = REG!(dwbc30, DWB_OGAM_RAMB_START_CNTL_R);
    gam_regs.start_base_cntl_b = REG!(dwbc30, DWB_OGAM_RAMB_START_BASE_CNTL_B);
    gam_regs.start_base_cntl_g = REG!(dwbc30, DWB_OGAM_RAMB_START_BASE_CNTL_G);
    gam_regs.start_base_cntl_r = REG!(dwbc30, DWB_OGAM_RAMB_START_BASE_CNTL_R);
    gam_regs.start_slope_cntl_b = REG!(dwbc30, DWB_OGAM_RAMB_START_SLOPE_CNTL_B);
    gam_regs.start_slope_cntl_g = REG!(dwbc30, DWB_OGAM_RAMB_START_SLOPE_CNTL_G);
    gam_regs.start_slope_cntl_r = REG!(dwbc30, DWB_OGAM_RAMB_START_SLOPE_CNTL_R);
    gam_regs.start_end_cntl1_b = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL1_B);
    gam_regs.start_end_cntl2_b = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL2_B);
    gam_regs.start_end_cntl1_g = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL1_G);
    gam_regs.start_end_cntl2_g = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL2_G);
    gam_regs.start_end_cntl1_r = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL1_R);
    gam_regs.start_end_cntl2_r = REG!(dwbc30, DWB_OGAM_RAMB_END_CNTL2_R);
    gam_regs.offset_b = REG!(dwbc30, DWB_OGAM_RAMB_OFFSET_B);
    gam_regs.offset_g = REG!(dwbc30, DWB_OGAM_RAMB_OFFSET_G);
    gam_regs.offset_r = REG!(dwbc30, DWB_OGAM_RAMB_OFFSET_R);
    gam_regs.region_start = REG!(dwbc30, DWB_OGAM_RAMB_REGION_0_1);
    gam_regs.region_end = REG!(dwbc30, DWB_OGAM_RAMB_REGION_32_33);
    cm_helper_program_gamcor_xfer_func((*dwbc30).base.ctx, params, &gam_regs);
}

unsafe fn dwb3_get_ogam_current(dwbc30: *mut dcn30_dwbc) -> dc_lut_mode {
    let (mut state_mode, mut ram_select) = (0u32, 0u32);
    REG_GET_2!(dwbc30, DWB_OGAM_CONTROL, DWB_OGAM_MODE_CURRENT, &mut state_mode, DWB_OGAM_SELECT_CURRENT, &mut ram_select);
    if state_mode == 0 { LUT_BYPASS } else if state_mode == 2 {
        if ram_select == 0 { LUT_RAM_A } else if ram_select == 1 { LUT_RAM_B } else { LUT_BYPASS }
    } else { BREAK_TO_DEBUGGER!(); LUT_BYPASS }
}

unsafe fn dwb3_configure_ogam_lut(dwbc30: *mut dcn30_dwbc, is_ram_a: bool) {
    REG_UPDATE_2!(dwbc30, DWB_OGAM_LUT_CONTROL, DWB_OGAM_LUT_WRITE_COLOR_MASK, 7, DWB_OGAM_LUT_HOST_SEL, if is_ram_a { 0 } else { 1 });
    REG_SET!(dwbc30, DWB_OGAM_LUT_INDEX, 0, DWB_OGAM_LUT_INDEX, 0);
}

unsafe fn dwb3_program_ogam_pwl(dwbc30: *mut dcn30_dwbc, rgb: *const pwl_result_data, num: u32) {
    let last = &*rgb.add((num - 1) as usize);
    let last_r = last.red_reg + last.delta_red_reg;
    let last_g = last.green_reg + last.delta_green_reg;
    let last_b = last.blue_reg + last.delta_blue_reg;
    if is_rgb_equal(rgb, num) {
        for i in 0..num { REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, (*rgb.add(i as usize)).red_reg); }
        REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, last_r);
    } else {
        REG_UPDATE!(dwbc30, DWB_OGAM_LUT_CONTROL, DWB_OGAM_LUT_WRITE_COLOR_MASK, 4);
        for i in 0..num { REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, (*rgb.add(i as usize)).red_reg); }
        REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, last_r);
        REG_SET!(dwbc30, DWB_OGAM_LUT_INDEX, 0, DWB_OGAM_LUT_INDEX, 0);
        REG_UPDATE!(dwbc30, DWB_OGAM_LUT_CONTROL, DWB_OGAM_LUT_WRITE_COLOR_MASK, 2);
        for i in 0..num { REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, (*rgb.add(i as usize)).green_reg); }
        REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, last_g);
        REG_SET!(dwbc30, DWB_OGAM_LUT_INDEX, 0, DWB_OGAM_LUT_INDEX, 0);
        REG_UPDATE!(dwbc30, DWB_OGAM_LUT_CONTROL, DWB_OGAM_LUT_WRITE_COLOR_MASK, 1);
        for i in 0..num { REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, (*rgb.add(i as usize)).blue_reg); }
        REG_SET!(dwbc30, DWB_OGAM_LUT_DATA, 0, DWB_OGAM_LUT_DATA, last_b);
    }
}

unsafe fn dwb3_program_ogam_lut(dwbc30: *mut dcn30_dwbc, params: *const pwl_params) -> bool {
    if params.is_null() { REG_SET!(dwbc30, DWB_OGAM_CONTROL, 0, DWB_OGAM_MODE, 0); return false; }
    if (*params).hw_points_num == 0 { return false; }
    REG_SET!(dwbc30, DWB_OGAM_CONTROL, 0, DWB_OGAM_MODE, 2);
    let current = dwb3_get_ogam_current(dwbc30);
    let next = if current == LUT_BYPASS || current == LUT_RAM_A { LUT_RAM_B } else { LUT_RAM_A };
    dwb3_configure_ogam_lut(dwbc30, next == LUT_RAM_A);
    if next == LUT_RAM_A { dwb3_program_ogam_luta_settings(dwbc30, params); } else { dwb3_program_ogam_lutb_settings(dwbc30, params); }
    dwb3_program_ogam_pwl(dwbc30, (*params).rgb_resulted, (*params).hw_points_num);
    REG_UPDATE!(dwbc30, DWB_OGAM_CONTROL, DWB_OGAM_SELECT, if next == LUT_RAM_A { 0 } else { 1 });
    true
}

pub unsafe fn dwb3_ogam_set_input_transfer_func(dwbc: *mut dwbc, input: *const dc_transfer_func) -> bool {
    if input.is_null() { return false; }
    let dwbc30 = TO_DCN30_DWBC!(dwbc);
    let lut = kzalloc_obj!(pwl_params);
    if !lut.is_null() {
        cm_helper_translate_curve_to_hw_format((*dwbc).ctx, input, lut, false);
        let result = dwb3_program_ogam_lut(dwbc30, lut);
        kfree!(lut);
        return result;
    }
    false
}

unsafe fn dwb3_program_gamut_remap(dwbc: *mut dwbc, regval: *const u16, coef_format: cm_gamut_coef_format, select: cm_gamut_remap_select) {
    let dwbc30 = TO_DCN30_DWBC!(dwbc);
    if regval.is_null() || select == CM_GAMUT_REMAP_MODE_BYPASS { REG_SET!(dwbc30, DWB_GAMUT_REMAP_MODE, 0, DWB_GAMUT_REMAP_MODE, 0); return; }
    REG_UPDATE!(dwbc30, DWB_GAMUT_REMAP_COEF_FORMAT, DWB_GAMUT_REMAP_COEF_FORMAT, coef_format);
    let mut gam_regs: color_matrices_reg = core::mem::zeroed();
    gam_regs.shifts.csc_c11 = (*dwbc30).dwbc_shift.DWB_GAMUT_REMAPA_C11;
    gam_regs.masks.csc_c11 = (*dwbc30).dwbc_mask.DWB_GAMUT_REMAPA_C11;
    gam_regs.shifts.csc_c12 = (*dwbc30).dwbc_shift.DWB_GAMUT_REMAPA_C12;
    gam_regs.masks.csc_c12 = (*dwbc30).dwbc_mask.DWB_GAMUT_REMAPA_C12;
    match select {
        CM_GAMUT_REMAP_MODE_RAMA_COEFF => { gam_regs.csc_c11_c12 = REG!(dwbc30, DWB_GAMUT_REMAPA_C11_C12); gam_regs.csc_c33_c34 = REG!(dwbc30, DWB_GAMUT_REMAPA_C33_C34); cm_helper_program_color_matrices((*dwbc30).base.ctx, regval, &gam_regs); }
        CM_GAMUT_REMAP_MODE_RAMB_COEFF => { gam_regs.csc_c11_c12 = REG!(dwbc30, DWB_GAMUT_REMAPB_C11_C12); gam_regs.csc_c33_c34 = REG!(dwbc30, DWB_GAMUT_REMAPB_C33_C34); cm_helper_program_color_matrices((*dwbc30).base.ctx, regval, &gam_regs); }
        CM_GAMUT_REMAP_MODE_RESERVED => { BREAK_TO_DEBUGGER!(); return; }
        _ => {}
    }
    REG_SET!(dwbc30, DWB_GAMUT_REMAP_MODE, 0, DWB_GAMUT_REMAP_MODE, select);
}

pub unsafe fn dwb3_set_gamut_remap(dwbc: *mut dwbc, params: *const dc_dwb_params) {
    let dwbc30 = TO_DCN30_DWBC!(dwbc);
    let adjust = (*params).csc_params;
    if adjust.gamut_adjust_type != CM_GAMUT_ADJUST_TYPE_SW { dwb3_program_gamut_remap(dwbc, core::ptr::null(), adjust.gamut_coef_format, CM_GAMUT_REMAP_MODE_BYPASS); }
    else {
        let mut arr_matrix: [fixed31_32; 12] = core::mem::zeroed();
        let mut arr_reg_val: [u16; 12] = [0; 12];
        for i in 0..12 { arr_matrix[i] = adjust.temperature_matrix[i]; }
        convert_float_matrix(arr_reg_val.as_mut_ptr(), arr_matrix.as_ptr(), CM_GAMUT_REMAP_COEF_FORMAT_S2_13, 12);
        let mut current_mode = 0u32;
        REG_GET!(dwbc30, DWB_GAMUT_REMAP_MODE, DWB_GAMUT_REMAP_MODE_CURRENT, &mut current_mode);
        let next = if current_mode == CM_GAMUT_REMAP_MODE_RAMA_COEFF { CM_GAMUT_REMAP_MODE_RAMB_COEFF } else { CM_GAMUT_REMAP_MODE_RAMA_COEFF };
        dwb3_program_gamut_remap(dwbc, arr_reg_val.as_ptr(), adjust.gamut_coef_format, next);
    }
}

pub unsafe fn dwb3_program_hdr_mult(dwbc: *mut dwbc, params: *const dc_dwb_params) {
    let dwbc30 = TO_DCN30_DWBC!(dwbc);
    REG_UPDATE!(dwbc30, DWB_HDR_MULT_COEF, DWB_HDR_MULT_COEF, (*params).hdr_mult);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
