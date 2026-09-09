// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translated repository.

macro_rules! REG { ($dpp:expr, $reg:ident) => { $dpp.tf_regs.$reg } }
macro_rules! CTX { ($dpp:expr) => { $dpp.base.ctx } }
macro_rules! FN { ($dpp:expr, $reg_name:ident, $field_name:ident) => {
    ($dpp.tf_shift.$field_name, $dpp.tf_mask.$field_name)
} }

unsafe fn dpp60_power_on_dscl(dpp_base: *mut dpp, power_on: bool) {
    let dpp = TO_DCN60_DPP(dpp_base);

    if power_on {
        REG_UPDATE!(dpp, DSCL_MEM_PWR_CTRL, LUT_MEM_PWR_FORCE, 0);
        REG_UPDATE!(dpp, DSCL_MEM_PWR_CTRL, LUT_MEM_PWR_DIS, 1);
        REG_WAIT!(dpp, DSCL_MEM_PWR_STATUS, LUT_MEM_PWR_STATE, 0, 1, 100);
    } else if (*(*(*dpp).base.ctx).dc).debug.enable_mem_low_power.bits.dscl {
        (*(*dpp).base.ctx).dc.optimized_required = true;
        (*dpp).base.deferred_reg_writes.bits.disable_dscl = true;
    }
}

pub unsafe fn dpp60_dscl_set_lb(
    dpp: *mut dcn60_dpp,
    lb_params: *const line_buffer_params,
    mem_size_config: lb_memory_config,
) {
    REG_SET!(dpp, LB_DATA_FORMAT, 0, LB_DATA_FORMAT__ALPHA_EN, (*lb_params).alpha_en);
    REG_SET_2!(dpp, LB_MEMORY_CTRL, 0,
        MEMORY_CONFIG, mem_size_config,
        LB_MAX_PARTITIONS, (*dpp).base.caps.max_lb_partitions);
}

