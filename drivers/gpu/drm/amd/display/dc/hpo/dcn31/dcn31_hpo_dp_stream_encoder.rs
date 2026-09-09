/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp2_pixel_encoding {
    DP_SYM32_ENC_PIXEL_ENCODING_RGB_YCBCR444,
    DP_SYM32_ENC_PIXEL_ENCODING_YCBCR422,
    DP_SYM32_ENC_PIXEL_ENCODING_YCBCR420,
    DP_SYM32_ENC_PIXEL_ENCODING_Y_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp2_uncompressed_component_depth {
    DP_SYM32_ENC_COMPONENT_DEPTH_6BPC,
    DP_SYM32_ENC_COMPONENT_DEPTH_8BPC,
    DP_SYM32_ENC_COMPONENT_DEPTH_10BPC,
    DP_SYM32_ENC_COMPONENT_DEPTH_12BPC,
}

unsafe fn dcn31_hpo_dp_stream_enc_enable_stream(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_CONTROL, DP_STREAM_ENC_CLOCK_EN, 1);
    REG_UPDATE!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_RESET, 1);
    REG_WAIT!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_RESET_DONE, 1, 1, 10);
    REG_UPDATE!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_RESET, 0);
    REG_WAIT!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_RESET_DONE, 0, 1, 10);
    REG_UPDATE!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_ENABLE, 1);
}

unsafe fn dcn31_hpo_dp_stream_enc_dp_unblank(enc: *mut hpo_dp_stream_encoder, stream_source: u32) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_STREAM_ENC_INPUT_MUX_CONTROL, DP_STREAM_ENC_INPUT_MUX_PIXEL_STREAM_SOURCE_SEL, stream_source);
    REG_UPDATE!(DP_SYM32_ENC_VID_STREAM_CONTROL, VID_STREAM_ENABLE, 1);
    REG_UPDATE!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_RESET, 1);
    REG_WAIT!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_RESET_DONE, 1, 1, 10);
    REG_UPDATE!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_RESET, 0);
    REG_WAIT!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_RESET_DONE, 0, 1, 10);
    REG_UPDATE!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_ENABLE, 1);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 1);
    REG_WAIT!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 1, 1, 10);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET, 0);
    REG_WAIT!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_RESET_DONE, 0, 1, 10);
    REG_UPDATE_2!(DP_SYM32_ENC_VID_CRC_CONTROL, CRC_ENABLE, 1, CRC_CONT_MODE_ENABLE, 1);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 1);
}

unsafe fn dcn31_hpo_dp_stream_enc_dp_blank(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_SYM32_ENC_VID_STREAM_CONTROL, VID_STREAM_ENABLE, 0);
    REG_WAIT!(DP_SYM32_ENC_VID_STREAM_CONTROL, VID_STREAM_STATUS, 0, 10, 5000);
    REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 0);
    REG_UPDATE!(DP_SYM32_ENC_VID_FIFO_CONTROL, PIXEL_TO_SYMBOL_FIFO_ENABLE, 0);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, FIFO_ENABLE, 0);
}

unsafe fn dcn31_hpo_dp_stream_enc_disable(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_ENABLE, 0);
    REG_UPDATE!(DP_STREAM_ENC_CLOCK_CONTROL, DP_STREAM_ENC_CLOCK_EN, 0);
}

