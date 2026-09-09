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

// C dependencies: stream_encoder.h and dcn10/dcn10_stream_encoder.h.

macro_rules! SE_DCN2_REG_LIST {
    ($id:ident) => {
        SE_COMMON_DCN_REG_LIST!($id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL4, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL5, DIG, $id),
        SRI!(DP_DSC_CNTL, DP, $id),
        SRI!(DP_DSC_BYTES_PER_PIXEL, DP, $id),
        SRI!(DME_CONTROL, DIG, $id),
        SRI!(DP_SEC_METADATA_TRANSMISSION, DP, $id),
        SRI!(HDMI_METADATA_PACKET_CONTROL, DIG, $id),
        SRI!(DP_SEC_FRAMING4, DP, $id)
    };
}

macro_rules! SE_COMMON_MASK_SH_LIST_DCN20 {
    ($mask_sh:ident) => {
        SE_COMMON_MASK_SH_LIST_SOC!($mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC0_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC1_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC1_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC2_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC2_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC3_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC3_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC4_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC4_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC5_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC5_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC6_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC6_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC7_CONT, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL0, HDMI_GENERIC7_SEND, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC0_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL1, HDMI_GENERIC1_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC2_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL2, HDMI_GENERIC3_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC4_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL3, HDMI_GENERIC5_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC6_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_GENERIC_PACKET_CONTROL4, HDMI_GENERIC7_LINE, $mask_sh),
        SE_SF!(DP0_DP_DSC_CNTL, DP_DSC_MODE, $mask_sh),
        SE_SF!(DP0_DP_DSC_CNTL, DP_DSC_SLICE_WIDTH, $mask_sh),
        SE_SF!(DP0_DP_DSC_BYTES_PER_PIXEL, DP_DSC_BYTES_PER_PIXEL, $mask_sh),
        SE_SF!(DP0_DP_MSA_VBID_MISC, DP_VBID6_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_MSA_VBID_MISC, DP_VBID6_LINE_NUM, $mask_sh),
        SE_SF!(DIG0_DME_CONTROL, METADATA_ENGINE_EN, $mask_sh),
        SE_SF!(DIG0_DME_CONTROL, METADATA_HUBP_REQUESTOR_ID, $mask_sh),
        SE_SF!(DIG0_DME_CONTROL, METADATA_STREAM_TYPE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_ENABLE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_LINE_REFERENCE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_LINE, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DOLBY_VISION_EN, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, DP_PIXEL_COMBINE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL1, DP_SEC_GSP5_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL5, DP_SEC_GSP5_LINE_NUM, $mask_sh),
        SE_SF!(DP0_DP_SEC_FRAMING4, DP_SST_SDP_SPLITTING, $mask_sh)
    };
}

extern "C" {
    pub fn dcn20_stream_encoder_construct(
        enc1: *mut dcn10_stream_encoder,
        ctx: *mut dc_context,
        bp: *mut dc_bios,
        eng_id: engine_id,
        regs: *const dcn10_stream_enc_registers,
        se_shift: *const dcn10_stream_encoder_shift,
        se_mask: *const dcn10_stream_encoder_mask,
    );

    pub fn enc2_stream_encoder_dp_set_stream_attribute(
        enc: *mut stream_encoder,
        crtc_timing: *mut dc_crtc_timing,
        output_color_space: dc_color_space,
        use_vsc_sdp_for_colorimetry: bool,
        enable_sdp_splitting: u32,
    );

    pub fn enc2_stream_encoder_dp_unblank(
        link: *mut dc_link,
        enc: *mut stream_encoder,
        param: *const encoder_unblank_param,
    );

    pub fn enc2_set_dynamic_metadata(
        enc: *mut stream_encoder,
        enable_dme: bool,
        hubp_requestor_id: u32,
        dmdata_mode: dynamic_metadata_mode,
    );

    pub fn enc2_get_fifo_cal_average_level(enc: *mut stream_encoder) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
