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
 *
 * Authors: AMD
 */

// C dependencies supplied by the surrounding translation unit.

macro_rules! REG { ($reg:ident) => { dwbc30.dwbc_regs.$reg }; }
macro_rules! CTX { () => { dwbc30.base.ctx }; }
macro_rules! FN { ($reg_name:ident, $field_name:ident) => { (dwbc30.dwbc_shift.$field_name, dwbc30.dwbc_mask.$field_name) }; }
macro_rules! DC_LOGGER { () => { dwbc30.base.ctx.logger }; }

unsafe fn dwb3_get_caps(dwbc: *mut dwbc, caps: *mut dwb_caps) -> bool {
    let _ = dwbc;
    if !caps.is_null() {
        (*caps).adapter_id = 0;
        (*caps).hw_version = DCN_VERSION_3_0;
        (*caps).num_pipes = 2;
        memset(&mut (*caps).reserved, 0, core::mem::size_of_val(&(*caps).reserved));
        memset(&mut (*caps).reserved2, 0, core::mem::size_of_val(&(*caps).reserved2));
        (*caps).sw_version = dwb_ver_2_0;
        (*caps).caps.support_dwb = true;
        (*caps).caps.support_ogam = true;
        (*caps).caps.support_wbscl = true;
        (*caps).caps.support_ocsc = false;
        (*caps).caps.support_stereo = true;
        true
    } else {
        false
    }
}

unsafe fn dwb3_config_fc(dwbc: *mut dwbc, params: *mut dc_dwb_params) {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    REG_UPDATE_2!(FC_SOURCE_SIZE, FC_SOURCE_WIDTH, (*params).cnv_params.src_width,
        FC_SOURCE_HEIGHT, (*params).cnv_params.src_height);
    if (*params).cnv_params.crop_en {
        REG_UPDATE!(FC_MODE_CTRL, FC_WINDOW_CROP_EN, 1);
        REG_UPDATE!(FC_WINDOW_START, FC_WINDOW_START_X, (*params).cnv_params.crop_x);
        REG_UPDATE!(FC_WINDOW_START, FC_WINDOW_START_Y, (*params).cnv_params.crop_y);
        REG_UPDATE!(FC_WINDOW_SIZE, FC_WINDOW_WIDTH, (*params).cnv_params.crop_width);
        REG_UPDATE!(FC_WINDOW_SIZE, FC_WINDOW_HEIGHT, (*params).cnv_params.crop_height);
    } else {
        REG_UPDATE!(FC_MODE_CTRL, FC_WINDOW_CROP_EN, 0);
    }
    REG_UPDATE!(FC_MODE_CTRL, FC_FRAME_CAPTURE_RATE, (*params).capture_rate);
    dwb3_set_stereo(dwbc, &mut (*params).stereo_params);
}

unsafe fn dwb3_enable(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    DC_LOG_DWB!("%s dwb3_enabled at inst = %d", __func__, (*dwbc).inst);
    REG_UPDATE!(DWB_ENABLE_CLK_CTRL, DWB_ENABLE, 1);
    dwb3_config_fc(dwbc, params);
    dwb3_program_hdr_mult(dwbc, params);
    dwb3_set_gamut_remap(dwbc, params);
    dwb3_ogam_set_input_transfer_func(dwbc, (*params).out_transfer_func);
    dwb3_set_denorm(dwbc, params);
    REG_UPDATE!(FC_MODE_CTRL, FC_FRAME_CAPTURE_EN, DWB_FRAME_CAPTURE_ENABLE);
    REG_UPDATE!(FC_FLOW_CTRL, FC_FIRST_PIXEL_DELAY_COUNT, 96);
    true
}

unsafe fn dwb3_disable(dwbc: *mut dwbc) -> bool {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    REG_UPDATE!(FC_MODE_CTRL, FC_FRAME_CAPTURE_EN, DWB_FRAME_CAPTURE_DISABLE);
    REG_UPDATE!(DWB_ENABLE_CLK_CTRL, DWB_ENABLE, 0);
    DC_LOG_DWB!("%s dwb3_disabled at inst = %d", __func__, (*dwbc).inst);
    true
}

unsafe fn dwb3_set_fc_enable(dwbc: *mut dwbc, enable: dwb_frame_capture_enable) {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    let mut pre_locked: u32 = 0;
    REG_GET!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, &mut pre_locked);
    if pre_locked == 0 { REG_UPDATE!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, 1); }
    REG_UPDATE!(FC_MODE_CTRL, FC_FRAME_CAPTURE_EN, enable);
    if pre_locked == 0 { REG_UPDATE!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, 0); }
    DC_LOG_DWB!("%s dwb3_fc_disabled at inst = %d", __func__, (*dwbc).inst);
}

