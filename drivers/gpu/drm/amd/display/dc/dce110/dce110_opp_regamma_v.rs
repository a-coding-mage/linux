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
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, dce/dce_11_0_d.h, dce/dce_11_0_sh_mask.h,
// dce110_transform_v.h

unsafe fn power_on_lut(xfm: *mut transform, power_on: bool, inputgamma: bool, regamma: bool) {
    let mut value = dm_read_reg((*xfm).ctx, mmDCFEV_MEM_PWR_CTRL);
    let mut i = 0;

    if power_on {
        if inputgamma {
            set_reg_field_value(value, 1, DCFEV_MEM_PWR_CTRL, COL_MAN_INPUT_GAMMA_MEM_PWR_DIS);
        }
        if regamma {
            set_reg_field_value(value, 1, DCFEV_MEM_PWR_CTRL, COL_MAN_GAMMA_CORR_MEM_PWR_DIS);
        }
    } else {
        if inputgamma {
            set_reg_field_value(value, 0, DCFEV_MEM_PWR_CTRL, COL_MAN_INPUT_GAMMA_MEM_PWR_DIS);
        }
        if regamma {
            set_reg_field_value(value, 0, DCFEV_MEM_PWR_CTRL, COL_MAN_GAMMA_CORR_MEM_PWR_DIS);
        }
    }

    dm_write_reg((*xfm).ctx, mmDCFEV_MEM_PWR_CTRL, value);

    while i < 3 {
        value = dm_read_reg((*xfm).ctx, mmDCFEV_MEM_PWR_CTRL);
        if get_reg_field_value(value, DCFEV_MEM_PWR_CTRL, COL_MAN_INPUT_GAMMA_MEM_PWR_DIS) != 0
            && get_reg_field_value(value, DCFEV_MEM_PWR_CTRL, COL_MAN_GAMMA_CORR_MEM_PWR_DIS) != 0
        {
            break;
        }
        udelay(2);
        i += 1;
    }
}

unsafe fn set_bypass_input_gamma(xfm_dce: *mut dce_transform) {
    let mut value = dm_read_reg((*xfm_dce).base.ctx, mmCOL_MAN_INPUT_GAMMA_CONTROL1);
    set_reg_field_value(value, 0, COL_MAN_INPUT_GAMMA_CONTROL1, INPUT_GAMMA_MODE);
    dm_write_reg((*xfm_dce).base.ctx, mmCOL_MAN_INPUT_GAMMA_CONTROL1, value);
}

unsafe fn configure_regamma_mode(xfm_dce: *mut dce_transform, mode: u32) {
    let mut value = 0;
    set_reg_field_value(value, mode, GAMMA_CORR_CONTROL, GAMMA_CORR_MODE);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_CONTROL, 0);
}

