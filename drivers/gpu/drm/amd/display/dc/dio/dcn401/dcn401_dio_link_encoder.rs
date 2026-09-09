/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

// Dependencies are supplied by the surrounding driver translation.

const HDMI_FRL_EQ__LEVEL__SHIFT: u32 = 0x0;
const HDMI_FRL_EQ__LEVEL__MASK: u32 = 0x3;
const HDMI_FRL_EQ__NO_PRE__SHIFT: u32 = 0x5;
const HDMI_FRL_EQ__NO_DEMPH__SHIFT: u32 = 0x6;
const HDMI_FRL_EQ__NO_FFE__SHIFT: u32 = 0x4;

pub unsafe fn enc401_hw_init(enc: *mut link_encoder) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    AUX_REG_WRITE!(enc10, AUX_DPHY_RX_CONTROL0, 0x103d1110);
    AUX_REG_WRITE!(enc10, AUX_DPHY_TX_CONTROL, 0x21c7a);
    // AUX_TX_REF_DIV is programmed by hardware defaults for the reference clock.
    REG_UPDATE!(enc10, TMDS_CTL_BITS, TMDS_CTL0, 1);
    dcn10_aux_initialize(enc10);
}

pub unsafe fn dcn401_link_encoder_enable_dp_output(
    enc: *mut link_encoder,
    link_settings: *const dc_link_settings,
    clock_source: clock_source_id,
) {
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        dcn10_link_encoder_enable_dp_output(enc, link_settings, clock_source);
        return;
    }
}

unsafe fn link_transmitter_control(
    enc10: *mut dcn10_link_encoder,
    cntl: *mut bp_transmitter_control,
) -> bp_result {
    let bp = (*(*enc10).base.ctx).dc_bios;
    ((*(*bp).funcs).transmitter_control)(bp, cntl)
}

pub unsafe fn dpcs401_program_eq_setting(
    enc: *mut link_encoder, FFE_Level: u8, mut de_emphasis_only: bool,
    mut pre_shoot_only: bool, no_ffe: bool,
    link_settings: *const dc_hdmi_frl_link_settings,
) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut cntl: bp_transmitter_control = core::mem::zeroed();
    if (*(*enc10).base.ctx).dc.debug.ignore_ffe { return; }
    if FFE_Level < 0x5 { (*enc10).base.txffe_state = FFE_Level; }
    if (*(*enc10).base.ctx).dc.debug.select_ffe != 0 {
        (*enc10).base.txffe_state = (*(*enc10).base.ctx).dc.debug.select_ffe as u8;
    }
    if FFE_Level == 0xEE {
        (*enc10).base.txffe_state = (*enc10).base.txffe_state.wrapping_add(1);
        if (*enc10).base.txffe_state > 3 { (*enc10).base.txffe_state = 0; }
    }
    if no_ffe { de_emphasis_only = true; pre_shoot_only = true; }
    cntl.lane_settings = ((de_emphasis_only as u32) << HDMI_FRL_EQ__NO_PRE__SHIFT)
        | ((pre_shoot_only as u32) << HDMI_FRL_EQ__NO_DEMPH__SHIFT)
        | (((*enc10).base.txffe_state as u32 & HDMI_FRL_EQ__LEVEL__MASK) << HDMI_FRL_EQ__LEVEL__SHIFT);
    cntl.lane_select = 0;
    cntl.action = TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS;
    cntl.transmitter = (*enc10).base.transmitter;
    cntl.connector_obj_id = (*enc10).base.connector;
    cntl.lanes_number = (*link_settings).frl_num_lanes;
    cntl.hpd_sel = (*enc10).base.hpd_source;
    cntl.pixel_clock = match (*link_settings).frl_link_rate {
        HDMI_FRL_LINK_RATE_3GBPS => 166667 / 10,
        HDMI_FRL_LINK_RATE_6GBPS | HDMI_FRL_LINK_RATE_6GBPS_4LANE => 333333 / 10,
        HDMI_FRL_LINK_RATE_8GBPS => 444444 / 10,
        HDMI_FRL_LINK_RATE_10GBPS => 555555 / 10,
        _ => 666667 / 10,
    };
    link_transmitter_control(enc10, &mut cntl);
}

pub unsafe fn dpcs401_get_txffe(enc: *mut link_encoder, lane_settings: *mut frl_txffe) {
    let _ = enc;
    let eq_main: u32 = 0; let eq_pre: u32 = 0; let eq_post: u32 = 0;
    for i in 0..4 {
        (*lane_settings).amplitude[i] = eq_main;
        (*lane_settings).pre_emphasis[i] = eq_pre;
        (*lane_settings).post_emphasis[i] = eq_post;
    }
}