unsafe fn dwb3_update(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    let mut pre_locked: u32 = 0;
    REG_GET!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, &mut pre_locked);
    DC_LOG_DWB!("%s dwb update, inst = %d", __func__, (*dwbc).inst);
    if pre_locked == 0 { REG_UPDATE!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, 1); }
    dwb3_config_fc(dwbc, params);
    dwb3_program_hdr_mult(dwbc, params);
    dwb3_set_gamut_remap(dwbc, params);
    dwb3_ogam_set_input_transfer_func(dwbc, (*params).out_transfer_func);
    dwb3_set_denorm(dwbc, params);
    if pre_locked == 0 { REG_UPDATE!(DWB_UPDATE_CTRL, DWB_UPDATE_LOCK, 0); }
    true
}

unsafe fn dwb3_is_enabled(dwbc: *mut dwbc) -> bool {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    let mut dwb_enabled: u32 = 0;
    let mut fc_frame_capture_en: u32 = 0;
    REG_GET!(DWB_ENABLE_CLK_CTRL, DWB_ENABLE, &mut dwb_enabled);
    REG_GET!(FC_MODE_CTRL, FC_FRAME_CAPTURE_EN, &mut fc_frame_capture_en);
    dwb_enabled != 0 && fc_frame_capture_en != 0
}

unsafe fn dwb3_set_stereo(dwbc: *mut dwbc, stereo_params: *mut dwb_stereo_params) {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    if (*stereo_params).stereo_enabled {
        REG_UPDATE!(FC_MODE_CTRL, FC_EYE_SELECTION, (*stereo_params).stereo_eye_select);
        REG_UPDATE!(FC_MODE_CTRL, FC_STEREO_EYE_POLARITY, (*stereo_params).stereo_polarity);
        DC_LOG_DWB!("%s dwb stereo enabled", __func__);
    } else {
        REG_UPDATE!(FC_MODE_CTRL, FC_EYE_SELECTION, 0);
        DC_LOG_DWB!("%s dwb stereo disabled", __func__);
    }
}

unsafe fn dwb3_set_new_content(dwbc: *mut dwbc, is_new_content: bool) {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    REG_UPDATE!(FC_MODE_CTRL, FC_NEW_CONTENT, is_new_content);
}

unsafe fn dwb3_set_denorm(dwbc: *mut dwbc, params: *mut dc_dwb_params) {
    let mut dwbc30 = TO_DCN30_DWBC(dwbc);
    REG_UPDATE!(DWB_OUT_CTRL, OUT_FORMAT, (*params).cnv_params.fc_out_format);
    if (*params).cnv_params.fc_out_format == DWB_OUT_FORMAT_32BPP_ARGB ||
       (*params).cnv_params.fc_out_format == DWB_OUT_FORMAT_32BPP_RGBA {
        REG_UPDATE!(DWB_OUT_CTRL, OUT_DENORM, (*params).cnv_params.out_denorm_mode);
        REG_UPDATE!(DWB_OUT_CTRL, OUT_MAX, (*params).cnv_params.out_max_pix_val);
        REG_UPDATE!(DWB_OUT_CTRL, OUT_MIN, (*params).cnv_params.out_min_pix_val);
    }
}

static dcn30_dwbc_funcs: dwbc_funcs = dwbc_funcs {
    get_caps: dwb3_get_caps,
    enable: dwb3_enable,
    disable: dwb3_disable,
    update: dwb3_update,
    is_enabled: dwb3_is_enabled,
    set_fc_enable: dwb3_set_fc_enable,
    set_stereo: dwb3_set_stereo,
    set_new_content: dwb3_set_new_content,
    dwb_ogam_set_input_transfer_func: dwb3_ogam_set_input_transfer_func, // TODO: rename
};

unsafe fn dcn30_dwbc_construct(
    dwbc30: *mut dcn30_dwbc,
    ctx: *mut dc_context,
    dwbc_regs: *const dcn30_dwbc_registers,
    dwbc_shift: *const dcn30_dwbc_shift,
    dwbc_mask: *const dcn30_dwbc_mask,
    inst: i32,
) {
    (*dwbc30).base.ctx = ctx;
    (*dwbc30).base.inst = inst;
    (*dwbc30).base.funcs = &dcn30_dwbc_funcs;
    (*dwbc30).dwbc_regs = dwbc_regs;
    (*dwbc30).dwbc_shift = dwbc_shift;
    (*dwbc30).dwbc_mask = dwbc_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