unsafe fn regamma_config_regions_and_segments(
    xfm_dce: *mut dce_transform,
    params: *const pwl_params,
) {
    let curve;
    let mut value = 0;

    set_reg_field_value(value, (*params).arr_points[0].custom_float_x, GAMMA_CORR_CNTLA_START_CNTL, GAMMA_CORR_CNTLA_EXP_REGION_START);
    set_reg_field_value(value, 0, GAMMA_CORR_CNTLA_START_CNTL, GAMMA_CORR_CNTLA_EXP_REGION_START_SEGMENT);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_CNTLA_START_CNTL, value);

    value = 0;
    set_reg_field_value(value, (*params).arr_points[0].custom_float_slope, GAMMA_CORR_CNTLA_SLOPE_CNTL, GAMMA_CORR_CNTLA_EXP_REGION_LINEAR_SLOPE);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_CNTLA_SLOPE_CNTL, value);

    value = 0;
    set_reg_field_value(value, (*params).arr_points[1].custom_float_x, GAMMA_CORR_CNTLA_END_CNTL1, GAMMA_CORR_CNTLA_EXP_REGION_END);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_CNTLA_END_CNTL1, value);

    value = 0;
    set_reg_field_value(value, (*params).arr_points[1].custom_float_slope, GAMMA_CORR_CNTLA_END_CNTL2, GAMMA_CORR_CNTLA_EXP_REGION_END_BASE);
    set_reg_field_value(value, (*params).arr_points[1].custom_float_y, GAMMA_CORR_CNTLA_END_CNTL2, GAMMA_CORR_CNTLA_EXP_REGION_END_SLOPE);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_CNTLA_END_CNTL2, value);

    curve = (*params).arr_curve_points;
    let regs = [
        (mmGAMMA_CORR_CNTLA_REGION_0_1, GAMMA_CORR_CNTLA_REGION_0_1, GAMMA_CORR_CNTLA_EXP_REGION0_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION0_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION1_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION1_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_2_3, GAMMA_CORR_CNTLA_REGION_2_3, GAMMA_CORR_CNTLA_EXP_REGION2_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION2_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION3_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION3_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_4_5, GAMMA_CORR_CNTLA_REGION_4_5, GAMMA_CORR_CNTLA_EXP_REGION4_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION4_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION5_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION5_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_6_7, GAMMA_CORR_CNTLA_REGION_6_7, GAMMA_CORR_CNTLA_EXP_REGION6_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION6_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION7_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION7_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_8_9, GAMMA_CORR_CNTLA_REGION_8_9, GAMMA_CORR_CNTLA_EXP_REGION8_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION8_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION9_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION9_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_10_11, GAMMA_CORR_CNTLA_REGION_10_11, GAMMA_CORR_CNTLA_EXP_REGION10_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION10_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION11_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION11_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_12_13, GAMMA_CORR_CNTLA_REGION_12_13, GAMMA_CORR_CNTLA_EXP_REGION12_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION12_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION13_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION13_NUM_SEGMENTS),
        (mmGAMMA_CORR_CNTLA_REGION_14_15, GAMMA_CORR_CNTLA_REGION_14_15, GAMMA_CORR_CNTLA_EXP_REGION14_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION14_NUM_SEGMENTS, GAMMA_CORR_CNTLA_EXP_REGION15_LUT_OFFSET, GAMMA_CORR_CNTLA_EXP_REGION15_NUM_SEGMENTS),
    ];
    for (n, &(reg, field, off0, seg0, off1, seg1)) in regs.iter().enumerate() {
        value = 0;
        set_reg_field_value(value, (*curve.add(n * 2)).offset, field, off0);
        set_reg_field_value(value, (*curve.add(n * 2)).segments_num, field, seg0);
        set_reg_field_value(value, (*curve.add(n * 2 + 1)).offset, field, off1);
        set_reg_field_value(value, (*curve.add(n * 2 + 1)).segments_num, field, seg1);
        dm_write_reg((*xfm_dce).base.ctx, reg, value);
    }
}

unsafe fn program_pwl(xfm_dce: *mut dce_transform, params: *const pwl_params) {
    let mut value = 0;
    set_reg_field_value(value, 7, GAMMA_CORR_LUT_WRITE_EN_MASK, GAMMA_CORR_LUT_WRITE_EN_MASK);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_LUT_WRITE_EN_MASK, value);
    dm_write_reg((*xfm_dce).base.ctx, mmGAMMA_CORR_LUT_INDEX, 0);

    let addr = mmGAMMA_CORR_LUT_DATA;
    let mut i = 0;
    let mut rgb = (*params).rgb_resulted;
    while i != (*params).hw_points_num {
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).red_reg);
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).green_reg);
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).blue_reg);
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).delta_red_reg);
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).delta_green_reg);
        dm_write_reg((*xfm_dce).base.ctx, addr, (*rgb).delta_blue_reg);
        rgb = rgb.add(1);
        i += 1;
    }
}

pub unsafe fn dce110_opp_program_regamma_pwl_v(xfm: *mut transform, params: *const pwl_params) {
    let xfm_dce = TO_DCE_TRANSFORM(xfm);
    regamma_config_regions_and_segments(xfm_dce, params);
    set_bypass_input_gamma(xfm_dce);
    power_on_lut(xfm, true, false, true);
    program_pwl(xfm_dce, params);
    configure_regamma_mode(xfm_dce, 1);
    power_on_lut(xfm, false, false, true);
}

pub unsafe fn dce110_opp_power_on_regamma_lut_v(xfm: *mut transform, power_on: bool) {
    let mut value = dm_read_reg((*xfm).ctx, mmDCFEV_MEM_PWR_CTRL);
    set_reg_field_value(value, 0, DCFEV_MEM_PWR_CTRL, COL_MAN_GAMMA_CORR_MEM_PWR_FORCE);
    set_reg_field_value(value, power_on, DCFEV_MEM_PWR_CTRL, COL_MAN_GAMMA_CORR_MEM_PWR_DIS);
    set_reg_field_value(value, 0, DCFEV_MEM_PWR_CTRL, COL_MAN_INPUT_GAMMA_MEM_PWR_FORCE);
    set_reg_field_value(value, power_on, DCFEV_MEM_PWR_CTRL, COL_MAN_INPUT_GAMMA_MEM_PWR_DIS);
    dm_write_reg((*xfm).ctx, mmDCFEV_MEM_PWR_CTRL, value);
}

pub unsafe fn dce110_opp_set_regamma_mode_v(xfm: *mut transform, mode: opp_regamma) {
    let _ = xfm;
    let _ = mode;
    // TODO: need to implement the function
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