pub unsafe fn dpcs401_set_txffe(enc: *mut link_encoder, lane_settings: *mut frl_txffe) {
    let _ = (enc, lane_settings);
    // TODO: Unused; the corresponding PHY fuse programming is not implemented.
}

pub unsafe fn dcn401_link_encoder_setup(enc: *mut link_encoder, signal: signal_type) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    match signal {
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT => REG_UPDATE!(enc10, DIG_BE_CLK_CNTL, DIG_BE_MODE, 0),
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => REG_UPDATE!(enc10, DIG_BE_CLK_CNTL, DIG_BE_MODE, 2),
        SIGNAL_TYPE_HDMI_TYPE_A => REG_UPDATE!(enc10, DIG_BE_CLK_CNTL, DIG_BE_MODE, 3),
        SIGNAL_TYPE_DISPLAY_PORT_MST => REG_UPDATE!(enc10, DIG_BE_CLK_CNTL, DIG_BE_MODE, 5),
        _ => { ASSERT_CRITICAL!(false); }
    }
    REG_UPDATE!(enc10, DIG_BE_CLK_CNTL, DIG_BE_CLK_EN, 1);
    REG_UPDATE!(enc10, DIG_BE_EN_CNTL, DIG_BE_ENABLE, 1);
}

pub unsafe fn dcn401_is_dig_enabled(enc: *mut link_encoder) -> bool {
    let enc10 = TO_DCN10_LINK_ENC(enc); let mut clk_enabled = 0; let mut dig_enabled = 0;
    REG_GET!(enc10, DIG_BE_CLK_CNTL, DIG_BE_CLK_EN, &mut clk_enabled);
    REG_GET!(enc10, DIG_BE_EN_CNTL, DIG_BE_ENABLE, &mut dig_enabled);
    clk_enabled == 1 && dig_enabled == 1
}

pub unsafe fn dcn401_get_dig_mode(enc: *mut link_encoder) -> signal_type {
    let enc10 = TO_DCN10_LINK_ENC(enc); let mut value = 0;
    REG_GET!(enc10, DIG_BE_CLK_CNTL, DIG_BE_MODE, &mut value);
    match value { 0 => SIGNAL_TYPE_DISPLAY_PORT, 2 => SIGNAL_TYPE_DVI_SINGLE_LINK,
        3 => SIGNAL_TYPE_HDMI_TYPE_A, 5 => SIGNAL_TYPE_DISPLAY_PORT_MST, _ => SIGNAL_TYPE_NONE }
}

pub unsafe fn dcn401_setup_ri_pj_check_in_sw_or_hw_mode(
    enc: *mut link_encoder, aux_or_ddc_instance: u8, enable_sw_mode: bool,
) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_UPDATE_2!(enc10, HDCP_I2C_CONTROL_0, HDCP_I2C_DISABLE, enable_sw_mode,
        HDCP_I2C_DDC_SELECT, aux_or_ddc_instance);
    REG_UPDATE!(enc10, HDCP_INT_CONTROL, HDCP_I2C_XFER_REQ_MASK, enable_sw_mode);
}

static dcn401_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: link_enc2_read_state,
    validate_output_with_stream: dcn30_link_encoder_validate_output_with_stream,
    hw_init: enc401_hw_init,
    setup: dcn401_link_encoder_setup,
    enable_tmds_output: dcn10_link_encoder_enable_tmds_output,
    enable_dp_output: dcn401_link_encoder_enable_dp_output,
    enable_dp_mst_output: dcn10_link_encoder_enable_dp_mst_output,
    disable_output: dcn10_link_encoder_disable_output,
    dp_set_lane_settings: dcn10_link_encoder_dp_set_lane_settings,
    dp_set_phy_pattern: dcn10_link_encoder_dp_set_phy_pattern,
    update_mst_stream_allocation_table: dcn10_link_encoder_update_mst_stream_allocation_table,
    psr_program_dp_dphy_fast_training: dcn10_psr_program_dp_dphy_fast_training,
    psr_program_secondary_packet: dcn10_psr_program_secondary_packet,
    connect_dig_be_to_fe: dcn10_link_encoder_connect_dig_be_to_fe,
    enable_hpd: dcn10_link_encoder_enable_hpd,
    disable_hpd: dcn10_link_encoder_disable_hpd,
    is_dig_enabled: dcn401_is_dig_enabled,
    destroy: dcn10_link_encoder_destroy,
    fec_set_enable: enc2_fec_set_enable,
    fec_set_ready: enc2_fec_set_ready,
    fec_is_active: enc2_fec_is_active,
    get_dig_frontend: dcn10_get_dig_frontend,
    get_dig_mode: dcn401_get_dig_mode,
    is_in_alt_mode: dcn32_link_encoder_is_in_alt_mode,
    get_max_link_cap: dcn32_link_encoder_get_max_link_cap,
    dpcstx_set_order_invert_18_bit: None,
    set_phy_source: None,
    dpcs_initialize_phy: None,
    dpcs_configure_phypll: None,
    dpcs_configure_dpcs: None,
    dpcs_enable_dpcs: None,
    prog_eq_setting: dpcs401_program_eq_setting,
    get_txffe: dpcs401_get_txffe,
    set_txffe: dpcs401_set_txffe,
    set_dio_phy_mux: dcn31_link_encoder_set_dio_phy_mux,
    setup_ri_pj_check_in_sw_or_hw_mode: dcn401_setup_ri_pj_check_in_sw_or_hw_mode,
    get_hpd_state: dcn10_get_hpd_state,
    program_hpd_filter: dcn10_program_hpd_filter,
};