unsafe fn dcn31_hpo_dp_stream_enc_set_stream_attribute(enc: *mut hpo_dp_stream_encoder, crtc_timing: *mut dc_crtc_timing, output_color_space: dc_color_space, use_vsc_sdp_for_colorimetry: bool, compressed_format: bool, double_buffer_en: bool) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    let hw_crtc_timing = *crtc_timing;
    let mut misc0: u8 = 0;
    let mut misc1: u8 = 0;
    let pixel_encoding;
    let component_depth;
    let (h_active_start, v_active_start, h_width, v_height, v_freq);
    let hsp: u8;
    let vsp: u8;
    if hw_crtc_timing.flags.INTERLACE { BREAK_TO_DEBUGGER!(); }
    REG_UPDATE!(DP_SYM32_ENC_VID_MSA_DOUBLE_BUFFER_CONTROL, MSA_DOUBLE_BUFFER_ENABLE, double_buffer_en);
    REG_UPDATE!(DP_SYM32_ENC_VID_PIXEL_FORMAT_DOUBLE_BUFFER_CONTROL, PIXEL_FORMAT_DOUBLE_BUFFER_ENABLE, double_buffer_en);
    match hw_crtc_timing.pixel_encoding {
        PIXEL_ENCODING_YCBCR422 => { pixel_encoding = dp2_pixel_encoding::DP_SYM32_ENC_PIXEL_ENCODING_YCBCR422; misc0 |= 0x2; }
        PIXEL_ENCODING_YCBCR444 => { pixel_encoding = dp2_pixel_encoding::DP_SYM32_ENC_PIXEL_ENCODING_RGB_YCBCR444; misc0 |= 0x4; if hw_crtc_timing.flags.Y_ONLY { if hw_crtc_timing.display_color_depth != COLOR_DEPTH_666 { misc1 |= 0x80; } } }
        PIXEL_ENCODING_YCBCR420 => { pixel_encoding = dp2_pixel_encoding::DP_SYM32_ENC_PIXEL_ENCODING_YCBCR420; misc1 |= 0x40; }
        _ => { pixel_encoding = dp2_pixel_encoding::DP_SYM32_ENC_PIXEL_ENCODING_RGB_YCBCR444; }
    }
    if use_vsc_sdp_for_colorimetry { misc1 |= 0x40; } else { misc1 &= !0x40; }
    match hw_crtc_timing.display_color_depth {
        COLOR_DEPTH_666 => component_depth = dp2_uncompressed_component_depth::DP_SYM32_ENC_COMPONENT_DEPTH_6BPC,
        COLOR_DEPTH_888 => { component_depth = dp2_uncompressed_component_depth::DP_SYM32_ENC_COMPONENT_DEPTH_8BPC; misc0 |= 0x20; }
        COLOR_DEPTH_101010 => { component_depth = dp2_uncompressed_component_depth::DP_SYM32_ENC_COMPONENT_DEPTH_10BPC; misc0 |= 0x40; }
        COLOR_DEPTH_121212 => { component_depth = dp2_uncompressed_component_depth::DP_SYM32_ENC_COMPONENT_DEPTH_12BPC; misc0 |= 0x60; }
        _ => component_depth = dp2_uncompressed_component_depth::DP_SYM32_ENC_COMPONENT_DEPTH_6BPC,
    }
    REG_UPDATE_3!(DP_SYM32_ENC_VID_PIXEL_FORMAT, PIXEL_ENCODING_TYPE, compressed_format, UNCOMPRESSED_PIXEL_ENCODING, pixel_encoding, UNCOMPRESSED_COMPONENT_DEPTH, component_depth);
    match output_color_space {
        COLOR_SPACE_SRGB => misc1 &= !0x80,
        COLOR_SPACE_SRGB_LIMITED => { misc0 |= 0x8; misc1 &= !0x80; }
        COLOR_SPACE_YCBCR601 | COLOR_SPACE_YCBCR601_LIMITED => { misc0 |= 0x8; misc1 &= !0x80; if hw_crtc_timing.pixel_encoding == PIXEL_ENCODING_YCBCR422 { misc0 |= 0x2; } else if hw_crtc_timing.pixel_encoding == PIXEL_ENCODING_YCBCR444 { misc0 |= 0x4; } }
        COLOR_SPACE_YCBCR709 | COLOR_SPACE_YCBCR709_LIMITED => { misc0 |= 0x18; misc1 &= !0x80; if hw_crtc_timing.pixel_encoding == PIXEL_ENCODING_YCBCR422 { misc0 |= 0x2; } else if hw_crtc_timing.pixel_encoding == PIXEL_ENCODING_YCBCR444 { misc0 |= 0x4; } }
        _ => {}
    }
    let h_blank = hw_crtc_timing.h_total - hw_crtc_timing.h_border_left - hw_crtc_timing.h_addressable - hw_crtc_timing.h_border_right;
    let h_back_porch = h_blank - hw_crtc_timing.h_front_porch - hw_crtc_timing.h_sync_width;
    h_active_start = hw_crtc_timing.h_sync_width + h_back_porch;
    v_active_start = hw_crtc_timing.v_total - hw_crtc_timing.v_border_top - hw_crtc_timing.v_addressable - hw_crtc_timing.v_border_bottom - hw_crtc_timing.v_front_porch;
    h_width = hw_crtc_timing.h_border_left + hw_crtc_timing.h_addressable + hw_crtc_timing.h_border_right;
    v_height = hw_crtc_timing.v_border_top + hw_crtc_timing.v_addressable + hw_crtc_timing.v_border_bottom;
    hsp = if hw_crtc_timing.flags.HSYNC_POSITIVE_POLARITY { 0 } else { 0x80 };
    vsp = if hw_crtc_timing.flags.VSYNC_POSITIVE_POLARITY { 0 } else { 0x80 };
    v_freq = (hw_crtc_timing.pix_clk_100hz as u64) * 100;
    REG_SET_4!(DP_SYM32_ENC_VID_MSA0, 0, MSA_DATA_LANE_0, 0, MSA_DATA_LANE_1, 0, MSA_DATA_LANE_2, 0, MSA_DATA_LANE_3, v_freq >> 40);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA1, 0, MSA_DATA_LANE_0, 0, MSA_DATA_LANE_1, 0, MSA_DATA_LANE_2, 0, MSA_DATA_LANE_3, (v_freq >> 32) & 0xff);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA2, 0, MSA_DATA_LANE_0, 0, MSA_DATA_LANE_1, 0, MSA_DATA_LANE_2, 0, MSA_DATA_LANE_3, (v_freq >> 24) & 0xff);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA3, 0, MSA_DATA_LANE_0, hw_crtc_timing.h_total >> 8, MSA_DATA_LANE_1, h_active_start >> 8, MSA_DATA_LANE_2, h_width >> 8, MSA_DATA_LANE_3, (v_freq >> 16) & 0xff);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA4, 0, MSA_DATA_LANE_0, hw_crtc_timing.h_total & 0xff, MSA_DATA_LANE_1, h_active_start & 0xff, MSA_DATA_LANE_2, h_width & 0xff, MSA_DATA_LANE_3, (v_freq >> 8) & 0xff);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA5, 0, MSA_DATA_LANE_0, hw_crtc_timing.v_total >> 8, MSA_DATA_LANE_1, v_active_start >> 8, MSA_DATA_LANE_2, v_height >> 8, MSA_DATA_LANE_3, v_freq & 0xff);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA6, 0, MSA_DATA_LANE_0, hw_crtc_timing.v_total & 0xff, MSA_DATA_LANE_1, v_active_start & 0xff, MSA_DATA_LANE_2, v_height & 0xff, MSA_DATA_LANE_3, misc0);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA7, 0, MSA_DATA_LANE_0, hsp | (hw_crtc_timing.h_sync_width >> 8), MSA_DATA_LANE_1, vsp | (hw_crtc_timing.v_sync_width >> 8), MSA_DATA_LANE_2, 0, MSA_DATA_LANE_3, misc1);
    REG_SET_4!(DP_SYM32_ENC_VID_MSA8, 0, MSA_DATA_LANE_0, hw_crtc_timing.h_sync_width & 0xff, MSA_DATA_LANE_1, hw_crtc_timing.v_sync_width & 0xff, MSA_DATA_LANE_2, 0, MSA_DATA_LANE_3, 0);
}

