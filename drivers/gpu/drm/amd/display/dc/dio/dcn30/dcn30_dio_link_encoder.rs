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

// C dependencies are supplied by the surrounding translation unit.

static unsafe fn dcn30_link_encoder_validate_hdmi_frl_output(
    enc10: *const dcn10_link_encoder,
    crtc_timing: *const dc_crtc_timing,
) -> bool {
    let max_deep_color = (*enc10).base.features.max_hdmi_deep_color;
    if !(*enc10).base.features.flags.bits.IS_HDMI_FRL_CAPABLE { return false; }
    if max_deep_color < (*crtc_timing).display_color_depth { return false; }
    if (*crtc_timing).display_color_depth < COLOR_DEPTH_888 { return false; }
    /* TODO: check if hdmi_charclk is above ASIC cap (10 GBS for DCN3AG) */
    true
}

pub unsafe fn dcn30_link_encoder_validate_output_with_stream(
    enc: *mut link_encoder, stream: *const dc_stream_state,
) -> bool {
    if dc_is_hdmi_frl_signal((*stream).signal) {
        let enc10 = TO_DCN10_LINK_ENC(enc);
        dcn30_link_encoder_validate_hdmi_frl_output(enc10, &(*stream).timing)
    } else {
        dcn10_link_encoder_validate_output_with_stream(enc, stream)
    }
}

// Task: Program EQ setting. EQ setting can be done during P2 or P0 state.
pub unsafe fn dpcs30_program_eq_setting(
    enc: *mut link_encoder, FFE_Level: u8, de_emphasis_only: bool,
    pre_shoot_only: bool, no_ffe: bool,
    _link_settings: *const dc_hdmi_frl_link_settings,
) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    if (*(*enc10).base.ctx).dc.debug.ignore_ffe { return; }
    if FFE_Level < 0x5 { (*enc10).base.txffe_state = FFE_Level; }
    if FFE_Level == 0xEE {
        (*enc10).base.txffe_state = (*enc10).base.txffe_state.wrapping_add(1);
        if (*enc10).base.txffe_state > 3 { (*enc10).base.txffe_state = 0; }
    }
    let (mut eq_main, mut eq_pre, mut eq_post) = match (*enc10).base.txffe_state {
        0 => (if de_emphasis_only { 0x36 } else if pre_shoot_only { 0x39 } else { 0x31 }, 0x5, 0x8),
        1 => (if de_emphasis_only { 0x34 } else if pre_shoot_only { 0x39 } else { 0x2F }, 0x5, 0xA),
        2 => (if de_emphasis_only { 0x31 } else if pre_shoot_only { 0x39 } else { 0x2C }, 0x5, 0xD),
        3 => (if de_emphasis_only { 0x2E } else if pre_shoot_only { 0x39 } else { 0x29 }, 0x5, 0x10),
        _ => return,
    };
    if de_emphasis_only { eq_pre = 0; }
    if pre_shoot_only { eq_post = 0; }
    if no_ffe { eq_pre = 0; eq_post = 0; eq_main = 0x3E; }
    REG_UPDATE_3!(enc10, RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_TX0_EQ_MAIN, eq_main, RDPCS_PHY_DP_TX0_EQ_PRE, eq_pre, RDPCS_PHY_DP_TX0_EQ_POST, eq_post);
    REG_UPDATE_3!(enc10, RDPCSTX_PHY_FUSE1, RDPCS_PHY_DP_TX1_EQ_MAIN, eq_main, RDPCS_PHY_DP_TX1_EQ_PRE, eq_pre, RDPCS_PHY_DP_TX1_EQ_POST, eq_post);
    REG_UPDATE_3!(enc10, RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_TX2_EQ_MAIN, eq_main, RDPCS_PHY_DP_TX2_EQ_PRE, eq_pre, RDPCS_PHY_DP_TX2_EQ_POST, eq_post);
    REG_UPDATE_3!(enc10, RDPCSTX_PHY_FUSE3, RDPCS_PHY_DP_TX3_EQ_MAIN, eq_main, RDPCS_PHY_DP_TX3_EQ_PRE, eq_pre, RDPCS_PHY_DP_TX3_EQ_POST, eq_post);
}

