/* Rust translation of dcn10_stream_encoder.h. */

// Dependency supplied by stream_encoder.h and the surrounding driver.

#[macro_export]
macro_rules! DCN10STRENC_FROM_STRENC {
    ($stream_encoder:expr) => { container_of!($stream_encoder, dcn10_stream_encoder, base) };
}

// The register-list macros are retained as token-producing Rust macros; SRI is
// supplied by the register definitions used by the including translation unit.
#[macro_export]
macro_rules! SE_COMMON_DCN_REG_LIST { ($id:expr) => {
    SRI!(AFMT_CNTL, DIG, $id), SRI!(AFMT_GENERIC_0, DIG, $id), SRI!(AFMT_GENERIC_1, DIG, $id),
    SRI!(AFMT_GENERIC_2, DIG, $id), SRI!(AFMT_GENERIC_3, DIG, $id), SRI!(AFMT_GENERIC_4, DIG, $id),
    SRI!(AFMT_GENERIC_5, DIG, $id), SRI!(AFMT_GENERIC_6, DIG, $id), SRI!(AFMT_GENERIC_7, DIG, $id),
    SRI!(AFMT_GENERIC_HDR, DIG, $id), SRI!(AFMT_INFOFRAME_CONTROL0, DIG, $id),
    SRI!(AFMT_VBI_PACKET_CONTROL, DIG, $id), SRI!(AFMT_VBI_PACKET_CONTROL1, DIG, $id),
    SRI!(AFMT_AUDIO_PACKET_CONTROL, DIG, $id), SRI!(AFMT_AUDIO_PACKET_CONTROL2, DIG, $id),
    SRI!(AFMT_AUDIO_SRC_CONTROL, DIG, $id), SRI!(AFMT_60958_0, DIG, $id), SRI!(AFMT_60958_1, DIG, $id),
    SRI!(AFMT_60958_2, DIG, $id), SRI!(DIG_FE_CNTL, DIG, $id), SRI!(DIG_FIFO_STATUS, DIG, $id),
    SRI!(HDMI_CONTROL, DIG, $id), SRI!(HDMI_DB_CONTROL, DIG, $id), SRI!(HDMI_GC, DIG, $id),
    SRI!(HDMI_GENERIC_PACKET_CONTROL0, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL1, DIG, $id),
    SRI!(HDMI_GENERIC_PACKET_CONTROL2, DIG, $id), SRI!(HDMI_GENERIC_PACKET_CONTROL3, DIG, $id),
    SRI!(HDMI_INFOFRAME_CONTROL0, DIG, $id), SRI!(HDMI_INFOFRAME_CONTROL1, DIG, $id),
    SRI!(HDMI_VBI_PACKET_CONTROL, DIG, $id), SRI!(HDMI_AUDIO_PACKET_CONTROL, DIG, $id),
    SRI!(HDMI_ACR_PACKET_CONTROL, DIG, $id), SRI!(HDMI_ACR_32_0, DIG, $id), SRI!(HDMI_ACR_32_1, DIG, $id),
    SRI!(HDMI_ACR_44_0, DIG, $id), SRI!(HDMI_ACR_44_1, DIG, $id), SRI!(HDMI_ACR_48_0, DIG, $id),
    SRI!(HDMI_ACR_48_1, DIG, $id), SRI!(DP_DB_CNTL, DP, $id), SRI!(DP_MSA_MISC, DP, $id),
    SRI!(DP_MSA_VBID_MISC, DP, $id), SRI!(DP_MSA_COLORIMETRY, DP, $id),
    SRI!(DP_MSA_TIMING_PARAM1, DP, $id), SRI!(DP_MSA_TIMING_PARAM2, DP, $id),
    SRI!(DP_MSA_TIMING_PARAM3, DP, $id), SRI!(DP_MSA_TIMING_PARAM4, DP, $id),
    SRI!(DP_MSE_RATE_CNTL, DP, $id), SRI!(DP_MSE_RATE_UPDATE, DP, $id), SRI!(DP_PIXEL_FORMAT, DP, $id),
    SRI!(DP_SEC_CNTL, DP, $id), SRI!(DP_SEC_CNTL1, DP, $id), SRI!(DP_SEC_CNTL2, DP, $id),
    SRI!(DP_SEC_CNTL5, DP, $id), SRI!(DP_SEC_CNTL6, DP, $id), SRI!(DP_STEER_FIFO, DP, $id),
    SRI!(DP_VID_M, DP, $id), SRI!(DP_VID_N, DP, $id), SRI!(DP_VID_STREAM_CNTL, DP, $id),
    SRI!(DP_VID_TIMING, DP, $id), SRI!(DP_SEC_AUD_N, DP, $id), SRI!(DP_SEC_AUD_N_READBACK, DP, $id),
    SRI!(DP_SEC_AUD_M_READBACK, DP, $id), SRI!(DP_SEC_TIMESTAMP, DP, $id), SRI!(DIG_CLOCK_PATTERN, DIG, $id)
} }
#[macro_export]
macro_rules! SE_DCN_REG_LIST { ($id:expr) => { SE_COMMON_DCN_REG_LIST!($id) } }

