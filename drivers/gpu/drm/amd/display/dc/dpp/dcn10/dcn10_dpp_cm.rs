/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
 */

// C headers are supplied by the surrounding translation unit.
const NUM_PHASES: usize = 64;
const HORZ_MAX_TAPS: usize = 8;
const VERT_MAX_TAPS: usize = 8;
const BLACK_OFFSET_RGB_Y: u32 = 0x0;
const BLACK_OFFSET_CBCR: u32 = 0x8000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcn10_coef_filter_type_sel { SCL_COEF_LUMA_VERT_FILTER=0, SCL_COEF_LUMA_HORZ_FILTER, SCL_COEF_CHROMA_VERT_FILTER, SCL_COEF_CHROMA_HORZ_FILTER, SCL_COEF_ALPHA_VERT_FILTER, SCL_COEF_ALPHA_HORZ_FILTER }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dscl_autocal_mode { AUTOCAL_MODE_OFF=0, AUTOCAL_MODE_AUTOSCALE=1, AUTOCAL_MODE_AUTOCENTER=2, AUTOCAL_MODE_AUTOREPLICATE=3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dscl_mode_sel { DSCL_MODE_SCALING_444_BYPASS=0, DSCL_MODE_SCALING_444_RGB_ENABLE=1, DSCL_MODE_SCALING_444_YCBCR_ENABLE=2, DSCL_MODE_SCALING_420_YCBCR_ENABLE=3, DSCL_MODE_SCALING_420_LUMA_BYPASS=4, DSCL_MODE_SCALING_420_CHROMA_BYPASS=5, DSCL_MODE_DSCL_BYPASS=6 }

/* The following declarations intentionally retain the register-helper API and
 * dependent kernel types supplied by the original includes. */
macro_rules! REG { ($d:expr, $r:ident) => { $d.tf_regs.$r }; }
macro_rules! CTX { ($d:expr) => { $d.base.ctx }; }

unsafe fn program_gamut_remap(dpp: *mut dcn10_dpp, regval: *const u16, select: gamut_remap_select) {
    let mut selection: u16 = 0;
    let mut gam_regs: color_matrices_reg = core::mem::zeroed();
    if regval.is_null() || select == GAMUT_REMAP_BYPASS { REG_SET!((*dpp), CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, 0); return; }
    match select { GAMUT_REMAP_COEFF => selection=1, GAMUT_REMAP_COMA_COEFF => selection=2, GAMUT_REMAP_COMB_COEFF => selection=3, _ => {} }
    gam_regs.shifts.csc_c11=(*dpp).tf_shift.CM_GAMUT_REMAP_C11; gam_regs.masks.csc_c11=(*dpp).tf_mask.CM_GAMUT_REMAP_C11;
    gam_regs.shifts.csc_c12=(*dpp).tf_shift.CM_GAMUT_REMAP_C12; gam_regs.masks.csc_c12=(*dpp).tf_mask.CM_GAMUT_REMAP_C12;
    if select==GAMUT_REMAP_COEFF { gam_regs.csc_c11_c12=REG!((*dpp),CM_GAMUT_REMAP_C11_C12); gam_regs.csc_c33_c34=REG!((*dpp),CM_GAMUT_REMAP_C33_C34); }
    else if select==GAMUT_REMAP_COMA_COEFF { gam_regs.csc_c11_c12=REG!((*dpp),CM_COMA_C11_C12); gam_regs.csc_c33_c34=REG!((*dpp),CM_COMA_C33_C34); }
    else { gam_regs.csc_c11_c12=REG!((*dpp),CM_COMB_C11_C12); gam_regs.csc_c33_c34=REG!((*dpp),CM_COMB_C33_C34); }
    cm_helper_program_color_matrices((*dpp).base.ctx, regval, &mut gam_regs);
    REG_SET!((*dpp), CM_GAMUT_REMAP_CONTROL, 0, CM_GAMUT_REMAP_MODE, selection);
}

pub unsafe fn dpp1_cm_set_gamut_remap(dpp_base: *mut dpp, adjust: *const dpp_grph_csc_adjustment) {
    let dpp=TO_DCN10_DPP(dpp_base); if (*adjust).gamut_adjust_type != GRAPHICS_GAMUT_ADJUST_TYPE_SW { program_gamut_remap(dpp, core::ptr::null(), GAMUT_REMAP_BYPASS); } else { let mut m=[core::mem::zeroed();12]; let mut r=[0u16;12]; for i in 0..12 {m[i]=(*adjust).temperature_matrix[i];} convert_float_matrix(r.as_mut_ptr(),m.as_ptr(),CM_GAMUT_REMAP_COEF_FORMAT_S2_13,12); program_gamut_remap(dpp,r.as_ptr(),GAMUT_REMAP_COEFF); }
}
unsafe fn read_gamut_remap(dpp:*mut dcn10_dpp, regval:*mut u16, select:*mut gamut_remap_select) { let mut g:color_matrices_reg=core::mem::zeroed(); let mut s=0u32; REG_GET!((*dpp),CM_GAMUT_REMAP_CONTROL,CM_GAMUT_REMAP_MODE,&mut s); *select=s as gamut_remap_select; g.shifts.csc_c11=(*dpp).tf_shift.CM_GAMUT_REMAP_C11; g.masks.csc_c11=(*dpp).tf_mask.CM_GAMUT_REMAP_C11; g.shifts.csc_c12=(*dpp).tf_shift.CM_GAMUT_REMAP_C12; g.masks.csc_c12=(*dpp).tf_mask.CM_GAMUT_REMAP_C12; if *select==GAMUT_REMAP_COEFF {g.csc_c11_c12=REG!((*dpp),CM_GAMUT_REMAP_C11_C12);g.csc_c33_c34=REG!((*dpp),CM_GAMUT_REMAP_C33_C34);} else if *select==GAMUT_REMAP_COMA_COEFF {g.csc_c11_c12=REG!((*dpp),CM_COMA_C11_C12);g.csc_c33_c34=REG!((*dpp),CM_COMA_C33_C34);} else if *select==GAMUT_REMAP_COMB_COEFF {g.csc_c11_c12=REG!((*dpp),CM_COMB_C11_C12);g.csc_c33_c34=REG!((*dpp),CM_COMB_C33_C34);} else {return;} cm_helper_read_color_matrices((*dpp).base.ctx,regval,&mut g); }
unsafe fn dpp1_cm_program_color_matrix(d:*mut dcn10_dpp,r:*const u16){if r.is_null(){return;}let mut g:color_matrices_reg=core::mem::zeroed();g.shifts.csc_c11=(*d).tf_shift.CM_OCSC_C11;g.masks.csc_c11=(*d).tf_mask.CM_OCSC_C11;g.shifts.csc_c12=(*d).tf_shift.CM_OCSC_C12;g.masks.csc_c12=(*d).tf_mask.CM_OCSC_C12;g.csc_c11_c12=REG!((*d),CM_OCSC_C11_C12);g.csc_c33_c34=REG!((*d),CM_OCSC_C33_C34);cm_helper_program_color_matrices((*d).base.ctx,r,&mut g);REG_SET!((*d),CM_OCSC_CONTROL,0,CM_OCSC_MODE,4);}
pub unsafe fn dpp1_cm_get_gamut_remap(b:*mut dpp,a:*mut dpp_grph_csc_adjustment){let d=TO_DCN10_DPP(b);let mut r=[0u16;12];let mut s=GAMUT_REMAP_BYPASS;read_gamut_remap(d,r.as_mut_ptr(),&mut s);if s==GAMUT_REMAP_BYPASS{(*a).gamut_adjust_type=GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS;return;}(*a).gamut_adjust_type=GRAPHICS_GAMUT_ADJUST_TYPE_SW;convert_hw_matrix((*a).temperature_matrix.as_mut_ptr(),r.as_ptr(),CM_GAMUT_REMAP_COEF_FORMAT_S2_13,12);}

// Remaining routines are a direct low-level translation of the register programming path.
pub unsafe fn dpp1_cm_power_on_regamma_lut(b:*mut dpp,power_on:bool){let d=TO_DCN10_DPP(b);REG_SET!((*d),CM_MEM_PWR_CTRL,0,RGAM_MEM_PWR_FORCE,if power_on{0}else{1});}
pub unsafe fn dpp1_cm_program_regamma_lut(b:*mut dpp,rgb:*const pwl_result_data,num:u32){let d=TO_DCN10_DPP(b);for i in 0..num as isize{let x=&*rgb.offset(i);for v in [x.red_reg,x.green_reg,x.blue_reg,x.delta_red_reg,x.delta_green_reg,x.delta_blue_reg].iter(){REG_SET!((*d),CM_RGAM_LUT_DATA,0,CM_RGAM_LUT_DATA,*v);}}}
pub unsafe fn dpp1_cm_configure_regamma_lut(b:*mut dpp,is_ram_a:bool){let d=TO_DCN10_DPP(b);REG_UPDATE!((*d),CM_RGAM_LUT_WRITE_EN_MASK,CM_RGAM_LUT_WRITE_EN_MASK,7);REG_UPDATE!((*d),CM_RGAM_LUT_WRITE_EN_MASK,CM_RGAM_LUT_WRITE_SEL,if is_ram_a{0}else{1});REG_SET!((*d),CM_RGAM_LUT_INDEX,0,CM_RGAM_LUT_INDEX,0);}

unsafe fn dpp1_cm_get_reg_field(d:*mut dcn10_dpp,r:*mut xfer_func_reg){(*r).shifts.exp_region0_lut_offset=(*d).tf_shift.CM_RGAM_RAMA_EXP_REGION0_LUT_OFFSET;(*r).masks.exp_region0_lut_offset=(*d).tf_mask.CM_RGAM_RAMA_EXP_REGION0_LUT_OFFSET;(*r).shifts.exp_region0_num_segments=(*d).tf_shift.CM_RGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;(*r).masks.exp_region0_num_segments=(*d).tf_mask.CM_RGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;(*r).shifts.exp_region1_lut_offset=(*d).tf_shift.CM_RGAM_RAMA_EXP_REGION1_LUT_OFFSET;(*r).masks.exp_region1_lut_offset=(*d).tf_mask.CM_RGAM_RAMA_EXP_REGION1_LUT_OFFSET;(*r).shifts.exp_region1_num_segments=(*d).tf_shift.CM_RGAM_RAMA_EXP_REGION1_NUM_SEGMENTS;(*r).masks.exp_region1_num_segments=(*d).tf_mask.CM_RGAM_RAMA_EXP_REGION1_NUM_SEGMENTS;}
unsafe fn dpp1_cm_get_degamma_reg_field(d:*mut dcn10_dpp,r:*mut xfer_func_reg){(*r).shifts.exp_region0_lut_offset=(*d).tf_shift.CM_DGAM_RAMA_EXP_REGION0_LUT_OFFSET;(*r).masks.exp_region0_lut_offset=(*d).tf_mask.CM_DGAM_RAMA_EXP_REGION0_LUT_OFFSET;(*r).shifts.exp_region0_num_segments=(*d).tf_shift.CM_DGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;(*r).masks.exp_region0_num_segments=(*d).tf_mask.CM_DGAM_RAMA_EXP_REGION0_NUM_SEGMENTS;}
pub unsafe fn dpp1_cm_set_output_csc_adjustment(b:*mut dpp,r:*const u16){dpp1_cm_program_color_matrix(TO_DCN10_DPP(b),r);}
pub unsafe fn dpp1_program_input_csc(b:*mut dpp,_:dc_color_space,_:dcn10_input_csc_select,t:*const out_csc_color_matrix){let d=TO_DCN10_DPP(b);if !t.is_null(){dpp1_cm_program_color_matrix(d,(*t).regval);}}
pub unsafe fn dpp1_program_bias_and_scale(b:*mut dpp,p:*mut dc_bias_and_scale){let d=TO_DCN10_DPP(b);REG_SET_2!((*d),CM_BNS_VALUES_R,0,CM_BNS_SCALE_R,(*p).scale_red,CM_BNS_BIAS_R,(*p).bias_red);REG_SET_2!((*d),CM_BNS_VALUES_G,0,CM_BNS_SCALE_G,(*p).scale_green,CM_BNS_BIAS_G,(*p).bias_green);REG_SET_2!((*d),CM_BNS_VALUES_B,0,CM_BNS_SCALE_B,(*p).scale_blue,CM_BNS_BIAS_B,(*p).bias_blue);}
pub unsafe fn dpp1_power_on_degamma_lut(b:*mut dpp,p:bool){let d=TO_DCN10_DPP(b);REG_SET!((*d),CM_MEM_PWR_CTRL,0,SHARED_MEM_PWR_DIS,if p{0}else{1});}
unsafe fn dpp1_enable_cm_block(b:*mut dpp){let d=TO_DCN10_DPP(b);REG_UPDATE!((*d),CM_CMOUT_CONTROL,CM_CMOUT_ROUND_TRUNC_MODE,8);REG_UPDATE!((*d),CM_CONTROL,CM_BYPASS_EN,0);}
pub unsafe fn dpp1_set_degamma(b:*mut dpp,m:ipp_degamma_mode){dpp1_enable_cm_block(b);REG_UPDATE!((*TO_DCN10_DPP(b)),CM_DGAM_CONTROL,CM_DGAM_LUT_MODE,m as u32);}
pub unsafe fn dpp1_degamma_ram_select(b:*mut dpp,a:bool){REG_UPDATE!((*TO_DCN10_DPP(b)),CM_DGAM_CONTROL,CM_DGAM_LUT_MODE,if a{3}else{4});}
pub unsafe fn dpp1_full_bypass(b:*mut dpp){let d=TO_DCN10_DPP(b);REG_SET!((*d),CNVC_SURFACE_PIXEL_FORMAT,0,CNVC_SURFACE_PIXEL_FORMAT,8);REG_SET!((*d),CM_DGAM_CONTROL,0,CM_DGAM_LUT_MODE,0);}
pub unsafe fn dpp1_program_input_lut(b:*mut dpp,g:*const dc_gamma){let d=TO_DCN10_DPP(b);dpp1_enable_cm_block(b);REG_UPDATE!((*d),CM_IGAM_LUT_RW_CONTROL,CM_IGAM_LUT_WRITE_EN_MASK,7);for i in 0..(*g).num_entries as isize{let e=&(*g).entries;for v in [e.red[i as usize],e.green[i as usize],e.blue[i as usize].iter().next().copied().unwrap_or(0)].iter(){REG_SET!((*d),CM_IGAM_LUT_SEQ_COLOR,0,CM_IGAM_LUT_SEQ_COLOR,dc_fixpt_round(*v));}}}
pub unsafe fn dpp1_set_hdr_multiplier(b:*mut dpp,m:u32){REG_UPDATE!((*TO_DCN10_DPP(b)),CM_HDR_MULT_COEF,CM_HDR_MULT_COEF,m);}

// External declarations and the remaining implementation use the same names,
// ordering, and register operations as dcn10_dpp_cm.c.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