pub unsafe fn dpcs30_get_txffe(enc: *mut link_encoder, lane_settings: *mut frl_txffe) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut eq_main = 0u32; let mut eq_pre = 0u32; let mut eq_post = 0u32;
    REG_GET_3!(enc10, RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_TX0_EQ_MAIN, &mut eq_main, RDPCS_PHY_DP_TX0_EQ_PRE, &mut eq_pre, RDPCS_PHY_DP_TX0_EQ_POST, &mut eq_post);
    (*lane_settings).amplitude[0]=eq_main; (*lane_settings).pre_emphasis[0]=eq_pre; (*lane_settings).post_emphasis[0]=eq_post;
    REG_GET_3!(enc10, RDPCSTX_PHY_FUSE1, RDPCS_PHY_DP_TX1_EQ_MAIN, &mut eq_main, RDPCS_PHY_DP_TX1_EQ_PRE, &mut eq_pre, RDPCS_PHY_DP_TX1_EQ_POST, &mut eq_post);
    (*lane_settings).amplitude[1]=eq_main; (*lane_settings).pre_emphasis[1]=eq_pre; (*lane_settings).post_emphasis[1]=eq_post;
    REG_GET_3!(enc10, RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_TX2_EQ_MAIN, &mut eq_main, RDPCS_PHY_DP_TX2_EQ_PRE, &mut eq_pre, RDPCS_PHY_DP_TX2_EQ_POST, &mut eq_post);
    (*lane_settings).amplitude[2]=eq_main; (*lane_settings).pre_emphasis[2]=eq_pre; (*lane_settings).post_emphasis[2]=eq_post;
    REG_GET_3!(enc10, RDPCSTX_PHY_FUSE3, RDPCS_PHY_DP_TX3_EQ_MAIN, &mut eq_main, RDPCS_PHY_DP_TX3_EQ_PRE, &mut eq_pre, RDPCS_PHY_DP_TX3_EQ_POST, &mut eq_post);
    (*lane_settings).amplitude[3]=eq_main; (*lane_settings).pre_emphasis[3]=eq_pre; (*lane_settings).post_emphasis[3]=eq_post;
}

pub unsafe fn dpcs30_set_txffe(enc: *mut link_encoder, lane_settings: *const frl_txffe) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    for i in 0..4 { let main=(*lane_settings).amplitude[i]; let pre=(*lane_settings).pre_emphasis[i]; let post=(*lane_settings).post_emphasis[i]; REG_UPDATE_3_LANE!(enc10, i, main, pre, post); }
}

static dcn30_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: link_enc2_read_state, validate_output_with_stream: dcn30_link_encoder_validate_output_with_stream,
    hw_init: enc3_hw_init, setup: dcn10_link_encoder_setup, enable_tmds_output: dcn10_link_encoder_enable_tmds_output,
    enable_dp_output: dcn20_link_encoder_enable_dp_output, enable_dp_mst_output: dcn10_link_encoder_enable_dp_mst_output,
    disable_output: dcn10_link_encoder_disable_output, dp_set_lane_settings: dcn10_link_encoder_dp_set_lane_settings,
    dp_set_phy_pattern: dcn10_link_encoder_dp_set_phy_pattern, update_mst_stream_allocation_table: dcn10_link_encoder_update_mst_stream_allocation_table,
    psr_program_dp_dphy_fast_training: dcn10_psr_program_dp_dphy_fast_training, psr_program_secondary_packet: dcn10_psr_program_secondary_packet,
    connect_dig_be_to_fe: dcn10_link_encoder_connect_dig_be_to_fe, enable_hpd: dcn10_link_encoder_enable_hpd, disable_hpd: dcn10_link_encoder_disable_hpd,
    is_dig_enabled: dcn10_is_dig_enabled, destroy: dcn10_link_encoder_destroy, fec_set_enable: enc2_fec_set_enable, fec_set_ready: enc2_fec_set_ready,
    fec_is_active: enc2_fec_is_active, get_dig_frontend: dcn10_get_dig_frontend, get_dig_mode: dcn10_get_dig_mode,
    is_in_alt_mode: dcn20_link_encoder_is_in_alt_mode, get_max_link_cap: dcn20_link_encoder_get_max_link_cap,
    dpcstx_set_order_invert_18_bit: None, set_phy_source: None, dpcs_initialize_phy: None, dpcs_configure_phypll: None,
    dpcs_configure_dpcs: None, dpcs_enable_dpcs: None, prog_eq_setting: dpcs30_program_eq_setting, get_txffe: dpcs30_get_txffe,
    set_txffe: dpcs30_set_txffe, get_hpd_state: dcn10_get_hpd_state, program_hpd_filter: dcn10_program_hpd_filter,
};

