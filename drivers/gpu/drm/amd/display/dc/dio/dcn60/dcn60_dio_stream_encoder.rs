// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.

// C headers and symbols are supplied by the surrounding translation unit.

const VBI_LINE_0: u32 = 0;
const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: i32 = 340000;
const DP_SEC_AUD_N__DP_SEC_AUD_N__DEFAULT: u32 = 0x8000;
const DP_SEC_TIMESTAMP__DP_SEC_TIMESTAMP_MODE__AUTO_CALC: u32 = 1;

unsafe fn enc60_dp_set_odm_combine(enc: *mut stream_encoder, odm_combine: bool) {
    let _ = (enc, odm_combine);
}

unsafe fn enc60_stream_encoder_hdmi_set_stream_attribute(
    enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing,
    actual_pix_clk_khz: i32, enable_audio: bool,
) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        let mut cntl: bp_encoder_control = core::mem::zeroed();
        cntl.action = ENCODER_CONTROL_SETUP;
        cntl.engine_id = (*enc1).base.id;
        cntl.signal = SIGNAL_TYPE_HDMI_TYPE_A;
        cntl.enable_dp_audio = enable_audio;
        cntl.pixel_clock = actual_pix_clk_khz;
        cntl.lanes_number = LANE_COUNT_FOUR;
        if ((*(*(*enc1).base.bp).funcs).encoder_control)((*enc1).base.bp, &mut cntl) != BP_RESULT_OK { return; }
    } else {
        REG_UPDATE!(enc1, DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, 0x1F);
    }
    enc401_stream_encoder_set_stream_attribute_helper(enc1, crtc_timing);
    REG_UPDATE_4!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_ENABLE, 0,
        HDMI_DATA_SCRAMBLE_EN, 0, HDMI_NO_EXTRA_NULL_PACKET_FILLED, 1,
        HDMI_CLOCK_CHANNEL_RATE, 0);
    match (*crtc_timing).display_color_depth {
        COLOR_DEPTH_888 => REG_UPDATE!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 0),
        COLOR_DEPTH_101010 => if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 {
            REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 1, HDMI_DEEP_COLOR_ENABLE, 0)
        } else { REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 1, HDMI_DEEP_COLOR_ENABLE, 1) },
        COLOR_DEPTH_121212 => if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 {
            REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 2, HDMI_DEEP_COLOR_ENABLE, 0)
        } else { REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 2, HDMI_DEEP_COLOR_ENABLE, 1) },
        COLOR_DEPTH_161616 => REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_DEPTH, 3, HDMI_DEEP_COLOR_ENABLE, 1),
        _ => {}
    }
    if actual_pix_clk_khz >= HDMI_CLOCK_CHANNEL_RATE_MORE_340M {
        REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 1);
    } else if (*crtc_timing).flags.LTE_340MCSC_SCRAMBLE {
        REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 0);
    }
    REG_UPDATE_3!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_GC_CONT, 1, HDMI_GC_SEND, 1, HDMI_NULL_SEND, 1);
    REG_UPDATE!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_ACP_SEND, 0);
    REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL0, HDMI_AUDIO_INFO_SEND, 1);
    REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL0, HDMI_AUDIO_INFO_LINE, VBI_LINE_0 + 2);
    REG_UPDATE!(enc1, HDMI_GC, HDMI_GC_AVMUTE, 0);
}

unsafe fn enc60_stream_encoder_update_hdmi_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_UPDATE!(enc1, HDMI_DB_CONTROL, HDMI_DB_DISABLE, 1);
    REG_UPDATE!(enc1, DIG_FE_AUDIO_CNTL, APG_CLOCK_ENABLE, 1);
    enc3_update_hdmi_info_packet(enc1, 0, &(*info_frame).avi);
    enc3_update_hdmi_info_packet(enc1, 5, &(*info_frame).hfvsif);
    enc3_update_hdmi_info_packet(enc1, 2, &(*info_frame).gamut);
    enc3_update_hdmi_info_packet(enc1, 1, &(*info_frame).vendor);
    enc3_update_hdmi_info_packet(enc1, 3, &(*info_frame).spd);
    enc3_update_hdmi_info_packet(enc1, 4, &(*info_frame).hdrsmd);
    enc3_update_hdmi_info_packet(enc1, 6, &(*info_frame).vtem);
}