unsafe fn dcn31_hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num(enc: *mut hpo_dp_stream_encoder, info_frame: *mut encoder_info_frame) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    if (*info_frame).adaptive_sync.valid == true && (*info_frame).sdp_line_num.adaptive_sync_line_num_valid == true {
        REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL5, GSP_SOF_REFERENCE, 1);
        REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL5, GSP_TRANSMISSION_LINE_NUMBER, (*info_frame).sdp_line_num.adaptive_sync_line_num);
    }
}

unsafe fn dcn31_hpo_dp_stream_enc_update_dp_info_packets(enc: *mut hpo_dp_stream_encoder, info_frame: *const encoder_info_frame) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    let mut dmdata_packet_enabled = 0u32;
    if (*info_frame).vsc.valid { (*(*enc).vpg).funcs.update_generic_info_packet((*enc).vpg, 0, &(*info_frame).vsc, true); }
    if (*info_frame).spd.valid { (*(*enc).vpg).funcs.update_generic_info_packet((*enc).vpg, 2, &(*info_frame).spd, true); }
    if (*info_frame).hdrsmd.valid && !(*info_frame).firmware_controlled_hdr_info_packet { (*(*enc).vpg).funcs.update_generic_info_packet((*enc).vpg, 3, &(*info_frame).hdrsmd, true); }
    if (*info_frame).adaptive_sync.valid { (*(*enc).vpg).funcs.update_generic_info_packet((*enc).vpg, 5, &(*info_frame).adaptive_sync, true); }
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL0, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, (*info_frame).vsc.valid);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL2, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, (*info_frame).spd.valid);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL3, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, (*info_frame).hdrsmd.valid);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL5, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, (*info_frame).adaptive_sync.valid);
    REG_GET!(DP_SYM32_ENC_SDP_METADATA_PACKET_CONTROL, METADATA_PACKET_ENABLE, &mut dmdata_packet_enabled);
    REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 1);
}