pub unsafe fn dcn30_link_encoder_construct(enc20: *mut dcn20_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask) {
    let enc10 = &mut (*enc20).enc10;
    enc10.base.funcs = &dcn30_link_enc_funcs; enc10.base.ctx=(*init_data).ctx; enc10.base.id=(*init_data).encoder;
    enc10.base.hpd_gpio=(*init_data).hpd_gpio; enc10.base.hpd_source=(*init_data).hpd_source; enc10.base.connector=(*init_data).connector;
    enc10.base.preferred_engine=ENGINE_ID_UNKNOWN; enc10.base.features=*enc_features; enc10.base.transmitter=(*init_data).transmitter;
    enc10.base.output_signals=SIGNAL_TYPE_DVI_SINGLE_LINK|SIGNAL_TYPE_DVI_DUAL_LINK|SIGNAL_TYPE_LVDS|SIGNAL_TYPE_DISPLAY_PORT|SIGNAL_TYPE_DISPLAY_PORT_MST|SIGNAL_TYPE_EDP|SIGNAL_TYPE_HDMI_TYPE_A;
    enc10.link_regs=link_regs; enc10.aux_regs=aux_regs; enc10.hpd_regs=hpd_regs; enc10.link_shift=link_shift; enc10.link_mask=link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter { TRANSMITTER_UNIPHY_A=>ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B=>ENGINE_ID_DIGB, TRANSMITTER_UNIPHY_C=>ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D=>ENGINE_ID_DIGD, TRANSMITTER_UNIPHY_E=>ENGINE_ID_DIGE, TRANSMITTER_UNIPHY_F=>ENGINE_ID_DIGF, TRANSMITTER_UNIPHY_G=>ENGINE_ID_DIGG, _=>{ ASSERT_CRITICAL!(false); ENGINE_ID_UNKNOWN } };
    enc10.base.features.flags.bits.HDMI_6GB_EN=1;
    let mut bp_cap_info = core::mem::zeroed::<bp_encoder_cap_info>();
    let result = ((*(*init_data).ctx).dc_bios.funcs).get_encoder_cap_info((*(*init_data).ctx).dc_bios, enc10.base.id, &mut bp_cap_info);
    if result == BP_RESULT_OK { let f=&mut enc10.base.features.flags.bits; f.IS_HBR2_CAPABLE=bp_cap_info.DP_HBR2_EN; f.IS_HBR3_CAPABLE=bp_cap_info.DP_HBR3_EN; f.HDMI_6GB_EN=bp_cap_info.HDMI_6GB_EN; f.IS_DP2_CAPABLE=bp_cap_info.IS_DP2_CAPABLE; f.IS_UHBR10_CAPABLE=bp_cap_info.DP_UHBR10_EN; f.IS_UHBR13_5_CAPABLE=bp_cap_info.DP_UHBR13_5_EN; f.IS_UHBR20_CAPABLE=bp_cap_info.DP_UHBR20_EN; f.DP_IS_USB_C=bp_cap_info.DP_IS_USB_C; f.IS_HDMI_FRL_CAPABLE=bp_cap_info.IS_HDMI_FRL_CAPABLE; f.IS_FRL_8G_CAPABLE=bp_cap_info.FRL_8G_EN; f.IS_FRL_10G_CAPABLE=bp_cap_info.FRL_10G_EN; f.IS_FRL_12G_CAPABLE=bp_cap_info.FRL_12G_EN; enc10.base.txffe_state=0; }
    if (*(*enc10).base.ctx).dc.config.force_hdmi21_frl_enc_enable { let f=&mut enc10.base.features.flags.bits; f.IS_HDMI_FRL_CAPABLE=1; f.IS_FRL_8G_CAPABLE=1; f.IS_FRL_10G_CAPABLE=1; f.IS_FRL_12G_CAPABLE=1; }
}

pub unsafe fn enc3_hw_init(enc: *mut link_encoder) {
    let enc10=TO_DCN10_LINK_ENC(enc);
    AUX_REG_WRITE!(enc10, AUX_DPHY_RX_CONTROL0, 0x103d1110);
    AUX_REG_WRITE!(enc10, AUX_DPHY_TX_CONTROL, 0x21c7a);
    // AUX_TX_REF_DIV generates a 2 MHz reference from refclk (27MHz -> 0xd, 100MHz -> 0x32, 48MHz -> 0x18).
    REG_UPDATE!(enc10, TMDS_CTL_BITS, TMDS_CTL0, 1);
    dcn10_aux_initialize(enc10);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