#[repr(C)]
pub struct dcn10_stream_enc_registers {
    pub AFMT_CNTL:u32, pub AFMT_AVI_INFO0:u32, pub AFMT_AVI_INFO1:u32, pub AFMT_AVI_INFO2:u32, pub AFMT_AVI_INFO3:u32,
    pub AFMT_GENERIC_0:u32, pub AFMT_GENERIC_1:u32, pub AFMT_GENERIC_2:u32, pub AFMT_GENERIC_3:u32, pub AFMT_GENERIC_4:u32,
    pub AFMT_GENERIC_5:u32, pub AFMT_GENERIC_6:u32, pub AFMT_GENERIC_7:u32, pub AFMT_GENERIC_HDR:u32,
    pub AFMT_INFOFRAME_CONTROL0:u32, pub AFMT_VBI_PACKET_CONTROL:u32, pub AFMT_VBI_PACKET_CONTROL1:u32,
    pub AFMT_AUDIO_PACKET_CONTROL:u32, pub AFMT_AUDIO_PACKET_CONTROL2:u32, pub AFMT_AUDIO_SRC_CONTROL:u32,
    pub AFMT_60958_0:u32, pub AFMT_60958_1:u32, pub AFMT_60958_2:u32, pub DIG_FE_CNTL:u32, pub DIG_FIFO_STATUS:u32,
    pub DP_MSE_RATE_CNTL:u32, pub DP_MSE_RATE_UPDATE:u32, pub DP_PIXEL_FORMAT:u32, pub DP_SEC_CNTL:u32,
    pub DP_SEC_CNTL1:u32, pub DP_SEC_CNTL2:u32, pub DP_SEC_CNTL5:u32, pub DP_SEC_CNTL6:u32, pub DP_STEER_FIFO:u32,
    pub DP_VID_M:u32, pub DP_VID_N:u32, pub DP_VID_STREAM_CNTL:u32, pub DP_VID_TIMING:u32, pub DP_SEC_AUD_N:u32,
    pub DP_SEC_AUD_N_READBACK:u32, pub DP_SEC_AUD_M_READBACK:u32, pub DP_SEC_TIMESTAMP:u32, pub HDMI_CONTROL:u32,
    pub HDMI_GC:u32, pub HDMI_GENERIC_PACKET_CONTROL0:u32, pub HDMI_GENERIC_PACKET_CONTROL1:u32,
    pub HDMI_GENERIC_PACKET_CONTROL2:u32, pub HDMI_GENERIC_PACKET_CONTROL3:u32, pub HDMI_GENERIC_PACKET_CONTROL4:u32,
    pub HDMI_GENERIC_PACKET_CONTROL5:u32, pub HDMI_INFOFRAME_CONTROL0:u32, pub HDMI_INFOFRAME_CONTROL1:u32,
    pub HDMI_VBI_PACKET_CONTROL:u32, pub HDMI_AUDIO_PACKET_CONTROL:u32, pub HDMI_ACR_PACKET_CONTROL:u32,
    pub HDMI_ACR_32_0:u32, pub HDMI_ACR_32_1:u32, pub HDMI_ACR_44_0:u32, pub HDMI_ACR_44_1:u32,
    pub HDMI_ACR_48_0:u32, pub HDMI_ACR_48_1:u32, pub DP_DB_CNTL:u32, pub DP_MSA_MISC:u32, pub DP_MSA_VBID_MISC:u32,
    pub DP_MSA_COLORIMETRY:u32, pub DP_MSA_TIMING_PARAM1:u32, pub DP_MSA_TIMING_PARAM2:u32, pub DP_MSA_TIMING_PARAM3:u32,
    pub DP_MSA_TIMING_PARAM4:u32, pub HDMI_DB_CONTROL:u32, pub DP_DSC_CNTL:u32, pub DP_DSC_BYTES_PER_PIXEL:u32,
    pub DME_CONTROL:u32, pub DP_SEC_METADATA_TRANSMISSION:u32, pub HDMI_METADATA_PACKET_CONTROL:u32, pub DP_SEC_FRAMING4:u32,
    pub DP_GSP11_CNTL:u32, pub HDMI_GENERIC_PACKET_CONTROL6:u32, pub HDMI_GENERIC_PACKET_CONTROL7:u32,
    pub HDMI_GENERIC_PACKET_CONTROL8:u32, pub HDMI_GENERIC_PACKET_CONTROL9:u32, pub HDMI_GENERIC_PACKET_CONTROL10:u32,
    pub DIG_CLOCK_PATTERN:u32, pub DIG_FIFO_CTRL0:u32, pub DIG_FE_CLK_CNTL:u32, pub DIG_FE_EN_CNTL:u32,
    pub STREAM_MAPPER_CONTROL:u32, pub DIG_FE_AUDIO_CNTL:u32,
}

