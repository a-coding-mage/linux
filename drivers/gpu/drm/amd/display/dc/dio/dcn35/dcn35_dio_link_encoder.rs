/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies: reg_helper.h, core_types.h, link_encoder.h,
// dcn31/dcn31_dio_link_encoder.h, dcn35_dio_link_encoder.h, dc_dmub_srv.h

pub const DCN35_DIG_FE_SOURCE_SELECT_INVALID: u32 = 0x0;
pub const DCN35_DIG_FE_SOURCE_SELECT_DIGA: u32 = 0x1;
pub const DCN35_DIG_FE_SOURCE_SELECT_DIGB: u32 = 0x2;
pub const DCN35_DIG_FE_SOURCE_SELECT_DIGC: u32 = 0x4;
pub const DCN35_DIG_FE_SOURCE_SELECT_DIGD: u32 = 0x08;
pub const DCN35_DIG_FE_SOURCE_SELECT_DIGE: u32 = 0x10;

pub unsafe fn dcn35_is_dig_enabled(enc: *mut link_encoder) -> bool {
    let mut enabled: u32 = 0;
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_GET((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_CLK_EN, &mut enabled);
    enabled == 1
}

pub unsafe fn dcn35_get_dig_mode(enc: *mut link_encoder) -> signal_type {
    let mut value: u32 = 0;
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_GET((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_MODE, &mut value);
    match value {
        0 => SIGNAL_TYPE_DISPLAY_PORT,
        2 => SIGNAL_TYPE_DVI_SINGLE_LINK,
        3 => SIGNAL_TYPE_HDMI_TYPE_A,
        5 => SIGNAL_TYPE_DISPLAY_PORT_MST,
        _ => SIGNAL_TYPE_NONE,
    }
}

pub unsafe fn dcn35_link_encoder_setup(enc: *mut link_encoder, signal: signal_type) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    match signal {
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT => REG_UPDATE((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_MODE, 0),
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => REG_UPDATE((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_MODE, 2),
        SIGNAL_TYPE_HDMI_TYPE_A => REG_UPDATE((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_MODE, 3),
        SIGNAL_TYPE_DISPLAY_PORT_MST => REG_UPDATE((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_MODE, 5),
        _ => { ASSERT_CRITICAL(false); }
    }
    REG_UPDATE((*enc10).link_regs, DIG_BE_CLK_CNTL, DIG_BE_CLK_EN, 1);
}

pub unsafe fn dcn35_link_encoder_init(enc: *mut link_encoder) { enc31_hw_init(enc); }

pub unsafe fn dcn35_link_encoder_set_fgcg(enc: *mut link_encoder, enable: bool) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_UPDATE((*enc10).link_regs, DIO_CLK_CNTL, DIO_FGCG_REP_DIS, !enable);
}

static mut dcn35_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: Some(link_enc2_read_state),
    validate_output_with_stream: Some(dcn30_link_encoder_validate_output_with_stream),
    hw_init: Some(dcn35_link_encoder_init), setup: Some(dcn35_link_encoder_setup),
    enable_tmds_output: Some(dcn10_link_encoder_enable_tmds_output),
    enable_dp_output: Some(dcn35_link_encoder_enable_dp_output),
    enable_dp_mst_output: Some(dcn35_link_encoder_enable_dp_mst_output),
    disable_output: Some(dcn35_link_encoder_disable_output),
    dp_set_lane_settings: Some(dcn10_link_encoder_dp_set_lane_settings),
    dp_set_phy_pattern: Some(dcn10_link_encoder_dp_set_phy_pattern),
    update_mst_stream_allocation_table: Some(dcn10_link_encoder_update_mst_stream_allocation_table),
    psr_program_dp_dphy_fast_training: Some(dcn10_psr_program_dp_dphy_fast_training),
    psr_program_secondary_packet: Some(dcn10_psr_program_secondary_packet),
    connect_dig_be_to_fe: Some(dcn10_link_encoder_connect_dig_be_to_fe),
    enable_hpd: Some(dcn10_link_encoder_enable_hpd), disable_hpd: Some(dcn10_link_encoder_disable_hpd),
    is_dig_enabled: Some(dcn35_is_dig_enabled), destroy: Some(dcn10_link_encoder_destroy),
    fec_set_enable: Some(enc2_fec_set_enable), fec_set_ready: Some(enc2_fec_set_ready), fec_is_active: Some(enc2_fec_is_active),
    get_dig_frontend: Some(dcn10_get_dig_frontend), get_dig_mode: Some(dcn35_get_dig_mode),
    is_in_alt_mode: Some(dcn31_link_encoder_is_in_alt_mode), get_max_link_cap: Some(dcn31_link_encoder_get_max_link_cap),
    dpcstx_set_order_invert_18_bit: None, set_phy_source: None, dpcs_initialize_phy: None, dpcs_configure_phypll: None,
    dpcs_configure_dpcs: None, dpcs_enable_dpcs: None, prog_eq_setting: Some(dpcs32_program_eq_setting),
    get_txffe: Some(dpcs32_get_txffe), set_txffe: Some(dpcs32_set_txffe), set_dio_phy_mux: Some(dcn31_link_encoder_set_dio_phy_mux),
    enable_dpia_output: Some(dcn35_link_encoder_enable_dpia_output), disable_dpia_output: Some(dcn35_link_encoder_disable_dpia_output),
    get_hpd_state: Some(dcn10_get_hpd_state), program_hpd_filter: Some(dcn10_program_hpd_filter),
};

pub unsafe fn dcn35_link_encoder_construct(enc20: *mut dcn20_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask) {
    let mut bp_cap_info = bp_connector_speed_cap_info::default();
    let mut result = BP_RESULT_OK;
    let enc10 = &mut (*enc20).enc10;
    enc10.base.funcs = &dcn35_link_enc_funcs;
    enc10.base.ctx = (*init_data).ctx; enc10.base.id = (*init_data).encoder;
    enc10.base.hpd_gpio = (*init_data).hpd_gpio; enc10.base.hpd_source = (*init_data).hpd_source; enc10.base.connector = (*init_data).connector;
    enc10.base.preferred_engine = ENGINE_ID_UNKNOWN; enc10.base.features = *enc_features;
    if enc10.base.connector.id == CONNECTOR_ID_USBC { enc10.base.features.flags.bits.DP_IS_USB_C = 1; }
    enc10.base.transmitter = (*init_data).transmitter;
    enc10.base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;
    enc10.link_regs = link_regs; enc10.aux_regs = aux_regs; enc10.hpd_regs = hpd_regs; enc10.link_shift = link_shift; enc10.link_mask = link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter { TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB, TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD, TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE, _ => { ASSERT_CRITICAL(false); ENGINE_ID_UNKNOWN } };
    enc10.base.features.flags.bits.HDMI_6GB_EN = 1;
    let bp_funcs = (*(*init_data).ctx).dc_bios.funcs;
    if let Some(f) = (*bp_funcs).get_connector_speed_cap_info { result = f((*(*init_data).ctx).dc_bios, enc10.base.connector, &mut bp_cap_info); }
    if result == BP_RESULT_OK {
        enc10.base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN; enc10.base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN; enc10.base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN; enc10.base.features.flags.bits.IS_DP2_CAPABLE = 1; enc10.base.features.flags.bits.IS_UHBR10_CAPABLE = bp_cap_info.DP_UHBR10_EN; enc10.base.features.flags.bits.IS_UHBR13_5_CAPABLE = bp_cap_info.DP_UHBR13_5_EN; enc10.base.features.flags.bits.IS_UHBR20_CAPABLE = bp_cap_info.DP_UHBR20_EN;
        enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE = bp_cap_info.FRL_8G_EN || bp_cap_info.FRL_10G_EN || bp_cap_info.FRL_12G_EN; enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE = bp_cap_info.FRL_8G_EN; enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE = bp_cap_info.FRL_10G_EN; enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE = bp_cap_info.FRL_12G_EN; enc10.base.txffe_state = 0;
    } else { DC_LOG_WARNING!("{}: Failed to get encoder_cap_info from VBIOS with error code {}!\n", "dcn35_link_encoder_construct", result); }
    if (*(*init_data).ctx).dc.debug.hdmi20_disable { enc10.base.features.flags.bits.HDMI_6GB_EN = 0; }
    if (*(*init_data).ctx).dc.config.force_hdmi21_frl_enc_enable { enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE = 1; }
}

unsafe fn link_dpia_control(dc_ctx: *mut dc_context, dpia_control: *const dmub_cmd_dig_dpia_control_data) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.dig1_dpia_control.header.type_ = DMUB_CMD__DPIA; cmd.dig1_dpia_control.header.sub_type = DMUB_CMD__DPIA_DIG1_DPIA_CONTROL; cmd.dig1_dpia_control.header.payload_bytes = core::mem::size_of_val(&cmd.dig1_dpia_control) - core::mem::size_of_val(&cmd.dig1_dpia_control.header); cmd.dig1_dpia_control.dpia_control = *dpia_control;
    dc_wake_and_execute_dmub_cmd(dc_ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT); true
}

unsafe fn link_encoder_disable(enc10: *mut dcn10_link_encoder) { REG_UPDATE((*enc10).link_regs, DP_LINK_CNTL, DP_LINK_TRAINING_COMPLETE, 0); }

pub unsafe fn dcn35_link_encoder_enable_dp_output(enc: *mut link_encoder, settings: *const dc_link_settings, clock_source: clock_source_id) { let enc10 = TO_DCN10_LINK_ENC(enc); if !(*(*enc).ctx).dc.config.unify_link_enc_assignment { dcn31_link_encoder_enable_dp_output(enc, settings, clock_source); } else { DC_LOG_DEBUG!("enc_id({})", (*enc).preferred_engine); dcn20_link_encoder_enable_dp_output(enc, settings, clock_source); } let _ = enc10; }
pub unsafe fn dcn35_link_encoder_enable_dp_mst_output(enc: *mut link_encoder, settings: *const dc_link_settings, clock_source: clock_source_id) { if !(*(*enc).ctx).dc.config.unify_link_enc_assignment { dcn31_link_encoder_enable_dp_mst_output(enc, settings, clock_source); } else { DC_LOG_DEBUG!("enc_id({})", (*enc).preferred_engine); dcn10_link_encoder_enable_dp_mst_output(enc, settings, clock_source); } }
pub unsafe fn dcn35_link_encoder_disable_output(enc: *mut link_encoder, signal: signal_type) { let enc10 = TO_DCN10_LINK_ENC(enc); if !(*(*enc).ctx).dc.config.unify_link_enc_assignment { dcn31_link_encoder_disable_output(enc, signal); } else { DC_LOG_DEBUG!("enc_id({})", (*enc).preferred_engine); dcn10_link_encoder_disable_output(enc, signal); } let _ = enc10; }

pub unsafe fn dcn35_link_encoder_enable_dpia_output(enc: *mut link_encoder, settings: *const dc_link_settings, dpia_id: u8, digmode: u8, fec_rdy: u8) { let enc10 = TO_DCN10_LINK_ENC(enc); let mut c: dmub_cmd_dig_dpia_control_data = core::mem::zeroed(); enc1_configure_encoder(enc10, settings); c.action = TRANSMITTER_CONTROL_ENABLE as u8; c.enc_id = (*enc).preferred_engine; c.mode_laneset.digmode = digmode; c.lanenum = (*settings).lane_count as u8; c.symclk_10khz = (*settings).link_rate * LINK_RATE_REF_FREQ_IN_KHZ / 10; c.hpdsel = 6; c.dpia_id = dpia_id; c.fec_rdy = fec_rdy; DC_LOG_DEBUG!("DPIA({}) - enc_id({})", c.dpia_id, c.enc_id); link_dpia_control((*enc).ctx, &c); }
pub unsafe fn dcn35_link_encoder_disable_dpia_output(enc: *mut link_encoder, dpia_id: u8, digmode: u8) { let enc10 = TO_DCN10_LINK_ENC(enc); let mut c: dmub_cmd_dig_dpia_control_data = core::mem::zeroed(); if (*enc).funcs.is_dig_enabled.is_some() && !dcn35_is_dig_enabled(enc) { return; } c.action = TRANSMITTER_CONTROL_DISABLE as u8; c.enc_id = (*enc).preferred_engine; c.mode_laneset.digmode = digmode; c.dpia_id = dpia_id; DC_LOG_DEBUG!("DPIA({}) - enc_id({})", c.dpia_id, c.enc_id); link_dpia_control((*enc).ctx, &c); link_encoder_disable(enc10); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
