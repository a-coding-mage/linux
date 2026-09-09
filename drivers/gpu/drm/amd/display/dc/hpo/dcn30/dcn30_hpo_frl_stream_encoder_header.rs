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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// dcn30/dcn30_vpg.h, dcn30/dcn30_afmt.h, stream_encoder.h,
// dml/dml1_frl_cap_chk.h

#[repr(C)]
pub struct dcn30_hpo_frl_stream_enc_registers {
    pub HDMI_STREAM_ENC_CLOCK_CONTROL: u32,
    pub HDMI_STREAM_ENC_INPUT_MUX_CONTROL: u32,
    pub HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0: u32,
    pub HDMI_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL2: u32,
    pub HDMI_STREAM_ENC_AUDIO_CONTROL: u32,
    pub HDMI_TB_ENC_CONTROL: u32,
    pub HDMI_TB_ENC_MODE: u32,
    pub HDMI_TB_ENC_H_ACTIVE_BLANK: u32,
    pub HDMI_TB_ENC_HC_ACTIVE_BLANK: u32,
    pub HDMI_TB_ENC_PACKET_CONTROL: u32,
    pub HDMI_TB_ENC_DB_CONTROL: u32,
    pub HDMI_TB_ENC_PIXEL_FORMAT: u32,
    pub HDMI_TB_ENC_VBI_PACKET_CONTROL1: u32,
    pub HDMI_TB_ENC_GC_CONTROL: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET_CONTROL0: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET_CONTROL1: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET0_1_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET2_3_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET4_5_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET6_7_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET8_9_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET10_11_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET12_13_LINE: u32,
    pub HDMI_TB_ENC_GENERIC_PACKET14_LINE: u32,
    pub HDMI_TB_ENC_ACR_PACKET_CONTROL: u32,
    pub HDMI_TB_ENC_ACR_32_0: u32,
    pub HDMI_TB_ENC_ACR_32_1: u32,
    pub HDMI_TB_ENC_ACR_44_0: u32,
    pub HDMI_TB_ENC_ACR_44_1: u32,
    pub HDMI_TB_ENC_ACR_48_0: u32,
    pub HDMI_TB_ENC_ACR_48_1: u32,
    pub HDMI_TB_ENC_CRC_CNTL: u32,
    pub DME_CONTROL: u32,
    pub HDMI_TB_ENC_METADATA_PACKET_CONTROL: u32,
    pub HDMI_TB_ENC_MEM_CTRL: u32,
    pub HDMI_FRL_ENC_MEM_CTRL: u32,
}

