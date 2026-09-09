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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
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

// Dependencies supplied by the surrounding driver translation unit:
// reg_helper.h, core_types.h, link_encoder.h, dcn20_link_encoder.h,
// stream_encoder.h, dc_bios_types.h, and gpio_service_interface.h.

static mut DCN2_MPLL_CFG: [mpll_cfg; 4] = [
    mpll_cfg { hdmimode_enable: 1, ref_range: 3, ref_clk_mpllb_div: 2, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 226, mpllb_fracn_en: 1, mpllb_fracn_quot: 39321, mpllb_fracn_rem: 3, mpllb_fracn_den: 5, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 38221, mpllb_ssc_stepsize: 49314, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 2, tx_vboost_lvl: 4, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 2, mpllb_ana_cp_int: 7, mpllb_ana_cp_prop: 18, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 1, ref_range: 3, ref_clk_mpllb_div: 2, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 184, mpllb_fracn_en: 0, mpllb_fracn_quot: 0, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 31850, mpllb_ssc_stepsize: 41095, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 1, tx_vboost_lvl: 4, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 3, mpllb_ana_cp_int: 7, mpllb_ana_cp_prop: 18, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 1, ref_range: 3, ref_clk_mpllb_div: 2, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 184, mpllb_fracn_en: 0, mpllb_fracn_quot: 0, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 31850, mpllb_ssc_stepsize: 41095, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 0, tx_vboost_lvl: 4, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 3, mpllb_ana_cp_int: 7, mpllb_ana_cp_prop: 18, hdmi_pixel_clk_div: 0 },
    mpll_cfg { hdmimode_enable: 1, ref_range: 3, ref_clk_mpllb_div: 2, mpllb_ssc_en: 1, mpllb_div5_clk_en: 1, mpllb_multiplier: 292, mpllb_fracn_en: 0, mpllb_fracn_quot: 0, mpllb_fracn_rem: 0, mpllb_fracn_den: 1, mpllb_ssc_up_spread: 0, mpllb_ssc_peak: 47776, mpllb_ssc_stepsize: 61642, mpllb_div_clk_en: 0, mpllb_div_multiplier: 0, mpllb_hdmi_div: 0, mpllb_tx_clk_div: 0, tx_vboost_lvl: 4, mpllb_pmix_en: 1, mpllb_word_div2_en: 0, mpllb_ana_v2i: 2, mpllb_ana_freq_vco: 0, mpllb_ana_cp_int: 7, mpllb_ana_cp_prop: 18, hdmi_pixel_clk_div: 0 },
];

pub unsafe fn enc2_fec_set_enable(enc: *mut link_encoder, enable: bool) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    DC_LOG_DSC!("%s FEC at link encoder inst %d", if enable { "Enabling" } else { "Disabling" }, (*enc).id.enum_id);
    REG_UPDATE!(enc10, DP_DPHY_CNTL, DPHY_FEC_EN, enable);
}

pub unsafe fn enc2_fec_set_ready(enc: *mut link_encoder, ready: bool) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_UPDATE!(enc10, DP_DPHY_CNTL, DPHY_FEC_READY_SHADOW, ready);
}

pub unsafe fn enc2_fec_is_active(enc: *mut link_encoder) -> bool {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut active: u32 = 0;
    REG_GET!(enc10, DP_DPHY_CNTL, DPHY_FEC_ACTIVE_STATUS, &mut active);
    active != 0
}

pub unsafe fn link_enc2_read_state(enc: *mut link_encoder, s: *mut link_enc_state) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    REG_GET!(enc10, DP_DPHY_CNTL, DPHY_FEC_EN, &mut (*s).dphy_fec_en);
    REG_GET!(enc10, DP_DPHY_CNTL, DPHY_FEC_READY_SHADOW, &mut (*s).dphy_fec_ready_shadow);
    REG_GET!(enc10, DP_DPHY_CNTL, DPHY_FEC_ACTIVE_STATUS, &mut (*s).dphy_fec_active_status);
    REG_GET!(enc10, DP_LINK_CNTL, DP_LINK_TRAINING_COMPLETE, &mut (*s).dp_link_training_complete);
}

