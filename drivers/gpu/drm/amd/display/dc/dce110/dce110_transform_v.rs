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

// Dependencies supplied by the surrounding translation unit.
const SCLV_PHASES: i32 = 64;

#[repr(C)]
struct SclvRatiosInits {
    h_int_scale_ratio_luma: u32,
    h_int_scale_ratio_chroma: u32,
    v_int_scale_ratio_luma: u32,
    v_int_scale_ratio_chroma: u32,
    h_init_luma: init_int_and_frac,
    h_init_chroma: init_int_and_frac,
    v_init_luma: init_int_and_frac,
    v_init_chroma: init_int_and_frac,
}

unsafe fn calculate_viewport(scl_data: *const scaler_data, luma: *mut rect, chroma: *mut rect) {
    (*luma).x = (*scl_data).viewport.x - (*scl_data).viewport.x % 2;
    (*luma).y = (*scl_data).viewport.y - (*scl_data).viewport.y % 2;
    (*luma).width = (*scl_data).viewport.width - (*scl_data).viewport.width % 2;
    (*luma).height = (*scl_data).viewport.height - (*scl_data).viewport.height % 2;
    (*chroma).x = (*luma).x; (*chroma).y = (*luma).y;
    (*chroma).height = (*luma).height; (*chroma).width = (*luma).width;
    if (*scl_data).format == PIXEL_FORMAT_420BPP8 {
        (*luma).height += (*luma).height % 2;
        (*luma).width += (*luma).width % 2;
        (*chroma).x = (*luma).x / 2; (*chroma).y = (*luma).y / 2;
        (*chroma).height = (*luma).height / 2; (*chroma).width = (*luma).width / 2;
    }
}

unsafe fn program_viewport(xfm: *mut dce_transform, luma: *mut rect, chroma: *mut rect) {
    let ctx = (*xfm).base.ctx; let mut value: u32; let mut addr: u32;
    if (*luma).width != 0 && (*luma).height != 0 {
        addr = mmSCLV_VIEWPORT_START; value = 0;
        set_reg_field_value(&mut value, (*luma).x, SCLV_VIEWPORT_START, VIEWPORT_X_START);
        set_reg_field_value(&mut value, (*luma).y, SCLV_VIEWPORT_START, VIEWPORT_Y_START);
        dm_write_reg(ctx, addr, value);
        addr = mmSCLV_VIEWPORT_SIZE; value = 0;
        set_reg_field_value(&mut value, (*luma).height, SCLV_VIEWPORT_SIZE, VIEWPORT_HEIGHT);
        set_reg_field_value(&mut value, (*luma).width, SCLV_VIEWPORT_SIZE, VIEWPORT_WIDTH);
        dm_write_reg(ctx, addr, value);
    }
    if (*chroma).width != 0 && (*chroma).height != 0 {
        addr = mmSCLV_VIEWPORT_START_C; value = 0;
        set_reg_field_value(&mut value, (*chroma).x, SCLV_VIEWPORT_START_C, VIEWPORT_X_START_C);
        set_reg_field_value(&mut value, (*chroma).y, SCLV_VIEWPORT_START_C, VIEWPORT_Y_START_C);
        dm_write_reg(ctx, addr, value);
        addr = mmSCLV_VIEWPORT_SIZE_C; value = 0;
        set_reg_field_value(&mut value, (*chroma).height, SCLV_VIEWPORT_SIZE_C, VIEWPORT_HEIGHT_C);
        set_reg_field_value(&mut value, (*chroma).width, SCLV_VIEWPORT_SIZE_C, VIEWPORT_WIDTH_C);
        dm_write_reg(ctx, addr, value);
    }
}

