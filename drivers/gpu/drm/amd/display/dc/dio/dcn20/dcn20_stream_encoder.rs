/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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

// External declarations and register-helper macros are supplied by the surrounding tree.

unsafe fn enc2_update_hdmi_info_packet(
    enc1: *mut dcn10_stream_encoder,
    packet_index: u32,
    info_packet: *const dc_info_packet,
) {
    let (cont, send, line): (u32, u32, u32);
    if (*info_packet).valid {
        enc1_update_generic_info_packet(enc1, packet_index, info_packet);
        cont = 1;
        send = 1;
        line = 2;
    } else { cont = 0; send = 0; line = 0; }
    match packet_index {
        0 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_CONT, cont, HDMI_GENERIC0_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC0_LINE, line); }
        1 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC1_CONT, cont, HDMI_GENERIC1_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC1_LINE, line); }
        2 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC2_CONT, cont, HDMI_GENERIC2_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC2_LINE, line); }
        3 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC3_CONT, cont, HDMI_GENERIC3_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC3_LINE, line); }
        4 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC4_CONT, cont, HDMI_GENERIC4_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC4_LINE, line); }
        5 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC5_CONT, cont, HDMI_GENERIC5_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC5_LINE, line); }
        6 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC6_CONT, cont, HDMI_GENERIC6_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC6_LINE, line); }
        7 => { REG_UPDATE_2(enc1, HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC7_CONT, cont, HDMI_GENERIC7_SEND, send); REG_UPDATE(enc1, HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC7_LINE, line); }
        _ => { DC_LOG_WARNING!("Invalid HW packet index: enc2_update_hdmi_info_packet()\n"); }
    }
}

unsafe fn enc2_stream_encoder_update_hdmi_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_UPDATE(enc1, HDMI_DB_CONTROL, HDMI_DB_DISABLE, 1);
    enc2_update_hdmi_info_packet(enc1, 0, &(*info_frame).avi);
    enc2_update_hdmi_info_packet(enc1, 1, &(*info_frame).hfvsif);
    enc2_update_hdmi_info_packet(enc1, 2, &(*info_frame).gamut);
    enc2_update_hdmi_info_packet(enc1, 3, &(*info_frame).vendor);
    enc2_update_hdmi_info_packet(enc1, 4, &(*info_frame).spd);
    enc2_update_hdmi_info_packet(enc1, 5, &(*info_frame).hdrsmd);
    enc2_update_hdmi_info_packet(enc1, 6, &(*info_frame).vtem);
}

unsafe fn enc2_stream_encoder_stop_hdmi_info_packets(enc: *mut stream_encoder) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_SET_4(enc1, HDMI_GENERIC_PACKET_CONTROL0, 0, HDMI_GENERIC0_CONT, 0, HDMI_GENERIC0_SEND, 0, HDMI_GENERIC1_CONT, 0, HDMI_GENERIC1_SEND, 0);
    REG_SET_2(enc1, HDMI_GENERIC_PACKET_CONTROL1, 0, HDMI_GENERIC0_LINE, 0, HDMI_GENERIC1_LINE, 0);
    REG_SET_4(enc1, HDMI_GENERIC_PACKET_CONTROL0, 0, HDMI_GENERIC2_CONT, 0, HDMI_GENERIC2_SEND, 0, HDMI_GENERIC3_CONT, 0, HDMI_GENERIC3_SEND, 0);
    REG_SET_2(enc1, HDMI_GENERIC_PACKET_CONTROL2, 0, HDMI_GENERIC2_LINE, 0, HDMI_GENERIC3_LINE, 0);
    REG_SET_4(enc1, HDMI_GENERIC_PACKET_CONTROL0, 0, HDMI_GENERIC4_CONT, 0, HDMI_GENERIC4_SEND, 0, HDMI_GENERIC5_CONT, 0, HDMI_GENERIC5_SEND, 0);
    REG_SET_2(enc1, HDMI_GENERIC_PACKET_CONTROL3, 0, HDMI_GENERIC4_LINE, 0, HDMI_GENERIC5_LINE, 0);
    REG_SET_4(enc1, HDMI_GENERIC_PACKET_CONTROL0, 0, HDMI_GENERIC6_CONT, 0, HDMI_GENERIC6_SEND, 0, HDMI_GENERIC7_CONT, 0, HDMI_GENERIC7_SEND, 0);
    REG_SET_2(enc1, HDMI_GENERIC_PACKET_CONTROL4, 0, HDMI_GENERIC6_LINE, 0, HDMI_GENERIC7_LINE, 0);
}

