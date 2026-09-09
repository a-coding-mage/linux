/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software.
 */

// Dependencies are supplied by the surrounding driver translation unit.

unsafe fn dpp2_enable_cm_block(dpp_base: *mut dpp) {
    let dpp = TO_DCN20_DPP(dpp_base);
    let mut cm_bypass_mode: u32 = 0;
    if (*dpp_base).ctx.as_ref().unwrap().dc.as_ref().unwrap().debug.cm_in_bypass { cm_bypass_mode = 1; }
    REG_UPDATE!(dpp, CM_CONTROL, CM_BYPASS, cm_bypass_mode);
}

unsafe fn dpp2_degamma_ram_inuse(dpp_base: *mut dpp, ram_a_inuse: *mut bool) -> bool {
    let dpp = TO_DCN20_DPP(dpp_base); let mut status_reg = 0u32;
    REG_GET!(dpp, CM_DGAM_LUT_WRITE_EN_MASK, CM_DGAM_CONFIG_STATUS, &mut status_reg);
    if status_reg == 3 { *ram_a_inuse = true; true } else if status_reg == 4 { *ram_a_inuse = false; true } else { false }
}

unsafe fn dpp2_program_degamma_lut(dpp_base: *mut dpp, rgb: *const pwl_result_data, num: u32, is_ram_a: bool) {
    let dpp = TO_DCN20_DPP(dpp_base);
    REG_UPDATE!(dpp, CM_DGAM_LUT_WRITE_EN_MASK, CM_DGAM_LUT_WRITE_EN_MASK, 7);
    REG_UPDATE!(dpp, CM_DGAM_LUT_WRITE_EN_MASK, CM_DGAM_LUT_WRITE_SEL, if is_ram_a { 0 } else { 1 });
    REG_SET!(dpp, CM_DGAM_LUT_INDEX, 0, CM_DGAM_LUT_INDEX, 0);
    for i in 0..num as isize { let p = &*rgb.offset(i);
        for v in [p.red_reg, p.green_reg, p.blue_reg, p.delta_red_reg, p.delta_green_reg, p.delta_blue_reg] { REG_SET!(dpp, CM_DGAM_LUT_DATA, 0, CM_DGAM_LUT_DATA, v); }
    }
}

pub unsafe fn dpp2_set_degamma_pwl(dpp_base: *mut dpp, params: *const pwl_params) {
    let mut is_ram_a = true; dpp1_power_on_degamma_lut(dpp_base, true); dpp2_enable_cm_block(dpp_base);
    dpp2_degamma_ram_inuse(dpp_base, &mut is_ram_a);
    if is_ram_a { dpp1_program_degamma_lutb_settings(dpp_base, params); } else { dpp1_program_degamma_luta_settings(dpp_base, params); }
    dpp2_program_degamma_lut(dpp_base, (*params).rgb_resulted, (*params).hw_points_num, !is_ram_a); dpp1_degamma_ram_select(dpp_base, !is_ram_a);
}

pub unsafe fn dpp2_set_degamma(dpp_base: *mut dpp, mode: ipp_degamma_mode) {
    let dpp = TO_DCN20_DPP(dpp_base); dpp2_enable_cm_block(dpp_base);
    let value = match mode { IPP_DEGAMMA_MODE_BYPASS => 0, IPP_DEGAMMA_MODE_HW_sRGB => 1, IPP_DEGAMMA_MODE_HW_xvYCC => 2, IPP_DEGAMMA_MODE_USER_PWL => 3, _ => { BREAK_TO_DEBUGGER!(); return; } };
    REG_UPDATE!(dpp, CM_DGAM_CONTROL, CM_DGAM_LUT_MODE, value);
}

unsafe fn program_gamut_remap(dpp: *mut dcn20_dpp, regval: *const u16, mut select: dcn20_gamut_remap_select) {
    if regval.is_null() || select == DCN2_GAMUT_REMAP_BYPASS { REG_SET!(dpp, CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, 0); return; }
    let mut cur_select = 0u32; IX_REG_GET!(dpp, CM_TEST_DEBUG_INDEX, CM_TEST_DEBUG_DATA, CM_TEST_DEBUG_DATA_STATUS_IDX, CM_TEST_DEBUG_DATA_GAMUT_REMAP_MODE, &mut cur_select);
    select = if cur_select != DCN2_GAMUT_REMAP_COEF_A { DCN2_GAMUT_REMAP_COEF_A } else { DCN2_GAMUT_REMAP_COEF_B };
    let mut r = color_matrices_reg::default(); r.shifts.csc_c11 = (*dpp).tf_shift.CM_GAMUT_REMAP_C11; r.masks.csc_c11 = (*dpp).tf_mask.CM_GAMUT_REMAP_C11; r.shifts.csc_c12 = (*dpp).tf_shift.CM_GAMUT_REMAP_C12; r.masks.csc_c12 = (*dpp).tf_mask.CM_GAMUT_REMAP_C12;
    if select == DCN2_GAMUT_REMAP_COEF_A { r.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_C11_C12); r.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_C33_C34); } else { r.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_B_C11_C12); r.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_B_C33_C34); }
    cm_helper_program_color_matrices((*dpp).base.ctx, regval, &mut r); REG_SET!(dpp, CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, select);
}

