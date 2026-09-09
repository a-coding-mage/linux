/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies: audio_types.h, hw_shared.h

pub enum dc_bios {}
pub enum dc_context {}
pub enum dc_crtc_timing {}
pub enum vpg {}
pub enum afmt {}
pub enum apg {}
pub enum dc_link {}
pub enum audio_info {}
pub enum audio_crtc_info {}
pub enum frl_borrow_params {}
pub enum audio_check {}
pub enum dc_hdmi_frl_link_settings {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp_pixel_encoding_type {
    DP_PIXEL_ENCODING_TYPE_RGB444 = 0x00000000,
    DP_PIXEL_ENCODING_TYPE_YCBCR422 = 0x00000001,
    DP_PIXEL_ENCODING_TYPE_YCBCR444 = 0x00000002,
    DP_PIXEL_ENCODING_TYPE_RGB_WIDE_GAMUT = 0x00000003,
    DP_PIXEL_ENCODING_TYPE_Y_ONLY = 0x00000004,
    DP_PIXEL_ENCODING_TYPE_YCBCR420 = 0x00000005,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp_component_depth {
    DP_COMPONENT_PIXEL_DEPTH_6BPC = 0x00000000,
    DP_COMPONENT_PIXEL_DEPTH_8BPC = 0x00000001,
    DP_COMPONENT_PIXEL_DEPTH_10BPC = 0x00000002,
    DP_COMPONENT_PIXEL_DEPTH_12BPC = 0x00000003,
    DP_COMPONENT_PIXEL_DEPTH_16BPC = 0x00000004,
}

#[repr(C)]
pub struct audio_clock_info {
    pub pixel_clock_in_10khz: u32,
    pub n_32khz: u32,
    pub cts_32khz: u32,
    pub n_44khz: u32,
    pub cts_44khz: u32,
    pub n_48khz: u32,
    pub cts_48khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dynamic_metadata_mode { dmdata_dp, dmdata_hdmi, dmdata_dolby_vision }

#[repr(C)]
pub struct frl_audio_clock_info {
    pub frl_character_clock_kHz: u32,
    pub n_32khz: u32,
    pub cts_32khz: u32,
    pub n_44khz: u32,
    pub cts_44khz: u32,
    pub n_48khz: u32,
    pub cts_48khz: u32,
}

#[repr(C)]
pub struct enc_sdp_line_num { pub adaptive_sync_line_num_valid: bool, pub adaptive_sync_line_num: u32 }

#[repr(C)]
pub struct encoder_info_frame {
    pub avi: dc_info_packet, pub gamut: dc_info_packet, pub vendor: dc_info_packet,
    pub hfvsif: dc_info_packet, pub vtem: dc_info_packet, pub spd: dc_info_packet,
    pub vsc: dc_info_packet, pub hdrsmd: dc_info_packet, pub adaptive_sync: dc_info_packet,
    pub sdp_line_num: enc_sdp_line_num, pub firmware_controlled_hdr_info_packet: bool,
}

#[repr(C)]
pub struct encoder_unblank_param { pub link_settings: dc_link_settings, pub timing: dc_crtc_timing, pub opp_cnt: i32, pub pix_per_cycle: u32 }

#[repr(C)]
pub struct encoder_set_dp_phy_pattern_param {
    pub dp_phy_pattern: dp_test_pattern, pub custom_pattern: *const u8,
    pub custom_pattern_size: u32, pub dp_panel_mode: dp_panel_mode,
}

#[repr(C)]
pub struct stream_encoder {
    pub funcs: *const stream_encoder_funcs, pub ctx: *mut dc_context, pub bp: *mut dc_bios,
    pub id: engine_id, pub stream_enc_inst: u32, pub vpg: *mut vpg, pub afmt: *mut afmt, pub apg: *mut apg,
}

#[repr(C)]
pub struct enc_state {
    pub dsc_mode: u32, pub dsc_slice_width: u32, pub sec_gsp_pps_line_num: u32,
    pub vbid6_line_reference: u32, pub vbid6_line_num: u32, pub sec_gsp_pps_enable: u32, pub sec_stream_enable: u32,
}

// C callback declarations are represented as nullable C-ABI function pointers.
#[repr(C)]
pub struct stream_encoder_funcs {
    pub dp_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, dc_color_space, bool, u32)>,
    pub hdmi_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, i32, bool)>,
    pub dvi_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, bool)>,
    pub lvds_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing)>,
    pub set_throttled_vcp_size: Option<unsafe extern "C" fn(*mut stream_encoder, fixed31_32)>,
    pub update_hdmi_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder, *const encoder_info_frame)>,
    pub stop_hdmi_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub update_dp_info_packets_sdp_line_num: Option<unsafe extern "C" fn(*mut stream_encoder, *mut encoder_info_frame)>,
    pub update_dp_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder, *const encoder_info_frame)>,
    pub send_immediate_sdp_message: Option<unsafe extern "C" fn(*mut stream_encoder, *const u8, u32)>,
    pub stop_dp_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub dp_blank: Option<unsafe extern "C" fn(*mut dc_link, *mut stream_encoder)>,
    pub dp_unblank: Option<unsafe extern "C" fn(*mut dc_link, *mut stream_encoder, *const encoder_unblank_param)>,
    pub audio_mute_control: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub dp_audio_setup: Option<unsafe extern "C" fn(*mut stream_encoder, u32, *mut audio_info)>,
    pub dp_audio_enable: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub dp_audio_disable: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub hdmi_audio_setup: Option<unsafe extern "C" fn(*mut stream_encoder, u32, *mut audio_info, *mut audio_crtc_info)>,
    pub hdmi_audio_disable: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub setup_stereo_sync: Option<unsafe extern "C" fn(*mut stream_encoder, i32, bool)>,
    pub set_avmute: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub dig_connect_to_otg: Option<unsafe extern "C" fn(*mut stream_encoder, i32)>,
    pub enable_stream: Option<unsafe extern "C" fn(*mut stream_encoder, signal_type, bool)>,
    pub hdmi_reset_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub dig_source_otg: Option<unsafe extern "C" fn(*mut stream_encoder) -> u32>,
    pub dp_get_pixel_format: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_pixel_encoding, *mut dc_color_depth) -> bool>,
    pub enc_read_state: Option<unsafe extern "C" fn(*mut stream_encoder, *mut enc_state)>,
    pub dp_set_dsc_config: Option<unsafe extern "C" fn(*mut stream_encoder, optc_dsc_mode, u32, u32)>,
    pub dp_set_dsc_pps_info_packet: Option<unsafe extern "C" fn(*mut stream_encoder, bool, *mut u8, bool)>,
    pub set_dynamic_metadata: Option<unsafe extern "C" fn(*mut stream_encoder, bool, u32, dynamic_metadata_mode)>,
    pub dp_set_odm_combine: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub get_fifo_cal_average_level: Option<unsafe extern "C" fn(*mut stream_encoder) -> u32>,
    pub set_input_mode: Option<unsafe extern "C" fn(*mut stream_encoder, u32)>,
    pub enable_fifo: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub disable_fifo: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub is_fifo_enabled: Option<unsafe extern "C" fn(*mut stream_encoder) -> bool>,
    pub map_stream_to_link: Option<unsafe extern "C" fn(*mut stream_encoder, u32, u32)>,
    pub get_pixels_per_cycle: Option<unsafe extern "C" fn(*mut stream_encoder) -> u32>,
}