unsafe fn dcn31_hpo_dp_stream_enc_stop_dp_info_packets(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    let (mut asp_enable, mut atp_enable, mut aip_enable, mut acm_enable) = (0u32, 0u32, 0u32, 0u32);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL0, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, 0);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL2, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, 0);
    REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL3, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, 0);
    REG_GET_4!(DP_SYM32_ENC_SDP_AUDIO_CONTROL0, ASP_ENABLE, &mut asp_enable, ATP_ENABLE, &mut atp_enable, AIP_ENABLE, &mut aip_enable, ACM_ENABLE, &mut acm_enable);
    if asp_enable == 0 && atp_enable == 0 && aip_enable == 0 && acm_enable == 0 { REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 0); }
}

unsafe fn hpo_dp_is_gsp_enabled(enc: *mut hpo_dp_stream_encoder) -> u32 {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    let (mut gsp0_enabled, mut gsp2_enabled, mut gsp3_enabled, mut gsp11_enabled) = (0u32, 0u32, 0u32, 0u32);
    REG_GET!(DP_SYM32_ENC_SDP_GSP_CONTROL0, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, &mut gsp0_enabled);
    REG_GET!(DP_SYM32_ENC_SDP_GSP_CONTROL2, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, &mut gsp2_enabled);
    REG_GET!(DP_SYM32_ENC_SDP_GSP_CONTROL3, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, &mut gsp3_enabled);
    REG_GET!(DP_SYM32_ENC_SDP_GSP_CONTROL11, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, &mut gsp11_enabled);
    (gsp0_enabled != 0 || gsp2_enabled != 0 || gsp3_enabled != 0 || gsp11_enabled != 0) as u32
}

unsafe fn dcn31_hpo_dp_stream_enc_set_dsc_pps_info_packet(enc: *mut hpo_dp_stream_encoder, enable: bool, dsc_packed_pps: *mut u8, immediate_update: bool) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    if enable {
        let mut pps_sdp: dc_info_packet = core::mem::zeroed();
        REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL11, GSP_PAYLOAD_SIZE, 3);
        pps_sdp.valid = true; pps_sdp.hb0 = 0; pps_sdp.hb1 = DC_DP_INFOFRAME_TYPE_PPS; pps_sdp.hb2 = 127; pps_sdp.hb3 = 0;
        for i in 0..4 { core::ptr::copy_nonoverlapping(dsc_packed_pps.add(i * 32), pps_sdp.sb.as_mut_ptr(), 32); (*(*enc3).base.vpg).funcs.update_generic_info_packet((*enc3).base.vpg, 11 + i, &pps_sdp, immediate_update); }
        REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL11, GSP_TRANSMISSION_LINE_NUMBER, 2);
        REG_UPDATE_2!(DP_SYM32_ENC_VID_VBID_CONTROL, VBID_6_COMPRESSEDSTREAM_FLAG_SOF_REFERENCE, 0, VBID_6_COMPRESSEDSTREAM_FLAG_LINE_NUMBER, 3);
        REG_UPDATE!(DP_SYM32_ENC_SDP_GSP_CONTROL11, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, 1);
        REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 1);
    } else { REG_UPDATE_2!(DP_SYM32_ENC_SDP_GSP_CONTROL11, GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE, 0, GSP_PAYLOAD_SIZE, 0); }
}

