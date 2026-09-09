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

// C dependencies supplied by the surrounding display-driver translation.

#[allow(non_upper_case_globals)]
static mut dcn21_mpll_cfg_ref: [mpll_cfg; 4] = [
    mpll_cfg { hdmimode_enable: 0, ref_range: 1, ref_clk_mpllb_div: 1, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 238, mpllb_fracn_en: 0, mpllb_fracn_quot: 0, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 44237, mpllb_ssc_stepsize: 59454, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 2, tx_vboost_lvl: 5, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 2, mpllb_ana_cp_int: 9, mpllb_ana_cp_prop: 15, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 0, ref_range: 1, ref_clk_mpllb_div: 1, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 192, mpllb_fracn_en: 1, mpllb_fracn_quot: 32768, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 36864, mpllb_ssc_stepsize: 49545, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 1, tx_vboost_lvl: 5, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 3, mpllb_ana_cp_int: 9, mpllb_ana_cp_prop: 15, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 0, ref_range: 1, ref_clk_mpllb_div: 1, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 192, mpllb_fracn_en: 1, mpllb_fracn_quot: 32768, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 36864, mpllb_ssc_stepsize: 49545, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 0, tx_vboost_lvl: 5, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 3, mpllb_ana_cp_int: 9, mpllb_ana_cp_prop: 15, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 0, ref_range: 1, ref_clk_mpllb_div: 1, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 304, mpllb_fracn_en: 1, mpllb_fracn_quot: 49152, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 55296, mpllb_ssc_stepsize: 74318, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 0, tx_vboost_lvl: 5, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 1, mpllb_ana_cp_int: 7, mpllb_ana_cp_prop: 16, hdmi_pixel_clk_div: 0 },
];

unsafe fn update_cfg_data(enc10: *mut dcn10_link_encoder, link_settings: *const dc_link_settings, cfg: *mut dpcssys_phy_seq_cfg) -> bool {
    (*cfg).load_sram_fw = false;
    (*cfg).use_calibration_setting = true;
    // TODO: need to implement a proper lane mapping for Renoir.
    for i in 0..4 { (*cfg).lane_en[i] = true; }
    match (*link_settings).link_rate {
        LINK_RATE_LOW => (*cfg).mpll_cfg = dcn21_mpll_cfg_ref[0],
        LINK_RATE_HIGH => (*cfg).mpll_cfg = dcn21_mpll_cfg_ref[1],
        LINK_RATE_HIGH2 => (*cfg).mpll_cfg = dcn21_mpll_cfg_ref[2],
        LINK_RATE_HIGH3 => (*cfg).mpll_cfg = dcn21_mpll_cfg_ref[3],
        _ => { DC_LOG_ERROR("%s: No supported link rate found %X!\n", "update_cfg_data", (*link_settings).link_rate); return false; }
    }
    true
}

unsafe fn dcn21_link_encoder_acquire_phy(enc: *mut link_encoder) -> bool {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut value: u32 = 0;
    if (*enc).features.flags.bits.DP_IS_USB_C {
        REG_GET!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE, &mut value);
        if value == 1 { ASSERT!(0); return false; }
        REG_UPDATE!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE_ACK, 0);
        udelay(40);
        REG_GET!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE, &mut value);
        if value == 1 { ASSERT!(0); REG_UPDATE!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE_ACK, 1); return false; }
    }
    REG_UPDATE!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DP_REF_CLK_EN, 1);
    true
}

unsafe fn dcn21_link_encoder_release_phy(enc: *mut link_encoder) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    if (*enc).features.flags.bits.DP_IS_USB_C { REG_UPDATE!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE_ACK, 1); }
    REG_UPDATE!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DP_REF_CLK_EN, 0);
}

pub unsafe fn dcn21_link_encoder_enable_dp_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let enc21 = enc10 as *mut dcn21_link_encoder;
    let cfg = &mut (*enc21).phy_seq_cfg;
    if !dcn21_link_encoder_acquire_phy(enc) { return; }
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table { dcn10_link_encoder_enable_dp_output(enc, link_settings, clock_source); return; }
    if !update_cfg_data(enc10, link_settings, cfg) { return; }
    enc1_configure_encoder(enc10, link_settings);
    dcn10_link_encoder_setup(enc, SIGNAL_TYPE_DISPLAY_PORT);
}

