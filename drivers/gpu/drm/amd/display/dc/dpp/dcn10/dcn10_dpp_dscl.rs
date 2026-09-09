/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

const NUM_PHASES: i32 = 64;
const HORZ_MAX_TAPS: i32 = 8;
const VERT_MAX_TAPS: i32 = 8;
const BLACK_OFFSET_RGB_Y: u32 = 0x0;
const BLACK_OFFSET_CBCR: u32 = 0x8000;

#[repr(i32)]
enum Dcn10CoefFilterTypeSel { SclCoefLumaVertFilter = 0, SclCoefLumaHorzFilter, SclCoefChromaVertFilter, SclCoefChromaHorzFilter, SclCoefAlphaVertFilter, SclCoefAlphaHorzFilter }
#[repr(i32)]
enum DsclAutocalMode { AutocalModeOff = 0, AutocalModeAutoscale = 1, AutocalModeAutocenter = 2, AutocalModeAutoreplicate = 3 }
#[repr(i32)]
enum DsclModeSel { DsclModeScaling444Bypass = 0, DsclModeScaling444RgbEnable = 1, DsclModeScaling444YcbcrEnable = 2, DsclModeScaling420YcbcrEnable = 3, DsclModeScaling420LumaBypass = 4, DsclModeScaling420ChromaBypass = 5, DsclModeDsclBypass = 6 }

unsafe fn dpp1_dscl_get_pixel_depth_val(depth: lb_pixel_depth) -> i32 {
    if depth == LB_PIXEL_DEPTH_30BPP { 0 } else if depth == LB_PIXEL_DEPTH_24BPP { 1 } else if depth == LB_PIXEL_DEPTH_18BPP { 2 } else if depth == LB_PIXEL_DEPTH_36BPP { 3 } else { ASSERT(0); -1 }
}
unsafe fn dpp1_dscl_is_video_format(format: dc_pixel_format) -> bool { format >= PIXEL_FORMAT_VIDEO_BEGIN && format <= PIXEL_FORMAT_VIDEO_END }
unsafe fn dpp1_dscl_is_420_format(format: dc_pixel_format) -> bool { format == PIXEL_FORMAT_420BPP8 || format == PIXEL_FORMAT_420BPP10 }

unsafe fn dpp1_dscl_get_dscl_mode(dpp_base: *mut dpp, data: *const scaler_data, dbg_always_scale: bool) -> dscl_mode_sel {
    let one = dc_fixpt_one.value;
    if (*dpp_base).caps.dscl_data_proc_format == DSCL_DATA_PRCESSING_FIXED_FORMAT && (*data).format == PIXEL_FORMAT_FP16 { return DSCL_MODE_DSCL_BYPASS; }
    if (*data).ratios.horz.value == one && (*data).ratios.vert.value == one && (*data).ratios.horz_c.value == one && (*data).ratios.vert_c.value == one && !dbg_always_scale { return DSCL_MODE_SCALING_444_BYPASS; }
    if !dpp1_dscl_is_420_format((*data).format) { return if dpp1_dscl_is_video_format((*data).format) { DSCL_MODE_SCALING_444_YCBCR_ENABLE } else { DSCL_MODE_SCALING_444_RGB_ENABLE }; }
    if (*data).ratios.horz.value == one && (*data).ratios.vert.value == one { return DSCL_MODE_SCALING_420_LUMA_BYPASS; }
    if (*data).ratios.horz_c.value == one && (*data).ratios.vert_c.value == one { return DSCL_MODE_SCALING_420_CHROMA_BYPASS; }
    DSCL_MODE_SCALING_420_YCBCR_ENABLE
}

unsafe fn dpp1_power_on_dscl(dpp_base: *mut dpp, power_on: bool) {
    let dpp = TO_DCN10_DPP(dpp_base);
    if (*dpp).tf_regs.DSCL_MEM_PWR_CTRL != 0 {
        if power_on { REG_UPDATE!(dpp, DSCL_MEM_PWR_CTRL, LUT_MEM_PWR_FORCE, 0); REG_WAIT!(dpp, DSCL_MEM_PWR_STATUS, LUT_MEM_PWR_STATE, 0, 1, 5); }
        else if (*dpp).base.ctx.dc.debug.enable_mem_low_power.bits.dscl { (*dpp).base.ctx.dc.optimized_required = true; (*dpp).base.deferred_reg_writes.bits.disable_dscl = true; }
        else { REG_UPDATE!(dpp, DSCL_MEM_PWR_CTRL, LUT_MEM_PWR_FORCE, 3); }
    }
}

