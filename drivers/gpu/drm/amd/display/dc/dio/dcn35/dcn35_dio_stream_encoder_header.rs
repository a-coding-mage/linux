/* SPDX-License-Identifier: MIT */
/* Translated from dcn35_dio_stream_encoder.h. */

// Dependencies supplied by the surrounding translation unit:
// dcn30_vpg, dcn30_afmt, stream_encoder, and dcn20_stream_encoder.

pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_GATE_DIS__SHIFT: u32 = 0x8;
pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_EN__SHIFT: u32 = 0x9;
pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_CLOCK_ON__SHIFT: u32 = 0xa;
pub const DPCSTX0_DPCSTX_TX_CNTL__DPCS_TX_DATA_SWAP__SHIFT: u32 = 0xe;
pub const DPCSTX0_DPCSTX_TX_CNTL__DPCS_TX_DATA_ORDER_INVERT__SHIFT: u32 = 0xf;
pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_GATE_DIS_MASK: u32 = 0x00000100;
pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_EN_MASK: u32 = 0x00000200;
pub const RDPCSTX0_RDPCSTX_CLOCK_CNTL__RDPCS_SYMCLK_DIV2_CLOCK_ON_MASK: u32 = 0x00000400;
pub const DPCSTX0_DPCSTX_TX_CNTL__DPCS_TX_DATA_SWAP_MASK: u32 = 0x00004000;
pub const DPCSTX0_DPCSTX_TX_CNTL__DPCS_TX_DATA_ORDER_INVERT_MASK: u32 = 0x00008000;

// C register-list and field-list macros are retained as Rust token macros.
#[macro_export]
macro_rules! SE_DCN35_REG_LIST {
    ($id:expr) => {
        SRI!(AFMT_CNTL, DIG, $id), SRI!(DIG_FE_CNTL, DIG, $id), SRI!(HDMI_CONTROL, DIG, $id),
        SRI!(HDMI_DB_CONTROL, DIG, $id), SRI!(HDMI_GC, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL0, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL1, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL2, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL3, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL4, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL5, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL6, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL7, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL8, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL9, DIG, $id),
        SRI!(HDMI_GENERIC_PACKET_CONTROL10, DIG, $id), SRI!(HDMI_INFOFRAME_CONTROL0, DIG, $id),
        SRI!(HDMI_INFOFRAME_CONTROL1, DIG, $id), SRI!(HDMI_VBI_PACKET_CONTROL, DIG, $id),
        SRI!(HDMI_AUDIO_PACKET_CONTROL, DIG, $id), SRI!(HDMI_ACR_PACKET_CONTROL, DIG, $id),
        SRI!(HDMI_ACR_32_0, DIG, $id), SRI!(HDMI_ACR_32_1, DIG, $id), SRI!(HDMI_ACR_44_0, DIG, $id),
        SRI!(HDMI_ACR_44_1, DIG, $id), SRI!(HDMI_ACR_48_0, DIG, $id), SRI!(HDMI_ACR_48_1, DIG, $id),
        SRI!(DP_DB_CNTL, DP, $id), SRI!(DP_MSA_MISC, DP, $id), SRI!(DP_MSA_VBID_MISC, DP, $id),
        SRI!(DP_MSA_COLORIMETRY, DP, $id), SRI!(DP_MSA_TIMING_PARAM1, DP, $id), SRI!(DP_MSA_TIMING_PARAM2, DP, $id),
        SRI!(DP_MSA_TIMING_PARAM3, DP, $id), SRI!(DP_MSA_TIMING_PARAM4, DP, $id),
        SRI!(DP_MSE_RATE_CNTL, DP, $id), SRI!(DP_MSE_RATE_UPDATE, DP, $id), SRI!(DP_PIXEL_FORMAT, DP, $id),
        SRI!(DP_SEC_CNTL, DP, $id), SRI!(DP_SEC_CNTL1, DP, $id), SRI!(DP_SEC_CNTL2, DP, $id),
        SRI!(DP_SEC_CNTL5, DP, $id), SRI!(DP_SEC_CNTL6, DP, $id), SRI!(DP_STEER_FIFO, DP, $id),
        SRI!(DP_VID_M, DP, $id), SRI!(DP_VID_N, DP, $id), SRI!(DP_VID_STREAM_CNTL, DP, $id),
        SRI!(DP_VID_TIMING, DP, $id), SRI!(DP_SEC_AUD_N, DP, $id), SRI!(DP_SEC_TIMESTAMP, DP, $id),
        SRI!(DP_DSC_CNTL, DP, $id), SRI!(DP_SEC_METADATA_TRANSMISSION, DP, $id),
        SRI!(HDMI_METADATA_PACKET_CONTROL, DIG, $id), SRI!(DP_SEC_FRAMING4, DP, $id),
        SRI!(DP_GSP11_CNTL, DP, $id), SRI!(DME_CONTROL, DME, $id), SRI!(DIG_FE_EN_CNTL, DIG, $id),
        SRI!(DIG_FE_CLK_CNTL, DIG, $id), SRI!(DIG_CLOCK_PATTERN, DIG, $id), SRI!(DIG_FIFO_CTRL0, DIG, $id),
        SRI!(STREAM_MAPPER_CONTROL, DIG, $id)
    };
}

