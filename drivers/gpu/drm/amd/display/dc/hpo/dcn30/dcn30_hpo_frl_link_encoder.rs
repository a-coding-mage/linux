/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translated repository.

pub unsafe fn hpo_frl_link_enc3_setup_link_encoder(
    enc: *mut hpo_frl_link_encoder,
    lane_count: i32,
) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);

    DC_LOG_DEBUG!("Entering [{}]\n", __func__);

    if (*(*(*enc).ctx).dc).caps.ips_v2_support {
        REG_UPDATE!(HDMI_FRL_ENC_MEM_CTRL, METERBUFFER_MEM_PWR_DIS, 1);
        REG_WAIT!(HDMI_FRL_ENC_MEM_CTRL, METERBUFFER_MEM_PWR_STATE, 0, 1, 100);
    }
    REG_UPDATE!(HDMI_LINK_ENC_CLK_CTRL, HDMI_LINK_ENC_CLOCK_EN, 1);
    REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_LANE_COUNT, if lane_count == 3 { 0 } else { 1 });
    REG_UPDATE_2!(HDMI_LINK_ENC_CONTROL,
        HDMI_LINK_ENC_ENABLE, 0,
        HDMI_LINK_ENC_SOFT_RESET, 1);
    REG_UPDATE!(HDMI_LINK_ENC_CONTROL, HDMI_LINK_ENC_SOFT_RESET, 0);
    REG_UPDATE!(HDMI_LINK_ENC_CONTROL, HDMI_LINK_ENC_ENABLE, 1);

    DC_LOG_HDMI_FRL!("Exiting [{}]\n", __func__);
}

pub unsafe fn hpo_frl_link_enc3_set_training_pattern(
    enc: *mut hpo_frl_link_encoder,
    lane0_pattern: u32, lane1_pattern: u32,
    lane2_pattern: u32, lane3_pattern: u32,
) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_TRAINING_ENABLE, 1);
    if lane0_pattern < 8 { REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_LANE0_TRAINING_PATTERN, lane0_pattern); }
    if lane1_pattern < 8 { REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_LANE1_TRAINING_PATTERN, lane1_pattern); }
    if lane2_pattern < 8 { REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_LANE2_TRAINING_PATTERN, lane2_pattern); }
    if lane3_pattern < 8 { REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_LANE3_TRAINING_PATTERN, lane3_pattern); }
}

pub unsafe fn hpo_frl_link_enc3_get_training_pattern(
    enc: *mut hpo_frl_link_encoder,
    lane0_pattern: *mut u32, lane1_pattern: *mut u32,
    lane2_pattern: *mut u32, lane3_pattern: *mut u32,
) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    REG_GET_4!(HDMI_FRL_ENC_CONFIG,
        HDMI_LINK_LANE0_TRAINING_PATTERN, lane0_pattern,
        HDMI_LINK_LANE1_TRAINING_PATTERN, lane1_pattern,
        HDMI_LINK_LANE2_TRAINING_PATTERN, lane2_pattern,
        HDMI_LINK_LANE3_TRAINING_PATTERN, lane3_pattern);
}

unsafe fn link_transmitter_control(
    enc10: *mut dcn10_link_encoder,
    cntl: *mut bp_transmitter_control,
) -> bp_result {
    let bp = (*(*enc10).base.ctx).dc_bios;
    ((*(*bp).funcs).transmitter_control)(bp, cntl)
}

unsafe fn hpo_frl_link_enc3_enable_phy_output(
    hpo_enc: *mut hpo_frl_link_encoder,
    enc: *mut link_encoder,
    clock_source: clock_source_id,
    frl_link_rate: hdmi_frl_link_rate,
) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(hpo_enc);
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut cntl: bp_transmitter_control = core::mem::zeroed();
    let result: bp_result;

    cntl.action = TRANSMITTER_CONTROL_ENABLE;
    cntl.engine_id = (*enc).preferred_engine;
    cntl.transmitter = (*enc10).base.transmitter;
    cntl.pll_id = clock_source;
    cntl.signal = SIGNAL_TYPE_HDMI_FRL;
    cntl.hpd_sel = (*enc10).base.hpd_source;

    cntl.pixel_clock = match frl_link_rate {
        HDMI_FRL_LINK_RATE_3GBPS => 166667,
        HDMI_FRL_LINK_RATE_6GBPS | HDMI_FRL_LINK_RATE_6GBPS_4LANE => 333333,
        HDMI_FRL_LINK_RATE_8GBPS => 444444,
        HDMI_FRL_LINK_RATE_10GBPS => 555555,
        _ => 666667,
    };
    cntl.hpo_engine_id = (*enc3).base.inst + ENGINE_ID_HPO_0;
    cntl.lanes_number = if frl_link_rate <= HDMI_FRL_LINK_RATE_6GBPS { 3 } else { 4 };

    result = link_transmitter_control(enc10, &mut cntl);
    if result != BP_RESULT_OK {
        DC_LOG_HDMI_FRL!("{}: Failed to execute VBIOS command table!\n", __func__);
        BREAK_TO_DEBUGGER!();
    }
}

