/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
// dcn30_vpg.h, dcn30_afmt.h, dcn30_hpo_frl_stream_encoder.h,
// stream_encoder.h, and dml1_frl_cap_chk.h.

// C macro intent:
// DCN401_HPO_FRL_STRENC_FROM_HPO_FRL_STRENC(hpo_frl_stream_encoder)
// expands to container_of(hpo_frl_stream_encoder,
//     struct dcn401_hpo_frl_stream_encoder, base).
// SE_SF(reg_name, field_name, post_fix) expands to the corresponding
// register-field member assignment using reg_name__field_name__post_fix.
// DCN401_HDMI_STREAM_ENC_MASK_SH_LIST(mask_sh),
// DCN401_HDMI_TB_ENC_MASK_SH_LIST(mask_sh), and
// DCN401_HPO_STREAM_ENC_MASK_SH_LIST(mask_sh) expand to the register-field
// assignment lists declared by the C header.

#[repr(C)]
pub struct dcn401_hpo_frl_stream_encoder_shift {
    pub HDMI_TB_ENC_EN: u8,
    pub HDMI_RESET: u8,
    pub HDMI_RESET_DONE: u8,
    pub HDMI_STREAM_ENC_CLOCK_EN: u8,
    pub HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL: u8,
    pub HDMI_MAX_PACKETS_PER_LINE: u8,
    pub FIFO_ENABLE: u8,
    pub FIFO_RESET: u8,
    pub FIFO_PIXEL_ENCODING_TYPE: u8,
    pub FIFO_UNCOMPRESSED_PIXEL_FORMAT: u8,
    pub FIFO_COMPRESSED_PIXEL_FORMAT: u8,
    pub FIFO_RESET_DONE: u8,
    pub HDMI_BORROW_MODE: u8,
    pub HDMI_H_ACTIVE: u8,
    pub HDMI_H_BLANK: u8,
    pub HDMI_HC_ACTIVE: u8,
    pub HDMI_HC_BLANK: u8,
    pub HDMI_DB_DISABLE: u8,
    pub HDMI_PIXEL_ENCODING: u8,
    pub HDMI_DEEP_COLOR_DEPTH: u8,
    pub HDMI_DEEP_COLOR_ENABLE: u8,
    pub HDMI_ODM_COMBINE_MODE: u8,
    pub HDMI_DSC_MODE: u8,
    pub HDMI_GC_CONT: u8,
    pub HDMI_GC_SEND: u8,
    pub HDMI_ACP_SEND: u8,
    pub HDMI_AUDIO_INFO_SEND: u8,
    pub HDMI_AUDIO_INFO_LINE: u8,
    pub HDMI_GC_AVMUTE: u8,
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
    pub HDMI_CRC_EN: u8, pub HDMI_CRC_CONT_EN: u8,
    pub METADATA_HUBP_REQUESTOR_ID: u8, pub METADATA_ENGINE_EN: u8,
    pub METADATA_STREAM_TYPE: u8, pub HDMI_METADATA_PACKET_ENABLE: u8,
    pub HDMI_METADATA_PACKET_LINE_REFERENCE: u8, pub HDMI_METADATA_PACKET_MISSED: u8,
    pub HDMI_METADATA_PACKET_LINE: u8,
    pub HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL: u8,
    pub HDMI_STREAM_ENC_APG_CLOCK_EN: u8,
}

