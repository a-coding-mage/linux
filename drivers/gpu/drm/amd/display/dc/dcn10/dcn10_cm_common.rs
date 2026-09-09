/* Direct translation of dcn10_cm_common.c. Included types, register helpers,
 * logging, and fixed-point/custom-float routines are supplied by dependencies. */

const MAX_REGIONS_NUMBER: usize = 34;
const MAX_LOW_POINT: i32 = 25;
const NUMBER_REGIONS: usize = 32;
const NUMBER_SW_SEGMENTS: u32 = 16;
const NUM_DEGAMMA_REGIONS: usize = 12;

pub unsafe fn cm_helper_program_color_matrices(ctx: *mut dc_context, regval: *const u16, reg: *const color_matrices_reg) {
    let mut i = 0usize;
    let mut cur = (*reg).csc_c11_c12;
    while cur <= (*reg).csc_c33_c34 {
        REG_SET_2(cur, 0, csc_c11, *regval.add(2*i), csc_c12, *regval.add(2*i+1));
        i += 1; cur += 1;
    }
}

pub unsafe fn cm_helper_read_color_matrices(ctx: *mut dc_context, regval: *mut u16, reg: *const color_matrices_reg) {
    let mut i = 0usize;
    let mut cur = (*reg).csc_c11_c12;
    while cur <= (*reg).csc_c33_c34 {
        let mut a = 0u32; let mut b = 0u32;
        REG_GET_2(cur, csc_c11, &mut a, csc_c12, &mut b);
        *regval.add(2*i) = a as u16; *regval.add(2*i+1) = b as u16;
        i += 1; cur += 1;
    }
}

pub unsafe fn cm_helper_program_xfer_func(ctx: *mut dc_context, params: *const pwl_params, reg: *const xfer_func_reg) {
    let p = &*params;
    REG_SET_2((*reg).start_cntl_b, 0, exp_region_start, p.corner_points[0].blue.custom_float_x, exp_resion_start_segment, 0);
    REG_SET_2((*reg).start_cntl_g, 0, exp_region_start, p.corner_points[0].green.custom_float_x, exp_resion_start_segment, 0);
    REG_SET_2((*reg).start_cntl_r, 0, exp_region_start, p.corner_points[0].red.custom_float_x, exp_resion_start_segment, 0);
    REG_SET((*reg).start_slope_cntl_b, 0, field_region_linear_slope, p.corner_points[0].blue.custom_float_slope);
    REG_SET((*reg).start_slope_cntl_g, 0, field_region_linear_slope, p.corner_points[0].green.custom_float_slope);
    REG_SET((*reg).start_slope_cntl_r, 0, field_region_linear_slope, p.corner_points[0].red.custom_float_slope);
    REG_SET((*reg).start_end_cntl1_b, 0, field_region_end, p.corner_points[1].blue.custom_float_x);
    REG_SET_2((*reg).start_end_cntl2_b, 0, field_region_end_slope, p.corner_points[1].blue.custom_float_slope, field_region_end_base, p.corner_points[1].blue.custom_float_y);
    REG_SET((*reg).start_end_cntl1_g, 0, field_region_end, p.corner_points[1].green.custom_float_x);
    REG_SET_2((*reg).start_end_cntl2_g, 0, field_region_end_slope, p.corner_points[1].green.custom_float_slope, field_region_end_base, p.corner_points[1].green.custom_float_y);
    REG_SET((*reg).start_end_cntl1_r, 0, field_region_end, p.corner_points[1].red.custom_float_x);
    REG_SET_2((*reg).start_end_cntl2_r, 0, field_region_end_slope, p.corner_points[1].red.custom_float_slope, field_region_end_base, p.corner_points[1].red.custom_float_y);
    let mut i = 0usize; let mut r = (*reg).region_start;
    while r <= (*reg).region_end {
        let a = &p.arr_curve_points[2*i]; let b = &p.arr_curve_points[2*i+1];
        REG_SET_4(r, 0, exp_region0_lut_offset, a.offset, exp_region0_num_segments, a.segments_num, exp_region1_lut_offset, b.offset, exp_region1_num_segments, b.segments_num);
        i += 1; r += 1;
    }
}