#[repr(C)]
pub struct dcn30_hpo_frl_stream_encoder_shift {
    pub HDMI_TB_ENC_EN: u8, pub HDMI_RESET: u8, pub HDMI_RESET_DONE: u8,
    pub HDMI_STREAM_ENC_CLOCK_EN: u8, pub HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL: u8,
    pub HDMI_MAX_PACKETS_PER_LINE: u8, pub FIFO_ENABLE: u8, pub FIFO_RESET: u8,
    pub FIFO_RESET_DONE: u8, pub FIFO_PIXEL_ENCODING: u8, pub FIFO_ODM_COMBINE_MODE: u8,
    pub FIFO_DSC_MODE: u8, pub FIFO_DB_DISABLE: u8, pub HDMI_BORROW_MODE: u8,
    pub HDMI_H_ACTIVE: u8, pub HDMI_H_BLANK: u8, pub HDMI_HC_ACTIVE: u8, pub HDMI_HC_BLANK: u8,
    pub HDMI_DB_DISABLE: u8, pub HDMI_PIXEL_ENCODING: u8, pub HDMI_DEEP_COLOR_DEPTH: u8,
    pub HDMI_DEEP_COLOR_ENABLE: u8, pub HDMI_ODM_COMBINE_MODE: u8, pub HDMI_DSC_MODE: u8,
    pub HDMI_GC_CONT: u8, pub HDMI_GC_SEND: u8, pub HDMI_ACP_SEND: u8,
    pub HDMI_AUDIO_INFO_SEND: u8, pub HDMI_AUDIO_INFO_LINE: u8, pub HDMI_GC_AVMUTE: u8,
    pub HDMI_GENERIC0_CONT: u8, pub HDMI_GENERIC0_SEND: u8, pub HDMI_GENERIC0_LINE: u8,
    pub HDMI_GENERIC1_CONT: u8, pub HDMI_GENERIC1_SEND: u8, pub HDMI_GENERIC1_LINE: u8,
    pub HDMI_GENERIC2_CONT: u8, pub HDMI_GENERIC2_SEND: u8, pub HDMI_GENERIC2_LINE: u8,
    pub HDMI_GENERIC3_CONT: u8, pub HDMI_GENERIC3_SEND: u8, pub HDMI_GENERIC3_LINE: u8,
    pub HDMI_GENERIC4_CONT: u8, pub HDMI_GENERIC4_SEND: u8, pub HDMI_GENERIC4_LINE: u8,
    pub HDMI_GENERIC5_CONT: u8, pub HDMI_GENERIC5_SEND: u8, pub HDMI_GENERIC5_LINE: u8,
    pub HDMI_GENERIC6_CONT: u8, pub HDMI_GENERIC6_SEND: u8, pub HDMI_GENERIC6_LINE: u8,
    pub HDMI_GENERIC7_CONT: u8, pub HDMI_GENERIC7_SEND: u8, pub HDMI_GENERIC7_LINE: u8,
    pub HDMI_GENERIC8_CONT: u8, pub HDMI_GENERIC8_SEND: u8, pub HDMI_GENERIC8_LINE: u8,
    pub HDMI_GENERIC9_CONT: u8, pub HDMI_GENERIC9_SEND: u8, pub HDMI_GENERIC9_LINE: u8,
    pub HDMI_GENERIC10_CONT: u8, pub HDMI_GENERIC10_SEND: u8, pub HDMI_GENERIC10_LINE: u8,
    pub HDMI_GENERIC11_CONT: u8, pub HDMI_GENERIC11_SEND: u8, pub HDMI_GENERIC11_LINE: u8,
    pub HDMI_GENERIC12_CONT: u8, pub HDMI_GENERIC12_SEND: u8, pub HDMI_GENERIC12_LINE: u8,
    pub HDMI_GENERIC13_CONT: u8, pub HDMI_GENERIC13_SEND: u8, pub HDMI_GENERIC13_LINE: u8,
    pub HDMI_GENERIC14_CONT: u8, pub HDMI_GENERIC14_SEND: u8, pub HDMI_GENERIC14_LINE: u8,
    pub HDMI_ACR_AUTO_SEND: u8, pub HDMI_ACR_SOURCE: u8, pub HDMI_ACR_AUDIO_PRIORITY: u8,
    pub HDMI_ACR_CTS_32: u8, pub HDMI_ACR_N_32: u8, pub HDMI_ACR_CTS_44: u8,
    pub HDMI_ACR_N_44: u8, pub HDMI_ACR_CTS_48: u8, pub HDMI_ACR_N_48: u8,
    pub HDMI_CRC_EN: u8, pub HDMI_CRC_CONT_EN: u8, pub METADATA_HUBP_REQUESTOR_ID: u8,
    pub METADATA_ENGINE_EN: u8, pub METADATA_STREAM_TYPE: u8, pub HDMI_METADATA_PACKET_ENABLE: u8,
    pub HDMI_METADATA_PACKET_LINE_REFERENCE: u8, pub HDMI_METADATA_PACKET_MISSED: u8,
    pub HDMI_METADATA_PACKET_LINE: u8, pub BORROWBUFFER_MEM_PWR_DIS: u8,
    pub BORROWBUFFER_MEM_PWR_FORCE: u8, pub BORROWBUFFER_MEM_PWR_STATE: u8,
    pub BORROWBUFFER_MEM_DEFAULT_MEM_LOW_POWER_STATE: u8,
}