unsafe fn dcn31_hpo_dp_stream_enc_map_stream_to_link(enc: *mut hpo_dp_stream_encoder, stream_enc_inst: u32, link_enc_inst: u32) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    ASSERT!(stream_enc_inst < 4 && link_enc_inst < 2);
    match stream_enc_inst { 0 => REG_UPDATE!(DP_STREAM_MAPPER_CONTROL0, DP_STREAM_LINK_TARGET, link_enc_inst), 1 => REG_UPDATE!(DP_STREAM_MAPPER_CONTROL1, DP_STREAM_LINK_TARGET, link_enc_inst), 2 => REG_UPDATE!(DP_STREAM_MAPPER_CONTROL2, DP_STREAM_LINK_TARGET, link_enc_inst), 3 => REG_UPDATE!(DP_STREAM_MAPPER_CONTROL3, DP_STREAM_LINK_TARGET, link_enc_inst), _ => {} }
}

unsafe fn dcn31_hpo_dp_stream_enc_audio_setup(enc: *mut hpo_dp_stream_encoder, az_inst: u32, info: *mut audio_info) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_STREAM_ENC_AUDIO_CONTROL, DP_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, az_inst);
    if (*enc3).hpo_se_mask.DP_STREAM_ENC_APG_CLOCK_EN != 0 { REG_UPDATE!(DP_STREAM_ENC_AUDIO_CONTROL, DP_STREAM_ENC_APG_CLOCK_EN, 1); }
    ASSERT!(!(*enc).apg.is_null()); (*(*enc).apg).funcs.se_audio_setup((*enc).apg, az_inst, info);
}

unsafe fn dcn31_hpo_dp_stream_enc_audio_enable(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE!(DP_SYM32_ENC_SDP_AUDIO_CONTROL0, ASP_ENABLE, 1);
    REG_UPDATE_2!(DP_SYM32_ENC_SDP_AUDIO_CONTROL0, ATP_ENABLE, 1, AIP_ENABLE, 1);
    REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 1);
    (*(*enc).apg).funcs.enable_apg((*enc).apg);
}

unsafe fn dcn31_hpo_dp_stream_enc_audio_disable(enc: *mut hpo_dp_stream_encoder) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_UPDATE_4!(DP_SYM32_ENC_SDP_AUDIO_CONTROL0, ASP_ENABLE, 0, ATP_ENABLE, 0, AIP_ENABLE, 0, ACM_ENABLE, 0);
    if hpo_dp_is_gsp_enabled(enc) == 0 { REG_UPDATE!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, 0); }
    (*(*enc).apg).funcs.disable_apg((*enc).apg);
}