unsafe fn update_cfg_data(enc10: *mut dcn10_link_encoder, link_settings: *const dc_link_settings, cfg: *mut dpcssys_phy_seq_cfg) -> bool {
    (*cfg).load_sram_fw = false;
    for i in 0..(*link_settings).lane_count as usize { (*cfg).lane_en[i] = true; }
    (*cfg).mpll_cfg = match (*link_settings).link_rate {
        LINK_RATE_LOW => DCN2_MPLL_CFG[0], LINK_RATE_HIGH => DCN2_MPLL_CFG[1],
        LINK_RATE_HIGH2 => DCN2_MPLL_CFG[2], LINK_RATE_HIGH3 => DCN2_MPLL_CFG[3],
        _ => { DC_LOG_ERROR!("%s: No supported link rate found %X!\n", "update_cfg_data", (*link_settings).link_rate); return false; }
    };
    true
}

pub unsafe fn dcn20_link_encoder_enable_dp_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let enc20 = enc10 as *mut dcn20_link_encoder;
    let cfg = &mut (*enc20).phy_seq_cfg;
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table { dcn10_link_encoder_enable_dp_output(enc, link_settings, clock_source); return; }
    if !update_cfg_data(enc10, link_settings, cfg) { return; }
    enc1_configure_encoder(enc10, link_settings);
    dcn10_link_encoder_setup(enc, SIGNAL_TYPE_DISPLAY_PORT);
}

pub unsafe fn dcn20_link_encoder_get_max_link_cap(enc: *mut link_encoder, link_settings: *mut dc_link_settings) {
    let enc10 = TO_DCN10_LINK_ENC(enc); let mut dp4: u32 = 0;
    dcn10_link_encoder_get_max_link_cap(enc, link_settings);
    if (*(*enc).funcs).is_in_alt_mode.is_some() && ((*(*enc).funcs).is_in_alt_mode.unwrap())(enc) {
        REG_GET!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DP4, &mut dp4);
        if dp4 == 0 { (*link_settings).lane_count = MIN!(LANE_COUNT_TWO, (*link_settings).lane_count); }
    }
}

pub unsafe fn dcn20_link_encoder_is_in_alt_mode(enc: *mut link_encoder) -> bool {
    let enc10 = TO_DCN10_LINK_ENC(enc); let mut disable: u32 = 0;
    if (*enc).features.flags.bits.DP_IS_USB_C { REG_GET!(enc10, RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE, &mut disable); return disable == 0; }
    false
}

pub unsafe fn enc2_hw_init(enc: *mut link_encoder) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    if (*(*enc).ctx).dc_bios.golden_table.dc_golden_table_ver > 0 {
        dm_write_reg((*enc).ctx, (*enc10).aux_regs.AUX_DPHY_RX_CONTROL0, (*(*enc).ctx).dc_bios.golden_table.aux_dphy_rx_control0_val);
        dm_write_reg((*enc).ctx, (*enc10).aux_regs.AUX_DPHY_TX_CONTROL, (*(*enc).ctx).dc_bios.golden_table.aux_dphy_tx_control_val);
        dm_write_reg((*enc).ctx, (*enc10).aux_regs.AUX_DPHY_RX_CONTROL1, (*(*enc).ctx).dc_bios.golden_table.aux_dphy_rx_control1_val);
    } else {
        dm_write_reg((*enc).ctx, (*enc10).aux_regs.AUX_DPHY_RX_CONTROL0, 0x103d1110);
        dm_write_reg((*enc).ctx, (*enc10).aux_regs.AUX_DPHY_TX_CONTROL, 0x21c7a);
    }
    REG_UPDATE!(enc10, TMDS_CTL_BITS, TMDS_CTL0, 1);
    dcn10_aux_initialize(enc10);
}

