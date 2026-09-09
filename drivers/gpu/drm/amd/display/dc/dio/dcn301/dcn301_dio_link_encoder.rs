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
 */

// Dependencies supplied by the surrounding display driver.

static dcn301_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: link_enc2_read_state,
    validate_output_with_stream: dcn10_link_encoder_validate_output_with_stream,
    hw_init: enc3_hw_init,
    setup: dcn10_link_encoder_setup,
    enable_tmds_output: dcn10_link_encoder_enable_tmds_output,
    enable_dp_output: dcn20_link_encoder_enable_dp_output,
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
    is_dig_enabled: dcn10_is_dig_enabled,
    destroy: dcn10_link_encoder_destroy,
    fec_set_enable: enc2_fec_set_enable,
    fec_set_ready: enc2_fec_set_ready,
    fec_is_active: enc2_fec_is_active,
    get_dig_frontend: dcn10_get_dig_frontend,
    get_dig_mode: dcn10_get_dig_mode,
    is_in_alt_mode: dcn20_link_encoder_is_in_alt_mode,
    get_max_link_cap: dcn20_link_encoder_get_max_link_cap,
    dpcstx_set_order_invert_18_bit: None,
    set_phy_source: None,
    dpcs_initialize_phy: None,
    dpcs_configure_phypll: None,
    dpcs_configure_dpcs: None,
    dpcs_enable_dpcs: None,
    prog_eq_setting: None,
    get_hpd_state: dcn10_get_hpd_state,
    program_hpd_filter: dcn10_program_hpd_filter,
};

pub unsafe fn dcn301_link_encoder_construct(
    enc20: *mut dcn20_link_encoder,
    init_data: *const encoder_init_data,
    enc_features: *const encoder_feature_support,
    link_regs: *const dcn10_link_enc_registers,
    aux_regs: *const dcn10_link_enc_aux_registers,
    hpd_regs: *const dcn10_link_enc_hpd_registers,
    link_shift: *const dcn10_link_enc_shift,
    link_mask: *const dcn10_link_enc_mask,
) {
    let mut bp_cap_info: bp_encoder_cap_info = core::mem::zeroed();
    let bp_funcs = (*(*init_data).ctx).dc_bios.funcs;
    let mut result: bp_result = BP_RESULT_OK;
    let enc10: *mut dcn10_link_encoder = &mut (*enc20).enc10;

    (*enc10).base.funcs = &dcn301_link_enc_funcs;
    (*enc10).base.ctx = (*init_data).ctx;
    (*enc10).base.id = (*init_data).encoder;
    (*enc10).base.hpd_gpio = (*init_data).hpd_gpio;
    (*enc10).base.hpd_source = (*init_data).hpd_source;
    (*enc10).base.connector = (*init_data).connector;
    (*enc10).base.preferred_engine = ENGINE_ID_UNKNOWN;
    (*enc10).base.features = *enc_features;
    (*enc10).base.transmitter = (*init_data).transmitter;

    /* The DP sink-detect data-pin polling feature is intentionally disabled. */
    (*enc10).base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK |
        SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS |
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST |
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A;

    (*enc10).link_regs = link_regs;
    (*enc10).aux_regs = aux_regs;
    (*enc10).hpd_regs = hpd_regs;
    (*enc10).link_shift = link_shift;
    (*enc10).link_mask = link_mask;

    (*enc10).base.preferred_engine = match (*enc10).base.transmitter {
        TRANSMITTER_UNIPHY_A => ENGINE_ID_DIGA,
        TRANSMITTER_UNIPHY_B => ENGINE_ID_DIGB,
        TRANSMITTER_UNIPHY_C => ENGINE_ID_DIGC,
        TRANSMITTER_UNIPHY_D => ENGINE_ID_DIGD,
        TRANSMITTER_UNIPHY_E => ENGINE_ID_DIGE,
        TRANSMITTER_UNIPHY_F => ENGINE_ID_DIGF,
        TRANSMITTER_UNIPHY_G => ENGINE_ID_DIGG,
        _ => { ASSERT_CRITICAL(false); ENGINE_ID_UNKNOWN },
    };

    (*enc10).base.features.flags.bits.HDMI_6GB_EN = 1;
    result = ((*bp_funcs).get_encoder_cap_info)((*enc10).base.ctx.dc_bios,
        (*enc10).base.id, &mut bp_cap_info);

    if result == BP_RESULT_OK {
        (*enc10).base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN;
        (*enc10).base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN;
        (*enc10).base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN;
        (*enc10).base.features.flags.bits.DP_IS_USB_C = bp_cap_info.DP_IS_USB_C;
        (*enc10).base.features.flags.bits.IS_HDMI_FRL_CAPABLE = bp_cap_info.IS_HDMI_FRL_CAPABLE;
        (*enc10).base.features.flags.bits.IS_FRL_8G_CAPABLE = bp_cap_info.FRL_8G_EN;
        (*enc10).base.features.flags.bits.IS_FRL_10G_CAPABLE = bp_cap_info.FRL_10G_EN;
        (*enc10).base.features.flags.bits.IS_FRL_12G_CAPABLE = bp_cap_info.FRL_12G_EN;
        (*enc10).base.txffe_state = 0;
    } else {
        DC_LOG_WARNING("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", __func__, result);
    }
    if (*enc10).base.ctx.dc.debug.hdmi20_disable {
        (*enc10).base.features.flags.bits.HDMI_6GB_EN = 0;
    }
    if (*enc10).base.ctx.dc.config.force_hdmi21_frl_enc_enable {
        (*enc10).base.features.flags.bits.IS_HDMI_FRL_CAPABLE = 1;
        (*enc10).base.features.flags.bits.IS_FRL_8G_CAPABLE = 1;
        (*enc10).base.features.flags.bits.IS_FRL_10G_CAPABLE = 1;
        (*enc10).base.features.flags.bits.IS_FRL_12G_CAPABLE = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