unsafe fn dcn31_hpo_dp_stream_enc_read_state(enc: *mut hpo_dp_stream_encoder, s: *mut hpo_dp_stream_encoder_state) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_GET!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC_ENABLE, &mut (*s).stream_enc_enabled);
    REG_GET!(DP_SYM32_ENC_VID_STREAM_CONTROL, VID_STREAM_ENABLE, &mut (*s).vid_stream_enabled);
    REG_GET!(DP_STREAM_ENC_INPUT_MUX_CONTROL, DP_STREAM_ENC_INPUT_MUX_PIXEL_STREAM_SOURCE_SEL, &mut (*s).otg_inst);
    REG_GET_3!(DP_SYM32_ENC_VID_PIXEL_FORMAT, PIXEL_ENCODING_TYPE, &mut (*s).compressed_format, UNCOMPRESSED_PIXEL_ENCODING, &mut (*s).pixel_encoding, UNCOMPRESSED_COMPONENT_DEPTH, &mut (*s).component_depth);
    REG_GET!(DP_SYM32_ENC_SDP_CONTROL, SDP_STREAM_ENABLE, &mut (*s).sdp_enabled);
    match (*enc).inst { 0 => REG_GET!(DP_STREAM_MAPPER_CONTROL0, DP_STREAM_LINK_TARGET, &mut (*s).mapped_to_link_enc), 1 => REG_GET!(DP_STREAM_MAPPER_CONTROL1, DP_STREAM_LINK_TARGET, &mut (*s).mapped_to_link_enc), 2 => REG_GET!(DP_STREAM_MAPPER_CONTROL2, DP_STREAM_LINK_TARGET, &mut (*s).mapped_to_link_enc), 3 => REG_GET!(DP_STREAM_MAPPER_CONTROL3, DP_STREAM_LINK_TARGET, &mut (*s).mapped_to_link_enc), _ => {} }
}

unsafe fn dcn31_set_hblank_min_symbol_width(enc: *mut hpo_dp_stream_encoder, width: u16) {
    let enc3 = DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC(enc);
    REG_SET!(DP_SYM32_ENC_HBLANK_CONTROL, 0, HBLANK_MINIMUM_SYMBOL_WIDTH, width);
}

static dcn30_str_enc_funcs: hpo_dp_stream_encoder_funcs = hpo_dp_stream_encoder_funcs {
    enable_stream: Some(dcn31_hpo_dp_stream_enc_enable_stream),
    dp_unblank: Some(dcn31_hpo_dp_stream_enc_dp_unblank),
    dp_blank: Some(dcn31_hpo_dp_stream_enc_dp_blank),
    disable: Some(dcn31_hpo_dp_stream_enc_disable),
    set_stream_attribute: Some(dcn31_hpo_dp_stream_enc_set_stream_attribute),
    update_dp_info_packets_sdp_line_num: Some(dcn31_hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num),
    update_dp_info_packets: Some(dcn31_hpo_dp_stream_enc_update_dp_info_packets),
    stop_dp_info_packets: Some(dcn31_hpo_dp_stream_enc_stop_dp_info_packets),
    dp_set_dsc_pps_info_packet: Some(dcn31_hpo_dp_stream_enc_set_dsc_pps_info_packet),
    map_stream_to_link: Some(dcn31_hpo_dp_stream_enc_map_stream_to_link),
    dp_audio_setup: Some(dcn31_hpo_dp_stream_enc_audio_setup),
    dp_audio_enable: Some(dcn31_hpo_dp_stream_enc_audio_enable),
    dp_audio_disable: Some(dcn31_hpo_dp_stream_enc_audio_disable),
    read_state: Some(dcn31_hpo_dp_stream_enc_read_state),
    set_hblank_min_symbol_width: Some(dcn31_set_hblank_min_symbol_width),
};

#[no_mangle]
pub unsafe extern "C" fn dcn31_hpo_dp_stream_encoder_construct(enc3: *mut dcn31_hpo_dp_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, inst: u32, eng_id: engine_id, vpg: *mut vpg, apg: *mut apg, regs: *const dcn31_hpo_dp_stream_encoder_registers, hpo_se_shift: *const dcn31_hpo_dp_stream_encoder_shift, hpo_se_mask: *const dcn31_hpo_dp_stream_encoder_mask) {
    (*enc3).base.funcs = &dcn30_str_enc_funcs;
    (*enc3).base.ctx = ctx; (*enc3).base.inst = inst; (*enc3).base.id = eng_id; (*enc3).base.bp = bp; (*enc3).base.vpg = vpg; (*enc3).base.apg = apg;
    (*enc3).regs = regs; (*enc3).hpo_se_shift = hpo_se_shift; (*enc3).hpo_se_mask = hpo_se_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
