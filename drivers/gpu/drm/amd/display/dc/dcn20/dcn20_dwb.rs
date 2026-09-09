/*
 * Copyright 2012-17 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translation unit.

#[allow(dead_code)]
enum DwbOutsidePixStrategy {
    Black = 0,
    Edge = 1,
}

unsafe fn dwb2_get_caps(dwbc: *mut dwbc, caps: *mut dwb_caps) -> bool {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    if !caps.is_null() {
        (*caps).adapter_id = 0;
        (*caps).hw_version = DCN_VERSION_2_0;
        (*caps).num_pipes = 1;
        memset!(&mut (*caps).reserved, 0, core::mem::size_of_val(&(*caps).reserved));
        memset!(&mut (*caps).reserved2, 0, core::mem::size_of_val(&(*caps).reserved2));
        (*caps).sw_version = dwb_ver_1_0;
        (*caps).caps.support_dwb = true;
        (*caps).caps.support_ogam = false;
        (*caps).caps.support_wbscl = false;
        (*caps).caps.support_ocsc = false;
        DC_LOG_DWB!("%s SUPPORTED! inst = %d", "dwb2_get_caps", (*dwbc20).base.inst);
        true
    } else {
        DC_LOG_DWB!("%s NOT SUPPORTED! inst = %d", "dwb2_get_caps", (*dwbc20).base.inst);
        false
    }
}

pub unsafe fn dwb2_config_dwb_cnv(dwbc: *mut dwbc, params: *mut dc_dwb_params) {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d", "dwb2_config_dwb_cnv", (*dwbc20).base.inst);
    REG_UPDATE_2!(dwbc20, CNV_SOURCE_SIZE, CNV_SOURCE_WIDTH, (*params).cnv_params.src_width,
        CNV_SOURCE_HEIGHT, (*params).cnv_params.src_height);
    if (*params).cnv_params.crop_en {
        REG_UPDATE!(dwbc20, CNV_MODE, CNV_WINDOW_CROP_EN, 1);
        REG_UPDATE!(dwbc20, CNV_WINDOW_START, CNV_WINDOW_START_X, (*params).cnv_params.crop_x);
        REG_UPDATE!(dwbc20, CNV_WINDOW_START, CNV_WINDOW_START_Y, (*params).cnv_params.crop_y);
        REG_UPDATE!(dwbc20, CNV_WINDOW_SIZE, CNV_WINDOW_WIDTH, (*params).cnv_params.crop_width);
        REG_UPDATE!(dwbc20, CNV_WINDOW_SIZE, CNV_WINDOW_HEIGHT, (*params).cnv_params.crop_height);
    } else {
        REG_UPDATE!(dwbc20, CNV_MODE, CNV_WINDOW_CROP_EN, 0);
    }
    REG_UPDATE!(dwbc20, CNV_MODE, CNV_FRAME_CAPTURE_RATE, (*params).capture_rate);
    REG_UPDATE!(dwbc20, CNV_MODE, CNV_OUT_BPC, (*params).cnv_params.cnv_out_bpc);
}

unsafe fn dwb2_enable(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    if (*params).cnv_params.src_width != (*params).dest_width || (*params).cnv_params.src_height != (*params).dest_height {
        DC_LOG_DWB!("%s inst = %d, FAILED!LUMA SCALING NOT SUPPORTED", "dwb2_enable", (*dwbc20).base.inst);
        return false;
    }
    DC_LOG_DWB!("%s inst = %d, ENABLED", "dwb2_enable", (*dwbc20).base.inst);
    REG_UPDATE!(dwbc20, WB_ENABLE, WB_ENABLE, 1);
    dwb2_config_dwb_cnv(dwbc, params);
    dwb2_set_scaler(dwbc, params);
    REG_UPDATE!(dwbc20, CNV_MODE, CNV_FRAME_CAPTURE_EN, DWB_FRAME_CAPTURE_ENABLE);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL1, GMC_WARM_UP_ENABLE, 0);
    true
}

pub unsafe fn dwb2_disable(dwbc: *mut dwbc) -> bool {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d, Disabled", "dwb2_disable", (*dwbc20).base.inst);
    REG_UPDATE!(dwbc20, CNV_MODE, CNV_FRAME_CAPTURE_EN, DWB_FRAME_CAPTURE_DISABLE);
    REG_UPDATE!(dwbc20, WB_ENABLE, WB_ENABLE, 0);
    REG_UPDATE!(dwbc20, WB_SOFT_RESET, WB_SOFT_RESET, 1);
    REG_UPDATE!(dwbc20, WB_SOFT_RESET, WB_SOFT_RESET, 0);
    true
}

unsafe fn dwb2_update(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    let mut pre_locked: u32 = 0;
    if (*params).cnv_params.src_width != (*params).dest_width || (*params).cnv_params.src_height != (*params).dest_height {
        DC_LOG_DWB!("%s inst = %d, FAILED!LUMA SCALING NOT SUPPORTED", "dwb2_update", (*dwbc20).base.inst);
        return false;
    }
    DC_LOG_DWB!("%s inst = %d, scaling", "dwb2_update", (*dwbc20).base.inst);
    REG_GET!(dwbc20, CNV_UPDATE, CNV_UPDATE_LOCK, &mut pre_locked);
    if pre_locked == 0 { REG_UPDATE!(dwbc20, CNV_UPDATE, CNV_UPDATE_LOCK, 1); }
    dwb2_config_dwb_cnv(dwbc, params);
    dwb2_set_scaler(dwbc, params);
    if pre_locked == 0 { REG_UPDATE!(dwbc20, CNV_UPDATE, CNV_UPDATE_LOCK, 0); }
    true
}

pub unsafe fn dwb2_is_enabled(dwbc: *mut dwbc) -> bool {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    let mut wb_enabled = 0u32;
    let mut cnv_frame_capture_en = 0u32;
    REG_GET!(dwbc20, WB_ENABLE, WB_ENABLE, &mut wb_enabled);
    REG_GET!(dwbc20, CNV_MODE, CNV_FRAME_CAPTURE_EN, &mut cnv_frame_capture_en);
    wb_enabled != 0 && cnv_frame_capture_en != 0
}

pub unsafe fn dwb2_set_stereo(dwbc: *mut dwbc, stereo_params: *mut dwb_stereo_params) {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d, enabled =%d", "dwb2_set_stereo", (*dwbc20).base.inst, (*stereo_params).stereo_enabled);
    if (*stereo_params).stereo_enabled {
        REG_UPDATE!(dwbc20, CNV_MODE, CNV_STEREO_TYPE, (*stereo_params).stereo_type);
        REG_UPDATE!(dwbc20, CNV_MODE, CNV_EYE_SELECTION, (*stereo_params).stereo_eye_select);
        REG_UPDATE!(dwbc20, CNV_MODE, CNV_STEREO_POLARITY, (*stereo_params).stereo_polarity);
    } else { REG_UPDATE!(dwbc20, CNV_MODE, CNV_EYE_SELECTION, 0); }
}

pub unsafe fn dwb2_set_new_content(dwbc: *mut dwbc, is_new_content: bool) {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d", "dwb2_set_new_content", (*dwbc20).base.inst);
    REG_UPDATE!(dwbc20, CNV_MODE, CNV_NEW_CONTENT, is_new_content);
}

unsafe fn dwb2_set_warmup(dwbc: *mut dwbc, warmup_params: *mut dwb_warmup_params) {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d", "dwb2_set_warmup", (*dwbc20).base.inst);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL1, GMC_WARM_UP_ENABLE, (*warmup_params).warmup_en);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL1, WIDTH_WARMUP, (*warmup_params).warmup_width);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL1, HEIGHT_WARMUP, (*warmup_params).warmup_height);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL2, DATA_VALUE_WARMUP, (*warmup_params).warmup_data);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL2, MODE_WARMUP, (*warmup_params).warmup_mode);
    REG_UPDATE!(dwbc20, WB_WARM_UP_MODE_CTL2, DATA_DEPTH_WARMUP, (*warmup_params).warmup_depth);
}

pub unsafe fn dwb2_set_scaler(dwbc: *mut dwbc, params: *mut dc_dwb_params) {
    let dwbc20 = TO_DCN20_DWBC!(dwbc);
    DC_LOG_DWB!("%s inst = %d", "dwb2_set_scaler", (*dwbc20).base.inst);
    REG_UPDATE_2!(dwbc20, WBSCL_MODE, WBSCL_MODE, (*params).out_format, WBSCL_OUT_BIT_DEPTH, (*params).output_depth);
    if (*params).out_format != dwb_scaler_mode_bypass444 {
        REG_UPDATE!(dwbc20, WBSCL_DEST_SIZE, WBSCL_DEST_WIDTH, (*params).dest_width);
        REG_UPDATE!(dwbc20, WBSCL_DEST_SIZE, WBSCL_DEST_HEIGHT, (*params).dest_height);
        REG_UPDATE!(dwbc20, WBSCL_ROUND_OFFSET, WBSCL_ROUND_OFFSET_Y_RGB, 0x40);
        REG_UPDATE!(dwbc20, WBSCL_ROUND_OFFSET, WBSCL_ROUND_OFFSET_CBCR, 0x200);
        REG_UPDATE!(dwbc20, WBSCL_CLAMP_Y_RGB, WBSCL_CLAMP_UPPER_Y_RGB, 0x3fe);
        REG_UPDATE!(dwbc20, WBSCL_CLAMP_Y_RGB, WBSCL_CLAMP_LOWER_Y_RGB, 0x1);
        REG_UPDATE!(dwbc20, WBSCL_CLAMP_CBCR, WBSCL_CLAMP_UPPER_CBCR, 0x3fe);
        REG_UPDATE!(dwbc20, WBSCL_CLAMP_CBCR, WBSCL_CLAMP_LOWER_CBCR, 0x1);
        REG_UPDATE!(dwbc20, WBSCL_OUTSIDE_PIX_STRATEGY, WBSCL_OUTSIDE_PIX_STRATEGY, 1);
        if (*params).cnv_params.crop_en {
            dwb_program_horz_scalar!((*dwbc20), (*params).cnv_params.crop_width, (*params).dest_width, (*params).scaler_taps);
            dwb_program_vert_scalar!((*dwbc20), (*params).cnv_params.crop_height, (*params).dest_height, (*params).scaler_taps, (*params).subsample_position);
        } else {
            dwb_program_horz_scalar!((*dwbc20), (*params).cnv_params.src_width, (*params).dest_width, (*params).scaler_taps);
            dwb_program_vert_scalar!((*dwbc20), (*params).cnv_params.src_height, (*params).dest_height, (*params).scaler_taps, (*params).subsample_position);
        }
    }
    if (*dwbc20).dwbc_mask.WBSCL_COEF_RAM_SEL != 0 {
        let wbscl_mode = REG_READ!(dwbc20, WBSCL_MODE);
        let coef_ram_current = get_reg_field_value_ex!(wbscl_mode, (*dwbc20).dwbc_mask.WBSCL_COEF_RAM_SEL_CURRENT, (*dwbc20).dwbc_shift.WBSCL_COEF_RAM_SEL_CURRENT);
        REG_UPDATE!(dwbc20, WBSCL_MODE, WBSCL_COEF_RAM_SEL, !coef_ram_current);
    }
}

pub unsafe fn dcn20_dwbc_construct(dwbc20: *mut dcn20_dwbc, ctx: *mut dc_context,
    dwbc_regs: *const dcn20_dwbc_registers, dwbc_shift: *const dcn20_dwbc_shift,
    dwbc_mask: *const dcn20_dwbc_mask, inst: i32) {
    (*dwbc20).base.ctx = ctx;
    (*dwbc20).base.inst = inst;
    (*dwbc20).base.funcs = &dcn20_dwbc_funcs;
    (*dwbc20).dwbc_regs = dwbc_regs;
    (*dwbc20).dwbc_shift = dwbc_shift;
    (*dwbc20).dwbc_mask = dwbc_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