static dcn20_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: Some(link_enc2_read_state), validate_output_with_stream: Some(dcn10_link_encoder_validate_output_with_stream), hw_init: Some(enc2_hw_init), setup: Some(dcn10_link_encoder_setup), enable_tmds_output: Some(dcn10_link_encoder_enable_tmds_output_with_clk_pattern_wa), enable_dp_output: Some(dcn20_link_encoder_enable_dp_output), enable_dp_mst_output: Some(dcn10_link_encoder_enable_dp_mst_output), disable_output: Some(dcn10_link_encoder_disable_output), dp_set_lane_settings: Some(dcn10_link_encoder_dp_set_lane_settings), dp_set_phy_pattern: Some(dcn10_link_encoder_dp_set_phy_pattern), update_mst_stream_allocation_table: Some(dcn10_link_encoder_update_mst_stream_allocation_table), psr_program_dp_dphy_fast_training: Some(dcn10_psr_program_dp_dphy_fast_training), psr_program_secondary_packet: Some(dcn10_psr_program_secondary_packet), connect_dig_be_to_fe: Some(dcn10_link_encoder_connect_dig_be_to_fe), enable_hpd: Some(dcn10_link_encoder_enable_hpd), disable_hpd: Some(dcn10_link_encoder_disable_hpd), is_dig_enabled: Some(dcn10_is_dig_enabled), destroy: Some(dcn10_link_encoder_destroy), fec_set_enable: Some(enc2_fec_set_enable), fec_set_ready: Some(enc2_fec_set_ready), fec_is_active: Some(enc2_fec_is_active), get_dig_mode: Some(dcn10_get_dig_mode), get_dig_frontend: Some(dcn10_get_dig_frontend), is_in_alt_mode: Some(dcn20_link_encoder_is_in_alt_mode), get_max_link_cap: Some(dcn20_link_encoder_get_max_link_cap), get_hpd_state: Some(dcn10_get_hpd_state), program_hpd_filter: Some(dcn10_program_hpd_filter),
};

pub unsafe fn dcn20_link_encoder_construct(enc20: *mut dcn20_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask) {
    let enc10 = &mut (*enc20).enc10;
    let mut bp_cap_info = core::mem::zeroed::<bp_encoder_cap_info>();
    let bp_funcs = (*(*(*init_data).ctx).dc_bios).funcs;
    let mut result = BP_RESULT_OK;
    enc10.base.funcs = &dcn20_link_enc_funcs; enc10.base.ctx = (*init_data).ctx; enc10.base.id = (*init_data).encoder;
    enc10.base.hpd_gpio = (*init_data).hpd_gpio; enc10.base.hpd_source = (*init_data).hpd_source; enc10.base.connector = (*init_data).connector;
    enc10.base.preferred_engine = ENGINE_ID_UNKNOWN; enc10.base.features = *enc_features; enc10.base.transmitter = (*init_data).transmitter;
    enc10.base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;
    enc10.link_regs = link_regs; enc10.aux_regs = aux_regs; enc10.hpd_regs = hpd_regs; enc10.link_shift = link_shift; enc10.link_mask = link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter { TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB, TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD, TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE, TRANSMITTER_UNIPHY_F => ENGINE_ID_DIGF, TRANSMITTER_UNIPHY_G => ENGINE_ID_DIGG, _ => { ASSERT_CRITICAL!(false); ENGINE_ID_UNKNOWN } };
    enc10.base.features.flags.bits.HDMI_6GB_EN = 1;
    result = ((*bp_funcs).get_encoder_cap_info.unwrap())((*enc10.base.ctx).dc_bios, enc10.base.id, &mut bp_cap_info);
    if result == BP_RESULT_OK { enc10.base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN; enc10.base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN; enc10.base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN; enc10.base.features.flags.bits.DP_IS_USB_C = bp_cap_info.DP_IS_USB_C; } else { DC_LOG_WARNING!("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", "dcn20_link_encoder_construct", result); }
    if (*enc10.base.ctx).dc.debug.hdmi20_disable { enc10.base.features.flags.bits.HDMI_6GB_EN = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