unsafe fn enc2_update_gsp7_128_info_packet(enc1: *mut dcn10_stream_encoder, info_packet: *const dc_info_packet_128, immediate_update: bool) {
    let max_retries: u32 = 50;
    let content = (*info_packet).sb.as_ptr() as *const u32;
    ASSERT!((*info_packet).hb1 == DC_DP_INFOFRAME_TYPE_PPS);
    REG_UPDATE(enc1, DP_SEC_CNTL2, DP_SEC_GSP7_PPS, 1);
    REG_UPDATE(enc1, AFMT_CNTL, AFMT_AUDIO_CLOCK_EN, 1);
    REG_WAIT(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_CONFLICT, 0, 10, max_retries);
    REG_UPDATE(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_CONFLICT_CLR, 1);
    REG_UPDATE(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_INDEX, 7);
    REG_SET_4(enc1, AFMT_GENERIC_HDR, 0, AFMT_GENERIC_HB0, (*info_packet).hb0, AFMT_GENERIC_HB1, (*info_packet).hb1, AFMT_GENERIC_HB2, (*info_packet).hb2, AFMT_GENERIC_HB3, (*info_packet).hb3);
    for i in 0..4 {
        REG_UPDATE(enc1, AFMT_VBI_PACKET_CONTROL, AFMT_GENERIC_INDEX, 7 + i);
        for reg in [AFMT_GENERIC_0, AFMT_GENERIC_1, AFMT_GENERIC_2, AFMT_GENERIC_3, AFMT_GENERIC_4, AFMT_GENERIC_5, AFMT_GENERIC_6, AFMT_GENERIC_7] {
            REG_WRITE(enc1, reg, *content.add((i * 8 + reg) as usize));
        }
    }
    REG_UPDATE_2(enc1, AFMT_VBI_PACKET_CONTROL1, AFMT_GENERIC7_FRAME_UPDATE, !immediate_update, AFMT_GENERIC7_IMMEDIATE_UPDATE, immediate_update);
}

unsafe fn enc2_dp_set_dsc_config(enc: *mut stream_encoder, dsc_mode: optc_dsc_mode, dsc_bytes_per_pixel: u32, dsc_slice_width: u32) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_UPDATE_2(enc1, DP_DSC_CNTL, DP_DSC_MODE, dsc_mode, DP_DSC_SLICE_WIDTH, dsc_slice_width);
    REG_SET(enc1, DP_DSC_BYTES_PER_PIXEL, 0, DP_DSC_BYTES_PER_PIXEL, dsc_bytes_per_pixel);
}