#[repr(C)]
pub struct dcn401_hpo_frl_stream_encoder_mask {
    pub HDMI_TB_ENC_EN: u32,
    pub HDMI_RESET: u32,
    pub HDMI_RESET_DONE: u32,
    pub HDMI_STREAM_ENC_CLOCK_EN: u32,
    pub HDMI_STREAM_ENC_INPUT_MUX_SOURCE_SEL: u32,
    pub HDMI_MAX_PACKETS_PER_LINE: u32,
    pub FIFO_ENABLE: u32,
    pub FIFO_RESET: u32,
    pub FIFO_PIXEL_ENCODING_TYPE: u32,
    pub FIFO_UNCOMPRESSED_PIXEL_FORMAT: u32,
    pub FIFO_COMPRESSED_PIXEL_FORMAT: u32,
    pub FIFO_RESET_DONE: u32,
    pub HDMI_BORROW_MODE: u32,
    pub HDMI_H_ACTIVE: u32,
    pub HDMI_H_BLANK: u32,
    pub HDMI_HC_ACTIVE: u32,
    pub HDMI_HC_BLANK: u32,
    pub HDMI_DB_DISABLE: u32,
    pub HDMI_PIXEL_ENCODING: u32,
    pub HDMI_DEEP_COLOR_DEPTH: u32,
    pub HDMI_DEEP_COLOR_ENABLE: u32,
    pub HDMI_ODM_COMBINE_MODE: u32,
    pub HDMI_DSC_MODE: u32,
    pub HDMI_GC_CONT: u32,
    pub HDMI_GC_SEND: u32,
    pub HDMI_ACP_SEND: u32,
    pub HDMI_AUDIO_INFO_SEND: u32,
    pub HDMI_AUDIO_INFO_LINE: u32,
    pub HDMI_GC_AVMUTE: u32,
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
    pub HDMI_CRC_EN: u32, pub HDMI_CRC_CONT_EN: u32,
    pub METADATA_HUBP_REQUESTOR_ID: u32, pub METADATA_ENGINE_EN: u32,
    pub METADATA_STREAM_TYPE: u32, pub HDMI_METADATA_PACKET_ENABLE: u32,
    pub HDMI_METADATA_PACKET_LINE_REFERENCE: u32, pub HDMI_METADATA_PACKET_MISSED: u32,
    pub HDMI_METADATA_PACKET_LINE: u32,
    pub HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL: u32,
    pub HDMI_STREAM_ENC_APG_CLOCK_EN: u32,
}

#[repr(C)]
pub struct dcn401_hpo_frl_stream_encoder {
    pub base: hpo_frl_stream_encoder,
    pub regs: *const dcn30_hpo_frl_stream_enc_registers,
    pub hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift,
    pub hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask,
}

extern "C" {
    pub fn hpo_enc401_enable(enc: *mut hpo_frl_stream_encoder, otg_inst: i32);
    pub fn hpo_enc401_unblank(enc: *mut hpo_frl_stream_encoder, otg_inst: i32);
    pub fn hpo_enc401_read_state(enc: *mut hpo_frl_stream_encoder, state: *mut hpo_frl_stream_encoder_state);
    pub fn hpo_enc401_blank(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc401_set_hdmi_stream_attribute(enc: *mut hpo_frl_stream_encoder, crtc_timing: *mut dc_crtc_timing, borrow_params: *mut frl_borrow_params, odm_combine_num_segments: i32);
    pub fn hpo_enc401_update_hdmi_info_packet(enc401: *mut dcn401_hpo_frl_stream_encoder, packet_index: u32, info_packet: *const dc_info_packet);
    pub fn hpo_enc401_update_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder, info_frame: *const encoder_info_frame);
    pub fn hpo_enc401_hdmi_set_dsc_config(enc: *mut hpo_frl_stream_encoder, timing: *mut dc_crtc_timing, dsc_packed_pps: *mut u8);
    pub fn hpo_enc401_stop_hdmi_info_packets(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc401_setup_hdmi_audio(enc: *mut hpo_frl_stream_encoder, crtc_info: *const audio_crtc_info);
    pub fn hpo_enc401_hdmi_audio_setup(enc: *mut hpo_frl_stream_encoder, az_inst: u32, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info);
    pub fn hpo_enc401_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder);
    pub fn hpo_enc401_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool);
    pub fn enc401_stream_encoder_set_avmute(enc: *mut hpo_frl_stream_encoder, enable: bool);
    pub fn hpo_enc401_set_dynamic_metadata(enc: *mut hpo_frl_stream_encoder, enable_dme: bool, hubp_requestor_id: u32, dmdata_mode: dynamic_metadata_mode);
    pub fn frl_get_audio_clock_info(color_depth: dc_color_depth, frl_character_clock_kHz: u32, audio_clock_info: *mut frl_audio_clock_info);
    pub fn dcn401_hpo_frl_stream_encoder_construct(enc401: *mut dcn401_hpo_frl_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, vpg: *mut vpg, afmt: *mut afmt, regs: *const dcn30_hpo_frl_stream_enc_registers, hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift, hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