unsafe fn enc60_se_enable_audio_clock(enc: *mut stream_encoder, enable: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_UPDATE!(enc1, DIG_FE_AUDIO_CNTL, APG_CLOCK_ENABLE, enable);
}

unsafe fn enc60_se_setup_hdmi_audio(enc: *mut stream_encoder, crtc_info: *const audio_crtc_info) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    let mut audio_clock_info: audio_clock_info = core::mem::zeroed();
    REG_UPDATE!(enc1, HDMI_AUDIO_PACKET_CONTROL, HDMI_AUDIO_DELAY_EN, 1);
    REG_UPDATE_2!(enc1, HDMI_ACR_PACKET_CONTROL, HDMI_ACR_AUTO_SEND, 1, HDMI_ACR_SOURCE, 0);
    get_audio_clock_info((*crtc_info).color_depth, (*crtc_info).requested_pixel_clock_100Hz,
        (*crtc_info).calculated_pixel_clock_100Hz, &mut audio_clock_info);
    REG_UPDATE!(enc1, HDMI_ACR_32_0, HDMI_ACR_CTS_32, audio_clock_info.cts_32khz);
    REG_UPDATE!(enc1, HDMI_ACR_32_1, HDMI_ACR_N_32, audio_clock_info.n_32khz);
    REG_UPDATE!(enc1, HDMI_ACR_44_0, HDMI_ACR_CTS_44, audio_clock_info.cts_44khz);
    REG_UPDATE!(enc1, HDMI_ACR_44_1, HDMI_ACR_N_44, audio_clock_info.n_44khz);
    REG_UPDATE!(enc1, HDMI_ACR_48_0, HDMI_ACR_CTS_48, audio_clock_info.cts_48khz);
    REG_UPDATE!(enc1, HDMI_ACR_48_1, HDMI_ACR_N_48, audio_clock_info.n_48khz);
}

unsafe fn enc60_se_hdmi_audio_setup(enc: *mut stream_encoder, az_inst: u32, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    enc60_se_enable_audio_clock(enc, true);
    enc60_se_setup_hdmi_audio(enc, audio_crtc_info);
    REG_UPDATE!(enc1, DIG_FE_AUDIO_CNTL, DIG_FE_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, az_inst);
    ASSERT!((*enc).apg);
    ((*(*(*enc).apg).funcs).se_audio_setup)((*enc).apg, az_inst, info);
}

unsafe fn enc60_se_hdmi_audio_disable(enc: *mut stream_encoder) {
    ASSERT!((*enc).apg);
    if !(*enc).apg.is_null() && (*(*enc).apg).funcs.disable_apg.is_some() { ((*(*enc).apg).funcs.disable_apg.unwrap())((*enc).apg); }
    enc60_se_enable_audio_clock(enc, false);
}

unsafe fn enc60_stream_encoder_stop_dp_info_packets(enc: *mut stream_encoder) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_SET_9!(enc1, DP_SEC_CNTL, 0, DP_SEC_GSP0_ENABLE, 0, DP_SEC_GSP1_ENABLE, 0, DP_SEC_GSP2_ENABLE, 0, DP_SEC_GSP3_ENABLE, 0, DP_SEC_GSP4_ENABLE, 0, DP_SEC_GSP5_ENABLE, 0, DP_SEC_GSP6_ENABLE, 0, DP_SEC_GSP7_ENABLE, 0, DP_SEC_STREAM_ENABLE, 0);
    let value = REG_READ!(enc1, DP_SEC_CNTL);
    if value != 0 { REG_UPDATE!(enc1, DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, 1); }
}