unsafe fn setup_scaling_configuration(xfm: *mut dce_transform, data: *const scaler_data) -> bool {
    let ctx = (*xfm).base.ctx; let mut value = 0; let mut needed = false;
    set_reg_field_value(&mut value, (*data).taps.h_taps - 1, SCLV_TAP_CONTROL, SCL_H_NUM_OF_TAPS);
    set_reg_field_value(&mut value, (*data).taps.v_taps - 1, SCLV_TAP_CONTROL, SCL_V_NUM_OF_TAPS);
    set_reg_field_value(&mut value, (*data).taps.h_taps_c - 1, SCLV_TAP_CONTROL, SCL_H_NUM_OF_TAPS_C);
    set_reg_field_value(&mut value, (*data).taps.v_taps_c - 1, SCLV_TAP_CONTROL, SCL_V_NUM_OF_TAPS_C);
    dm_write_reg(ctx, mmSCLV_TAP_CONTROL, value); value = 0;
    if (*data).taps.h_taps + (*data).taps.v_taps > 2 { set_reg_field_value(&mut value,1,SCLV_MODE,SCL_MODE); set_reg_field_value(&mut value,1,SCLV_MODE,SCL_PSCL_EN); needed=true; }
    else { set_reg_field_value(&mut value,0,SCLV_MODE,SCL_MODE); set_reg_field_value(&mut value,0,SCLV_MODE,SCL_PSCL_EN); }
    if (*data).taps.h_taps_c + (*data).taps.v_taps_c > 2 { set_reg_field_value(&mut value,1,SCLV_MODE,SCL_MODE_C); set_reg_field_value(&mut value,1,SCLV_MODE,SCL_PSCL_EN_C); needed=true; }
    else if (*data).format != PIXEL_FORMAT_420BPP8 { let a=get_reg_field_value(value,SCLV_MODE,SCL_MODE); let b=get_reg_field_value(value,SCLV_MODE,SCL_PSCL_EN); set_reg_field_value(&mut value,a,SCLV_MODE,SCL_MODE_C); set_reg_field_value(&mut value,b,SCLV_MODE,SCL_PSCL_EN_C); }
    else { set_reg_field_value(&mut value,0,SCLV_MODE,SCL_MODE_C); set_reg_field_value(&mut value,0,SCLV_MODE,SCL_PSCL_EN_C); }
    dm_write_reg(ctx, mmSCLV_MODE, value); value=0;
    set_reg_field_value(&mut value,1,SCLV_CONTROL,SCL_BOUNDARY_MODE); dm_write_reg(ctx,mmSCLV_CONTROL,value); needed
}

unsafe fn program_overscan(xfm: *mut dce_transform, data: *const scaler_data) {
    let mut lr=0; let mut tb=0; let mut right=(*data).h_active-(*data).recout.x-(*data).recout.width; let mut bottom=(*data).v_active-(*data).recout.y-(*data).recout.height;
    if (*(*xfm).base.ctx).dc.debug.visual_confirm != VISUAL_CONFIRM_DISABLE { bottom+=2; right+=2; }
    if right<0 { BREAK_TO_DEBUGGER!(); right=0; } if bottom<0 { BREAK_TO_DEBUGGER!(); bottom=0; }
    set_reg_field_value(&mut lr,(*data).recout.x,EXT_OVERSCAN_LEFT_RIGHT,EXT_OVERSCAN_LEFT); set_reg_field_value(&mut lr,right,EXT_OVERSCAN_LEFT_RIGHT,EXT_OVERSCAN_RIGHT);
    set_reg_field_value(&mut tb,(*data).recout.y,EXT_OVERSCAN_TOP_BOTTOM,EXT_OVERSCAN_TOP); set_reg_field_value(&mut tb,bottom,EXT_OVERSCAN_TOP_BOTTOM,EXT_OVERSCAN_BOTTOM);
    dm_write_reg((*xfm).base.ctx,mmSCLV_EXT_OVERSCAN_LEFT_RIGHT,lr); dm_write_reg((*xfm).base.ctx,mmSCLV_EXT_OVERSCAN_TOP_BOTTOM,tb);
}

unsafe fn set_coeff_update_complete(xfm:*mut dce_transform) { let mut v=dm_read_reg((*xfm).base.ctx,mmSCLV_UPDATE); set_reg_field_value(&mut v,1,SCLV_UPDATE,SCL_COEF_UPDATE_COMPLETE); dm_write_reg((*xfm).base.ctx,mmSCLV_UPDATE,v); }

unsafe fn get_filter_coeffs_64p(taps:i32, ratio:fixed31_32)->*const u16 { if taps==4 {get_filter_4tap_64p(ratio)} else if taps==2 {get_filter_2tap_64p()} else if taps==1 {core::ptr::null()} else {BREAK_TO_DEBUGGER!(); core::ptr::null()} }

unsafe fn dce110_xfmv_power_up_line_buffer(xfm:*mut transform)->bool { let d=TO_DCE_TRANSFORM(xfm); let mut v=dm_read_reg((*d).base.ctx,mmLBV_MEMORY_CTRL); set_reg_field_value(&mut v,0,LBV_MEMORY_CTRL,LB_MEMORY_CONFIG); set_reg_field_value(&mut v,(*d).lb_memory_size,LBV_MEMORY_CTRL,LB_MEMORY_SIZE); dm_write_reg((*d).base.ctx,mmLBV_MEMORY_CTRL,v); true }

