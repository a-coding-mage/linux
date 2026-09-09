/* Translated from dcn30_cm_common.c. External kernel/display types and helpers
 * are supplied by the surrounding translation unit. */

const MAX_REGIONS_NUMBER: usize = 34;
const MAX_LOW_POINT: i32 = 25;
const NUMBER_REGIONS: usize = 32;
const NUMBER_SW_SEGMENTS: u32 = 16;

pub unsafe fn cm_helper_program_gamcor_xfer_func(
    ctx: *mut dc_context,
    params: *const pwl_params,
    reg: *const dcn3_xfer_func_reg,
) {
    let mut reg_region_cur: u32;
    let mut i: usize = 0;
    REG_SET_2!((*reg).start_cntl_b, 0, exp_region_start, (*params).corner_points[0].blue.custom_float_x, exp_resion_start_segment, 0);
    REG_SET_2!((*reg).start_cntl_g, 0, exp_region_start, (*params).corner_points[0].green.custom_float_x, exp_resion_start_segment, 0);
    REG_SET_2!((*reg).start_cntl_r, 0, exp_region_start, (*params).corner_points[0].red.custom_float_x, exp_resion_start_segment, 0);
    REG_SET!((*reg).start_slope_cntl_b, 0, field_region_linear_slope, (*params).corner_points[0].blue.custom_float_slope);
    REG_SET!((*reg).start_slope_cntl_g, 0, field_region_linear_slope, (*params).corner_points[0].green.custom_float_slope);
    REG_SET!((*reg).start_slope_cntl_r, 0, field_region_linear_slope, (*params).corner_points[0].red.custom_float_slope);
    REG_SET!((*reg).start_end_cntl1_b, 0, field_region_end_base, (*params).corner_points[1].blue.custom_float_y);
    REG_SET!((*reg).start_end_cntl1_g, 0, field_region_end_base, (*params).corner_points[1].green.custom_float_y);
    REG_SET!((*reg).start_end_cntl1_r, 0, field_region_end_base, (*params).corner_points[1].red.custom_float_y);
    REG_SET_2!((*reg).start_end_cntl2_b, 0, field_region_end_slope, (*params).corner_points[1].blue.custom_float_slope, field_region_end, (*params).corner_points[1].blue.custom_float_x);
    REG_SET_2!((*reg).start_end_cntl2_g, 0, field_region_end_slope, (*params).corner_points[1].green.custom_float_slope, field_region_end, (*params).corner_points[1].green.custom_float_x);
    REG_SET_2!((*reg).start_end_cntl2_r, 0, field_region_end_slope, (*params).corner_points[1].red.custom_float_slope, field_region_end, (*params).corner_points[1].red.custom_float_x);
    reg_region_cur = (*reg).region_start;
    while reg_region_cur <= (*reg).region_end {
        let curve0 = &(*params).arr_curve_points[2 * i];
        let curve1 = &(*params).arr_curve_points[2 * i + 1];
        REG_SET_4!(reg_region_cur, 0, exp_region0_lut_offset, curve0.offset, exp_region0_num_segments, curve0.segments_num, exp_region1_lut_offset, curve1.offset, exp_region1_num_segments, curve1.segments_num);
        i += 1;
        reg_region_cur += 1;
    }
}

unsafe fn interp_tf_pts(output_tf_channel: *const fixed31_32, i: i32) -> fixed31_32 {
    let t = (i & 0xf) as u32;
    let in_plus_one = *output_tf_channel.add(((i >> 4) + 1) as usize);
    let input = *output_tf_channel.add((i >> 4) as usize);
    let value = dc_fixpt_shr(dc_fixpt_mul_int(dc_fixpt_sub(in_plus_one, input), t), 4);
    dc_fixpt_add(input, value)
}