#[macro_export]
macro_rules! SE_COMMON_MASK_SH_LIST_DCN35 {
    ($mask_sh:expr) => {
        SE_SF!(DP0_DP_PIXEL_FORMAT, DP_PIXEL_ENCODING, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, DP_COMPONENT_DEPTH, $mask_sh),
        SE_SF!(DP0_DP_PIXEL_FORMAT, DP_PIXEL_PER_CYCLE_PROCESSING_MODE, $mask_sh),
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
        SE_SF!(DIG0_HDMI_CONTROL, TMDS_PIXEL_ENCODING, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, TMDS_COLOR_FORMAT, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DIG_STEREOSYNC_SELECT, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DIG_STEREOSYNC_GATE_EN, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP4_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP5_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP6_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL, DP_SEC_GSP7_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL2, DP_SEC_GSP7_SEND, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL6, DP_SEC_GSP7_LINE_NUM, $mask_sh),
        SE_SF!(DP0_DP_SEC_CNTL2, DP_SEC_GSP11_PPS, $mask_sh),
        SE_SF!(DP0_DP_GSP11_CNTL, DP_SEC_GSP11_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_GSP11_CNTL, DP_SEC_GSP11_LINE_NUM, $mask_sh),
        SE_SF!(DP0_DP_DB_CNTL, DP_DB_DISABLE, $mask_sh),
        SE_SF!(DP0_DP_MSA_COLORIMETRY, DP_MSA_MISC0, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM1, DP_MSA_HTOTAL, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM1, DP_MSA_VTOTAL, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM2, DP_MSA_HSTART, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM2, DP_MSA_VSTART, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM3, DP_MSA_HSYNCWIDTH, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM3, DP_MSA_HSYNCPOLARITY, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM3, DP_MSA_VSYNCWIDTH, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM3, DP_MSA_VSYNCPOLARITY, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM4, DP_MSA_HWIDTH, $mask_sh),
        SE_SF!(DP0_DP_MSA_TIMING_PARAM4, DP_MSA_VHEIGHT, $mask_sh),
        SE_SF!(DIG0_HDMI_DB_CONTROL, HDMI_DB_DISABLE, $mask_sh),
        SE_SF!(DP0_DP_VID_TIMING, DP_VID_N_MUL, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CNTL, DIG_SOURCE_SELECT, $mask_sh),
        SE_SF!(DP0_DP_DSC_CNTL, DP_DSC_MODE, $mask_sh),
        SE_SF!(DP0_DP_MSA_VBID_MISC, DP_VBID6_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_MSA_VBID_MISC, DP_VBID6_LINE_NUM, $mask_sh),
        SE_SF!(DME0_DME_CONTROL, METADATA_ENGINE_EN, $mask_sh),
        SE_SF!(DME0_DME_CONTROL, METADATA_HUBP_REQUESTOR_ID, $mask_sh),
        SE_SF!(DME0_DME_CONTROL, METADATA_STREAM_TYPE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_ENABLE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_LINE_REFERENCE, $mask_sh),
        SE_SF!(DP0_DP_SEC_METADATA_TRANSMISSION, DP_SEC_METADATA_PACKET_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_ENABLE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_LINE_REFERENCE, $mask_sh),
        SE_SF!(DIG0_HDMI_METADATA_PACKET_CONTROL, HDMI_METADATA_PACKET_LINE, $mask_sh),
        SE_SF!(DIG0_HDMI_CONTROL, DOLBY_VISION_EN, $mask_sh),
        SE_SF!(DIG0_DIG_FE_EN_CNTL, DIG_FE_ENABLE, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_MODE, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_CLK_EN, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_SOFT_RESET, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_DISPCLK_G_CLOCK_ON, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_SYMCLK_FE_G_CLOCK_ON, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_SYMCLK_FE_G_AFMT_CLOCK_ON, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_SYMCLK_FE_G_TMDS_CLOCK_ON, $mask_sh),
        SE_SF!(DIG0_DIG_FE_CLK_CNTL, DIG_FE_SOCCLK_G_AFMT_CLOCK_ON, $mask_sh),
        SE_SF!(DP0_DP_SEC_FRAMING4, DP_SST_SDP_SPLITTING, $mask_sh),
        SE_SF!(DIG0_DIG_CLOCK_PATTERN, DIG_CLOCK_PATTERN, $mask_sh),
        SE_SF!(DIG0_DIG_FIFO_CTRL0, DIG_FIFO_OUTPUT_PIXEL_MODE, $mask_sh),
        SE_SF!(DIG0_DIG_FIFO_CTRL0, DIG_FIFO_READ_START_LEVEL, $mask_sh),
        SE_SF!(DIG0_DIG_FIFO_CTRL0, DIG_FIFO_ENABLE, $mask_sh),
        SE_SF!(DIG0_DIG_FIFO_CTRL0, DIG_FIFO_RESET, $mask_sh),
        SE_SF!(DIG0_DIG_FIFO_CTRL0, DIG_FIFO_RESET_DONE, $mask_sh),
        SE_SF!(DIG0_STREAM_MAPPER_CONTROL, DIG_STREAM_LINK_TARGET, $mask_sh)
    };
}