pub unsafe fn dpp60_dscl_set_manual_ratio_init(
    dpp: *mut dcn60_dpp, data: *const scaler_data,
) {
    let mut init_frac: u32 = 0;
    let mut init_int: u32 = 0;
    if (*(*dpp).base.ctx).dc.config.use_spl && !(*(*dpp).base.ctx).dc.debug.disable_spl {
        REG_SET!(dpp, SCL_HORZ_FILTER_SCALE_RATIO, 0, SCL_H_SCALE_RATIO, (*data).dscl_prog_data.ratios.h_scale_ratio);
        REG_SET!(dpp, SCL_VERT_FILTER_SCALE_RATIO, 0, SCL_V_SCALE_RATIO, (*data).dscl_prog_data.ratios.v_scale_ratio);
        REG_SET!(dpp, SCL_HORZ_FILTER_SCALE_RATIO_C, 0, SCL_H_SCALE_RATIO_C, (*data).dscl_prog_data.ratios.h_scale_ratio_c);
        REG_SET!(dpp, SCL_VERT_FILTER_SCALE_RATIO_C, 0, SCL_V_SCALE_RATIO_C, (*data).dscl_prog_data.ratios.v_scale_ratio_c);
        REG_SET_2!(dpp, SCL_HORZ_FILTER_INIT, 0, SCL_H_INIT_FRAC, (*data).dscl_prog_data.init.h_filter_init_frac, SCL_H_INIT_INT, (*data).dscl_prog_data.init.h_filter_init_int);
        REG_SET_2!(dpp, SCL_HORZ_FILTER_INIT_C, 0, SCL_H_INIT_FRAC_C, (*data).dscl_prog_data.init.h_filter_init_frac_c, SCL_H_INIT_INT_C, (*data).dscl_prog_data.init.h_filter_init_int_c);
        REG_SET_2!(dpp, SCL_VERT_FILTER_INIT, 0, SCL_V_INIT_FRAC, (*data).dscl_prog_data.init.v_filter_init_frac, SCL_V_INIT_INT, (*data).dscl_prog_data.init.v_filter_init_int);
        REG_SET_2!(dpp, SCL_VERT_FILTER_INIT_C, 0, SCL_V_INIT_FRAC_C, (*data).dscl_prog_data.init.v_filter_init_frac_c, SCL_V_INIT_INT_C, (*data).dscl_prog_data.init.v_filter_init_int_c);
        return;
    }
    REG_SET!(dpp, SCL_HORZ_FILTER_SCALE_RATIO, 0, SCL_H_SCALE_RATIO, dc_fixpt_u3d19((*data).ratios.horz) << 5);
    REG_SET!(dpp, SCL_VERT_FILTER_SCALE_RATIO, 0, SCL_V_SCALE_RATIO, dc_fixpt_u3d19((*data).ratios.vert) << 5);
    REG_SET!(dpp, SCL_HORZ_FILTER_SCALE_RATIO_C, 0, SCL_H_SCALE_RATIO_C, dc_fixpt_u3d19((*data).ratios.horz_c) << 5);
    REG_SET!(dpp, SCL_VERT_FILTER_SCALE_RATIO_C, 0, SCL_V_SCALE_RATIO_C, dc_fixpt_u3d19((*data).ratios.vert_c) << 5);
    init_frac = dc_fixpt_u0d19((*data).inits.h) << 5; init_int = dc_fixpt_floor((*data).inits.h);
    REG_SET_2!(dpp, SCL_HORZ_FILTER_INIT, 0, SCL_H_INIT_FRAC, init_frac, SCL_H_INIT_INT, init_int);
    init_frac = dc_fixpt_u0d19((*data).inits.h_c) << 5; init_int = dc_fixpt_floor((*data).inits.h_c);
    REG_SET_2!(dpp, SCL_HORZ_FILTER_INIT_C, 0, SCL_H_INIT_FRAC_C, init_frac, SCL_H_INIT_INT_C, init_int);
    init_frac = dc_fixpt_u0d19((*data).inits.v) << 5; init_int = dc_fixpt_floor((*data).inits.v);
    REG_SET_2!(dpp, SCL_VERT_FILTER_INIT, 0, SCL_V_INIT_FRAC, init_frac, SCL_V_INIT_INT, init_int);
    init_frac = dc_fixpt_u0d19((*data).inits.v_c) << 5; init_int = dc_fixpt_floor((*data).inits.v_c);
    REG_SET_2!(dpp, SCL_VERT_FILTER_INIT_C, 0, SCL_V_INIT_FRAC_C, init_frac, SCL_V_INIT_INT_C, init_int);
}

