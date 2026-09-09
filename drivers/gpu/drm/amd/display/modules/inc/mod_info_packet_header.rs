/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit:
// dm_services.h, mod_info_packet_types.h, mod_shared.h

// Forward declarations.
#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_info_packet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mod_vrr_params {
    _private: [u8; 0],
}

// dc_color_space, color_transfer_func, and adaptive_sync_type are supplied by
// the translated dependency headers.

extern "C" {
    pub fn set_vsc_packet_colorimetry_data(
        stream: *const dc_stream_state,
        info_packet: *mut dc_info_packet,
        cs: dc_color_space,
        tf: color_transfer_func,
    );

    pub fn mod_build_vsc_infopacket(
        stream: *const dc_stream_state,
        info_packet: *mut dc_info_packet,
        cs: dc_color_space,
        tf: color_transfer_func,
    );

    pub fn mod_build_hf_vsif_infopacket(
        stream: *const dc_stream_state,
        info_packet: *mut dc_info_packet,
        ALLMEnabled: ::core::ffi::c_int,
        ALLMValue: ::core::ffi::c_int,
    );
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adaptive_sync_sdp_version {
    AS_SDP_VER_0 = 0x0,
    AS_SDP_VER_1 = 0x1,
    AS_SDP_VER_2 = 0x2,
}

pub const AS_DP_SDP_LENGTH: usize = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_duration_op {
    pub support: bool,
    pub frame_duration_hex: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AS_Df_params {
    pub supportMode: bool,
    pub increase: frame_duration_op,
    pub decrease: frame_duration_op,
}

extern "C" {
    pub fn mod_build_adaptive_sync_infopacket(
        stream: *const dc_stream_state,
        asType: adaptive_sync_type,
        param: *const AS_Df_params,
        info_packet: *mut dc_info_packet,
    );

    pub fn mod_build_adaptive_sync_infopacket_v2(
        stream: *const dc_stream_state,
        param: *const AS_Df_params,
        info_packet: *mut dc_info_packet,
    );

    pub fn mod_build_adaptive_sync_infopacket_v1(info_packet: *mut dc_info_packet);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
