// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding display-driver translation.

const PRESET_LEVEL_BYTE_LEVEL_SHIFT: u32 = 0x0;
const PRESET_LEVEL_BYTE_LEVEL_MASK: u32 = 0x1f;
const PRESET_LEVEL_BYTE_NO_PRE_SHIFT: u32 = 0x5;
const PRESET_LEVEL_BYTE_NO_DEMPH_SHIFT: u32 = 0x6;

static dcn60_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: Some(link_enc2_read_state), validate_output_with_stream: Some(dcn30_link_encoder_validate_output_with_stream),
    hw_init: Some(enc60_hw_init), setup: Some(dcn401_link_encoder_setup),
    enable_tmds_output: Some(dcn10_link_encoder_enable_tmds_output), enable_dp_output: Some(dcn401_link_encoder_enable_dp_output),
    enable_dp_mst_output: Some(dcn10_link_encoder_enable_dp_mst_output), disable_output: Some(dcn10_link_encoder_disable_output),
    dp_set_lane_settings: Some(dcn10_link_encoder_dp_set_lane_settings), dp_set_phy_pattern: Some(dcn10_link_encoder_dp_set_phy_pattern),
    update_mst_stream_allocation_table: Some(dcn10_link_encoder_update_mst_stream_allocation_table),
    psr_program_dp_dphy_fast_training: Some(dcn10_psr_program_dp_dphy_fast_training), psr_program_secondary_packet: Some(dcn10_psr_program_secondary_packet),
    connect_dig_be_to_fe: Some(dcn10_link_encoder_connect_dig_be_to_fe), enable_hpd: Some(dcn10_link_encoder_enable_hpd), disable_hpd: Some(dcn10_link_encoder_disable_hpd),
    is_dig_enabled: Some(dcn401_is_dig_enabled), destroy: Some(dcn10_link_encoder_destroy), fec_set_enable: Some(enc2_fec_set_enable), fec_set_ready: Some(enc2_fec_set_ready), fec_is_active: Some(enc2_fec_is_active),
    get_dig_frontend: Some(dcn10_get_dig_frontend), get_dig_mode: Some(dcn401_get_dig_mode), is_in_alt_mode: Some(dcn32_link_encoder_is_in_alt_mode), get_max_link_cap: Some(dcn32_link_encoder_get_max_link_cap),
    dpcstx_set_order_invert_18_bit: None, set_phy_source: None, dpcs_initialize_phy: None, dpcs_configure_phypll: None, dpcs_configure_dpcs: None, dpcs_enable_dpcs: None,
    prog_eq_setting: Some(dpcs60_program_eq_setting), get_txffe: Some(dpcs401_get_txffe), set_txffe: Some(dpcs401_set_txffe), set_dio_phy_mux: Some(dcn31_link_encoder_set_dio_phy_mux),
    setup_ri_pj_check_in_sw_or_hw_mode: Some(dcn401_setup_ri_pj_check_in_sw_or_hw_mode), get_hpd_state: Some(dcn42_get_hpd_state), program_hpd_filter: Some(dcn42_program_hpd_filter),
};

unsafe fn link_transmitter_control(
    enc10: *mut dcn10_link_encoder,
    cntl: *mut bp_transmitter_control,
) -> bp_result {
    let bp = (*(*enc10).base.ctx).dc_bios;
    ((*(*bp).funcs).transmitter_control)(bp, cntl)
}

pub unsafe extern "C" fn enc60_hw_init(enc: *mut link_encoder) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    match (*enc10).base.connector.id {
        CONNECTOR_ID_DISPLAY_PORT | CONNECTOR_ID_EDP => {
            dm_write_reg((*enc10).base.ctx, (*enc10).aux_regs.AUX_DPHY_RX_CONTROL0, 0x103d1110);
            dm_write_reg((*enc10).base.ctx, (*enc10).aux_regs.AUX_DPHY_TX_CONTROL, 0x21c7a);
            dcn10_aux_initialize(enc10);
        }
        _ => {}
    }
    REG_UPDATE!(enc10, TMDS_CTL_BITS, TMDS_CTL0, 1);
    // Polarity update is handled by DMU init.
    // AUX_PAD_MODE update done by DMU init.
}

pub unsafe extern "C" fn dpcs60_program_eq_setting(
    enc: *mut link_encoder,
    ffe_level: u8,
    mut de_emphasis_only: bool,
    mut pre_shoot_only: bool,
    no_ffe: bool,
    link_settings: *const dc_hdmi_frl_link_settings,
) {
    let max_ffe_level: u8 = if (*link_settings).frl_link_rate > HDMI_FRL_LINK_RATE_12GBPS { 0x7 } else { 0x3 };
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut cntl: bp_transmitter_control = core::mem::zeroed();

    if (*(*(*enc10).base.ctx).dc).debug.ignore_ffe { return; }
    if ffe_level <= max_ffe_level { (*enc10).base.txffe_state = ffe_level; }
    if (*(*(*enc10).base.ctx).dc).debug.select_ffe != 0 {
        (*enc10).base.txffe_state = (*(*(*enc10).base.ctx).dc).debug.select_ffe as u8;
    }
    if ffe_level == 0xee {
        (*enc10).base.txffe_state = (*enc10).base.txffe_state.wrapping_add(1);
        if (*enc10).base.txffe_state > max_ffe_level { (*enc10).base.txffe_state = 0; }
    }
    if no_ffe { de_emphasis_only = true; pre_shoot_only = true; }
    (*(&mut cntl)).lane_settings =
        ((de_emphasis_only as u32) << PRESET_LEVEL_BYTE_NO_PRE_SHIFT) |
        ((pre_shoot_only as u32) << PRESET_LEVEL_BYTE_NO_DEMPH_SHIFT) |
        (((*enc10).base.txffe_state as u32 & PRESET_LEVEL_BYTE_LEVEL_MASK) << PRESET_LEVEL_BYTE_LEVEL_SHIFT);
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
        HDMI_FRL_LINK_RATE_12GBPS => 666667 / 10,
        HDMI_FRL_LINK_RATE_16GBPS => 888889 / 10,
        HDMI_FRL_LINK_RATE_20GBPS | _ => 1111111 / 10,
    };
    link_transmitter_control(enc10, &mut cntl);
}