extern "C" {
    pub fn dcn35_dio_stream_encoder_construct(enc1: *mut dcn10_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, vpg: *mut vpg, afmt: *mut afmt, regs: *const dcn10_stream_enc_registers, se_shift: *const dcn10_stream_encoder_shift, se_mask: *const dcn10_stream_encoder_mask);
    pub fn enc3_stream_encoder_update_hdmi_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame);
    pub fn enc3_stream_encoder_stop_hdmi_info_packets(enc: *mut stream_encoder);
    pub fn enc3_stream_encoder_update_dp_info_packets_sdp_line_num(enc: *mut stream_encoder, info_frame: *mut encoder_info_frame);
    pub fn enc3_stream_encoder_update_dp_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame);
    pub fn enc3_audio_mute_control(enc: *mut stream_encoder, mute: bool);
    pub fn enc3_se_dp_audio_setup(enc: *mut stream_encoder, az_inst: c_uint, info: *mut audio_info);
    pub fn enc3_se_dp_audio_enable(enc: *mut stream_encoder);
    pub fn enc3_se_hdmi_audio_setup(enc: *mut stream_encoder, az_inst: c_uint, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info);
    pub fn enc3_dp_set_dsc_pps_info_packet(enc: *mut stream_encoder, enable: bool, dsc_packed_pps: *mut u8, immediate_update: bool);
    pub fn enc35_disable_fifo(enc: *mut stream_encoder);
    pub fn enc35_enable_fifo(enc: *mut stream_encoder);
}

use core::ffi::c_uint;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