macro_rules! SE_REG_FIELD_LIST_DCN1_0 { ($t:ty) => { $( $t )* }; }
// Field-name lists are represented explicitly by the following declaration macro.
macro_rules! se_fields {
    ($t:ty; $($n:ident),* $(,)?) => { $(pub $n:$t,)* };
}

#[repr(C)]
pub struct dcn10_stream_encoder_shift {
    se_fields!(u8; AFMT_GENERIC_INDEX, AFMT_GENERIC_HB0, AFMT_GENERIC_HB1, AFMT_GENERIC_HB2, AFMT_GENERIC_HB3,
    AFMT_GENERIC_LOCK_STATUS, AFMT_GENERIC_CONFLICT, AFMT_GENERIC_CONFLICT_CLR, AFMT_GENERIC0_FRAME_UPDATE_PENDING,
    AFMT_GENERIC1_FRAME_UPDATE_PENDING, AFMT_GENERIC2_FRAME_UPDATE_PENDING, AFMT_GENERIC3_FRAME_UPDATE_PENDING,
    AFMT_GENERIC4_FRAME_UPDATE_PENDING, AFMT_GENERIC4_IMMEDIATE_UPDATE_PENDING, AFMT_GENERIC5_FRAME_UPDATE_PENDING,
    AFMT_GENERIC6_FRAME_UPDATE_PENDING, AFMT_GENERIC7_FRAME_UPDATE_PENDING, AFMT_GENERIC0_FRAME_UPDATE,
    AFMT_GENERIC1_FRAME_UPDATE, AFMT_GENERIC2_FRAME_UPDATE, AFMT_GENERIC3_FRAME_UPDATE, AFMT_GENERIC4_FRAME_UPDATE,
    AFMT_GENERIC0_IMMEDIATE_UPDATE, AFMT_GENERIC1_IMMEDIATE_UPDATE, AFMT_GENERIC2_IMMEDIATE_UPDATE,
    AFMT_GENERIC3_IMMEDIATE_UPDATE, AFMT_GENERIC4_IMMEDIATE_UPDATE, AFMT_GENERIC5_IMMEDIATE_UPDATE,
    AFMT_GENERIC6_IMMEDIATE_UPDATE, AFMT_GENERIC7_IMMEDIATE_UPDATE, AFMT_GENERIC5_FRAME_UPDATE,
    AFMT_GENERIC6_FRAME_UPDATE, AFMT_GENERIC7_FRAME_UPDATE, HDMI_GENERIC0_CONT, HDMI_GENERIC0_SEND, HDMI_GENERIC0_LINE,
    HDMI_GENERIC1_CONT, HDMI_GENERIC1_SEND, HDMI_GENERIC1_LINE, DP_PIXEL_ENCODING, DP_COMPONENT_DEPTH,
    HDMI_PACKET_GEN_VERSION, HDMI_KEEPOUT_MODE, HDMI_DEEP_COLOR_ENABLE, HDMI_CLOCK_CHANNEL_RATE,
    HDMI_DEEP_COLOR_DEPTH, HDMI_GC_CONT, HDMI_GC_SEND, HDMI_NULL_SEND, HDMI_DATA_SCRAMBLE_EN,
    HDMI_NO_EXTRA_NULL_PACKET_FILLED, HDMI_AUDIO_INFO_SEND, AFMT_AUDIO_INFO_UPDATE, HDMI_AUDIO_INFO_LINE, HDMI_GC_AVMUTE,
    DP_MSE_RATE_X, DP_MSE_RATE_Y, DP_MSE_RATE_UPDATE_PENDING, DP_SEC_GSP0_ENABLE, DP_SEC_STREAM_ENABLE,
    DP_SEC_GSP1_ENABLE, DP_SEC_GSP2_ENABLE, DP_SEC_GSP3_ENABLE, DP_SEC_GSP4_ENABLE, DP_SEC_GSP5_ENABLE,
    DP_SEC_GSP6_ENABLE, DP_SEC_GSP7_ENABLE, DP_SEC_GSP7_PPS, DP_SEC_GSP7_SEND, DP_SEC_GSP4_SEND,
    DP_SEC_GSP4_SEND_PENDING, DP_SEC_GSP4_LINE_NUM, DP_SEC_GSP4_SEND_ANY_LINE, DP_SEC_MPG_ENABLE,
    DP_VID_STREAM_DIS_DEFER, DP_VID_STREAM_ENABLE, DP_VID_STREAM_STATUS, DP_STEER_FIFO_RESET, DP_VID_M_N_GEN_EN,
    DP_VID_N, DP_VID_M, DIG_START, AFMT_AUDIO_SRC_SELECT, AFMT_AUDIO_CHANNEL_ENABLE, HDMI_AUDIO_PACKETS_PER_LINE,
    HDMI_AUDIO_DELAY_EN, AFMT_60958_CS_UPDATE, AFMT_AUDIO_LAYOUT_OVRD, AFMT_60958_OSF_OVRD, HDMI_ACR_AUTO_SEND,
    HDMI_ACR_SOURCE, HDMI_ACR_AUDIO_PRIORITY, HDMI_ACR_CTS_32, HDMI_ACR_N_32, HDMI_ACR_CTS_44, HDMI_ACR_N_44,
    HDMI_ACR_CTS_48, HDMI_ACR_N_48, AFMT_60958_CS_CHANNEL_NUMBER_L, AFMT_60958_CS_CLOCK_ACCURACY,
    AFMT_60958_CS_CHANNEL_NUMBER_R, AFMT_AUDIO_SAMPLE_SEND, AFMT_AUDIO_CLOCK_EN, TMDS_PIXEL_ENCODING,
    TMDS_COLOR_FORMAT, DIG_STEREOSYNC_SELECT, DIG_STEREOSYNC_GATE_EN, DP_DB_DISABLE, DP_MSA_MISC0, DP_MSA_HTOTAL,
    DP_MSA_VTOTAL, DP_MSA_HSTART, DP_MSA_VSTART, DP_MSA_HSYNCWIDTH, DP_MSA_HSYNCPOLARITY, DP_MSA_VSYNCWIDTH,
    DP_MSA_VSYNCPOLARITY, DP_MSA_HWIDTH, DP_MSA_VHEIGHT, HDMI_DB_DISABLE, DP_VID_N_MUL, DIG_SOURCE_SELECT,
    DIG_FIFO_LEVEL_ERROR, DIG_FIFO_USE_OVERWRITE_LEVEL, DIG_FIFO_OVERWRITE_LEVEL, DIG_FIFO_ERROR_ACK,
    DIG_FIFO_CAL_AVERAGE_LEVEL, DIG_FIFO_MAXIMUM_LEVEL, DIG_FIFO_MINIMUM_LEVEL, DIG_FIFO_READ_CLOCK_SRC,
    DIG_FIFO_CALIBRATED, DIG_FIFO_FORCE_RECAL_AVERAGE, DIG_FIFO_FORCE_RECOMP_MINMAX, SE_CLOCK_PATTERN);
    pub HDMI_ACP_SEND:u8,
    se_fields!(u8; DP_DSC_MODE, DP_DSC_SLICE_WIDTH, DP_DSC_BYTES_PER_PIXEL, DP_VBID6_LINE_REFERENCE, DP_VBID6_LINE_NUM,
    METADATA_ENGINE_EN, METADATA_HUBP_REQUESTOR_ID, METADATA_STREAM_TYPE, DP_SEC_METADATA_PACKET_ENABLE,
    DP_SEC_METADATA_PACKET_LINE_REFERENCE, DP_SEC_METADATA_PACKET_LINE, HDMI_METADATA_PACKET_ENABLE,
    HDMI_METADATA_PACKET_LINE_REFERENCE, HDMI_METADATA_PACKET_LINE, DOLBY_VISION_EN, DP_PIXEL_COMBINE, DP_SST_SDP_SPLITTING,
    HDMI_GENERIC8_CONT, HDMI_GENERIC8_SEND, HDMI_GENERIC8_LINE, DIG_FIFO_OUTPUT_PIXEL_MODE,
    DP_PIXEL_PER_CYCLE_PROCESSING_MODE, DIG_SYMCLK_FE_ON, DIG_FIFO_READ_START_LEVEL, DIG_FIFO_ENABLE, DIG_FIFO_RESET,
    DIG_FIFO_RESET_DONE, PIXEL_ENCODING_TYPE, UNCOMPRESSED_PIXEL_FORMAT, UNCOMPRESSED_COMPONENT_DEPTH,
    DIG_FE_CLK_EN, DIG_FE_MODE, DIG_FE_SOFT_RESET, DIG_FE_ENABLE, DIG_FE_SYMCLK_FE_G_CLOCK_ON,
    DIG_FE_DISPCLK_G_CLOCK_ON, DIG_FE_SYMCLK_FE_G_AFMT_CLOCK_ON, DIG_FE_SYMCLK_FE_G_TMDS_CLOCK_ON,
    DIG_FE_SOCCLK_G_AFMT_CLOCK_ON, DIG_STREAM_LINK_TARGET);
}