pub unsafe fn cm3_helper_translate_curve_to_hw_format(ctx: *mut dc_context, output_tf: *const dc_transfer_func, lut_params: *mut pwl_params, fixpoint: bool) -> bool {
    if output_tf.is_null() || lut_params.is_null() || (*output_tf).type_ == TF_TYPE_BYPASS { return false; }
    let corner_points = (*lut_params).corner_points;
    let rgb_resulted = (*lut_params).rgb_resulted;
    let mut seg_distr = [0u32; MAX_REGIONS_NUMBER];
    let mut hw_points: u32 = 0;
    core::ptr::write_bytes(lut_params, 0, 1);
    if (*output_tf).tf == TRANSFER_FUNCTION_PQ || (*output_tf).tf == TRANSFER_FUNCTION_GAMMA22 || (*output_tf).tf == TRANSFER_FUNCTION_HLG {
        for x in 0..NUMBER_REGIONS { seg_distr[x] = 3; }
    } else {
        for x in 0..12 { seg_distr[x] = 4; }
        seg_distr[12] = 1;
    }
    let (region_start, region_end) = if (*output_tf).tf == TRANSFER_FUNCTION_PQ || (*output_tf).tf == TRANSFER_FUNCTION_GAMMA22 || (*output_tf).tf == TRANSFER_FUNCTION_HLG { (-MAX_LOW_POINT, NUMBER_REGIONS as i32 - MAX_LOW_POINT) } else { (-12, 1) };
    for x in (region_end - region_start) as usize..MAX_REGIONS_NUMBER { seg_distr[x] = u32::MAX; }
    for &x in &seg_distr { if x != u32::MAX { hw_points += 1u32 << x; } }
    hw_points = if fixpoint { hw_points - 1 } else { hw_points };
    let span = (region_end - region_start) as u32;
    let mut j = 0u32;
    for k in 0..span {
        let increment = NUMBER_SW_SEGMENTS / (1u32 << seg_distr[k as usize]);
        let start_index = ((region_start + k as i32 + MAX_LOW_POINT) as u32) * NUMBER_SW_SEGMENTS;
        let end = start_index + NUMBER_SW_SEGMENTS;
        let mut i = start_index;
        while i < end {
            if j == hw_points { break; }
            if i >= TRANSFER_FUNC_POINTS { return false; }
            (*rgb_resulted.add(j as usize)).red = (*output_tf).tf_pts.red[i as usize];
            (*rgb_resulted.add(j as usize)).green = (*output_tf).tf_pts.green[i as usize];
            (*rgb_resulted.add(j as usize)).blue = (*output_tf).tf_pts.blue[i as usize];
            j += 1; i += increment;
        }
    }
    let last = ((region_end + MAX_LOW_POINT) as u32 * NUMBER_SW_SEGMENTS) as usize;
    (*rgb_resulted.add(hw_points as usize)).red = (*output_tf).tf_pts.red[last];
    (*rgb_resulted.add(hw_points as usize)).green = (*output_tf).tf_pts.green[last];
    (*rgb_resulted.add(hw_points as usize)).blue = (*output_tf).tf_pts.blue[last];
    *rgb_resulted.add(hw_points as usize + 1) = *rgb_resulted.add(hw_points as usize);
    (*corner_points).red.x = dc_fixpt_pow(dc_fixpt_from_int(2), dc_fixpt_from_int(region_start));
    (*corner_points).green.x = (*corner_points).red.x; (*corner_points).blue.x = (*corner_points).red.x;
    (*corner_points.add(1)).red.x = dc_fixpt_pow(dc_fixpt_from_int(2), dc_fixpt_from_int(region_end));
    (*corner_points.add(1)).green.x = (*corner_points.add(1)).red.x; (*corner_points.add(1)).blue.x = (*corner_points.add(1)).red.x;
    (*corner_points).red.y = (*rgb_resulted).red; (*corner_points).green.y = (*rgb_resulted).green; (*corner_points).blue.y = (*rgb_resulted).blue;
    (*corner_points).red.slope = dc_fixpt_div((*corner_points).red.y, (*corner_points).red.x); (*corner_points).green.slope = dc_fixpt_div((*corner_points).green.y, (*corner_points).green.x); (*corner_points).blue.slope = dc_fixpt_div((*corner_points).blue.y, (*corner_points).blue.x);
    (*corner_points.add(1)).red.y = (*rgb_resulted.add(hw_points as usize)).red; (*corner_points.add(1)).green.y = (*rgb_resulted.add(hw_points as usize)).green; (*corner_points.add(1)).blue.y = (*rgb_resulted.add(hw_points as usize)).blue;
    (*corner_points.add(1)).red.slope = dc_fixpt_zero; (*corner_points.add(1)).green.slope = dc_fixpt_zero; (*corner_points.add(1)).blue.slope = dc_fixpt_zero;
    (*lut_params).hw_points_num = hw_points + 1;
    let mut k = 0usize;
    for i in 1..MAX_REGIONS_NUMBER { if seg_distr[k] != u32::MAX { (*lut_params).arr_curve_points[k].segments_num = seg_distr[k]; (*lut_params).arr_curve_points[i].offset = (*lut_params).arr_curve_points[k].offset + (1u32 << seg_distr[k]); } k += 1; }
    if seg_distr[k] != u32::MAX { (*lut_params).arr_curve_points[k].segments_num = seg_distr[k]; }
    cm3_helper_convert_to_custom_float(rgb_resulted, (*lut_params).corner_points, hw_points + 1, fixpoint)
}

