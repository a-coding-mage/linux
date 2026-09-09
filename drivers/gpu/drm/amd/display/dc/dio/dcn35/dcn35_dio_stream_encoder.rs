/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 */

// External declarations corresponding to the C headers are supplied by the
// surrounding translation unit.

const VBI_LINE_0: u32 = 0;
const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: u32 = 340000;

unsafe fn enc35_stream_encoder_dvi_set_stream_attribute(
    enc: *mut stream_encoder,
    crtc_timing: *mut dc_crtc_timing,
    is_dual_link: bool,
) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        let mut cntl: bp_encoder_control = core::mem::zeroed();
        cntl.action = ENCODER_CONTROL_SETUP;
        cntl.engine_id = (*enc1).base.id;
        cntl.signal = if is_dual_link { SIGNAL_TYPE_DVI_DUAL_LINK } else { SIGNAL_TYPE_DVI_SINGLE_LINK };
        cntl.enable_dp_audio = false;
        cntl.pixel_clock = (*crtc_timing).pix_clk_100hz / 10;
        cntl.lanes_number = if is_dual_link { LANE_COUNT_EIGHT } else { LANE_COUNT_FOUR };
        if (*(*(*enc1).base.bp).funcs).encoder_control((*enc1).base.bp, &mut cntl) != BP_RESULT_OK { return; }
    } else {
        REG_UPDATE!(enc1, DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, 0x1F);
    }
    ASSERT!((*crtc_timing).pixel_encoding == PIXEL_ENCODING_RGB);
    ASSERT!((*crtc_timing).display_color_depth == COLOR_DEPTH_888);
    enc1_stream_encoder_set_stream_attribute_helper(enc1, crtc_timing);
}

unsafe fn enc35_stream_encoder_hdmi_set_stream_attribute(
    enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing,
    actual_pix_clk_khz: i32, enable_audio: bool,
) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if !(*(*enc).ctx).dc.debug.avoid_vbios_exec_table {
        let mut cntl: bp_encoder_control = core::mem::zeroed();
        cntl.action = ENCODER_CONTROL_SETUP; cntl.engine_id = (*enc1).base.id;
        cntl.signal = SIGNAL_TYPE_HDMI_TYPE_A; cntl.enable_dp_audio = enable_audio;
        cntl.pixel_clock = actual_pix_clk_khz; cntl.lanes_number = LANE_COUNT_FOUR;
        if (*(*(*enc1).base.bp).funcs).encoder_control((*enc1).base.bp, &mut cntl) != BP_RESULT_OK { return; }
    } else {
        REG_UPDATE!(enc1, DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, 0x1F);
        enc314_enable_fifo(enc);
    }
    enc1_stream_encoder_set_stream_attribute_helper(enc1, crtc_timing);
    REG_UPDATE_6!(enc1, HDMI_CONTROL, HDMI_PACKET_GEN_VERSION, 1, HDMI_KEEPOUT_MODE, 1,
        HDMI_DEEP_COLOR_ENABLE, 0, HDMI_DATA_SCRAMBLE_EN, 0,
        HDMI_NO_EXTRA_NULL_PACKET_FILLED, 1, HDMI_CLOCK_CHANNEL_RATE, 0);
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
    if actual_pix_clk_khz >= HDMI_CLOCK_CHANNEL_RATE_MORE_340M as i32 {
        REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 1);
    } else if (*crtc_timing).flags.LTE_340MCSC_SCRAMBLE {
        REG_UPDATE_2!(enc1, HDMI_CONTROL, HDMI_DATA_SCRAMBLE_EN, 1, HDMI_CLOCK_CHANNEL_RATE, 0);
    }
    REG_UPDATE_3!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_GC_CONT, 1, HDMI_GC_SEND, 1, HDMI_NULL_SEND, 1);
    REG_UPDATE!(enc1, HDMI_VBI_PACKET_CONTROL, HDMI_ACP_SEND, 0);
    REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL0, HDMI_AUDIO_INFO_SEND, 1);
    ASSERT!(!(*enc).afmt.is_null());
    ((*(*(*enc).afmt).funcs).audio_info_immediate_update)((*enc).afmt);
    REG_UPDATE!(enc1, HDMI_INFOFRAME_CONTROL1, HDMI_AUDIO_INFO_LINE, VBI_LINE_0 + 2);
    REG_UPDATE!(enc1, HDMI_GC, HDMI_GC_AVMUTE, 0);
    REG_UPDATE!(enc1, HDMI_CONTROL, TMDS_PIXEL_ENCODING, if (*crtc_timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 { 1 } else { 0 });
    REG_UPDATE!(enc1, HDMI_CONTROL, TMDS_COLOR_FORMAT, 0);
}