unsafe fn enc60_se_setup_dp_audio(enc: *mut stream_encoder) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_SET!(enc1, DP_SEC_AUD_N, 0, DP_SEC_AUD_N, DP_SEC_AUD_N__DP_SEC_AUD_N__DEFAULT);
    REG_SET!(enc1, DP_SEC_TIMESTAMP, 0, DP_SEC_TIMESTAMP_MODE, DP_SEC_TIMESTAMP__DP_SEC_TIMESTAMP_MODE__AUTO_CALC);
}

unsafe fn enc60_se_dp_audio_enable(enc: *mut stream_encoder) { enc60_se_enable_audio_clock(enc, true); enc60_se_setup_dp_audio(enc); enc1_se_enable_dp_audio(enc); ((*(*(*enc).apg).funcs).enable_apg)((*enc).apg); }

unsafe fn enc60_se_disable_dp_audio(enc: *mut stream_encoder) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_UPDATE_5!(enc1, DP_SEC_CNTL, DP_SEC_ASP_ENABLE, 0, DP_SEC_ATP_ENABLE, 0, DP_SEC_AIP_ENABLE, 0, DP_SEC_ACM_ENABLE, 0, DP_SEC_STREAM_ENABLE, 0);
    let value = REG_READ!(enc1, DP_SEC_CNTL); if value != 0 { REG_UPDATE!(enc1, DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, 1); }
}
unsafe fn enc60_se_dp_audio_disable(enc: *mut stream_encoder) { enc60_se_disable_dp_audio(enc); ((*(*(*enc).apg).funcs).disable_apg)((*enc).apg); enc60_se_enable_audio_clock(enc, false); }
unsafe fn enc60_audio_mute_control(enc: *mut stream_encoder, mute: bool) { ASSERT!((*enc).apg); if mute { ((*(*(*enc).apg).funcs).disable_apg)((*enc).apg); } else { ((*(*(*enc).apg).funcs).enable_apg)((*enc).apg); } }
unsafe fn enc60_se_dp_audio_setup(enc: *mut stream_encoder, az_inst: u32, info: *mut audio_info) { let enc1 = DCN10STRENC_FROM_STRENC(enc); enc60_se_enable_audio_clock(enc, true); REG_UPDATE!(enc1, DIG_FE_AUDIO_CNTL, DIG_FE_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, az_inst); ASSERT!((*enc).apg); ((*(*(*enc).apg).funcs).se_audio_setup)((*enc).apg, az_inst, info); }

unsafe fn enc60_dp_set_dsc_pps_info_packet(enc: *mut stream_encoder, enable: bool, dsc_packed_pps: *mut u8, immediate_update: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if enable {
        let mut pps_sdp: dc_info_packet = core::mem::zeroed();
        REG_UPDATE!(enc1, DP_SEC_CNTL2, DP_SEC_GSP11_PPS, 1);
        pps_sdp.valid = true; pps_sdp.hb0 = 0; pps_sdp.hb1 = DC_DP_INFOFRAME_TYPE_PPS; pps_sdp.hb2 = 127; pps_sdp.hb3 = 0;
        for i in 0..4 { core::ptr::copy_nonoverlapping(dsc_packed_pps.add(i * 32), pps_sdp.sb.as_mut_ptr(), 32); ((*(*(*enc1).base.vpg).funcs).update_generic_info_packet)((*enc1).base.vpg, 11 + i, &mut pps_sdp, immediate_update); }
        REG_UPDATE!(enc1, DP_GSP11_CNTL, DP_SEC_GSP11_LINE_NUM, 2);
        REG_UPDATE_2!(enc1, DP_MSA_VBID_MISC, DP_VBID6_LINE_REFERENCE, 0, DP_VBID6_LINE_NUM, 3);
        REG_UPDATE!(enc1, DP_GSP11_CNTL, DP_SEC_GSP11_ENABLE, 1); REG_UPDATE!(enc1, DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, 1);
    } else { REG_UPDATE!(enc1, DP_GSP11_CNTL, DP_SEC_GSP11_ENABLE, 0); REG_UPDATE!(enc1, DP_SEC_CNTL2, DP_SEC_GSP11_PPS, 0); }
}