#[repr(C)]
pub struct hpo_frl_stream_encoder_state { pub stream_enc_enabled: u32, pub otg_inst: u32, pub color_depth: u32, pub num_odm_segments: u32, pub h_active: u32, pub h_blank: u32, pub borrow_mode: u32, pub pixel_format: dc_pixel_encoding }

#[repr(C)]
pub struct hpo_frl_stream_encoder {
    pub funcs: *const hpo_frl_stream_encoder_funcs, pub stream_enc_inst: u32, pub ctx: *mut dc_context,
    pub bp: *mut dc_bios, pub id: engine_id, pub afmt: *mut afmt, pub vpg: *mut vpg, pub apg: *mut apg,
}

#[repr(C)]
pub struct hpo_frl_stream_encoder_funcs {
    pub hdmi_frl_set_dsc_config: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, *mut dc_crtc_timing, *mut u8)>,
    pub hdmi_frl_enable: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, i32)>,
    pub hdmi_frl_unblank: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, i32)>,
    pub hdmi_frl_blank: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder)>,
    pub hdmi_frl_fifo_odm_enabled: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder) -> bool>,
    pub hdmi_frl_set_stream_attribute: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, *mut dc_crtc_timing, *mut frl_borrow_params, i32)>,
    pub update_hdmi_info_packets: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, *const encoder_info_frame)>,
    pub stop_hdmi_info_packets: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder)>,
    pub audio_mute_control: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, bool)>,
    pub hdmi_audio_setup: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, u32, *mut audio_info, *mut audio_crtc_info)>,
    pub hdmi_audio_disable: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder)>,
    pub set_avmute: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, bool)>,
    pub validate_hdmi_frl_output: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, *const dc_crtc_timing, *const audio_check, *mut dc_hdmi_frl_link_settings, u32) -> bool>,
    pub read_state: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, *mut hpo_frl_stream_encoder_state)>,
    pub set_dynamic_metadata: Option<unsafe extern "C" fn(*mut hpo_frl_stream_encoder, bool, u32, dynamic_metadata_mode)>,
}

#[repr(C)]
pub struct hpo_dp_stream_encoder_state { pub stream_enc_enabled: u32, pub vid_stream_enabled: u32, pub otg_inst: u32, pub pixel_encoding: u32, pub component_depth: u32, pub compressed_format: u32, pub sdp_enabled: u32, pub mapped_to_link_enc: u32 }

#[repr(C)]
pub struct hpo_dp_stream_encoder { pub funcs: *const hpo_dp_stream_encoder_funcs, pub ctx: *mut dc_context, pub bp: *mut dc_bios, pub inst: u32, pub id: engine_id, pub vpg: *mut vpg, pub apg: *mut apg }

#[repr(C)]
pub struct hpo_dp_stream_encoder_funcs {
    pub enable_stream: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub dp_unblank: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, u32)>,
    pub dp_blank: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub disable: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub set_stream_attribute: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, *mut dc_crtc_timing, dc_color_space, bool, bool, bool)>,
    pub update_dp_info_packets_sdp_line_num: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, *mut encoder_info_frame)>,
    pub update_dp_info_packets: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, *const encoder_info_frame)>,
    pub stop_dp_info_packets: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub dp_set_dsc_pps_info_packet: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, bool, *mut u8, bool)>,
    pub map_stream_to_link: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, u32, u32)>,
    pub dp_audio_setup: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, u32, *mut audio_info)>,
    pub dp_audio_enable: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub dp_audio_disable: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder)>,
    pub read_state: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, *mut hpo_dp_stream_encoder_state)>,
    pub set_hblank_min_symbol_width: Option<unsafe extern "C" fn(*mut hpo_dp_stream_encoder, u16)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