unsafe fn dce110_xfmv_reset(xfm:*mut transform) { let d=TO_DCE_TRANSFORM(xfm); (*d).filter_h=core::ptr::null(); (*d).filter_v=core::ptr::null(); (*d).filter_h_c=core::ptr::null(); (*d).filter_v_c=core::ptr::null(); }

unsafe fn dce110_xfmv_set_gamut_remap(_xfm:*mut transform,_adjust:*const xfm_grph_csc_adjustment) {}

unsafe fn dce110_xfmv_set_pixel_storage_depth(xfm:*mut transform, depth:lb_pixel_depth, _params:*const bit_depth_reduction_params) { let d=TO_DCE_TRANSFORM(xfm); let (pd,em) = match depth { LB_PIXEL_DEPTH_18BPP=>(2,1), LB_PIXEL_DEPTH_24BPP=>(1,1), LB_PIXEL_DEPTH_30BPP=>(0,1), LB_PIXEL_DEPTH_36BPP=>(3,0), _=>{BREAK_TO_DEBUGGER!();(0,0)} }; let mut v=0; set_reg_field_value(&mut v,em,LBV_DATA_FORMAT,PIXEL_EXPAN_MODE); set_reg_field_value(&mut v,pd,LBV_DATA_FORMAT,PIXEL_DEPTH); dm_write_reg((*xfm).ctx,mmLBV_DATA_FORMAT,v); if (*d).lb_pixel_depth_supported & depth == 0 { DC_LOG_WARNING!("{}: Capability not supported", "dce110_xfmv_set_pixel_storage_depth"); } }

unsafe fn dce110_xfmv_set_scaler(xfm:*mut transform,data:*const scaler_data) { let d=TO_DCE_TRANSFORM(xfm); dce110_xfmv_power_up_line_buffer(xfm); let mut l=rect{ x:0,y:0,width:0,height:0 }; let mut c=rect{ x:0,y:0,width:0,height:0 }; calculate_viewport(data,&mut l,&mut c); program_overscan(d,data); if setup_scaling_configuration(d,data) { let mut i=SclvRatiosInits{h_int_scale_ratio_luma:dc_fixpt_u2d19((*data).ratios.horz)<<5,h_int_scale_ratio_chroma:dc_fixpt_u2d19((*data).ratios.horz_c)<<5,v_int_scale_ratio_luma:dc_fixpt_u2d19((*data).ratios.vert)<<5,v_int_scale_ratio_chroma:dc_fixpt_u2d19((*data).ratios.vert_c)<<5,h_init_luma:init_int_and_frac{integer:1,fraction:0},h_init_chroma:init_int_and_frac{integer:1,fraction:0},v_init_luma:init_int_and_frac{integer:1,fraction:0},v_init_chroma:init_int_and_frac{integer:1,fraction:0}}; program_scl_ratios_inits(d,&mut i); } program_viewport(d,&mut l,&mut c); }

unsafe fn program_scl_ratios_inits(xfm:*mut dce_transform,i:*mut SclvRatiosInits) {
    let ctx=(*xfm).base.ctx; let mut v=0;
    set_reg_field_value(&mut v,(*i).h_int_scale_ratio_luma,SCLV_HORZ_FILTER_SCALE_RATIO,SCL_H_SCALE_RATIO); dm_write_reg(ctx,mmSCLV_HORZ_FILTER_SCALE_RATIO,v);
    v=0; set_reg_field_value(&mut v,(*i).v_int_scale_ratio_luma,SCLV_VERT_FILTER_SCALE_RATIO,SCL_V_SCALE_RATIO); dm_write_reg(ctx,mmSCLV_VERT_FILTER_SCALE_RATIO,v);
    v=0; set_reg_field_value(&mut v,(*i).h_int_scale_ratio_chroma,SCLV_HORZ_FILTER_SCALE_RATIO_C,SCL_H_SCALE_RATIO_C); dm_write_reg(ctx,mmSCLV_HORZ_FILTER_SCALE_RATIO_C,v);
    v=0; set_reg_field_value(&mut v,(*i).v_int_scale_ratio_chroma,SCLV_VERT_FILTER_SCALE_RATIO_C,SCL_V_SCALE_RATIO_C); dm_write_reg(ctx,mmSCLV_VERT_FILTER_SCALE_RATIO_C,v);
    v=0; set_reg_field_value(&mut v,(*i).h_init_luma.fraction,SCLV_HORZ_FILTER_INIT,SCL_H_INIT_FRAC); set_reg_field_value(&mut v,(*i).h_init_luma.integer,SCLV_HORZ_FILTER_INIT,SCL_H_INIT_INT); dm_write_reg(ctx,mmSCLV_HORZ_FILTER_INIT,v);
    v=0; set_reg_field_value(&mut v,(*i).v_init_luma.fraction,SCLV_VERT_FILTER_INIT,SCL_V_INIT_FRAC); set_reg_field_value(&mut v,(*i).v_init_luma.integer,SCLV_VERT_FILTER_INIT,SCL_V_INIT_INT); dm_write_reg(ctx,mmSCLV_VERT_FILTER_INIT,v);
    v=0; set_reg_field_value(&mut v,(*i).h_init_chroma.fraction,SCLV_HORZ_FILTER_INIT_C,SCL_H_INIT_FRAC_C); set_reg_field_value(&mut v,(*i).h_init_chroma.integer,SCLV_HORZ_FILTER_INIT_C,SCL_H_INIT_INT_C); dm_write_reg(ctx,mmSCLV_HORZ_FILTER_INIT_C,v);
    v=0; set_reg_field_value(&mut v,(*i).v_init_chroma.fraction,SCLV_VERT_FILTER_INIT_C,SCL_V_INIT_FRAC_C); set_reg_field_value(&mut v,(*i).v_init_chroma.integer,SCLV_VERT_FILTER_INIT_C,SCL_V_INIT_INT_C); dm_write_reg(ctx,mmSCLV_VERT_FILTER_INIT_C,v);
}