unsafe fn enc2_dp_set_dsc_pps_info_packet(enc: *mut stream_encoder, enable: bool, dsc_packed_pps: *mut u8, immediate_update: bool) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if enable {
        let mut pps_sdp: dc_info_packet_128 = core::mem::zeroed();
        ASSERT!(!dsc_packed_pps.is_null());
        pps_sdp.valid = true; pps_sdp.hb0 = 0; pps_sdp.hb1 = DC_DP_INFOFRAME_TYPE_PPS; pps_sdp.hb2 = 127; pps_sdp.hb3 = 0;
        core::ptr::copy_nonoverlapping(dsc_packed_pps, pps_sdp.sb.as_mut_ptr(), pps_sdp.sb.len());
        enc2_update_gsp7_128_info_packet(enc1, &pps_sdp, immediate_update);
        REG_UPDATE(enc1, DP_SEC_CNTL6, DP_SEC_GSP7_LINE_NUM, 2);
        REG_UPDATE_2(enc1, DP_MSA_VBID_MISC, DP_VBID6_LINE_REFERENCE, 0, DP_VBID6_LINE_NUM, 3);
        REG_UPDATE_2(enc1, DP_SEC_CNTL, DP_SEC_GSP7_ENABLE, 1, DP_SEC_STREAM_ENABLE, 1);
    } else { REG_UPDATE(enc1, DP_SEC_CNTL, DP_SEC_GSP7_ENABLE, 0); REG_UPDATE(enc1, DP_SEC_CNTL2, DP_SEC_GSP7_PPS, 0); }
}

unsafe fn enc2_read_state(enc: *mut stream_encoder, s: *mut enc_state) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    REG_GET(enc1, DP_DSC_CNTL, DP_DSC_MODE, &mut (*s).dsc_mode);
    if (*s).dsc_mode != 0 {
        REG_GET(enc1, DP_DSC_CNTL, DP_DSC_SLICE_WIDTH, &mut (*s).dsc_slice_width);
        REG_GET(enc1, DP_SEC_CNTL6, DP_SEC_GSP7_LINE_NUM, &mut (*s).sec_gsp_pps_line_num);
        REG_GET(enc1, DP_MSA_VBID_MISC, DP_VBID6_LINE_REFERENCE, &mut (*s).vbid6_line_reference);
        REG_GET(enc1, DP_MSA_VBID_MISC, DP_VBID6_LINE_NUM, &mut (*s).vbid6_line_num);
        REG_GET(enc1, DP_SEC_CNTL, DP_SEC_GSP7_ENABLE, &mut (*s).sec_gsp_pps_enable);
        REG_GET(enc1, DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, &mut (*s).sec_stream_enable);
    }
}

pub unsafe fn enc2_set_dynamic_metadata(enc: *mut stream_encoder, enable_dme: bool, hubp_requestor_id: u32, dmdata_mode: dynamic_metadata_mode) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if enable_dme {
        REG_UPDATE_2(enc1, DME_CONTROL, METADATA_HUBP_REQUESTOR_ID, hubp_requestor_id, METADATA_STREAM_TYPE, (dmdata_mode == dmdata_dolby_vision) as u32);
        if dmdata_mode == dmdata_dp { REG_UPDATE_3(enc1, DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_ENABLE, 1, DP_SEC_METADATA_PACKET_LINE_REFERENCE, 0, DP_SEC_METADATA_PACKET_LINE, 20); }
        else { REG_UPDATE_3(enc1, HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_ENABLE, 1, HDMI_METADATA_PACKET_LINE_REFERENCE, 0, HDMI_METADATA_PACKET_LINE, 2); if dmdata_mode == dmdata_dolby_vision { REG_UPDATE(enc1, DIG_FE_CNTL, DOLBY_VISION_EN, 1); } }
        REG_UPDATE(enc1, DME_CONTROL, METADATA_ENGINE_EN, 1);
    } else {
        REG_UPDATE(enc1, DME_CONTROL, METADATA_ENGINE_EN, 0);
        if dmdata_mode == dmdata_dp { REG_UPDATE(enc1, DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_ENABLE, 0); }
        else { REG_UPDATE(enc1, HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_ENABLE, 0); REG_UPDATE(enc1, DIG_FE_CNTL, DOLBY_VISION_EN, 0); }
    }
}

