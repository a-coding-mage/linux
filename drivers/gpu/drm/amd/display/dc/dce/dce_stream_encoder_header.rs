/*
 * Rust translation of dce_stream_encoder.h.
 *
 * The C register-list and field-list macros depend on the register-generation
 * macros and token-pasting environment supplied by the including translation
 * unit; their source-level intent is retained below as declarative Rust
 * macros.  External types and functions remain external dependencies.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code)]

pub const TMDS_CNTL__TMDS_PIXEL_ENCODING_MASK: u32 = 0x00000010;
pub const TMDS_CNTL__TMDS_COLOR_FORMAT_MASK: u32 = 0x00000300;
pub const TMDS_CNTL__TMDS_PIXEL_ENCODING__SHIFT: u32 = 0x00000004;
pub const TMDS_CNTL__TMDS_COLOR_FORMAT__SHIFT: u32 = 0x00000008;

/* C preprocessor register-list macros (SE_COMMON_REG_LIST_*, SE_DCN_REG_LIST)
 * require SRI/SR from the target register-definition environment. */
macro_rules! SE_SF { ($reg:ident, $field:ident, $post:ident) => { ($reg, $field, $post) }; }
macro_rules! DCE110STRENC_FROM_STRENC {
    ($stream_encoder:expr) => { unsafe { &mut *((($stream_encoder as *mut u8).sub(0)) as *mut dce110_stream_encoder) } };
}