pub unsafe extern "C" fn dcn60_link_encoder_construct(
    enc20: *mut dcn20_link_encoder,
    init_data: *const encoder_init_data,
    enc_features: *const encoder_feature_support,
    link_regs: *const dcn10_link_enc_registers,
    aux_regs: *const dcn10_link_enc_aux_registers,
    hpd_regs: *const dcn10_link_enc_hpd_registers,
    link_shift: *const dcn10_link_enc_shift,
    link_mask: *const dcn10_link_enc_mask,
) {
    let mut bp_cap_info: bp_connector_speed_cap_info = core::mem::zeroed();
    let bp_funcs = (*(*init_data).ctx).dc_bios.funcs;
    let mut result = BP_RESULT_OK;
    let enc10 = &mut (*enc20).enc10;
    enc10.base.funcs = &dcn60_link_enc_funcs;
    enc10.base.ctx = (*init_data).ctx;
    enc10.base.id = (*init_data).encoder;
    enc10.base.hpd_source = (*init_data).hpd_source;
    enc10.base.hpd_active_high = (*init_data).hpd_active_high;
    enc10.base.connector = (*init_data).connector;
    enc10.base.preferred_engine = ENGINE_ID_UNKNOWN;
    enc10.base.features = *enc_features;
    if enc10.base.connector.id == CONNECTOR_ID_USBC { enc10.base.features.flags.bits.DP_IS_USB_C = 1; }
    enc10.base.transmitter = (*init_data).transmitter;
    enc10.base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS |
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;
    enc10.link_regs = link_regs; enc10.aux_regs = aux_regs; enc10.hpd_regs = hpd_regs;
    enc10.link_shift = link_shift; enc10.link_mask = link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter {
        TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB,
        TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD,
        TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE, _ => { ASSERT_CRITICAL!(false); ENGINE_ID_UNKNOWN }
    };
    enc10.base.features.flags.bits.HDMI_6GB_EN = 1;
    if let Some(f) = (*bp_funcs).get_connector_speed_cap_info { result = f(enc10.base.ctx.dc_bios, enc10.base.connector, &mut bp_cap_info); }
    if result == BP_RESULT_OK {
        enc10.base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN;
        enc10.base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN;
        enc10.base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN;
        enc10.base.features.flags.bits.IS_DP2_CAPABLE = 1;
        enc10.base.features.flags.bits.IS_UHBR10_CAPABLE = bp_cap_info.DP_UHBR10_EN;
        enc10.base.features.flags.bits.IS_UHBR13_5_CAPABLE = bp_cap_info.DP_UHBR13_5_EN;
        enc10.base.features.flags.bits.IS_UHBR20_CAPABLE = bp_cap_info.DP_UHBR20_EN;
        enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE = bp_cap_info.FRL_8G_EN || bp_cap_info.FRL_10G_EN || bp_cap_info.FRL_12G_EN || bp_cap_info.FRL_16G_EN || bp_cap_info.FRL_20G_EN || bp_cap_info.FRL_24G_EN;
        enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE = bp_cap_info.FRL_8G_EN; enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE = bp_cap_info.FRL_10G_EN;
        enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE = bp_cap_info.FRL_12G_EN; enc10.base.features.flags.bits.IS_FRL_16G_CAPABLE = bp_cap_info.FRL_16G_EN;
        enc10.base.features.flags.bits.IS_FRL_20G_CAPABLE = bp_cap_info.FRL_20G_EN; enc10.base.features.flags.bits.IS_FRL_24G_CAPABLE = bp_cap_info.FRL_24G_EN;
        enc10.base.txffe_state = 0;
    } else { DC_LOG_WARNING!("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", "dcn60_link_encoder_construct", result); }
    if (*enc10.base.ctx).dc.debug.hdmi20_disable { enc10.base.features.flags.bits.HDMI_6GB_EN = 0; }
    if (*enc10.base.ctx).dc.config.force_hdmi21_frl_enc_enable {
        enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE = 1;
        enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE = 1;
        enc10.base.features.flags.bits.IS_FRL_16G_CAPABLE = 1; enc10.base.features.flags.bits.IS_FRL_20G_CAPABLE = 1;
        enc10.base.features.flags.bits.IS_FRL_24G_CAPABLE = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