pub unsafe fn dcn401_link_encoder_construct(
    enc20: *mut dcn20_link_encoder, init_data: *const encoder_init_data,
    enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers,
    aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers,
    link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask,
) {
    let mut bp_cap_info: bp_connector_speed_cap_info = core::mem::zeroed();
    let bp_funcs = (*(*init_data).ctx).dc_bios.funcs;
    let mut result = BP_RESULT_OK;
    let enc10 = &mut (*enc20).enc10;
    (*enc10).base.funcs = &dcn401_link_enc_funcs;
    (*enc10).base.ctx = (*init_data).ctx; (*enc10).base.id = (*init_data).encoder;
    (*enc10).base.hpd_gpio = (*init_data).hpd_gpio; (*enc10).base.hpd_source = (*init_data).hpd_source;
    (*enc10).base.connector = (*init_data).connector; (*enc10).base.preferred_engine = ENGINE_ID_UNKNOWN;
    (*enc10).base.features = *enc_features;
    if (*enc10).base.connector.id == CONNECTOR_ID_USBC { (*enc10).base.features.flags.bits.DP_IS_USB_C = 1; }
    (*enc10).base.transmitter = (*init_data).transmitter;
    (*enc10).base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK |
        SIGNAL_TYPE_LVDS | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST |
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;
    (*enc10).link_regs = link_regs; (*enc10).aux_regs = aux_regs; (*enc10).hpd_regs = hpd_regs;
    (*enc10).link_shift = link_shift; (*enc10).link_mask = link_mask;
    (*enc10).base.preferred_engine = match (*enc10).base.transmitter {
        TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB,
        TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD,
        TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE, _ => { ASSERT_CRITICAL!(false); ENGINE_ID_UNKNOWN }
    };
    (*enc10).base.features.flags.bits.HDMI_6GB_EN = 1;
    if !(*bp_funcs).get_connector_speed_cap_info.is_null() {
        result = ((*bp_funcs).get_connector_speed_cap_info)((*enc10).base.ctx.dc_bios,
            (*enc10).base.connector, &mut bp_cap_info);
    }
    if result == BP_RESULT_OK {
        let f = &mut (*enc10).base.features.flags.bits;
        f.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN; f.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN;
        f.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN; f.IS_DP2_CAPABLE = 1;
        f.IS_UHBR10_CAPABLE = bp_cap_info.DP_UHBR10_EN; f.IS_UHBR13_5_CAPABLE = bp_cap_info.DP_UHBR13_5_EN;
        f.IS_UHBR20_CAPABLE = bp_cap_info.DP_UHBR20_EN;
        f.IS_HDMI_FRL_CAPABLE = bp_cap_info.FRL_8G_EN || bp_cap_info.FRL_10G_EN || bp_cap_info.FRL_12G_EN;
        f.IS_FRL_8G_CAPABLE = bp_cap_info.FRL_8G_EN; f.IS_FRL_10G_CAPABLE = bp_cap_info.FRL_10G_EN;
        f.IS_FRL_12G_CAPABLE = bp_cap_info.FRL_12G_EN; (*enc10).base.txffe_state = 0;
    } else { DC_LOG_WARNING!("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", __func__, result); }
    if (*enc10).base.ctx.dc.debug.hdmi20_disable { (*enc10).base.features.flags.bits.HDMI_6GB_EN = 0; }
    if (*enc10).base.ctx.dc.config.force_hdmi21_frl_enc_enable {
        let f = &mut (*enc10).base.features.flags.bits; f.IS_HDMI_FRL_CAPABLE = 1;
        f.IS_FRL_8G_CAPABLE = 1; f.IS_FRL_10G_CAPABLE = 1; f.IS_FRL_12G_CAPABLE = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