unsafe fn enc35_stream_encoder_enable(enc: *mut stream_encoder, signal: signal_type, enable: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if enable { match signal {
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => REG_UPDATE!(enc1, DIG_FE_CLK_CNTL, DIG_FE_MODE, 2),
        SIGNAL_TYPE_HDMI_TYPE_A => REG_UPDATE!(enc1, DIG_FE_CLK_CNTL, DIG_FE_MODE, 3),
        SIGNAL_TYPE_DISPLAY_PORT_MST => REG_UPDATE!(enc1, DIG_FE_CLK_CNTL, DIG_FE_MODE, 5),
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_VIRTUAL => REG_UPDATE!(enc1, DIG_FE_CLK_CNTL, DIG_FE_MODE, 0),
        _ => ASSERT_CRITICAL!(false),
    }}
}

unsafe fn is_two_pixels_per_containter(timing: *const dc_crtc_timing) -> bool {
    (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420 ||
        ((*timing).flags.DSC && (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 && !(*timing).dsc_cfg.ycbcr422_simple)
}

unsafe fn enc35_stream_encoder_dp_unblank(link: *mut dc_link, enc: *mut stream_encoder, param: *const encoder_unblank_param) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if (*param).link_settings.link_rate != LINK_RATE_UNKNOWN {
        let n_vid: u32 = 0x8000; let mut n_multiply = 0; let mut pix_per_cycle = 0; let mut m_vid_l: u64 = n_vid as u64;
        if is_two_pixels_per_containter(&(*param).timing) || (*param).opp_cnt > 1 || (*param).pix_per_cycle > 1 { n_multiply = 1; pix_per_cycle = 1; }
        m_vid_l = m_vid_l.wrapping_mul(((*param).timing.pix_clk_100hz / 10) as u64);
        m_vid_l = div_u64(m_vid_l, (*param).link_settings.link_rate as u64 * LINK_RATE_REF_FREQ_IN_KHZ as u64);
        REG_UPDATE!(enc1, DP_VID_TIMING, DP_VID_M_N_GEN_EN, 0); REG_UPDATE!(enc1, DP_VID_N, DP_VID_N, n_vid);
        REG_UPDATE!(enc1, DP_VID_M, DP_VID_M, m_vid_l as u32);
        REG_UPDATE_2!(enc1, DP_VID_TIMING, DP_VID_M_N_GEN_EN, 1, DP_VID_N_MUL, n_multiply);
        REG_UPDATE!(enc1, DP_PIXEL_FORMAT, DP_PIXEL_PER_CYCLE_PROCESSING_MODE, pix_per_cycle);
    }
    REG_UPDATE!(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_ENABLE, false); REG_WAIT!(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_STATUS, 0, 10, 5000);
    REG_UPDATE!(enc1, DP_STEER_FIFO, DP_STEER_FIFO_RESET, 1); udelay(10); REG_UPDATE!(enc1, DP_STEER_FIFO, DP_STEER_FIFO_RESET, 0); udelay(100);
    REG_UPDATE!(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_ENABLE, true); enc314_enable_fifo(enc);
    (*(*(*link).dc).link_srv).dp_trace_source_sequence(link, DPCD_SOURCE_SEQ_AFTER_ENABLE_DP_VID_STREAM);
}

unsafe fn enc35_stream_encoder_map_to_link(enc: *mut stream_encoder, stream_enc_inst: u32, link_enc_inst: u32) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc); ASSERT!(stream_enc_inst < 5 && link_enc_inst < 5);
    REG_UPDATE!(enc1, STREAM_MAPPER_CONTROL, DIG_STREAM_LINK_TARGET, link_enc_inst);
}

unsafe fn enc35_reset_fifo(enc: *mut stream_encoder, reset: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc); let reset_val = if reset { 1 } else { 0 }; let mut is_symclk_on = 0;
    REG_UPDATE!(enc1, DIG_FIFO_CTRL0, DIG_FIFO_RESET, reset_val); REG_GET!(enc1, DIG_FE_CLK_CNTL, DIG_FE_SYMCLK_FE_G_CLOCK_ON, &mut is_symclk_on);
    if is_symclk_on != 0 { REG_WAIT!(enc1, DIG_FIFO_CTRL0, DIG_FIFO_RESET_DONE, reset_val, 10, 5000); } else { udelay(10); }
}