unsafe fn dcn21_link_encoder_enable_dp_mst_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id) {
    if !dcn21_link_encoder_acquire_phy(enc) { return; }
    dcn10_link_encoder_enable_dp_mst_output(enc, link_settings, clock_source);
}

unsafe fn dcn21_link_encoder_disable_output(enc: *mut link_encoder, signal: signal_type) {
    dcn10_link_encoder_disable_output(enc, signal);
    if dc_is_dp_signal(signal) { dcn21_link_encoder_release_phy(enc); }
}

static dcn21_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: link_enc2_read_state, validate_output_with_stream: dcn10_link_encoder_validate_output_with_stream,
    hw_init: enc2_hw_init, setup: dcn10_link_encoder_setup, enable_tmds_output: dcn10_link_encoder_enable_tmds_output,
    enable_dp_output: dcn21_link_encoder_enable_dp_output, enable_dp_mst_output: dcn21_link_encoder_enable_dp_mst_output,
    disable_output: dcn21_link_encoder_disable_output, dp_set_lane_settings: dcn10_link_encoder_dp_set_lane_settings,
    dp_set_phy_pattern: dcn10_link_encoder_dp_set_phy_pattern, update_mst_stream_allocation_table: dcn10_link_encoder_update_mst_stream_allocation_table,
    psr_program_dp_dphy_fast_training: dcn10_psr_program_dp_dphy_fast_training, psr_program_secondary_packet: dcn10_psr_program_secondary_packet,
    connect_dig_be_to_fe: dcn10_link_encoder_connect_dig_be_to_fe, enable_hpd: dcn10_link_encoder_enable_hpd, disable_hpd: dcn10_link_encoder_disable_hpd,
    is_dig_enabled: dcn10_is_dig_enabled, destroy: dcn10_link_encoder_destroy, fec_set_enable: enc2_fec_set_enable, fec_set_ready: enc2_fec_set_ready,
    fec_is_active: enc2_fec_is_active, get_dig_frontend: dcn10_get_dig_frontend, is_in_alt_mode: dcn20_link_encoder_is_in_alt_mode,
    get_max_link_cap: dcn20_link_encoder_get_max_link_cap, get_hpd_state: dcn10_get_hpd_state, program_hpd_filter: dcn10_program_hpd_filter,
};

pub unsafe fn dcn21_link_encoder_construct(enc21: *mut dcn21_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask) {
    let mut bp_cap_info = core::mem::zeroed::<bp_encoder_cap_info>();
    let bp_funcs = (*(*init_data).ctx).dc_bios.funcs;
    let mut result = BP_RESULT_OK;
    let enc10 = &mut (*enc21).enc10;
    enc10.base.funcs = &dcn21_link_enc_funcs;
    enc10.base.ctx = (*init_data).ctx; enc10.base.id = (*init_data).encoder;
    enc10.base.hpd_gpio = (*init_data).hpd_gpio; enc10.base.hpd_source = (*init_data).hpd_source; enc10.base.connector = (*init_data).connector;
    enc10.base.preferred_engine = ENGINE_ID_UNKNOWN; enc10.base.features = *enc_features; enc10.base.transmitter = (*init_data).transmitter;
    enc10.base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;
    enc10.link_regs = link_regs; enc10.aux_regs = aux_regs; enc10.hpd_regs = hpd_regs; enc10.link_shift = link_shift; enc10.link_mask = link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter { TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB, TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD, TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE, TRANSMITTER_UNIPHY_F => ENGINE_ID_DIGF, TRANSMITTER_UNIPHY_G => ENGINE_ID_DIGG, _ => { ASSERT_CRITICAL(false); ENGINE_ID_UNKNOWN } };
    enc10.base.features.flags.bits.HDMI_6GB_EN = 1;
    result = ((*bp_funcs).get_encoder_cap_info)(enc10.base.ctx.dc_bios, enc10.base.id, &mut bp_cap_info);
    if result == BP_RESULT_OK { enc10.base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN; enc10.base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN; enc10.base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN; enc10.base.features.flags.bits.DP_IS_USB_C = bp_cap_info.DP_IS_USB_C; } else { DC_LOG_WARNING("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", "dcn21_link_encoder_construct", result); }
    if enc10.base.ctx.dc.debug.hdmi20_disable { enc10.base.features.flags.bits.HDMI_6GB_EN = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