unsafe fn enc60_reset_hdmi_stream_attribute(enc: *mut stream_encoder) { let enc1 = DCN10STRENC_FROM_STRENC(enc); REG_UPDATE_3!(enc1, HDMI_CONTROL, HDMI_DEEP_COLOR_ENABLE, 0, HDMI_DATA_SCRAMBLE_EN, 0, HDMI_CLOCK_CHANNEL_RATE, 0); }

// Function table mirrors dcn60_str_enc_funcs; external function symbols are supplied by dependencies.
static mut dcn60_str_enc_funcs: stream_encoder_funcs = stream_encoder_funcs {
    dp_set_odm_combine: Some(enc60_dp_set_odm_combine), dp_set_stream_attribute: Some(enc401_stream_encoder_dp_set_stream_attribute), hdmi_set_stream_attribute: Some(enc60_stream_encoder_hdmi_set_stream_attribute), dvi_set_stream_attribute: Some(enc401_stream_encoder_dvi_set_stream_attribute), set_throttled_vcp_size: Some(enc1_stream_encoder_set_throttled_vcp_size), update_hdmi_info_packets: Some(enc60_stream_encoder_update_hdmi_info_packets), stop_hdmi_info_packets: Some(enc3_stream_encoder_stop_hdmi_info_packets), update_dp_info_packets_sdp_line_num: Some(enc3_stream_encoder_update_dp_info_packets_sdp_line_num), update_dp_info_packets: Some(enc3_stream_encoder_update_dp_info_packets), stop_dp_info_packets: Some(enc60_stream_encoder_stop_dp_info_packets), dp_blank: Some(enc1_stream_encoder_dp_blank), dp_unblank: Some(enc401_stream_encoder_dp_unblank), audio_mute_control: Some(enc60_audio_mute_control), dp_audio_setup: Some(enc60_se_dp_audio_setup), dp_audio_enable: Some(enc60_se_dp_audio_enable), dp_audio_disable: Some(enc60_se_dp_audio_disable), hdmi_audio_setup: Some(enc60_se_hdmi_audio_setup), hdmi_audio_disable: Some(enc60_se_hdmi_audio_disable), setup_stereo_sync: Some(enc1_setup_stereo_sync), set_avmute: Some(enc1_stream_encoder_set_avmute), dig_connect_to_otg: Some(enc1_dig_connect_to_otg), dig_source_otg: Some(enc1_dig_source_otg), dp_get_pixel_format: Some(enc1_stream_encoder_dp_get_pixel_format), enc_read_state: Some(enc401_read_state), dp_set_dsc_config: None, dp_set_dsc_pps_info_packet: Some(enc60_dp_set_dsc_pps_info_packet), set_dynamic_metadata: Some(enc401_set_dynamic_metadata), hdmi_reset_stream_attribute: Some(enc60_reset_hdmi_stream_attribute), enable_stream: Some(enc401_stream_encoder_enable), set_input_mode: Some(enc401_set_dig_input_mode), enable_fifo: Some(enc35_enable_fifo), disable_fifo: Some(enc35_disable_fifo), map_stream_to_link: Some(enc401_stream_encoder_map_to_link),
};

pub unsafe fn dcn60_dio_stream_encoder_construct(enc1: *mut dcn10_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, vpg: *mut vpg, apg: *mut apg, regs: *const dcn10_stream_enc_registers, se_shift: *const dcn10_stream_encoder_shift, se_mask: *const dcn10_stream_encoder_mask) {
    (*enc1).base.funcs = &mut dcn60_str_enc_funcs; (*enc1).base.ctx = ctx; (*enc1).base.id = eng_id; (*enc1).base.bp = bp; (*enc1).base.vpg = vpg; (*enc1).base.apg = apg; (*enc1).regs = regs; (*enc1).se_shift = se_shift; (*enc1).se_mask = se_mask; (*enc1).base.stream_enc_inst = (*vpg).inst;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