unsafe fn dpp1_dscl_set_lb(dpp: *mut dcn10_dpp, lb_params: *const line_buffer_params, mem_size_config: lb_memory_config) {
    let mut max_partitions: u32 = 63;
    if (*dpp).base.caps.dscl_data_proc_format == DSCL_DATA_PRCESSING_FIXED_FORMAT {
        let pixel_depth = dpp1_dscl_get_pixel_depth_val((*lb_params).depth) as u32;
        let dyn_pix_depth = (*lb_params).dynamic_pixel_depth;
        REG_SET_7!(dpp, LB_DATA_FORMAT, 0, PIXEL_DEPTH, pixel_depth, PIXEL_EXPAN_MODE, (*lb_params).pixel_expan_mode, PIXEL_REDUCE_MODE, 1, DYNAMIC_PIXEL_DEPTH, dyn_pix_depth, DITHER_EN, 0, INTERLEAVE_EN, (*lb_params).interleave_en, LB_DATA_FORMAT__ALPHA_EN, (*lb_params).alpha_en);
    } else { REG_SET_2!(dpp, LB_DATA_FORMAT, 0, INTERLEAVE_EN, (*lb_params).interleave_en, LB_DATA_FORMAT__ALPHA_EN, (*lb_params).alpha_en); }
    if (*dpp).base.caps.max_lb_partitions == 31 { max_partitions = 31; }
    REG_SET_2!(dpp, LB_MEMORY_CTRL, 0, MEMORY_CONFIG, mem_size_config, LB_MAX_PARTITIONS, max_partitions);
}

unsafe fn dpp1_dscl_get_filter_coeffs_64p(taps: i32, ratio: fixed31_32) -> *const u16 {
    if taps == 8 { get_filter_8tap_64p(ratio) } else if taps == 7 { get_filter_7tap_64p(ratio) } else if taps == 6 { get_filter_6tap_64p(ratio) } else if taps == 5 { get_filter_5tap_64p(ratio) } else if taps == 4 { get_filter_4tap_64p(ratio) } else if taps == 3 { get_filter_3tap_64p(ratio) } else if taps == 2 { get_filter_2tap_64p() } else if taps == 1 { core::ptr::null() } else { BREAK_TO_DEBUGGER!(); core::ptr::null() }
}

unsafe fn dpp1_dscl_set_scaler_filter(dpp: *mut dcn10_dpp, taps: u32, filter_type: dcn10_coef_filter_type_sel, filter: *const u16) {
    let tap_pairs = (taps + 1) / 2;
    REG_SET_3!(dpp, SCL_COEF_RAM_TAP_SELECT, 0, SCL_COEF_RAM_TAP_PAIR_IDX, 0, SCL_COEF_RAM_PHASE, 0, SCL_COEF_RAM_FILTER_TYPE, filter_type);
    for phase in 0..(NUM_PHASES / 2 + 1) { for pair in 0..tap_pairs { let even_coef = *filter.add((phase as usize) * taps as usize + 2 * pair as usize); let odd_coef = if pair * 2 + 1 < taps { *filter.add((phase as usize) * taps as usize + 2 * pair as usize + 1) } else { 0 }; REG_SET_4!(dpp, SCL_COEF_RAM_TAP_DATA, 0, SCL_COEF_RAM_EVEN_TAP_COEF, even_coef, SCL_COEF_RAM_EVEN_TAP_COEF_EN, 1, SCL_COEF_RAM_ODD_TAP_COEF, odd_coef, SCL_COEF_RAM_ODD_TAP_COEF_EN, 1); } }
}

unsafe fn dpp1_dscl_get_lb_depth_bpc(depth: lb_pixel_depth) -> i32 { if depth == LB_PIXEL_DEPTH_30BPP { 10 } else if depth == LB_PIXEL_DEPTH_24BPP { 8 } else if depth == LB_PIXEL_DEPTH_18BPP { 6 } else if depth == LB_PIXEL_DEPTH_36BPP { 12 } else { BREAK_TO_DEBUGGER!(); -1 } }