unsafe fn enc35_is_fifo_enabled(enc: *mut stream_encoder) -> bool { let enc1 = DCN10STRENC_FROM_STRENC(enc); let mut v = 0; REG_GET!(enc1, DIG_FIFO_CTRL0, DIG_FIFO_ENABLE, &mut v); v != 0 }
pub unsafe fn enc35_disable_fifo(enc: *mut stream_encoder) { let e = DCN10STRENC_FROM_STRENC(enc); REG_UPDATE!(e, DIG_FIFO_CTRL0, DIG_FIFO_ENABLE, 0); REG_UPDATE!(e, DIG_FE_EN_CNTL, DIG_FE_ENABLE, 0); REG_UPDATE!(e, DIG_FE_CLK_CNTL, DIG_FE_CLK_EN, 0); }
pub unsafe fn enc35_enable_fifo(enc: *mut stream_encoder) { let e = DCN10STRENC_FROM_STRENC(enc); REG_UPDATE!(e, DIG_FIFO_CTRL0, DIG_FIFO_READ_START_LEVEL, 0x7); REG_UPDATE!(e, DIG_FE_CLK_CNTL, DIG_FE_CLK_EN, 1); REG_UPDATE!(e, DIG_FE_EN_CNTL, DIG_FE_ENABLE, 1); enc35_reset_fifo(enc, true); enc35_reset_fifo(enc, false); REG_UPDATE!(e, DIG_FIFO_CTRL0, DIG_FIFO_ENABLE, 1); }
unsafe fn enc35_get_pixels_per_cycle(enc: *mut stream_encoder) -> u32 { let e = DCN10STRENC_FROM_STRENC(enc); let mut v = 0; REG_GET!(e, DIG_FIFO_CTRL0, DIG_FIFO_OUTPUT_PIXEL_MODE, &mut v); match v { 0 => 1, 1 => 2, _ => { ASSERT_CRITICAL!(false); 1 } } }

// The function table preserves the C interface and references implementations
// supplied by the surrounding translation unit.
static dcn35_str_enc_funcs: stream_encoder_funcs = stream_encoder_funcs {
    dp_set_odm_combine: Some(enc314_dp_set_odm_combine), dp_set_stream_attribute: Some(enc2_stream_encoder_dp_set_stream_attribute),
    hdmi_set_stream_attribute: Some(enc35_stream_encoder_hdmi_set_stream_attribute), dvi_set_stream_attribute: Some(enc35_stream_encoder_dvi_set_stream_attribute),
    set_throttled_vcp_size: Some(enc1_stream_encoder_set_throttled_vcp_size), update_hdmi_info_packets: Some(enc3_stream_encoder_update_hdmi_info_packets),
    stop_hdmi_info_packets: Some(enc3_stream_encoder_stop_hdmi_info_packets), update_dp_info_packets_sdp_line_num: Some(enc3_stream_encoder_update_dp_info_packets_sdp_line_num),
    update_dp_info_packets: Some(enc3_stream_encoder_update_dp_info_packets), stop_dp_info_packets: Some(enc1_stream_encoder_stop_dp_info_packets),
    dp_blank: Some(enc314_stream_encoder_dp_blank), dp_unblank: Some(enc35_stream_encoder_dp_unblank), audio_mute_control: Some(enc3_audio_mute_control),
    dp_audio_setup: Some(enc3_se_dp_audio_setup), dp_audio_enable: Some(enc3_se_dp_audio_enable), dp_audio_disable: Some(enc1_se_dp_audio_disable),
    hdmi_audio_setup: Some(enc3_se_hdmi_audio_setup), hdmi_audio_disable: Some(enc1_se_hdmi_audio_disable), setup_stereo_sync: Some(enc1_setup_stereo_sync),
    set_avmute: Some(enc1_stream_encoder_set_avmute), dig_connect_to_otg: Some(enc1_dig_connect_to_otg), dig_source_otg: Some(enc1_dig_source_otg),
    dp_get_pixel_format: Some(enc1_stream_encoder_dp_get_pixel_format), enc_read_state: Some(enc314_read_state), dp_set_dsc_config: Some(enc314_dp_set_dsc_config),
    dp_set_dsc_pps_info_packet: Some(enc3_dp_set_dsc_pps_info_packet), set_dynamic_metadata: Some(enc2_set_dynamic_metadata), hdmi_reset_stream_attribute: Some(enc1_reset_hdmi_stream_attribute),
    enable_stream: Some(enc35_stream_encoder_enable), set_input_mode: Some(enc314_set_dig_input_mode), enable_fifo: Some(enc35_enable_fifo), disable_fifo: Some(enc35_disable_fifo),
    is_fifo_enabled: Some(enc35_is_fifo_enabled), map_stream_to_link: Some(enc35_stream_encoder_map_to_link), get_pixels_per_cycle: Some(enc35_get_pixels_per_cycle),
};

pub unsafe fn dcn35_dio_stream_encoder_construct(enc1: *mut dcn10_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, vpg: *mut vpg, afmt: *mut afmt, regs: *const dcn10_stream_enc_registers, se_shift: *const dcn10_stream_encoder_shift, se_mask: *const dcn10_stream_encoder_mask) {
    (*enc1).base.funcs = &dcn35_str_enc_funcs; (*enc1).base.ctx = ctx; (*enc1).base.id = eng_id; (*enc1).base.bp = bp; (*enc1).base.vpg = vpg; (*enc1).base.afmt = afmt;
    (*enc1).regs = regs; (*enc1).se_shift = se_shift; (*enc1).se_mask = se_mask; (*enc1).base.stream_enc_inst = (*vpg).inst;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