unsafe fn program_multi_taps_filter(xfm:*mut dce_transform,taps:i32,coeffs:*const u16,filter_type:ram_filter_type) {
    if coeffs.is_null(){return} let ctx=(*xfm).base.ctx; let old=dm_read_reg(ctx,mmDCFEV_MEM_PWR_CTRL); let mut p=old; set_reg_field_value(&mut p,1,DCFEV_MEM_PWR_CTRL,SCLV_COEFF_MEM_PWR_DIS); dm_write_reg(ctx,mmDCFEV_MEM_PWR_CTRL,p);
    let pairs=(taps+1)/2; let phases=SCLV_PHASES/2+1; let mut sel=0; set_reg_field_value(&mut sel,filter_type,SCLV_COEF_RAM_SELECT,SCL_C_RAM_FILTER_TYPE);
    for phase in 0..phases { set_reg_field_value(&mut sel,phase,SCLV_COEF_RAM_SELECT,SCL_C_RAM_PHASE); for pair in 0..pairs { set_reg_field_value(&mut sel,pair,SCLV_COEF_RAM_SELECT,SCL_C_RAM_TAP_PAIR_IDX); dm_write_reg(ctx,mmSCLV_COEF_RAM_SELECT,sel); let mut d=0; set_reg_field_value(&mut d,1,SCLV_COEF_RAM_TAP_DATA,SCL_C_RAM_EVEN_TAP_COEF_EN); set_reg_field_value(&mut d,*coeffs.add((phase*pairs*2+pair*2) as usize),SCLV_COEF_RAM_TAP_DATA,SCL_C_RAM_EVEN_TAP_COEF); if taps%2!=0 && pair==pairs-1 { set_reg_field_value(&mut d,0,SCLV_COEF_RAM_TAP_DATA,SCL_C_RAM_ODD_TAP_COEF_EN); } else { set_reg_field_value(&mut d,1,SCLV_COEF_RAM_TAP_DATA,SCL_C_RAM_ODD_TAP_COEF_EN); set_reg_field_value(&mut d,*coeffs.add((phase*pairs*2+pair*2+1) as usize),SCLV_COEF_RAM_TAP_DATA,SCL_C_RAM_ODD_TAP_COEF); } dm_write_reg(ctx,mmSCLV_COEF_RAM_TAP_DATA,d); } } dm_write_reg(ctx,mmDCFEV_MEM_PWR_CTRL,old);
}

#[no_mangle]
pub unsafe extern "C" fn dce110_transform_v_construct(xfm_dce:*mut dce_transform,ctx:*mut dc_context)->bool { (*xfm_dce).base.ctx=ctx; (*xfm_dce).lb_pixel_depth_supported=LB_PIXEL_DEPTH_18BPP|LB_PIXEL_DEPTH_24BPP|LB_PIXEL_DEPTH_30BPP|LB_PIXEL_DEPTH_36BPP; (*xfm_dce).prescaler_on=true; (*xfm_dce).lb_bits_per_entry=LB_BITS_PER_ENTRY; (*xfm_dce).lb_memory_size=LB_TOTAL_NUMBER_OF_ENTRIES; true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