pub unsafe fn cm_helper_convert_to_custom_float(rgb_resulted: *mut pwl_result_data, corner_points: *mut curve_points3, hw_points_num: u32, fixpoint: bool) -> bool {
    let mut fmt = custom_float_format { exponenta_bits: 6, mantissa_bits: 12, sign: false };
    macro_rules! cv { ($v:expr, $d:expr) => { if !convert_to_custom_float_format($v, &mut fmt, $d) { BREAK_TO_DEBUGGER!(); return false; } }; }
    cv!((*corner_points).red.x, &mut (*corner_points).red.custom_float_x); cv!((*corner_points).green.x, &mut (*corner_points).green.custom_float_x); cv!((*corner_points).blue.x, &mut (*corner_points).blue.custom_float_x);
    cv!((*corner_points).red.offset, &mut (*corner_points).red.custom_float_offset); cv!((*corner_points).green.offset, &mut (*corner_points).green.custom_float_offset); cv!((*corner_points).blue.offset, &mut (*corner_points).blue.custom_float_offset);
    cv!((*corner_points).red.slope, &mut (*corner_points).red.custom_float_slope); cv!((*corner_points).green.slope, &mut (*corner_points).green.custom_float_slope); cv!((*corner_points).blue.slope, &mut (*corner_points).blue.custom_float_slope);
    fmt.mantissa_bits = 10;
    cv!((*corner_points.add(1)).red.x, &mut (*corner_points.add(1)).red.custom_float_x); cv!((*corner_points.add(1)).green.x, &mut (*corner_points.add(1)).green.custom_float_x); cv!((*corner_points.add(1)).blue.x, &mut (*corner_points.add(1)).blue.custom_float_x);
    if fixpoint { (*corner_points.add(1)).red.custom_float_y=dc_fixpt_clamp_u0d14((*corner_points.add(1)).red.y); (*corner_points.add(1)).green.custom_float_y=dc_fixpt_clamp_u0d14((*corner_points.add(1)).green.y); (*corner_points.add(1)).blue.custom_float_y=dc_fixpt_clamp_u0d14((*corner_points.add(1)).blue.y); }
    else { cv!((*corner_points.add(1)).red.y, &mut (*corner_points.add(1)).red.custom_float_y); cv!((*corner_points.add(1)).green.y, &mut (*corner_points.add(1)).green.custom_float_y); cv!((*corner_points.add(1)).blue.y, &mut (*corner_points.add(1)).blue.custom_float_y); }
    cv!((*corner_points.add(1)).red.slope, &mut (*corner_points.add(1)).red.custom_float_slope); cv!((*corner_points.add(1)).green.slope, &mut (*corner_points.add(1)).green.custom_float_slope); cv!((*corner_points.add(1)).blue.slope, &mut (*corner_points.add(1)).blue.custom_float_slope);
    if hw_points_num == 0 || rgb_resulted.is_null() || fixpoint { return true; }
    fmt.mantissa_bits=12; fmt.sign=true; let mut rgb=rgb_resulted; let mut i=0;
    while i != hw_points_num { cv!((*rgb).red, &mut (*rgb).red_reg); cv!((*rgb).green, &mut (*rgb).green_reg); cv!((*rgb).blue, &mut (*rgb).blue_reg); cv!((*rgb).delta_red, &mut (*rgb).delta_red_reg); cv!((*rgb).delta_green, &mut (*rgb).delta_green_reg); cv!((*rgb).delta_blue, &mut (*rgb).delta_blue_reg); rgb=rgb.add(1); i+=1; }
    true
}