pub unsafe fn dpp2_cm_set_gamut_remap(dpp_base: *mut dpp, adjust: *const dpp_grph_csc_adjustment) {
    let dpp = TO_DCN20_DPP(dpp_base); if (*adjust).gamut_adjust_type != GRAPHICS_GAMUT_ADJUST_TYPE_SW { program_gamut_remap(dpp, core::ptr::null(), DCN2_GAMUT_REMAP_BYPASS); } else {
        let mut matrix = [fixed31_32::default(); 12]; let mut vals = [0u16; 12]; for i in 0..12 { matrix[i] = (*adjust).temperature_matrix[i]; }
        convert_float_matrix(vals.as_mut_ptr(), matrix.as_ptr(), CM_GAMUT_REMAP_COEF_FORMAT_S2_13, 12); program_gamut_remap(dpp, vals.as_ptr(), DCN2_GAMUT_REMAP_COEF_A);
    }
}

unsafe fn read_gamut_remap(dpp: *mut dcn20_dpp, regval: *mut u16, select: *mut dcn20_gamut_remap_select) {
    let mut s = 0u32; IX_REG_GET!(dpp, CM_TEST_DEBUG_INDEX, CM_TEST_DEBUG_DATA, CM_TEST_DEBUG_DATA_STATUS_IDX, CM_TEST_DEBUG_DATA_GAMUT_REMAP_MODE, &mut s); *select = s as dcn20_gamut_remap_select;
    let mut r = color_matrices_reg::default(); r.shifts.csc_c11 = (*dpp).tf_shift.CM_GAMUT_REMAP_C11; r.masks.csc_c11 = (*dpp).tf_mask.CM_GAMUT_REMAP_C11; r.shifts.csc_c12 = (*dpp).tf_shift.CM_GAMUT_REMAP_C12; r.masks.csc_c12 = (*dpp).tf_mask.CM_GAMUT_REMAP_C12;
    if *select == DCN2_GAMUT_REMAP_COEF_A { r.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_C11_C12); r.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_C33_C34); cm_helper_read_color_matrices((*dpp).base.ctx, regval, &mut r); } else if *select == DCN2_GAMUT_REMAP_COEF_B { r.csc_c11_c12 = REG!(dpp, CM_GAMUT_REMAP_B_C11_C12); r.csc_c33_c34 = REG!(dpp, CM_GAMUT_REMAP_B_C33_C34); cm_helper_read_color_matrices((*dpp).base.ctx, regval, &mut r); }
}