unsafe fn enc2_stream_encoder_update_dp_info_packets_sdp_line_num(enc: *mut stream_encoder, info_frame: *mut encoder_info_frame) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if (*info_frame).adaptive_sync.valid && (*info_frame).sdp_line_num.adaptive_sync_line_num_valid {
        REG_UPDATE(enc1, DP_SEC_CNTL1, DP_SEC_GSP5_LINE_REFERENCE, 1);
        REG_UPDATE(enc1, DP_SEC_CNTL5, DP_SEC_GSP5_LINE_NUM, (*info_frame).sdp_line_num.adaptive_sync_line_num);
    }
}

unsafe fn enc2_stream_encoder_update_dp_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc); let mut dmdata_packet_enabled = 0;
    enc1_stream_encoder_update_dp_info_packets(enc, info_frame);
    REG_GET(enc1, DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_ENABLE, &mut dmdata_packet_enabled);
    if dmdata_packet_enabled != 0 { REG_UPDATE(enc1, DP_SEC_CNTL, DP_SEC_STREAM_ENABLE, 1); }
}

unsafe fn is_two_pixels_per_containter(timing: *const dc_crtc_timing) -> bool {
    let mut two_pix = (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR420;
    two_pix = two_pix || ((*timing).flags.DSC && (*timing).pixel_encoding == PIXEL_ENCODING_YCBCR422 && !(*timing).dsc_cfg.ycbcr422_simple);
    two_pix
}

pub unsafe fn enc2_stream_encoder_dp_unblank(link: *mut dc_link, enc: *mut stream_encoder, param: *const encoder_unblank_param) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc);
    if (*param).link_settings.link_rate != LINK_RATE_UNKNOWN {
        let n_vid: u32 = 0x8000; let mut m_vid: u32; let mut n_multiply = 0; let mut m_vid_l: u64 = n_vid as u64;
        if is_two_pixels_per_containter(&(*param).timing) || (*param).opp_cnt > 1 { n_multiply = 1; }
        m_vid_l = div_u64(m_vid_l * ((*param).timing.pix_clk_100hz / 10) as u64, ((*param).link_settings.link_rate * LINK_RATE_REF_FREQ_IN_KHZ) as u64);
        m_vid = m_vid_l as u32;
        REG_UPDATE(enc1, DP_VID_TIMING, DP_VID_M_N_GEN_EN, 0); REG_UPDATE(enc1, DP_VID_N, DP_VID_N, n_vid); REG_UPDATE(enc1, DP_VID_M, DP_VID_M, m_vid);
        REG_UPDATE_2(enc1, DP_VID_TIMING, DP_VID_M_N_GEN_EN, 1, DP_VID_N_MUL, n_multiply);
    }
    REG_UPDATE(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_ENABLE, false); REG_WAIT(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_STATUS, 0, 10, 5000);
    REG_UPDATE(enc1, DIG_FE_CNTL, DIG_START, 1); udelay(1); REG_UPDATE(enc1, DIG_FE_CNTL, DIG_START, 0);
    REG_UPDATE(enc1, DP_STEER_FIFO, DP_STEER_FIFO_RESET, 1); udelay(10); REG_UPDATE(enc1, DP_STEER_FIFO, DP_STEER_FIFO_RESET, 0); udelay(100);
    REG_UPDATE(enc1, DP_VID_STREAM_CNTL, DP_VID_STREAM_ENABLE, true);
    (*(*link).dc).link_srv.dp_trace_source_sequence(link, DPCD_SOURCE_SEQ_AFTER_ENABLE_DP_VID_STREAM);
}

unsafe fn enc2_dp_set_odm_combine(enc: *mut stream_encoder, odm_combine: bool) { let enc1 = DCN10STRENC_FROM_STRENC(enc); REG_UPDATE(enc1, DP_PIXEL_FORMAT, DP_PIXEL_COMBINE, odm_combine); }

pub unsafe fn enc2_stream_encoder_dp_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, output_color_space: dc_color_space, use_vsc_sdp_for_colorimetry: bool, enable_sdp_splitting: u32) {
    let enc1 = DCN10STRENC_FROM_STRENC(enc); enc1_stream_encoder_dp_set_stream_attribute(enc, crtc_timing, output_color_space, use_vsc_sdp_for_colorimetry, enable_sdp_splitting); REG_UPDATE(enc1, DP_SEC_FRAMING4, DP_SST_SDP_SPLITTING, enable_sdp_splitting);
}