// The remaining scaler programming routine is kept as a direct unsafe translation;
// register helpers and DCN structures are provided by external dependencies.
pub unsafe fn dpp60_dscl_set_scaler_manual_scale(dpp_base: *mut dpp, scl_data: *const scaler_data) {
    let dpp = TO_DCN60_DPP(dpp_base);
    let dpp401 = TO_DCN401_DPP(dpp_base);
    let mut rect = &(*scl_data).recout as *const rect;
    let mut mpc_width = (*scl_data).h_active;
    let mut mpc_height = (*scl_data).v_active;
    let mut v_num_taps = (*scl_data).taps.v_taps - 1;
    let mut v_num_taps_c = (*scl_data).taps.v_taps_c - 1;
    let mut h_num_taps = (*scl_data).taps.h_taps - 1;
    let mut h_num_taps_c = (*scl_data).taps.h_taps_c - 1;
    let mut dscl_mode = dpp401_dscl_get_dscl_mode(dpp_base, scl_data, (*(*dpp_base).ctx).dc.debug.always_scale);
    let ycbcr = (*scl_data).format >= PIXEL_FORMAT_VIDEO_BEGIN && (*scl_data).format <= PIXEL_FORMAT_VIDEO_END;
    let mut program_isharp_1dlut = false;
    let mut bs_coeffs_updated = false;
    if memcmp(&(*dpp).scl_data as *const _, scl_data as *const _, core::mem::size_of::<scaler_data>()) == 0 { return; }
    PERF_TRACE!();
    if (*scl_data).dscl_prog_data.isharp_en && (*dpp).scl_data.dscl_prog_data.sharpness_level != (*scl_data).dscl_prog_data.sharpness_level {
        dpp401_dscl_set_isharp_filter(dpp401, (*scl_data).dscl_prog_data.isharp_delta);
        (*dpp).scl_data.dscl_prog_data.sharpness_level = (*scl_data).dscl_prog_data.sharpness_level;
        memcpy((*dpp).scl_data.dscl_prog_data.isharp_delta, (*scl_data).dscl_prog_data.isharp_delta, 4 * ISHARP_LUT_TABLE_SIZE);
        if memcmp(&(*dpp).scl_data as *const _, scl_data as *const _, core::mem::size_of::<scaler_data>()) == 0 { return; }
        program_isharp_1dlut = true;
    }
    (*dpp).scl_data = *scl_data;
    if (*(*dpp).base.ctx).dc.config.use_spl && !(*(*dpp).base.ctx).dc.debug.disable_spl {
        dscl_mode = (*scl_data).dscl_prog_data.dscl_mode as dcn401_dscl_mode_sel;
        rect = &(*scl_data).dscl_prog_data.recout as *const _ as *const rect;
        mpc_width = (*scl_data).dscl_prog_data.mpc_size.width; mpc_height = (*scl_data).dscl_prog_data.mpc_size.height;
        v_num_taps = (*scl_data).dscl_prog_data.taps.v_taps; v_num_taps_c = (*scl_data).dscl_prog_data.taps.v_taps_c;
        h_num_taps = (*scl_data).dscl_prog_data.taps.h_taps; h_num_taps_c = (*scl_data).dscl_prog_data.taps.h_taps_c;
    }
    if dscl_mode != DCN401_DSCL_MODE_DSCL_BYPASS { dpp60_power_on_dscl(dpp_base, true); }
    REG_SET_4!(dpp, DSCL_AUTOCAL, 0, AUTOCAL_MODE, 0, AUTOCAL_FRAC_MODE, 0, AUTOCAL_NUM_PIPE, 0, AUTOCAL_PIPE_ID, 0);
    REG_SET!(dpp, DSCL_CONTROL, 0, SCL_BOUNDARY_MODE, 0);
    dpp401_dscl_set_recout(dpp401, rect);
    REG_SET_2!(dpp, MPC_SIZE, 0, MPC_WIDTH, mpc_width, MPC_HEIGHT, mpc_height);
    REG_UPDATE!(dpp, SCL_MODE, DSCL_MODE, dscl_mode);
    if dscl_mode == DCN401_DSCL_MODE_DSCL_BYPASS { dpp60_power_on_dscl(dpp_base, false); return; }
    let lb_config = dpp401_dscl_find_lb_memory_config(dpp401, scl_data);
    dpp60_dscl_set_lb(dpp, &(*scl_data).lb_params, lb_config);
    if dscl_mode == DCN401_DSCL_MODE_SCALING_444_BYPASS {
        if (*(*dpp).base.ctx).dc.config.prefer_easf { dpp401_dscl_disable_easf(dpp_base, scl_data); }
        dpp401_dscl_program_isharp(dpp_base, scl_data, program_isharp_1dlut, &mut bs_coeffs_updated); return;
    }
    let black = if ycbcr { BLACK_OFFSET_CBCR } else { BLACK_OFFSET_RGB_Y };
    REG_SET_2!(dpp, SCL_BLACK_COLOR, 0, SCL_BLACK_COLOR_RGB_Y, BLACK_OFFSET_RGB_Y, SCL_BLACK_COLOR_CBCR, black);
    dpp60_dscl_set_manual_ratio_init(dpp, scl_data);
    REG_SET_4!(dpp, SCL_TAP_CONTROL, 0, SCL_V_NUM_TAPS, v_num_taps, SCL_H_NUM_TAPS, h_num_taps, SCL_V_NUM_TAPS_C, v_num_taps_c, SCL_H_NUM_TAPS_C, h_num_taps_c);
    dpp401_dscl_program_isharp(dpp_base, scl_data, program_isharp_1dlut, &mut bs_coeffs_updated);
    dpp401_dscl_set_scl_filter(dpp401, scl_data, ycbcr, bs_coeffs_updated);
    if (*(*dpp).base.ctx).dc.config.prefer_easf { dpp401_dscl_program_easf(dpp_base, scl_data); }
    PERF_TRACE!();
}