pub unsafe fn cm3_helper_convert_to_custom_float(rgb_resulted: *mut pwl_result_data, corner_points: *mut curve_points3, hw_points_num: u32, fixpoint: bool) -> bool {
    let mut fmt = custom_float_format { exponenta_bits: 6, mantissa_bits: 12, sign: false };
    macro_rules! cv { ($v:expr, $d:expr) => { if !convert_to_custom_float_format($v, &mut fmt, $d) { BREAK_TO_DEBUGGER!(); return false; } }; }
    cv!((*corner_points).red.x, &mut (*corner_points).red.custom_float_x); cv!((*corner_points).green.x, &mut (*corner_points).green.custom_float_x); cv!((*corner_points).blue.x, &mut (*corner_points).blue.custom_float_x);
    cv!((*corner_points).red.offset, &mut (*corner_points).red.custom_float_offset); cv!((*corner_points).green.offset, &mut (*corner_points).green.custom_float_offset); cv!((*corner_points).blue.offset, &mut (*corner_points).blue.custom_float_offset);
    cv!((*corner_points).red.slope, &mut (*corner_points).red.custom_float_slope); cv!((*corner_points).green.slope, &mut (*corner_points).green.custom_float_slope); cv!((*corner_points).blue.slope, &mut (*corner_points).blue.custom_float_slope);
    if fixpoint { (*corner_points.add(1)).red.custom_float_y = dc_fixpt_clamp_u0d14((*corner_points.add(1)).red.y); (*corner_points.add(1)).green.custom_float_y = dc_fixpt_clamp_u0d14((*corner_points.add(1)).green.y); (*corner_points.add(1)).blue.custom_float_y = dc_fixpt_clamp_u0d14((*corner_points.add(1)).blue.y); } else { cv!((*corner_points.add(1)).red.y, &mut (*corner_points.add(1)).red.custom_float_y); cv!((*corner_points.add(1)).green.y, &mut (*corner_points.add(1)).green.custom_float_y); cv!((*corner_points.add(1)).blue.y, &mut (*corner_points.add(1)).blue.custom_float_y); }
    fmt.mantissa_bits = 10;
    cv!((*corner_points.add(1)).red.x, &mut (*corner_points.add(1)).red.custom_float_x); cv!((*corner_points.add(1)).green.x, &mut (*corner_points.add(1)).green.custom_float_x); cv!((*corner_points.add(1)).blue.x, &mut (*corner_points.add(1)).blue.custom_float_x);
    cv!((*corner_points.add(1)).red.slope, &mut (*corner_points.add(1)).red.custom_float_slope); cv!((*corner_points.add(1)).green.slope, &mut (*corner_points.add(1)).green.custom_float_slope); cv!((*corner_points.add(1)).blue.slope, &mut (*corner_points.add(1)).blue.custom_float_slope);
    if hw_points_num == 0 || rgb_resulted.is_null() || fixpoint { return true; }
    fmt.mantissa_bits = 12;
    for i in 0..hw_points_num as usize { let p = &mut *rgb_resulted.add(i); cv!(p.red, &mut p.red_reg); cv!(p.green, &mut p.green_reg); cv!(p.blue, &mut p.blue_reg); }
    true
}

