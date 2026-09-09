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
 */

// Types, constants, allocation routines, and BREAK_TO_DEBUGGER are supplied
// by the surrounding display-core dependencies.

#[repr(C)]
pub struct stream_encoder {
    pub funcs: *const stream_encoder_funcs,
    pub ctx: *mut dc_context,
    pub id: i32,
    pub bp: *mut dc_bios,
}

#[repr(C)]
pub struct stream_encoder_funcs {
    pub dp_set_odm_combine: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub dp_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, dc_color_space, bool, u32)>,
    pub hdmi_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, i32, bool)>,
    pub dvi_set_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder, *mut dc_crtc_timing, bool)>,
    pub set_throttled_vcp_size: Option<unsafe extern "C" fn(*mut stream_encoder, fixed31_32)>,
    pub update_hdmi_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder, *const encoder_info_frame)>,
    pub stop_hdmi_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub update_dp_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder, *const encoder_info_frame)>,
    pub stop_dp_info_packets: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub dp_blank: Option<unsafe extern "C" fn(*mut dc_link, *mut stream_encoder)>,
    pub dp_unblank: Option<unsafe extern "C" fn(*mut dc_link, *mut stream_encoder, *const encoder_unblank_param)>,
    pub audio_mute_control: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub set_avmute: Option<unsafe extern "C" fn(*mut stream_encoder, bool)>,
    pub hdmi_reset_stream_attribute: Option<unsafe extern "C" fn(*mut stream_encoder)>,
    pub dig_connect_to_otg: Option<unsafe extern "C" fn(*mut stream_encoder, i32)>,
    pub setup_stereo_sync: Option<unsafe extern "C" fn(*mut stream_encoder, i32, bool)>,
    pub dp_set_dsc_pps_info_packet: Option<unsafe extern "C" fn(*mut stream_encoder, bool, *mut u8, bool)>,
}

pub enum dc_context {}
pub enum dc_bios {}
pub enum dc_link {}
pub enum dc_crtc_timing {}
pub enum encoder_info_frame {}
pub enum encoder_unblank_param {}
pub enum dc_color_space {}
pub enum fixed31_32 {}

unsafe extern "C" fn virtual_stream_encoder_dp_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, output_color_space: dc_color_space, use_vsc_sdp_for_colorimetry: bool, enable_sdp_splitting: u32) { let _ = (enc, crtc_timing, output_color_space, use_vsc_sdp_for_colorimetry, enable_sdp_splitting); }
unsafe extern "C" fn virtual_stream_encoder_hdmi_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, actual_pix_clk_khz: i32, enable_audio: bool) { let _ = (enc, crtc_timing, actual_pix_clk_khz, enable_audio); }
unsafe extern "C" fn virtual_stream_encoder_dvi_set_stream_attribute(enc: *mut stream_encoder, crtc_timing: *mut dc_crtc_timing, is_dual_link: bool) { let _ = (enc, crtc_timing, is_dual_link); }
unsafe extern "C" fn virtual_stream_encoder_set_throttled_vcp_size(enc: *mut stream_encoder, avg_time_slots_per_mtp: fixed31_32) { let _ = (enc, avg_time_slots_per_mtp); }
unsafe extern "C" fn virtual_stream_encoder_update_hdmi_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame) { let _ = (enc, info_frame); }
unsafe extern "C" fn virtual_stream_encoder_stop_hdmi_info_packets(enc: *mut stream_encoder) { let _ = enc; }
unsafe extern "C" fn virtual_stream_encoder_set_avmute(enc: *mut stream_encoder, enable: bool) { let _ = (enc, enable); }
unsafe extern "C" fn virtual_stream_encoder_update_dp_info_packets(enc: *mut stream_encoder, info_frame: *const encoder_info_frame) { let _ = (enc, info_frame); }
unsafe extern "C" fn virtual_stream_encoder_stop_dp_info_packets(enc: *mut stream_encoder) { let _ = enc; }
unsafe extern "C" fn virtual_stream_encoder_dp_blank(link: *mut dc_link, enc: *mut stream_encoder) { let _ = (link, enc); }
unsafe extern "C" fn virtual_stream_encoder_dp_unblank(link: *mut dc_link, enc: *mut stream_encoder, param: *const encoder_unblank_param) { let _ = (enc, link, param); }
unsafe extern "C" fn virtual_audio_mute_control(enc: *mut stream_encoder, mute: bool) { let _ = (enc, mute); }
unsafe extern "C" fn virtual_stream_encoder_reset_hdmi_stream_attribute(enc: *mut stream_encoder) { let _ = enc; }
unsafe extern "C" fn virtual_enc_dp_set_odm_combine(enc: *mut stream_encoder, odm_combine: bool) { let _ = (enc, odm_combine); }
unsafe extern "C" fn virtual_dig_connect_to_otg(enc: *mut stream_encoder, tg_inst: i32) { let _ = (enc, tg_inst); }
unsafe extern "C" fn virtual_setup_stereo_sync(enc: *mut stream_encoder, tg_inst: i32, enable: bool) { let _ = (enc, tg_inst, enable); }
unsafe extern "C" fn virtual_stream_encoder_set_dsc_pps_info_packet(enc: *mut stream_encoder, enable: bool, dsc_packed_pps: *mut u8, immediate_update: bool) { let _ = (enc, enable, dsc_packed_pps, immediate_update); }

