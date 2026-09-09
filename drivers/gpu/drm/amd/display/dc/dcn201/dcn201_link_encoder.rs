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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies are supplied by the surrounding driver translation.

unsafe fn dcn201_link_encoder_get_max_link_cap(
    enc: *mut link_encoder,
    link_settings: *mut dc_link_settings,
) {
    let mut value1: u32 = 0;
    let mut value2: u32 = 0;
    let enc10: *mut dcn10_link_encoder = TO_DCN10_LINK_ENC(enc);

    dcn10_link_encoder_get_max_link_cap(enc, link_settings);
    REG_GET_2!(
        (*(*enc10).link_regs).RDPCSTX_PHY_CNTL2,
        (*(*enc10).link_shift).RDPCS_PHY_DPALT_DISABLE,
        &mut value1,
        (*(*enc10).link_shift).RDPCS_PHY_DPALT_DP4,
        &mut value2
    );
    /*limit to combo_phy*/
    if (*enc).usbc_combo_phy {
        if value1 == 0 && value2 == 0 && (*link_settings).lane_count > LANE_COUNT_TWO {
            (*link_settings).lane_count = LANE_COUNT_TWO;
        }
    }
}

unsafe fn dcn201_link_encoder_is_in_alt_mode(enc: *mut link_encoder) -> bool {
    let mut value: u32 = 0;
    let enc10: *mut dcn10_link_encoder = TO_DCN10_LINK_ENC(enc);

    REG_GET!(
        (*(*enc10).link_regs).RDPCSTX_PHY_CNTL2,
        (*(*enc10).link_shift).RDPCS_PHY_DPALT_DISABLE,
        &mut value
    );

    // if value == 1 alt mode is disabled, otherwise it is enabled
    value == 0
}

static dcn201_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: link_enc2_read_state,
    validate_output_with_stream: dcn10_link_encoder_validate_output_with_stream,
    hw_init: enc2_hw_init,
    setup: dcn10_link_encoder_setup,
    enable_tmds_output: dcn10_link_encoder_enable_tmds_output,
    enable_dp_output: dcn10_link_encoder_enable_dp_output,
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
    get_dig_frontend: dcn10_get_dig_frontend,
    fec_is_active: enc2_fec_is_active,
    is_in_alt_mode: dcn201_link_encoder_is_in_alt_mode,
    get_max_link_cap: dcn201_link_encoder_get_max_link_cap,
    get_hpd_state: dcn10_get_hpd_state,
    program_hpd_filter: dcn10_program_hpd_filter,
};

pub unsafe fn dcn201_link_encoder_construct(
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
    let bp_funcs = (*(*(*init_data).ctx).dc_bios).funcs;
    let mut result: bp_result = BP_RESULT_OK;
    let enc10: *mut dcn10_link_encoder = &mut (*enc20).enc10;

    (*enc10).base.funcs = &dcn201_link_enc_funcs;
    (*enc10).base.ctx = (*init_data).ctx;
    (*enc10).base.id = (*init_data).encoder;
    (*enc10).base.hpd_gpio = (*init_data).hpd_gpio;
    (*enc10).base.hpd_source = (*init_data).hpd_source;
    (*enc10).base.connector = (*init_data).connector;
    (*enc10).base.preferred_engine = ENGINE_ID_UNKNOWN;
    (*enc10).base.features = *enc_features;
    (*enc10).base.transmitter = (*init_data).transmitter;

    /* The DP sink detect data-pin polling feature is intentionally left at
     * its default, matching the commented-out C implementation. */

    (*enc10).base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK
        | SIGNAL_TYPE_DVI_DUAL_LINK
        | SIGNAL_TYPE_LVDS
        | SIGNAL_TYPE_DISPLAY_PORT
        | SIGNAL_TYPE_DISPLAY_PORT_MST
        | SIGNAL_TYPE_EDP
        | SIGNAL_TYPE_HDMI_TYPE_A;

    (*enc10).link_regs = link_regs;
    (*enc10).aux_regs = aux_regs;
    (*enc10).hpd_regs = hpd_regs;
    (*enc10).link_shift = link_shift;
    (*enc10).link_mask = link_mask;

    match (*enc10).base.transmitter {
        TRANSMITTER_UNIPHY_A => (*enc10).base.preferred_engine = ENGINE_ID_DIGA,
        TRANSMITTER_UNIPHY_B => (*enc10).base.preferred_engine = ENGINE_ID_DIGB,
        _ => {
            ASSERT_CRITICAL!(false);
            (*enc10).base.preferred_engine = ENGINE_ID_UNKNOWN;
        }
    }

    /* default to one to mirror Windows behavior */
    (*enc10).base.features.flags.bits.HDMI_6GB_EN = 1;

    result = ((*bp_funcs).get_encoder_cap_info)(
        (*(*enc10).base.ctx).dc_bios,
        (*enc10).base.id,
        &mut bp_cap_info,
    );

    /* Override features with DCE-specific values */
    if result == BP_RESULT_OK {
        (*enc10).base.features.flags.bits.IS_HBR2_CAPABLE = bp_cap_info.DP_HBR2_EN;
        (*enc10).base.features.flags.bits.IS_HBR3_CAPABLE = bp_cap_info.DP_HBR3_EN;
        (*enc10).base.features.flags.bits.HDMI_6GB_EN = bp_cap_info.HDMI_6GB_EN;
        (*enc10).base.features.flags.bits.DP_IS_USB_C = bp_cap_info.DP_IS_USB_C;
    } else {
        DC_LOG_WARNING!(
            "%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n",
            "dcn201_link_encoder_construct",
            result
        );
    }
    if (*(*(*enc10).base.ctx).dc).debug.hdmi20_disable {
        (*enc10).base.features.flags.bits.HDMI_6GB_EN = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
