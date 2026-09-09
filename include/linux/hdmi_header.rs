/*
 * Copyright (C) 2012 Avionic Design GmbH
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sub license,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hdmi_packet_type { HDMI_PACKET_TYPE_NULL = 0x00, HDMI_PACKET_TYPE_AUDIO_CLOCK_REGEN = 0x01, HDMI_PACKET_TYPE_AUDIO_SAMPLE = 0x02, HDMI_PACKET_TYPE_GENERAL_CONTROL = 0x03, HDMI_PACKET_TYPE_ACP = 0x04, HDMI_PACKET_TYPE_ISRC1 = 0x05, HDMI_PACKET_TYPE_ISRC2 = 0x06, HDMI_PACKET_TYPE_ONE_BIT_AUDIO_SAMPLE = 0x07, HDMI_PACKET_TYPE_DST_AUDIO = 0x08, HDMI_PACKET_TYPE_HBR_AUDIO_STREAM = 0x09, HDMI_PACKET_TYPE_GAMUT_METADATA = 0x0a }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_infoframe_type { HDMI_INFOFRAME_TYPE_VENDOR = 0x81, HDMI_INFOFRAME_TYPE_AVI = 0x82, HDMI_INFOFRAME_TYPE_SPD = 0x83, HDMI_INFOFRAME_TYPE_AUDIO = 0x84, HDMI_INFOFRAME_TYPE_DRM = 0x87 }

pub const HDMI_TMDS_CHAR_RATE_MIN_HZ: u32 = 25000000;
pub const HDMI_1_0_TMDS_CHAR_RATE_MAX_HZ: u32 = 165000000;
pub const HDMI_1_3_TMDS_CHAR_RATE_MAX_HZ: u32 = 340000000;
pub const HDMI_2_0_TMDS_CHAR_RATE_MAX_HZ: u32 = 600000000;
pub const HDMI_IEEE_OUI: u32 = 0x000c03;
pub const HDMI_FORUM_IEEE_OUI: u32 = 0xc45dd8;
pub const HDMI_INFOFRAME_HEADER_SIZE: usize = 4;
pub const HDMI_AVI_INFOFRAME_SIZE: usize = 13;
pub const HDMI_SPD_INFOFRAME_SIZE: usize = 25;
pub const HDMI_AUDIO_INFOFRAME_SIZE: usize = 10;
pub const HDMI_DRM_INFOFRAME_SIZE: usize = 26;
pub const HDMI_VENDOR_INFOFRAME_SIZE: usize = 4;
pub const HDMI_MAX_INFOFRAME_SIZE: usize = 27;

#[macro_export]
macro_rules! HDMI_INFOFRAME_SIZE {
    (AVI) => { HDMI_INFOFRAME_HEADER_SIZE + HDMI_AVI_INFOFRAME_SIZE };
    (SPD) => { HDMI_INFOFRAME_HEADER_SIZE + HDMI_SPD_INFOFRAME_SIZE };
    (AUDIO) => { HDMI_INFOFRAME_HEADER_SIZE + HDMI_AUDIO_INFOFRAME_SIZE };
    (DRM) => { HDMI_INFOFRAME_HEADER_SIZE + HDMI_DRM_INFOFRAME_SIZE };
    (VENDOR) => { HDMI_INFOFRAME_HEADER_SIZE + HDMI_VENDOR_INFOFRAME_SIZE };
}

#[repr(C)] pub struct hdmi_any_infoframe { pub type_: hdmi_infoframe_type, pub version: u8, pub length: u8 }

#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_colorspace { HDMI_COLORSPACE_RGB, HDMI_COLORSPACE_YUV422, HDMI_COLORSPACE_YUV444, HDMI_COLORSPACE_YUV420, HDMI_COLORSPACE_RESERVED4, HDMI_COLORSPACE_RESERVED5, HDMI_COLORSPACE_RESERVED6, HDMI_COLORSPACE_IDO_DEFINED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_scan_mode { HDMI_SCAN_MODE_NONE, HDMI_SCAN_MODE_OVERSCAN, HDMI_SCAN_MODE_UNDERSCAN, HDMI_SCAN_MODE_RESERVED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_colorimetry { HDMI_COLORIMETRY_NONE, HDMI_COLORIMETRY_ITU_601, HDMI_COLORIMETRY_ITU_709, HDMI_COLORIMETRY_EXTENDED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_picture_aspect { HDMI_PICTURE_ASPECT_NONE, HDMI_PICTURE_ASPECT_4_3, HDMI_PICTURE_ASPECT_16_9, HDMI_PICTURE_ASPECT_64_27, HDMI_PICTURE_ASPECT_256_135, HDMI_PICTURE_ASPECT_RESERVED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_active_aspect { HDMI_ACTIVE_ASPECT_16_9_TOP = 2, HDMI_ACTIVE_ASPECT_14_9_TOP, HDMI_ACTIVE_ASPECT_16_9_CENTER, HDMI_ACTIVE_ASPECT_PICTURE = 8, HDMI_ACTIVE_ASPECT_4_3, HDMI_ACTIVE_ASPECT_16_9, HDMI_ACTIVE_ASPECT_14_9, HDMI_ACTIVE_ASPECT_4_3_SP_14_9 = 13, HDMI_ACTIVE_ASPECT_16_9_SP_14_9, HDMI_ACTIVE_ASPECT_16_9_SP_4_3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_extended_colorimetry { HDMI_EXTENDED_COLORIMETRY_XV_YCC_601, HDMI_EXTENDED_COLORIMETRY_XV_YCC_709, HDMI_EXTENDED_COLORIMETRY_S_YCC_601, HDMI_EXTENDED_COLORIMETRY_OPYCC_601, HDMI_EXTENDED_COLORIMETRY_OPRGB, HDMI_EXTENDED_COLORIMETRY_BT2020_CONST_LUM, HDMI_EXTENDED_COLORIMETRY_BT2020, HDMI_EXTENDED_COLORIMETRY_RESERVED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_quantization_range { HDMI_QUANTIZATION_RANGE_DEFAULT, HDMI_QUANTIZATION_RANGE_LIMITED, HDMI_QUANTIZATION_RANGE_FULL, HDMI_QUANTIZATION_RANGE_RESERVED }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_nups { HDMI_NUPS_UNKNOWN, HDMI_NUPS_HORIZONTAL, HDMI_NUPS_VERTICAL, HDMI_NUPS_BOTH }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_ycc_quantization_range { HDMI_YCC_QUANTIZATION_RANGE_LIMITED, HDMI_YCC_QUANTIZATION_RANGE_FULL }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_content_type { HDMI_CONTENT_TYPE_GRAPHICS, HDMI_CONTENT_TYPE_PHOTO, HDMI_CONTENT_TYPE_CINEMA, HDMI_CONTENT_TYPE_GAME }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_metadata_type { HDMI_STATIC_METADATA_TYPE1 = 0 }
#[repr(C)] #[derive(Copy, Clone)] pub enum hdmi_eotf { HDMI_EOTF_TRADITIONAL_GAMMA_SDR, HDMI_EOTF_TRADITIONAL_GAMMA_HDR, HDMI_EOTF_SMPTE_ST2084, HDMI_EOTF_BT_2100_HLG }

#[repr(C)] pub struct hdmi_avi_infoframe { pub type_: hdmi_infoframe_type, pub version: u8, pub length: u8, pub itc: bool, pub pixel_repeat: u8, pub colorspace: hdmi_colorspace, pub scan_mode: hdmi_scan_mode, pub colorimetry: hdmi_colorimetry, pub picture_aspect: hdmi_picture_aspect, pub active_aspect: hdmi_active_aspect, pub extended_colorimetry: hdmi_extended_colorimetry, pub quantization_range: hdmi_quantization_range, pub nups: hdmi_nups, pub video_code: u8, pub ycc_quantization_range: hdmi_ycc_quantization_range, pub content_type: hdmi_content_type, pub top_bar: u16, pub bottom_bar: u16, pub left_bar: u16, pub right_bar: u16 }
#[repr(C)] pub struct hdmi_display_primary { pub x: u16, pub y: u16 }
#[repr(C)] pub struct hdmi_drm_infoframe { pub type_: hdmi_infoframe_type, pub version: u8, pub length: u8, pub eotf: hdmi_eotf, pub metadata_type: hdmi_metadata_type, pub display_primaries: [hdmi_display_primary; 3], pub white_point: hdmi_display_primary, pub max_display_mastering_luminance: u16, pub min_display_mastering_luminance: u16, pub max_cll: u16, pub max_fall: u16 }

extern "C" {
    pub fn hdmi_avi_infoframe_init(frame: *mut hdmi_avi_infoframe);
    pub fn hdmi_avi_infoframe_pack(frame: *mut hdmi_avi_infoframe, buffer: *mut core::ffi::c_void, size: usize) -> isize;
    pub fn hdmi_avi_infoframe_pack_only(frame: *const hdmi_avi_infoframe, buffer: *mut core::ffi::c_void, size: usize) -> isize;
    pub fn hdmi_avi_infoframe_check(frame: *mut hdmi_avi_infoframe) -> i32;
    pub fn hdmi_drm_infoframe_init(frame: *mut hdmi_drm_infoframe) -> i32;
    pub fn hdmi_drm_infoframe_pack(frame: *mut hdmi_drm_infoframe, buffer: *mut core::ffi::c_void, size: usize) -> isize;
    pub fn hdmi_drm_infoframe_pack_only(frame: *const hdmi_drm_infoframe, buffer: *mut core::ffi::c_void, size: usize) -> isize;
    pub fn hdmi_drm_infoframe_check(frame: *mut hdmi_drm_infoframe) -> i32;
    pub fn hdmi_drm_infoframe_unpack_only(frame: *mut hdmi_drm_infoframe, buffer: *const core::ffi::c_void, size: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