static virtual_str_enc_funcs: stream_encoder_funcs = stream_encoder_funcs {
    dp_set_odm_combine: Some(virtual_enc_dp_set_odm_combine),
    dp_set_stream_attribute: Some(virtual_stream_encoder_dp_set_stream_attribute),
    hdmi_set_stream_attribute: Some(virtual_stream_encoder_hdmi_set_stream_attribute),
    dvi_set_stream_attribute: Some(virtual_stream_encoder_dvi_set_stream_attribute),
    set_throttled_vcp_size: Some(virtual_stream_encoder_set_throttled_vcp_size),
    update_hdmi_info_packets: Some(virtual_stream_encoder_update_hdmi_info_packets),
    stop_hdmi_info_packets: Some(virtual_stream_encoder_stop_hdmi_info_packets),
    update_dp_info_packets: Some(virtual_stream_encoder_update_dp_info_packets),
    stop_dp_info_packets: Some(virtual_stream_encoder_stop_dp_info_packets),
    dp_blank: Some(virtual_stream_encoder_dp_blank),
    dp_unblank: Some(virtual_stream_encoder_dp_unblank),
    audio_mute_control: Some(virtual_audio_mute_control),
    set_avmute: Some(virtual_stream_encoder_set_avmute),
    hdmi_reset_stream_attribute: Some(virtual_stream_encoder_reset_hdmi_stream_attribute),
    dig_connect_to_otg: Some(virtual_dig_connect_to_otg),
    setup_stereo_sync: Some(virtual_setup_stereo_sync),
    dp_set_dsc_pps_info_packet: Some(virtual_stream_encoder_set_dsc_pps_info_packet),
};

pub unsafe extern "C" fn virtual_stream_encoder_construct(enc: *mut stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios) -> bool {
    if enc.is_null() || bp.is_null() { return false; }
    (*enc).funcs = &virtual_str_enc_funcs;
    (*enc).ctx = ctx;
    (*enc).id = ENGINE_ID_VIRTUAL;
    (*enc).bp = bp;
    true
}

pub unsafe extern "C" fn virtual_stream_encoder_create(ctx: *mut dc_context, bp: *mut dc_bios) -> *mut stream_encoder {
    let enc = libc::calloc(1, core::mem::size_of::<stream_encoder>()) as *mut stream_encoder;
    if enc.is_null() { return core::ptr::null_mut(); }
    if virtual_stream_encoder_construct(enc, ctx, bp) { return enc; }
    BREAK_TO_DEBUGGER!();
    libc::free(enc as *mut libc::c_void);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