pub unsafe fn enc2_get_fifo_cal_average_level(enc: *mut stream_encoder) -> u32 { let enc1 = DCN10STRENC_FROM_STRENC(enc); let mut fifo_level = 0; REG_GET(enc1, DIG_FIFO_STATUS, DIG_FIFO_CAL_AVERAGE_LEVEL, &mut fifo_level); fifo_level }

pub static mut dcn20_str_enc_funcs: stream_encoder_funcs = stream_encoder_funcs {
    dp_set_odm_combine: Some(enc2_dp_set_odm_combine), dp_set_stream_attribute: Some(enc2_stream_encoder_dp_set_stream_attribute),
    hdmi_set_stream_attribute: Some(enc1_stream_encoder_hdmi_set_stream_attribute), dvi_set_stream_attribute: Some(enc1_stream_encoder_dvi_set_stream_attribute),
    set_throttled_vcp_size: Some(enc1_stream_encoder_set_throttled_vcp_size), update_hdmi_info_packets: Some(enc2_stream_encoder_update_hdmi_info_packets), stop_hdmi_info_packets: Some(enc2_stream_encoder_stop_hdmi_info_packets),
    update_dp_info_packets_sdp_line_num: Some(enc2_stream_encoder_update_dp_info_packets_sdp_line_num), update_dp_info_packets: Some(enc2_stream_encoder_update_dp_info_packets), send_immediate_sdp_message: Some(enc1_stream_encoder_send_immediate_sdp_message), stop_dp_info_packets: Some(enc1_stream_encoder_stop_dp_info_packets), dp_blank: Some(enc1_stream_encoder_dp_blank), dp_unblank: Some(enc2_stream_encoder_dp_unblank), audio_mute_control: Some(enc1_se_audio_mute_control), dp_audio_setup: Some(enc1_se_dp_audio_setup), dp_audio_enable: Some(enc1_se_dp_audio_enable), dp_audio_disable: Some(enc1_se_dp_audio_disable), hdmi_audio_setup: Some(enc1_se_hdmi_audio_setup), hdmi_audio_disable: Some(enc1_se_hdmi_audio_disable), setup_stereo_sync: Some(enc1_setup_stereo_sync), set_avmute: Some(enc1_stream_encoder_set_avmute), dig_connect_to_otg: Some(enc1_dig_connect_to_otg), dig_source_otg: Some(enc1_dig_source_otg), dp_get_pixel_format: Some(enc1_stream_encoder_dp_get_pixel_format), enc_read_state: Some(enc2_read_state), dp_set_dsc_config: Some(enc2_dp_set_dsc_config), dp_set_dsc_pps_info_packet: Some(enc2_dp_set_dsc_pps_info_packet), set_dynamic_metadata: Some(enc2_set_dynamic_metadata), hdmi_reset_stream_attribute: Some(enc1_reset_hdmi_stream_attribute), get_fifo_cal_average_level: Some(enc2_get_fifo_cal_average_level),
};

pub unsafe fn dcn20_stream_encoder_construct(enc1: *mut dcn10_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, regs: *const dcn10_stream_enc_registers, se_shift: *const dcn10_stream_encoder_shift, se_mask: *const dcn10_stream_encoder_mask) {
    (*enc1).base.funcs = &raw mut dcn20_str_enc_funcs; (*enc1).base.ctx = ctx; (*enc1).base.id = eng_id; (*enc1).base.bp = bp; (*enc1).regs = regs; (*enc1).se_shift = se_shift; (*enc1).se_mask = se_mask; (*enc1).base.stream_enc_inst = eng_id - ENGINE_ID_DIGA;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