// The two curve translators below retain the original pointer-oriented algorithm.
pub unsafe fn cm_helper_translate_curve_to_hw_format(ctx: *mut dc_context, output_tf: *const dc_transfer_func, lut_params: *mut pwl_params, fixpoint: bool) -> bool {
    if output_tf.is_null() || lut_params.is_null() || (*output_tf).type_ == TF_TYPE_BYPASS { return false; }
    let corners=(*lut_params).corner_points; let rgb=(*lut_params).rgb_resulted; let mut seg=[0u32;MAX_REGIONS_NUMBER];
    let (rs,re) = if (*output_tf).tf==TRANSFER_FUNCTION_PQ || (*output_tf).tf==TRANSFER_FUNCTION_GAMMA22 { for x in &mut seg[..NUMBER_REGIONS] {*x=3;} (-25,7) } else { for x in &mut seg[..12]{*x=4;} seg[12]=1; (-12,1) };
    for x in &mut seg[(re-rs) as usize..] {*x=u32::MAX;}
    let mut hw=0u32; for x in seg { if x!=u32::MAX {hw += 1<<x;} } let mut j=0usize;
    for k in 0..(re-rs) { let inc=NUMBER_SW_SEGMENTS/(1<<seg[k as usize]); let start=((rs+k+25) as u32)*NUMBER_SW_SEGMENTS; let mut i=start; while i<start+NUMBER_SW_SEGMENTS { if j as u32==hw-1 {break;} if i>=TRANSFER_FUNC_POINTS as u32{return false;} (*rgb.add(j)).red=(*output_tf).tf_pts.red[i as usize]; (*rgb.add(j)).green=(*output_tf).tf_pts.green[i as usize]; (*rgb.add(j)).blue=(*output_tf).tf_pts.blue[i as usize]; j+=1;i+=inc;} }
    let start=((re+25) as u32)*NUMBER_SW_SEGMENTS; (*rgb.add((hw-1) as usize)).red=(*output_tf).tf_pts.red[start as usize]; (*rgb.add((hw-1) as usize)).green=(*output_tf).tf_pts.green[start as usize]; (*rgb.add((hw-1) as usize)).blue=(*output_tf).tf_pts.blue[start as usize]; *rgb.add(hw as usize)=*rgb.add((hw-1) as usize);
    (*corners).red.x=dc_fixpt_pow(dc_fixpt_from_int(2),dc_fixpt_from_int(rs)); (*corners).green.x=(*corners).red.x; (*corners).blue.x=(*corners).red.x; (*corners.add(1)).red.x=dc_fixpt_pow(dc_fixpt_from_int(2),dc_fixpt_from_int(re)); (*corners.add(1)).green.x=(*corners.add(1)).red.x; (*corners.add(1)).blue.x=(*corners.add(1)).red.x;
    (*corners).red.y=(*rgb).red; (*corners).green.y=(*rgb).green; (*corners).blue.y=(*rgb).blue; (*corners).red.slope=dc_fixpt_div((*corners).red.y,(*corners).red.x); (*corners).green.slope=dc_fixpt_div((*corners).green.y,(*corners).green.x); (*corners).blue.slope=dc_fixpt_div((*corners).blue.y,(*corners).blue.x);
    (*corners.add(1)).red.y=(*rgb.add((hw-1) as usize)).red; (*corners.add(1)).green.y=(*rgb.add((hw-1) as usize)).green; (*corners.add(1)).blue.y=(*rgb.add((hw-1) as usize)).blue; (*corners.add(1)).red.slope=dc_fixpt_zero; (*corners.add(1)).green.slope=dc_fixpt_zero; (*corners.add(1)).blue.slope=dc_fixpt_zero;
    (*lut_params).hw_points_num=hw; cm_helper_convert_to_custom_float(rgb,corners,hw,fixpoint); true
}

pub unsafe fn cm_helper_translate_curve_to_degamma_hw_format(output_tf: *const dc_transfer_func, lut_params: *mut pwl_params) -> bool {
    cm_helper_translate_curve_to_hw_format(core::ptr::null_mut(), output_tf, lut_params, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