#[repr(C)]
pub struct dcn30_hpo_frl_stream_encoder_mask {
    pub HDMI_TB_ENC_EN: u32, pub HDMI_RESET: u32, pub HDMI_RESET_DONE: u32,
    pub HDMI_STREAM_ENC_CLOCK_EN: u32, pub HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL: u32,
    pub HDMI_MAX_PACKETS_PER_LINE: u32, pub FIFO_ENABLE: u32, pub FIFO_RESET: u32,
    pub FIFO_RESET_DONE: u32, pub FIFO_PIXEL_ENCODING: u32, pub FIFO_ODM_COMBINE_MODE: u32,
    pub FIFO_DSC_MODE: u32, pub FIFO_DB_DISABLE: u32, pub HDMI_BORROW_MODE: u32,
    pub HDMI_H_ACTIVE: u32, pub HDMI_H_BLANK: u32, pub HDMI_HC_ACTIVE: u32, pub HDMI_HC_BLANK: u32,
    pub HDMI_DB_DISABLE: u32, pub HDMI_PIXEL_ENCODING: u32, pub HDMI_DEEP_COLOR_DEPTH: u32,
    pub HDMI_DEEP_COLOR_ENABLE: u32, pub HDMI_ODM_COMBINE_MODE: u32, pub HDMI_DSC_MODE: u32,
    pub HDMI_GC_CONT: u32, pub HDMI_GC_SEND: u32, pub HDMI_ACP_SEND: u32,
    pub HDMI_AUDIO_INFO_SEND: u32, pub HDMI_AUDIO_INFO_LINE: u32, pub HDMI_GC_AVMUTE: u32,
    pub HDMI_GENERIC0_CONT: u32, pub HDMI_GENERIC0_SEND: u32, pub HDMI_GENERIC0_LINE: u32,
    pub HDMI_GENERIC1_CONT: u32, pub HDMI_GENERIC1_SEND: u32, pub HDMI_GENERIC1_LINE: u32,
    pub HDMI_GENERIC2_CONT: u32, pub HDMI_GENERIC2_SEND: u32, pub HDMI_GENERIC2_LINE: u32,
    pub HDMI_GENERIC3_CONT: u32, pub HDMI_GENERIC3_SEND: u32, pub HDMI_GENERIC3_LINE: u32,
    pub HDMI_GENERIC4_CONT: u32, pub HDMI_GENERIC4_SEND: u32, pub HDMI_GENERIC4_LINE: u32,
    pub HDMI_GENERIC5_CONT: u32, pub HDMI_GENERIC5_SEND: u32, pub HDMI_GENERIC5_LINE: u32,
    pub HDMI_GENERIC6_CONT: u32, pub HDMI_GENERIC6_SEND: u32, pub HDMI_GENERIC6_LINE: u32,
    pub HDMI_GENERIC7_CONT: u32, pub HDMI_GENERIC7_SEND: u32, pub HDMI_GENERIC7_LINE: u32,
    pub HDMI_GENERIC8_CONT: u32, pub HDMI_GENERIC8_SEND: u32, pub HDMI_GENERIC8_LINE: u32,
    pub HDMI_GENERIC9_CONT: u32, pub HDMI_GENERIC9_SEND: u32, pub HDMI_GENERIC9_LINE: u32,
    pub HDMI_GENERIC10_CONT: u32, pub HDMI_GENERIC10_SEND: u32, pub HDMI_GENERIC10_LINE: u32,
    pub HDMI_GENERIC11_CONT: u32, pub HDMI_GENERIC11_SEND: u32, pub HDMI_GENERIC11_LINE: u32,
    pub HDMI_GENERIC12_CONT: u32, pub HDMI_GENERIC12_SEND: u32, pub HDMI_GENERIC12_LINE: u32,
    pub HDMI_GENERIC13_CONT: u32, pub HDMI_GENERIC13_SEND: u32, pub HDMI_GENERIC13_LINE: u32,
    pub HDMI_GENERIC14_CONT: u32, pub HDMI_GENERIC14_SEND: u32, pub HDMI_GENERIC14_LINE: u32,
    pub HDMI_ACR_AUTO_SEND: u32, pub HDMI_ACR_SOURCE: u32, pub HDMI_ACR_AUDIO_PRIORITY: u32,
    pub HDMI_ACR_CTS_32: u32, pub HDMI_ACR_N_32: u32, pub HDMI_ACR_CTS_44: u32,
    pub HDMI_ACR_N_44: u32, pub HDMI_ACR_CTS_48: u32, pub HDMI_ACR_N_48: u32,
    pub HDMI_CRC_EN: u32, pub HDMI_CRC_CONT_EN: u32, pub METADATA_HUBP_REQUESTOR_ID: u32,
    pub METADATA_ENGINE_EN: u32, pub METADATA_STREAM_TYPE: u32, pub HDMI_METADATA_PACKET_ENABLE: u32,
    pub HDMI_METADATA_PACKET_LINE_REFERENCE: u32, pub HDMI_METADATA_PACKET_MISSED: u32,
    pub HDMI_METADATA_PACKET_LINE: u32, pub BORROWBUFFER_MEM_PWR_DIS: u32,
    pub BORROWBUFFER_MEM_PWR_FORCE: u32, pub BORROWBUFFER_MEM_PWR_STATE: u32,
    pub BORROWBUFFER_MEM_DEFAULT_MEM_LOW_POWER_STATE: u32,
}

