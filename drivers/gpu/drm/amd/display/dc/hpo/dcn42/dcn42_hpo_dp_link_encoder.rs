// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies:
// dc_bios_types.h, dcn31/dcn31_hpo_dp_link_encoder.h,
// dcn32/dcn32_hpo_dp_link_encoder.h, dcn42_hpo_dp_link_encoder.h,
// reg_helper.h, and stream_encoder.h

unsafe fn dcn42_hpo_dp_link_enc_read_state(
    enc: *mut hpo_dp_link_encoder,
    state: *mut hpo_dp_link_enc_state,
) {
    let enc3 = DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc);

    ASSERT!(!state.is_null());

    REG_GET!(
        enc3,
        DP_DPHY_SYM32_STATUS,
        STATUS,
        &mut (*state).link_enc_enabled
    );
    REG_GET!(
        enc3,
        DP_DPHY_SYM32_CONTROL,
        NUM_LANES,
        &mut (*state).lane_count
    );
    REG_GET!(
        enc3,
        DP_DPHY_SYM32_CONTROL,
        MODE,
        &mut (*state).link_mode as *mut _ as *mut u32
    );

    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_SAT_VC0,
        SAT_STREAM_SOURCE,
        &mut (*state).stream_src[0],
        SAT_SLOT_COUNT,
        &mut (*state).slot_count[0]
    );
    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_SAT_VC1,
        SAT_STREAM_SOURCE,
        &mut (*state).stream_src[1],
        SAT_SLOT_COUNT,
        &mut (*state).slot_count[1]
    );
    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_SAT_VC2,
        SAT_STREAM_SOURCE,
        &mut (*state).stream_src[2],
        SAT_SLOT_COUNT,
        &mut (*state).slot_count[2]
    );

    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_VC_RATE_CNTL0,
        STREAM_VC_RATE_X,
        &mut (*state).vc_rate_x[0],
        STREAM_VC_RATE_Y,
        &mut (*state).vc_rate_y[0]
    );
    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_VC_RATE_CNTL1,
        STREAM_VC_RATE_X,
        &mut (*state).vc_rate_x[1],
        STREAM_VC_RATE_Y,
        &mut (*state).vc_rate_y[1]
    );
    REG_GET_2!(
        enc3,
        DP_DPHY_SYM32_VC_RATE_CNTL2,
        STREAM_VC_RATE_X,
        &mut (*state).vc_rate_x[2],
        STREAM_VC_RATE_Y,
        &mut (*state).vc_rate_y[2]
    );
}

static dcn42_hpo_dp_link_encoder_funcs: hpo_dp_link_encoder_funcs =
    hpo_dp_link_encoder_funcs {
        enable_link_phy: Some(dcn31_hpo_dp_link_enc_enable_dp_output),
        disable_link_phy: Some(dcn31_hpo_dp_link_enc_disable_output),
        link_enable: Some(dcn31_hpo_dp_link_enc_enable),
        link_disable: Some(dcn31_hpo_dp_link_enc_disable),
        set_link_test_pattern: Some(dcn31_hpo_dp_link_enc_set_link_test_pattern),
        update_stream_allocation_table: Some(
            dcn31_hpo_dp_link_enc_update_stream_allocation_table,
        ),
        set_throttled_vcp_size: Some(dcn31_hpo_dp_link_enc_set_throttled_vcp_size),
        is_in_alt_mode: Some(dcn32_hpo_dp_link_enc_is_in_alt_mode),
        read_state: Some(dcn42_hpo_dp_link_enc_read_state),
        set_ffe: Some(dcn31_hpo_dp_link_enc_set_ffe),
    };

unsafe fn hpo_dp_link_encoder42_construct(
    enc31: *mut dcn31_hpo_dp_link_encoder,
    ctx: *mut dc_context,
    inst: u32,
    hpo_le_regs: *const dcn31_hpo_dp_link_encoder_registers,
    hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
    hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
) {
    (*enc31).base.ctx = ctx;

    (*enc31).base.inst = inst;
    (*enc31).base.funcs = &dcn42_hpo_dp_link_encoder_funcs;
    (*enc31).base.hpd_source = HPD_SOURCEID_UNKNOWN;
    (*enc31).base.transmitter = TRANSMITTER_UNKNOWN;

    (*enc31).regs = hpo_le_regs;
    (*enc31).hpo_le_shift = hpo_le_shift;
    (*enc31).hpo_le_mask = hpo_le_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