pub unsafe fn dpp60_dscl_program_upsp(dpp_base: *mut dpp, dscl_prog_data: *const dscl_prog_data) {
    let dpp = TO_DCN60_DPP(dpp_base);
    REG_SET_8!(dpp, UPSP_MODE, 0, UPSP_MODE, (*dscl_prog_data).upsp_mode, UPSP_V_NUM_TAPS, (*dscl_prog_data).upsp_v_num_taps, UPSP_V_INIT_INT, (*dscl_prog_data).upsp_v_init_int, UPSP_V_INIT_FRAC, (*dscl_prog_data).upsp_v_init_frac, UPSP_H_NUM_TAPS, (*dscl_prog_data).upsp_h_num_taps, UPSP_H_INIT_INT, (*dscl_prog_data).upsp_h_init_int, UPSP_H_INIT_FRAC, (*dscl_prog_data).upsp_h_init_frac, UPSP_BOUNDARY_MODE, (*dscl_prog_data).upsp_boundary_mode);
    REG_SET_4!(dpp, UPSP_V_COEF_P0, 0, UPSP_V_COEF_TAP0_P0, (*dscl_prog_data).upsp_v_coef_tap0_p0, UPSP_V_COEF_TAP1_P0, (*dscl_prog_data).upsp_v_coef_tap1_p0, UPSP_V_COEF_TAP2_P0, (*dscl_prog_data).upsp_v_coef_tap2_p0, UPSP_V_COEF_TAP3_P0, (*dscl_prog_data).upsp_v_coef_tap3_p0);
    REG_SET_4!(dpp, UPSP_V_COEF_P1, 0, UPSP_V_COEF_TAP0_P1, (*dscl_prog_data).upsp_v_coef_tap0_p1, UPSP_V_COEF_TAP1_P1, (*dscl_prog_data).upsp_v_coef_tap1_p1, UPSP_V_COEF_TAP2_P1, (*dscl_prog_data).upsp_v_coef_tap2_p1, UPSP_V_COEF_TAP3_P1, (*dscl_prog_data).upsp_v_coef_tap3_p1);
    REG_SET_4!(dpp, UPSP_H_COEF_P0, 0, UPSP_H_COEF_TAP0_P0, (*dscl_prog_data).upsp_h_coef_tap0_p0, UPSP_H_COEF_TAP1_P0, (*dscl_prog_data).upsp_h_coef_tap1_p0, UPSP_H_COEF_TAP2_P0, (*dscl_prog_data).upsp_h_coef_tap2_p0, UPSP_H_COEF_TAP3_P0, (*dscl_prog_data).upsp_h_coef_tap3_p0);
    REG_SET_4!(dpp, UPSP_H_COEF_P1, 0, UPSP_H_COEF_TAP0_P1, (*dscl_prog_data).upsp_h_coef_tap0_p1, UPSP_H_COEF_TAP1_P1, (*dscl_prog_data).upsp_h_coef_tap1_p1, UPSP_H_COEF_TAP2_P1, (*dscl_prog_data).upsp_h_coef_tap2_p1, UPSP_H_COEF_TAP3_P1, (*dscl_prog_data).upsp_h_coef_tap3_p1);
    REG_SET_2!(dpp, UPSP_CLAMP, 0, UPSP_CLAMP_MAX, (*dscl_prog_data).upsp_clamp_max, UPSP_CLAMP_MIN, (*dscl_prog_data).upsp_clamp_min);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