#[repr(C)]
pub struct dce_stream_encoder_shift {
    pub AFMT_GENERIC_INDEX: u8, pub AFMT_GENERIC0_UPDATE: u8, pub AFMT_GENERIC2_UPDATE: u8,
    pub AFMT_GENERIC_HB0: u8, pub AFMT_GENERIC_HB1: u8, pub AFMT_GENERIC_HB2: u8, pub AFMT_GENERIC_HB3: u8,
    pub AFMT_GENERIC_LOCK_STATUS: u8, pub AFMT_GENERIC_CONFLICT: u8, pub AFMT_GENERIC_CONFLICT_CLR: u8,
    pub AFMT_GENERIC0_FRAME_UPDATE_PENDING: u8, pub AFMT_GENERIC1_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC2_FRAME_UPDATE_PENDING: u8, pub AFMT_GENERIC3_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC4_FRAME_UPDATE_PENDING: u8, pub AFMT_GENERIC5_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC6_FRAME_UPDATE_PENDING: u8, pub AFMT_GENERIC7_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC0_FRAME_UPDATE: u8, pub AFMT_GENERIC1_FRAME_UPDATE: u8, pub AFMT_GENERIC2_FRAME_UPDATE: u8,
    pub AFMT_GENERIC3_FRAME_UPDATE: u8, pub AFMT_GENERIC4_FRAME_UPDATE: u8, pub AFMT_GENERIC5_FRAME_UPDATE: u8,
    pub AFMT_GENERIC6_FRAME_UPDATE: u8, pub AFMT_GENERIC7_FRAME_UPDATE: u8,
    pub HDMI_GENERIC0_CONT: u8, pub HDMI_GENERIC0_SEND: u8, pub HDMI_GENERIC0_LINE: u8,
    pub HDMI_GENERIC1_CONT: u8, pub HDMI_GENERIC1_SEND: u8, pub HDMI_GENERIC1_LINE: u8,
    pub DP_PIXEL_ENCODING: u8, pub DP_COMPONENT_DEPTH: u8, pub DP_DYN_RANGE: u8, pub DP_YCBCR_RANGE: u8,
    pub HDMI_PACKET_GEN_VERSION: u8, pub HDMI_KEEPOUT_MODE: u8, pub HDMI_DEEP_COLOR_ENABLE: u8,
    pub HDMI_CLOCK_CHANNEL_RATE: u8, pub HDMI_DEEP_COLOR_DEPTH: u8, pub HDMI_GC_CONT: u8, pub HDMI_GC_SEND: u8,
    pub HDMI_NULL_SEND: u8, pub HDMI_DATA_SCRAMBLE_EN: u8, pub HDMI_ACP_SEND: u8,
    pub HDMI_AUDIO_INFO_SEND: u8, pub AFMT_AUDIO_INFO_UPDATE: u8, pub HDMI_AUDIO_INFO_LINE: u8,
    pub HDMI_GC_AVMUTE: u8, pub DP_MSE_RATE_X: u8, pub DP_MSE_RATE_Y: u8, pub DP_MSE_RATE_UPDATE_PENDING: u8,
    pub AFMT_AVI_INFO_VERSION: u8, pub HDMI_AVI_INFO_SEND: u8, pub HDMI_AVI_INFO_CONT: u8, pub HDMI_AVI_INFO_LINE: u8,
    pub DP_SEC_GSP0_ENABLE: u8, pub DP_SEC_STREAM_ENABLE: u8, pub DP_SEC_GSP1_ENABLE: u8, pub DP_SEC_GSP2_ENABLE: u8,
    pub DP_SEC_GSP3_ENABLE: u8, pub DP_SEC_GSP4_ENABLE: u8, pub DP_SEC_GSP5_ENABLE: u8, pub DP_SEC_GSP6_ENABLE: u8,
    pub DP_SEC_GSP7_ENABLE: u8, pub DP_SEC_AVI_ENABLE: u8, pub DP_SEC_MPG_ENABLE: u8,
    pub DP_VID_STREAM_DIS_DEFER: u8, pub DP_VID_STREAM_ENABLE: u8, pub DP_VID_STREAM_STATUS: u8,
    pub DP_STEER_FIFO_RESET: u8, pub DP_VID_M_N_GEN_EN: u8, pub DP_VID_N: u8, pub DP_VID_M: u8, pub DIG_START: u8,
    pub AFMT_AUDIO_SRC_SELECT: u8, pub AFMT_AUDIO_CHANNEL_ENABLE: u8, pub HDMI_AUDIO_PACKETS_PER_LINE: u8,
    pub HDMI_AUDIO_DELAY_EN: u8, pub AFMT_60958_CS_UPDATE: u8, pub AFMT_AUDIO_LAYOUT_OVRD: u8,
    pub AFMT_60958_OSF_OVRD: u8, pub HDMI_ACR_AUTO_SEND: u8, pub HDMI_ACR_SOURCE: u8,
    pub HDMI_ACR_AUDIO_PRIORITY: u8, pub HDMI_ACR_CTS_32: u8, pub HDMI_ACR_N_32: u8, pub HDMI_ACR_CTS_44: u8,
    pub HDMI_ACR_N_44: u8, pub HDMI_ACR_CTS_48: u8, pub HDMI_ACR_N_48: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_L: u8, pub AFMT_60958_CS_CLOCK_ACCURACY: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_R: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_2: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_3: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_4: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_5: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_6: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_7: u8, pub DP_SEC_AUD_N: u8, pub DP_SEC_TIMESTAMP_MODE: u8,
    pub DP_SEC_ASP_ENABLE: u8, pub DP_SEC_ATP_ENABLE: u8, pub DP_SEC_AIP_ENABLE: u8, pub DP_SEC_ACM_ENABLE: u8,
    pub AFMT_AUDIO_SAMPLE_SEND: u8, pub AFMT_AUDIO_CLOCK_EN: u8, pub TMDS_PIXEL_ENCODING: u8,
    pub TMDS_COLOR_FORMAT: u8, pub DIG_STEREOSYNC_SELECT: u8, pub DIG_STEREOSYNC_GATE_EN: u8,
    pub DP_DB_DISABLE: u8, pub DP_MSA_MISC0: u8, pub DP_MSA_HTOTAL: u8, pub DP_MSA_VTOTAL: u8,
    pub DP_MSA_HSTART: u8, pub DP_MSA_VSTART: u8, pub DP_MSA_HSYNCWIDTH: u8, pub DP_MSA_HSYNCPOLARITY: u8,
    pub DP_MSA_VSYNCWIDTH: u8, pub DP_MSA_VSYNCPOLARITY: u8, pub DP_MSA_HWIDTH: u8, pub DP_MSA_VHEIGHT: u8,
    pub HDMI_DB_DISABLE: u8, pub DP_VID_N_MUL: u8, pub DP_VID_M_DOUBLE_VALUE_EN: u8,
    pub DIG_SOURCE_SELECT: u8, pub DAC_SOURCE_SELECT: u8,
}

/* The mask has the identical field set and C uint32_t representation. */
pub type dce_stream_encoder_mask = dce_stream_encoder_shift;