pub unsafe fn cm3_helper_translate_curve_to_degamma_hw_format(output_tf: *const dc_transfer_func, lut_params: *mut pwl_params) -> bool {
    if output_tf.is_null() || lut_params.is_null() || (*output_tf).type_ == TF_TYPE_BYPASS { return false; }
    let cp = (*lut_params).corner_points; let out = (*lut_params).rgb_resulted;
    let mut seg = [u32::MAX; MAX_REGIONS_NUMBER]; let mut hp = 0u32;
    core::ptr::write_bytes(lut_params, 0, 1);
    let (rs, re) = if (*output_tf).tf == TRANSFER_FUNCTION_PQ || (*output_tf).tf == TRANSFER_FUNCTION_SRGB { seg[0]=0; for k in 1..9 { seg[k]=(k-1) as u32; } (-9,0) } else { for k in 0..12 { seg[k]=4; } (-12,0) };
    for &x in &seg { if x != u32::MAX { hp += 1 << x; } }
    let mut j=0u32;
    for k in 0..(re-rs) as usize { let inc=(NUMBER_SW_SEGMENTS<<4)/(1<<seg[k]); let start=((rs+k as i32+MAX_LOW_POINT) as u32)*NUMBER_SW_SEGMENTS; let mut i=start<<4; while i < (start<<4)+(NUMBER_SW_SEGMENTS<<4) { if j==hp-1 { break; } if (i>>4)+1 >= TRANSFER_FUNC_POINTS { return false; } (*out.add(j as usize)).red=interp_tf_pts((*output_tf).tf_pts.red.as_ptr(),i as i32); (*out.add(j as usize)).green=interp_tf_pts((*output_tf).tf_pts.green.as_ptr(),i as i32); (*out.add(j as usize)).blue=interp_tf_pts((*output_tf).tf_pts.blue.as_ptr(),i as i32); j+=1; i+=inc; } }
    let last=((re+MAX_LOW_POINT) as u32*NUMBER_SW_SEGMENTS) as usize; (*out.add((hp-1) as usize)).red=(*output_tf).tf_pts.red[last]; (*out.add((hp-1) as usize)).green=(*output_tf).tf_pts.green[last]; (*out.add((hp-1) as usize)).blue=(*output_tf).tf_pts.blue[last];
    (*cp).red.x=dc_fixpt_pow(dc_fixpt_from_int(2),dc_fixpt_from_int(rs)); (*cp).green.x=(*cp).red.x; (*cp).blue.x=(*cp).red.x; (*cp.add(1)).red.x=dc_fixpt_pow(dc_fixpt_from_int(2),dc_fixpt_from_int(re)); (*cp.add(1)).green.x=(*cp.add(1)).red.x; (*cp.add(1)).blue.x=(*cp.add(1)).red.x;
    (*cp).red.y=(*out).red; (*cp).green.y=(*out).green; (*cp).blue.y=(*out).blue; (*cp.add(1)).red.y=(*out.add((hp-1) as usize)).red; (*cp.add(1)).green.y=(*out.add((hp-1) as usize)).green; (*cp.add(1)).blue.y=(*out.add((hp-1) as usize)).blue;
    (*cp.add(1)).red.slope=dc_fixpt_zero; (*cp.add(1)).green.slope=dc_fixpt_zero; (*cp.add(1)).blue.slope=dc_fixpt_zero;
    (*lut_params).hw_points_num=hp; let mut k=0usize; for i in 1..MAX_REGIONS_NUMBER { if seg[k]!=u32::MAX { (*lut_params).arr_curve_points[k].segments_num=seg[k]; (*lut_params).arr_curve_points[i].offset=(*lut_params).arr_curve_points[k].offset+(1<<seg[k]); } k+=1; } if seg[k]!=u32::MAX { (*lut_params).arr_curve_points[k].segments_num=seg[k]; }
    let mut p=out; let mut q=out.add(1); for _ in 1..hp { if dc_fixpt_lt((*q).red,(*p).red) { (*q).red=(*p).red; } if dc_fixpt_lt((*q).green,(*p).green) { (*q).green=(*p).green; } if dc_fixpt_lt((*q).blue,(*p).blue) { (*q).blue=(*p).blue; } (*p).delta_red=dc_fixpt_sub((*q).red,(*p).red); (*p).delta_green=dc_fixpt_sub((*q).green,(*p).green); (*p).delta_blue=dc_fixpt_sub((*q).blue,(*p).blue); p=p.add(1); q=q.add(1); }
    cm3_helper_convert_to_custom_float(out,(*lut_params).corner_points,hp,false)
}

pub unsafe fn is_rgb_equal(rgb: *const pwl_result_data, num: u32) -> bool {
    for i in 0..num as usize { let p = &*rgb.add(i); if p.red_reg != p.green_reg || p.blue_reg != p.red_reg || p.blue_reg != p.green_reg { return false; } }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