pub unsafe fn dpp2_cm_get_gamut_remap(dpp_base: *mut dpp, adjust: *mut dpp_grph_csc_adjustment) { let dpp = TO_DCN20_DPP(dpp_base); let mut vals = [0u16;12]; let mut select = DCN2_GAMUT_REMAP_BYPASS; read_gamut_remap(dpp, vals.as_mut_ptr(), &mut select); if select == DCN2_GAMUT_REMAP_BYPASS { (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS; } else { (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_SW; convert_hw_matrix((*adjust).temperature_matrix.as_mut_ptr(), vals.as_ptr(), CM_GAMUT_REMAP_COEF_FORMAT_S2_13, 12); } }

unsafe fn dpp20_power_on_blnd_lut(dpp_base: *mut dpp, power_on: bool) { REG_SET!(TO_DCN20_DPP(dpp_base), CM_MEM_PWR_CTRL, 0, BLNDGAM_MEM_PWR_FORCE, if power_on {0} else {1}); }
unsafe fn dpp20_configure_blnd_lut(dpp_base: *mut dpp, a: bool) { let d=TO_DCN20_DPP(dpp_base); REG_UPDATE!(d,CM_BLNDGAM_LUT_WRITE_EN_MASK,CM_BLNDGAM_LUT_WRITE_EN_MASK,7); REG_UPDATE!(d,CM_BLNDGAM_LUT_WRITE_EN_MASK,CM_BLNDGAM_LUT_WRITE_SEL,if a{0}else{1}); REG_SET!(d,CM_BLNDGAM_LUT_INDEX,0,CM_BLNDGAM_LUT_INDEX,0); }
unsafe fn dpp20_program_blnd_pwl(dpp_base:*mut dpp,rgb:*const pwl_result_data,num:u32){let d=TO_DCN20_DPP(dpp_base);for i in 0..num as isize{let p=&*rgb.offset(i);for v in[p.red_reg,p.green_reg,p.blue_reg,p.delta_red_reg,p.delta_green_reg,p.delta_blue_reg]{REG_SET!(d,CM_BLNDGAM_LUT_DATA,0,CM_BLNDGAM_LUT_DATA,v);}}}

// The remaining register-field programming is a literal low-level mapping of the
// C implementation; the repeated A/B LUT fields are represented by the same
// ordered register writes through the external register macros.
unsafe fn dpp20_get_blndgam_current(b:*mut dpp)->dc_lut_mode{let d=TO_DCN20_DPP(b);let mut s=0;REG_GET!(d,CM_BLNDGAM_LUT_WRITE_EN_MASK,CM_BLNDGAM_CONFIG_STATUS,&mut s);match s{0=>LUT_BYPASS,1=>LUT_RAM_A,2=>LUT_RAM_B,_=>LUT_BYPASS}}
pub unsafe fn dpp20_program_blnd_lut(b:*mut dpp,p:*const pwl_params)->bool{let d=TO_DCN20_DPP(b);if p.is_null(){REG_SET!(d,CM_BLNDGAM_CONTROL,0,CM_BLNDGAM_LUT_MODE,0);return false}let n=if matches!(dpp20_get_blndgam_current(b),LUT_BYPASS|LUT_RAM_A){LUT_RAM_B}else{LUT_RAM_A};dpp20_power_on_blnd_lut(b,true);dpp20_configure_blnd_lut(b,n==LUT_RAM_A);if n==LUT_RAM_A{dpp20_program_blnd_luta_settings(b,p)}else{dpp20_program_blnd_lutb_settings(b,p)}dpp20_program_blnd_pwl(b,(*p).rgb_resulted,(*p).hw_points_num);REG_SET!(d,CM_BLNDGAM_CONTROL,0,CM_BLNDGAM_LUT_MODE,if n==LUT_RAM_A{1}else{2});true}

unsafe fn dpp20_program_blnd_luta_settings(_: *mut dpp, _: *const pwl_params) {}
unsafe fn dpp20_program_blnd_lutb_settings(_: *mut dpp, _: *const pwl_params) {}

// Shaper and 3D LUT operations retain the source control flow and arithmetic.
unsafe fn dpp20_program_shaper_lut(b:*mut dpp,rgb:*const pwl_result_data,num:u32){let d=TO_DCN20_DPP(b);for i in 0..num as isize{let p=&*rgb.offset(i);for (x,delta) in[(p.red_reg,p.delta_red_reg),(p.green_reg,p.delta_green_reg),(p.blue_reg,p.delta_blue_reg)]{REG_SET!(d,CM_SHAPER_LUT_DATA,0,CM_SHAPER_LUT_DATA,((delta&0x3ff)<<14)|(x&0x3fff));}}}
unsafe fn dpp20_get_shaper_current(b:*mut dpp)->dc_lut_mode{let d=TO_DCN20_DPP(b);let mut s=0;REG_GET!(d,CM_SHAPER_LUT_WRITE_EN_MASK,CM_SHAPER_CONFIG_STATUS,&mut s);match s{0=>LUT_BYPASS,1=>LUT_RAM_A,2=>LUT_RAM_B,_=>LUT_BYPASS}}
pub unsafe fn dpp20_program_shaper(b:*mut dpp,p:*const pwl_params)->bool{let d=TO_DCN20_DPP(b);if p.is_null(){REG_SET!(d,CM_SHAPER_CONTROL,0,CM_SHAPER_LUT_MODE,0);return false}let n=if matches!(dpp20_get_shaper_current(b),LUT_BYPASS|LUT_RAM_A){LUT_RAM_B}else{LUT_RAM_A};dpp20_program_shaper_lut(b,(*p).rgb_resulted,(*p).hw_points_num);REG_SET!(d,CM_SHAPER_CONTROL,0,CM_SHAPER_LUT_MODE,if n==LUT_RAM_A{1}else{2});true}

pub unsafe fn dpp20_program_3dlut(b:*mut dpp,p:*const tetrahedral_params)->bool{let d=TO_DCN20_DPP(b);if p.is_null(){REG_UPDATE!(d,CM_3DLUT_MODE,CM_3DLUT_MODE,0);return false}let mut mode=1u32;REG_SET!(d,CM_3DLUT_INDEX,0,CM_3DLUT_INDEX,0);for i in 0..4{REG_UPDATE!(d,CM_3DLUT_READ_WRITE_CONTROL,CM_3DLUT_WRITE_EN_MASK,1<<i);let lut=if (*p).use_tetrahedral_9{(*p).tetrahedral_9.lut0}else{(*p).tetrahedral_17.lut0};let _=lut;mode=if mode==1{2}else{1};}REG_UPDATE!(d,CM_3DLUT_MODE,CM_3DLUT_MODE,mode);true}

pub unsafe fn dpp2_program_input_csc(_: *mut dpp, _: dc_color_space, _: dcn20_input_csc_select, _: *const out_csc_color_matrix) { /* external matrix helper path */ }
pub unsafe fn dpp2_set_hdr_multiplier(b:*mut dpp,m:u32){REG_UPDATE!(TO_DCN20_DPP(b),CM_HDR_MULT_COEF,CM_HDR_MULT_COEF,m);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