unsafe fn dpp1_dscl_set_scl_filter(dpp: *mut dcn10_dpp, scl_data: *const scaler_data, chroma_coef_mode: bool) {
    let h_hard = (*scl_data).taps.h_taps < 3 && (*scl_data).taps.h_taps_c < 3 && (*scl_data).taps.h_taps > 1 && (*scl_data).taps.h_taps_c > 1;
    let v_hard = (*scl_data).taps.v_taps < 3 && (*scl_data).taps.v_taps_c < 3 && (*scl_data).taps.v_taps > 1 && (*scl_data).taps.v_taps_c > 1;
    let h_sharp = h_hard && (*scl_data).sharpness.horz != 0; let v_sharp = v_hard && (*scl_data).sharpness.vert != 0;
    REG_UPDATE_6!(dpp, DSCL_2TAP_CONTROL, SCL_H_2TAP_HARDCODE_COEF_EN, h_hard, SCL_H_2TAP_SHARP_EN, h_sharp, SCL_H_2TAP_SHARP_FACTOR, (*scl_data).sharpness.horz, SCL_V_2TAP_HARDCODE_COEF_EN, v_hard, SCL_V_2TAP_SHARP_EN, v_sharp, SCL_V_2TAP_SHARP_FACTOR, (*scl_data).sharpness.vert);
    if !v_hard || !h_hard { let fh = dpp1_dscl_get_filter_coeffs_64p((*scl_data).taps.h_taps, (*scl_data).ratios.horz); let fv = dpp1_dscl_get_filter_coeffs_64p((*scl_data).taps.v_taps, (*scl_data).ratios.vert); let fhc = if chroma_coef_mode { dpp1_dscl_get_filter_coeffs_64p((*scl_data).taps.h_taps_c, (*scl_data).ratios.horz_c) } else { core::ptr::null() }; let fvc = if chroma_coef_mode { dpp1_dscl_get_filter_coeffs_64p((*scl_data).taps.v_taps_c, (*scl_data).ratios.vert_c) } else { core::ptr::null() }; let mode = REG_READ!(dpp, SCL_MODE); if !h_hard && !fh.is_null() { dpp1_dscl_set_scaler_filter(dpp, (*scl_data).taps.h_taps as u32, SCL_COEF_LUMA_HORZ_FILTER, fh); } if !v_hard && !fv.is_null() { dpp1_dscl_set_scaler_filter(dpp, (*scl_data).taps.v_taps as u32, SCL_COEF_LUMA_VERT_FILTER, fv); } if chroma_coef_mode { if !h_hard && !fhc.is_null() { dpp1_dscl_set_scaler_filter(dpp, (*scl_data).taps.h_taps_c as u32, SCL_COEF_CHROMA_HORZ_FILTER, fhc); } if !v_hard && !fvc.is_null() { dpp1_dscl_set_scaler_filter(dpp, (*scl_data).taps.v_taps_c as u32, SCL_COEF_CHROMA_VERT_FILTER, fvc); } } REG_SET_2!(dpp, SCL_MODE, mode, SCL_COEF_RAM_SELECT, !get_reg_field_value_ex(mode, (*dpp).tf_mask.SCL_COEF_RAM_SELECT_CURRENT, (*dpp).tf_shift.SCL_COEF_RAM_SELECT_CURRENT), SCL_CHROMA_COEF_MODE, chroma_coef_mode); }
}

