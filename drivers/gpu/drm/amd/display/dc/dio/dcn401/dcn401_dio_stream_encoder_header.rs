/*
 * Copyright 2021 - Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependencies supplied by the surrounding translation unit:
// dcn30_vpg, dcn30_afmt, stream_encoder, and dcn20_stream_encoder.

// C preprocessor macro translated as a Rust macro.  The SE_SF! operation and
// register-field identifiers are supplied by the dependent register headers.
// The complete field list is retained in the invocation form below; callers
// may provide the corresponding SE_SF implementation through dependencies.
#[allow(unused_macros)]
macro_rules! SE_COMMON_MASK_SH_LIST_DCN401 {
    ($mask_sh:expr) => {
        SE_SF!(DP0_DP_PIXEL_FORMAT, PIXEL_ENCODING_TYPE, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, UNCOMPRESSED_PIXEL_FORMAT, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, UNCOMPRESSED_COMPONENT_DEPTH, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, COMPRESSED_PIXEL_FORMAT, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_PACKET_GEN_VERSION, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_KEEPOUT_MODE, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_DEEP_COLOR_ENABLE, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_NO_EXTRA_NULL_PACKET_FILLED, $mask_sh),
        SE_SF!(DIG0_HDMI_VBI_PACKET_CONTROL, HDMI_GC_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_VBI_PACKET_CONTROL, HDMI_GC_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_VBI_PACKET_CONTROL, HDMI_NULL_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_VBI_PACKET_CONTROL, HDMI_ACP_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_INFOFRAME_CONTROL0, HDMI_AUDIO_INFO_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_INFOFRAME_CONTROL1, HDMI_AUDIO_INFO_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GC, HDMI_GC_AVMUTE, $mask_sh),
        SE_SF!(DP0_DP_MSE_RATE_CNTL, DP_MSE_RATE_X, $mask_sh),
        SE_SF!(DP0_DP_MSE_RATE_CNTL, DP_MSE_RATE_Y, $mask_sh),
        SE_SF!(DP0_DP_MSE_RATE_UPDATE, DP_MSE_RATE_UPDATE_PENDING, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP0_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP1_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP2_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP3_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_MPG_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL1, DP_SEC_GSP5_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL2, DP_SEC_GSP4_SEND, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL2, DP_SEC_GSP4_SEND_PENDING, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL4, DP_SEC_GSP4_LINE_NUM, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL5, DP_SEC_GSP5_LINE_NUM, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL2, DP_SEC_GSP4_SEND_ANY_LINE, $mask_sh),
        SE_SF!(DP0_DP_VID_STREAM_CNTL, DP_VID_STREAM_DIS_DEFER, $mask_sh),
        SE_SF!(DP0_DP_VID_STREAM_CNTL, DP_VID_STREAM_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_VID_STREAM_CNTL, DP_VID_STREAM_STATUS, $mask_sh),
        SE_SF!(DP0_DP_STEER_FIFO, DP_STEER_FIFO_RESET, $mask_sh),
        SE_SF!(DP0_DP_STEER_FIFO, DP_STEER_FIFO_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_VID_TIMING, DP_VID_M_N_GEN_EN, $mask_sh),
        SE_SF!(DP0_DP_VID_N, DP_VID_N, $mask_sh),
        SE_SF!(DP0_DP_VID_M, DP_VID_M, $mask_sh),
        SE_SF!(DIG0_HDMI_AUDIO_PACKET_CONTROL, HDMI_AUDIO_DELAY_EN, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_PACKET_CONTROL, HDMI_ACR_AUTO_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_PACKET_CONTROL, HDMI_ACR_SOURCE, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_PACKET_CONTROL, HDMI_ACR_AUDIO_PRIORITY, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_32_0, HDMI_ACR_CTS_32, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_32_1, HDMI_ACR_N_32, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_44_0, HDMI_ACR_CTS_44, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_44_1, HDMI_ACR_N_44, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_48_0, HDMI_ACR_CTS_48, $mask_sh),
        SE_SF!(DIG0_HDMI_ACR_48_1, HDMI_ACR_N_48, $mask_sh),
        SE_SF!(DP0_DP_SEC_AUD_N, DP_SEC_AUD_N, $mask_sh),
        SE_SF!(DP0_DP_SEC_TIMESTAMP, DP_SEC_TIMESTAMP_MODE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_ASP_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_ATP_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_AIP_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_ACM_ENABLE, $mask_sh),
        SE_SF!(DIG0_AFMT_CNTL, AFMT_AUDIO_CLOCK_EN, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, HDMI_CLOCK_CHANNEL_RATE, $mask_sh),
        SE_SF!(DIG1_HDMI_CONTROL, TMDS_PIXEL_ENCODING, $mask_sh),
        SE_SF!(DIG1_HDMI_CONTROL, TMDS_COLOR_FORMAT, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DIG_STEREOSYNC_SELECT, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DIG_STEREOSYNC_GATE_EN, $mask_sh)
    };
}

extern "C" {
    pub fn dcn401_dio_stream_encoder_construct(
        enc1: *mut dcn10_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios,
        eng_id: engine_id, vpg: *mut vpg, afmt: *mut afmt,
        regs: *const dcn10_stream_enc_registers,
        se_shift: *const dcn10_stream_encoder_shift,
        se_mask: *const dcn10_stream_encoder_mask,
    );
    pub fn enc401_set_dynamic_metadata(enc: *mut stream_encoder, enable_dme: bool,
        hubp_requestor_id: u32, dmdata_mode: dynamic_metadata_mode);
    pub fn enc401_stream_encoder_set_stream_attribute_helper(enc1: *mut dcn10_stream_encoder,
        crtc_timing: *mut dc_crtc_timing);
    pub fn enc401_stream_encoder_dp_set_stream_attribute(enc: *mut stream_encoder,
        crtc_timing: *mut dc_crtc_timing, output_color_space: dc_color_space,
        use_vsc_sdp_for_colorimetry: bool, enable_sdp_splitting: u32);
    pub fn enc401_stream_encoder_dvi_set_stream_attribute(enc: *mut stream_encoder,
        crtc_timing: *mut dc_crtc_timing, is_dual_link: bool);
    pub fn enc401_stream_encoder_dp_unblank(link: *mut dc_link, enc: *mut stream_encoder,
        param: *const encoder_unblank_param);
    pub fn enc401_stream_encoder_enable(enc: *mut stream_encoder, signal: signal_type, enable: bool);
    pub fn enc401_set_dig_input_mode(enc: *mut stream_encoder, pix_per_container: c_uint);
    pub fn enc401_stream_encoder_map_to_link(enc: *mut stream_encoder,
        stream_enc_inst: u32, link_enc_inst: u32);
    pub fn enc401_read_state(enc: *mut stream_encoder, s: *mut enc_state);
    pub fn enc401_stream_encoder_hdmi_set_stream_attribute(enc: *mut stream_encoder,
        crtc_timing: *mut dc_crtc_timing, actual_pix_clk_khz: c_int, enable_audio: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