#[repr(C)]
pub struct dce110_stream_enc_registers {
    pub AFMT_CNTL: u32, pub AFMT_AVI_INFO0: u32, pub AFMT_AVI_INFO1: u32, pub AFMT_AVI_INFO2: u32,
    pub AFMT_AVI_INFO3: u32, pub AFMT_GENERIC_0: u32, pub AFMT_GENERIC_1: u32, pub AFMT_GENERIC_2: u32,
    pub AFMT_GENERIC_3: u32, pub AFMT_GENERIC_4: u32, pub AFMT_GENERIC_5: u32, pub AFMT_GENERIC_6: u32,
    pub AFMT_GENERIC_7: u32, pub AFMT_GENERIC_HDR: u32, pub AFMT_INFOFRAME_CONTROL0: u32,
    pub AFMT_VBI_PACKET_CONTROL: u32, pub AFMT_VBI_PACKET_CONTROL1: u32, pub AFMT_AUDIO_PACKET_CONTROL: u32,
    pub AFMT_AUDIO_PACKET_CONTROL2: u32, pub AFMT_AUDIO_SRC_CONTROL: u32, pub AFMT_60958_0: u32,
    pub AFMT_60958_1: u32, pub AFMT_60958_2: u32, pub DIG_FE_CNTL: u32, pub DAC_SOURCE_SELECT: u32,
    pub DP_MSE_RATE_CNTL: u32, pub DP_MSE_RATE_UPDATE: u32, pub DP_PIXEL_FORMAT: u32, pub DP_SEC_CNTL: u32,
    pub DP_STEER_FIFO: u32, pub DP_VID_M: u32, pub DP_VID_N: u32, pub DP_VID_STREAM_CNTL: u32,
    pub DP_VID_TIMING: u32, pub DP_SEC_AUD_N: u32, pub DP_SEC_TIMESTAMP: u32, pub HDMI_CONTROL: u32,
    pub HDMI_GC: u32, pub HDMI_GENERIC_PACKET_CONTROL0: u32, pub HDMI_GENERIC_PACKET_CONTROL1: u32,
    pub HDMI_GENERIC_PACKET_CONTROL2: u32, pub HDMI_GENERIC_PACKET_CONTROL3: u32, pub HDMI_INFOFRAME_CONTROL0: u32,
    pub HDMI_INFOFRAME_CONTROL1: u32, pub HDMI_VBI_PACKET_CONTROL: u32, pub HDMI_AUDIO_PACKET_CONTROL: u32,
    pub HDMI_ACR_PACKET_CONTROL: u32, pub HDMI_ACR_32_0: u32, pub HDMI_ACR_32_1: u32, pub HDMI_ACR_44_0: u32,
    pub HDMI_ACR_44_1: u32, pub HDMI_ACR_48_0: u32, pub HDMI_ACR_48_1: u32, pub TMDS_CNTL: u32,
    pub DP_DB_CNTL: u32, pub DP_MSA_MISC: u32, pub DP_MSA_COLORIMETRY: u32, pub DP_MSA_TIMING_PARAM1: u32,
    pub DP_MSA_TIMING_PARAM2: u32, pub DP_MSA_TIMING_PARAM3: u32, pub DP_MSA_TIMING_PARAM4: u32,
    pub HDMI_DB_CONTROL: u32,
}

#[repr(C)]
pub struct dce110_stream_encoder {
    pub base: stream_encoder,
    pub regs: *const dce110_stream_enc_registers,
    pub se_shift: *const dce_stream_encoder_shift,
    pub se_mask: *const dce_stream_encoder_mask,
}

/* External declarations supplied by stream_encoder.h and the implementation. */
extern "C" {
    pub fn dce110_stream_encoder_construct(enc110: *mut dce110_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, regs: *const dce110_stream_enc_registers, se_shift: *const dce_stream_encoder_shift, se_mask: *const dce_stream_encoder_mask);
    pub fn dce110_analog_stream_encoder_construct(enc110: *mut dce110_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, regs: *const dce110_stream_enc_registers, se_shift: *const dce_stream_encoder_shift, se_mask: *const dce_stream_encoder_mask);
    pub fn dce110_se_audio_mute_control(enc: *mut stream_encoder, mute: bool);
    pub fn dce110_se_dp_audio_setup(enc: *mut stream_encoder, az_inst: c_uint, info: *mut audio_info);
    pub fn dce110_se_dp_audio_enable(enc: *mut stream_encoder);
    pub fn dce110_se_dp_audio_disable(enc: *mut stream_encoder);
    pub fn dce110_se_hdmi_audio_setup(enc: *mut stream_encoder, az_inst: c_uint, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info);
    pub fn dce110_se_hdmi_audio_disable(enc: *mut stream_encoder);
}

use core::ffi::c_uint;
extern "C" { pub type stream_encoder; pub type dc_context; pub type dc_bios; pub type audio_info; pub type audio_crtc_info; pub type engine_id; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