#[repr(C)]
pub struct dcn30_hpo_frl_stream_encoder {
    pub base: hpo_frl_stream_encoder,
    pub regs: *const dcn30_hpo_frl_stream_enc_registers,
    pub hpo_se_shift: *const dcn30_hpo_frl_stream_encoder_shift,
    pub hpo_se_mask: *const dcn30_hpo_frl_stream_encoder_mask,
}

extern "C" {
    pub fn hpo_enc3_enable(enc: *mut hpo_frl_stream_encoder, otg_inst: i32);
    pub fn hpo_enc3_unblank(enc: *mut hpo_frl_stream_encoder, otg_inst: i32);
    pub fn hpo_enc3_read_state(enc: *mut hpo_frl_stream_encoder, state: *mut hpo_frl_stream_encoder_state);
    pub fn hpo_enc3_fifo_odm_enabled(enc: *mut hpo_frl_stream_encoder) -> bool;
    pub fn hpo_enc3_blank(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc3_set_hdmi_stream_attribute(enc: *mut hpo_frl_stream_encoder, crtc_timing: *mut dc_crtc_timing, borrow_params: *mut frl_borrow_params, odm_combine_num_segments: i32);
    pub fn hpo_enc3_update_hdmi_info_packet(enc3: *mut dcn30_hpo_frl_stream_encoder, packet_index: u32, info_packet: *const dc_info_packet);
    pub fn hpo_enc3_update_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder, info_frame: *const encoder_info_frame);
    pub fn hpo_enc3_hdmi_set_dsc_config(enc: *mut hpo_frl_stream_encoder, timing: *mut dc_crtc_timing, dsc_packed_pps: *mut u8);
    pub fn hpo_enc3_stop_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc3_setup_hdmi_audio(enc: *mut hpo_frl_stream_encoder, crtc_info: *const audio_crtc_info);
    pub fn hpo_enc3_hdmi_audio_setup(enc: *mut hpo_frl_stream_encoder, az_inst: c_uint, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info);
    pub fn hpo_enc3_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc3_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool);
    pub fn enc3_stream_encoder_set_avmute(enc: *mut hpo_frl_stream_encoder, enable: bool);
    pub fn hpo_enc3_validate_hdmi_frl_output(enc: *mut hpo_frl_stream_encoder, timing: *const dc_crtc_timing, audio: *const audio_check, frl_link_settings: *mut dc_hdmi_frl_link_settings, dsc_max_rate: c_uint) -> bool;
    pub fn hpo_enc3_set_dynamic_metadata(enc: *mut hpo_frl_stream_encoder, enable_dme: bool, hubp_requestor_id: u32, dmdata_mode: dynamic_metadata_mode);
    pub fn dcn30_hpo_frl_stream_encoder_construct(enc3: *mut dcn30_hpo_frl_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, vpg: *mut vpg, afmt: *mut afmt, regs: *const dcn30_hpo_frl_stream_enc_registers, hpo_se_shift: *const dcn30_hpo_frl_stream_encoder_shift, hpo_se_mask: *const dcn30_hpo_frl_stream_encoder_mask);
    pub fn convert_dc_info_packet_to_128(info_packet: *const dc_info_packet, info_packet_128: *mut dc_info_packet_128);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