pub unsafe fn hpo_frl_link_enc3_enable_output(enc: *mut hpo_frl_link_encoder) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    DC_LOG_HDMI_FRL!("Entering [{}]\n", __func__);
    REG_UPDATE!(HDMI_FRL_ENC_CONFIG, HDMI_LINK_TRAINING_ENABLE, 0);
    DC_LOG_HDMI_FRL!("Exiting [{}]\n", __func__);
}

pub unsafe fn hpo_frl_link_enc3_disable(enc: *mut hpo_frl_link_encoder) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    DC_LOG_HDMI_FRL!("Entering [{}]\n", __func__);
    REG_UPDATE_5!(HDMI_FRL_ENC_CONFIG,
        HDMI_LINK_TRAINING_ENABLE, 1,
        HDMI_LINK_LANE0_TRAINING_PATTERN, 0,
        HDMI_LINK_LANE1_TRAINING_PATTERN, 0,
        HDMI_LINK_LANE2_TRAINING_PATTERN, 0,
        HDMI_LINK_LANE3_TRAINING_PATTERN, 0);
    REG_UPDATE!(HDMI_LINK_ENC_CONTROL, HDMI_LINK_ENC_ENABLE, 0);
    REG_UPDATE!(HDMI_LINK_ENC_CLK_CTRL, HDMI_LINK_ENC_CLOCK_EN, 0);
    REG_UPDATE!(HDMI_FRL_ENC_CONFIG2, HDMI_LINK_RC_COMPRESS_DISABLE, 0);
    DC_LOG_HDMI_FRL!("Exiting [{}]\n", __func__);
}

pub unsafe fn hpo_frl_link_enc3_read_state(
    enc: *mut hpo_frl_link_encoder,
    state: *mut hpo_frl_link_enc_state,
) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    let mut link_training_enabled: u32 = 0;
    let mut lane_count_field: u32 = 0;
    ASSERT!(state);
    REG_GET!(HDMI_LINK_ENC_CONTROL, HDMI_LINK_ENC_ENABLE, &mut (*state).link_enc_enabled);
    REG_GET_2!(HDMI_FRL_ENC_CONFIG,
        HDMI_LINK_TRAINING_ENABLE, &mut link_training_enabled,
        HDMI_LINK_LANE_COUNT, &mut lane_count_field);
    (*state).link_active = link_training_enabled == 1;
    (*state).lane_count = if lane_count_field == 1 { 4 } else { 3 };
}

pub unsafe fn hpo_frl_link_enc3_destroy(enc: *mut *mut hpo_frl_link_encoder) {
    kfree!(DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(*enc));
    *enc = core::ptr::null_mut();
}

pub unsafe fn hpo_frl_link_enc3_apply_vsdb_rcc_wa(enc: *mut hpo_frl_link_encoder) {
    let enc3 = DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(enc);
    REG_UPDATE!(HDMI_FRL_ENC_CONFIG2, HDMI_LINK_RC_COMPRESS_DISABLE, 1);
}

static mut dcn30_hpo_frl_link_encoder_funcs: hpo_frl_link_encoder_funcs = hpo_frl_link_encoder_funcs {
    setup_link_encoder: Some(hpo_frl_link_enc3_setup_link_encoder),
    set_hdmi_training_pattern: Some(hpo_frl_link_enc3_set_training_pattern),
    get_hdmi_training_pattern: Some(hpo_frl_link_enc3_get_training_pattern),
    enable_frl_phy_output: Some(hpo_frl_link_enc3_enable_phy_output),
    enable_output: Some(hpo_frl_link_enc3_enable_output),
    disable_link_encoder: Some(hpo_frl_link_enc3_disable),
    read_state: Some(hpo_frl_link_enc3_read_state),
    destroy: Some(hpo_frl_link_enc3_destroy),
    apply_vsdb_rcc_wa: Some(hpo_frl_link_enc3_apply_vsdb_rcc_wa),
};

pub unsafe fn hpo_frl_link_encoder3_construct(
    enc3: *mut dcn30_hpo_frl_link_encoder,
    ctx: *mut dc_context,
    inst: u32,
    hpo_le_regs: *const dcn30_hpo_frl_link_encoder_registers,
    hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
    hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
) {
    (*enc3).base.ctx = ctx;
    (*enc3).base.inst = inst;
    (*enc3).base.funcs = &raw mut dcn30_hpo_frl_link_encoder_funcs;
    (*enc3).regs = hpo_le_regs;
    (*enc3).hpo_le_shift = hpo_le_shift;
    (*enc3).hpo_le_mask = hpo_le_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