#[repr(C)]
pub struct dcn10_stream_encoder_mask { pub fields: dcn10_stream_encoder_shift }

#[repr(C)]
pub struct dcn10_stream_encoder { pub base: stream_encoder, pub regs:*const dcn10_stream_enc_registers,
    pub se_shift:*const dcn10_stream_encoder_shift, pub se_mask:*const dcn10_stream_encoder_mask }

extern "C" {
    pub fn dcn10_stream_encoder_construct(enc1:*mut dcn10_stream_encoder, ctx:*mut dc_context, bp:*mut dc_bios,
        eng_id:engine_id, regs:*const dcn10_stream_enc_registers, se_shift:*const dcn10_stream_encoder_shift,
        se_mask:*const dcn10_stream_encoder_mask);
    pub fn enc1_update_generic_info_packet(enc1:*mut dcn10_stream_encoder, packet_index:u32, info_packet:*const dc_info_packet);
    pub fn enc1_stream_encoder_dp_set_stream_attribute(enc:*mut stream_encoder, crtc_timing:*mut dc_crtc_timing,
        output_color_space:dc_color_space, use_vsc_sdp_for_colorimetry:bool, enable_sdp_splitting:u32);
    pub fn enc1_stream_encoder_hdmi_set_stream_attribute(enc:*mut stream_encoder, crtc_timing:*mut dc_crtc_timing, actual_pix_clk_khz:i32, enable_audio:bool);
    pub fn enc1_stream_encoder_dvi_set_stream_attribute(enc:*mut stream_encoder, crtc_timing:*mut dc_crtc_timing, is_dual_link:bool);
    pub fn enc1_stream_encoder_set_throttled_vcp_size(enc:*mut stream_encoder, avg_time_slots_per_mtp:fixed31_32);
    pub fn enc1_stream_encoder_update_dp_info_packets(enc:*mut stream_encoder, info_frame:*const encoder_info_frame);
    pub fn enc1_stream_encoder_send_immediate_sdp_message(enc:*mut stream_encoder, custom_sdp_message:*const u8, sdp_message_size:c_uint);
    pub fn enc1_stream_encoder_stop_dp_info_packets(enc:*mut stream_encoder);
    pub fn enc1_stream_encoder_dp_blank(link:*mut dc_link, enc:*mut stream_encoder);
    pub fn enc1_stream_encoder_dp_unblank(link:*mut dc_link, enc:*mut stream_encoder, param:*const encoder_unblank_param);
    pub fn enc1_setup_stereo_sync(enc:*mut stream_encoder, tg_inst:i32, enable:bool);
    pub fn enc1_stream_encoder_set_avmute(enc:*mut stream_encoder, enable:bool);
    pub fn enc1_se_audio_mute_control(enc:*mut stream_encoder, mute:bool);
    pub fn enc1_se_dp_audio_setup(enc:*mut stream_encoder, az_inst:c_uint, info:*mut audio_info);
    pub fn enc1_se_dp_audio_enable(enc:*mut stream_encoder); pub fn enc1_se_dp_audio_disable(enc:*mut stream_encoder);
    pub fn enc1_se_hdmi_audio_setup(enc:*mut stream_encoder, az_inst:c_uint, info:*mut audio_info, audio_crtc_info:*mut audio_crtc_info);
    pub fn enc1_se_hdmi_audio_disable(enc:*mut stream_encoder); pub fn enc1_dig_connect_to_otg(enc:*mut stream_encoder, tg_inst:i32);
    pub fn enc1_dig_source_otg(enc:*mut stream_encoder)->c_uint;
    pub fn enc1_stream_encoder_set_stream_attribute_helper(enc1:*mut dcn10_stream_encoder, crtc_timing:*mut dc_crtc_timing);
    pub fn enc1_se_enable_audio_clock(enc:*mut stream_encoder, enable:bool); pub fn enc1_se_enable_dp_audio(enc:*mut stream_encoder);
    pub fn get_audio_clock_info(color_depth:dc_color_depth, crtc_pixel_clock_100Hz:u32, actual_pixel_clock_100Hz:u32, audio_clock_info:*mut audio_clock_info);
    pub fn enc1_reset_hdmi_stream_attribute(enc:*mut stream_encoder);
    pub fn enc1_stream_encoder_dp_get_pixel_format(enc:*mut stream_encoder, encoding:*mut dc_pixel_encoding, depth:*mut dc_color_depth)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