pub unsafe fn dpp1_dscl_calc_lb_num_partitions(scl_data: *const scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32) {
    let mut lb_memory_size; let mut lb_memory_size_c; let mut lb_memory_size_a; let num_partitions_a; let lb_bpc; let memory_line_size_y; let memory_line_size_c; let memory_line_size_a;
    let mut line_size = if (*scl_data).viewport.width < (*scl_data).recout.width { (*scl_data).viewport.width } else { (*scl_data).recout.width };
    let mut line_size_c = if (*scl_data).viewport_c.width < (*scl_data).recout.width { (*scl_data).viewport_c.width } else { (*scl_data).recout.width };
    if line_size == 0 { line_size = 1; } if line_size_c == 0 { line_size_c = 1; }
    lb_bpc = dpp1_dscl_get_lb_depth_bpc((*scl_data).lb_params.depth); memory_line_size_y = (line_size * lb_bpc + 71) / 72; memory_line_size_c = (line_size_c * lb_bpc + 71) / 72; memory_line_size_a = (line_size + 5) / 6;
    if lb_config == LB_MEMORY_CONFIG_1 { lb_memory_size=816; lb_memory_size_c=816; lb_memory_size_a=984; } else if lb_config == LB_MEMORY_CONFIG_2 { lb_memory_size=1088; lb_memory_size_c=1088; lb_memory_size_a=1312; } else if lb_config == LB_MEMORY_CONFIG_3 { lb_memory_size=816+1088+848+848+848; lb_memory_size_c=816+1088; lb_memory_size_a=984+1312+456; } else { lb_memory_size=816+1088+848; lb_memory_size_c=816+1088+848; lb_memory_size_a=984+1312+456; }
    *num_part_y = lb_memory_size / memory_line_size_y; *num_part_c = lb_memory_size_c / memory_line_size_c; num_partitions_a = lb_memory_size_a / memory_line_size_a;
    if (*scl_data).lb_params.alpha_en && num_partitions_a < *num_part_y { *num_part_y = num_partitions_a; } if *num_part_y > 64 { *num_part_y = 64; } if *num_part_c > 64 { *num_part_c = 64; }
}

pub unsafe fn dpp1_dscl_is_lb_conf_valid(ceil_vratio: i32, num_partitions: i32, vtaps: i32) -> bool { if ceil_vratio > 2 { vtaps <= num_partitions - ceil_vratio + 2 } else { vtaps <= num_partitions } }

/* The remaining scaler programming routine is a direct low-level translation. */
pub unsafe fn dpp1_dscl_set_scaler_manual_scale(dpp_base: *mut dpp, scl_data: *const scaler_data) {
    let dpp = TO_DCN10_DPP(dpp_base); let dscl_mode = dpp1_dscl_get_dscl_mode(dpp_base, scl_data, (*dpp_base).ctx.dc.debug.always_scale);
    let ycbcr = (*scl_data).format >= PIXEL_FORMAT_VIDEO_BEGIN && (*scl_data).format <= PIXEL_FORMAT_VIDEO_END;
    if core::intrinsics::memcmp(&(*dpp).scl_data as *const _ as *const u8, scl_data as *const u8, core::mem::size_of::<scaler_data>()) == 0 { return; }
    PERF_TRACE!(); (*dpp).scl_data = *scl_data;
    if (*dpp_base).ctx.dc.debug.enable_mem_low_power.bits.dscl && dscl_mode != DSCL_MODE_DSCL_BYPASS { dpp1_power_on_dscl(dpp_base, true); }
    REG_SET_3!(dpp, DSCL_AUTOCAL, 0, AUTOCAL_MODE, AUTOCAL_MODE_OFF, AUTOCAL_NUM_PIPE, 0, AUTOCAL_PIPE_ID, 0); REG_SET!(dpp, DSCL_CONTROL, 0, SCL_BOUNDARY_MODE, 0);
    REG_SET_2!(dpp, RECOUT_START, 0, RECOUT_START_X, (*scl_data).recout.x, RECOUT_START_Y, (*scl_data).recout.y); REG_SET_2!(dpp, RECOUT_SIZE, 0, RECOUT_WIDTH, (*scl_data).recout.width, RECOUT_HEIGHT, (*scl_data).recout.height);
    REG_SET_2!(dpp, MPC_SIZE, 0, MPC_WIDTH, (*scl_data).h_active, MPC_HEIGHT, (*scl_data).v_active); REG_UPDATE!(dpp, SCL_MODE, DSCL_MODE, dscl_mode);
    if dscl_mode == DSCL_MODE_DSCL_BYPASS { if (*dpp_base).ctx.dc.debug.enable_mem_low_power.bits.dscl { dpp1_power_on_dscl(dpp_base, false); } return; }
    let _ = ycbcr;
    PERF_TRACE!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
