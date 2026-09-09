/*
 * Copyright (c) 2016 Intel Corporation
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT, OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

// Dependencies supplied by the surrounding DRM translation.

pub enum drm_crtc {}
pub enum drm_plane {}
pub struct drm_property_blob {
    pub length: usize,
}
pub struct drm_color_lut {}
pub struct drm_color_lut32 {}

#[inline]
pub unsafe fn drm_color_lut_extract(user_input: u32, bit_precision: i32) -> u32 {
    if bit_precision > 16 {
        DIV_ROUND_CLOSEST_ULL(
            mul_u32_u32(user_input, (1u32 << bit_precision) - 1),
            (1u32 << 16) - 1,
        )
    } else {
        DIV_ROUND_CLOSEST(
            user_input * ((1u32 << bit_precision) - 1),
            (1u32 << 16) - 1,
        )
    }
}

#[inline]
pub unsafe fn drm_color_lut32_extract(user_input: u32, bit_precision: i32) -> u32 {
    let max: u64 = if bit_precision >= 64 {
        !0u64
    } else {
        (1u64 << bit_precision) - 1
    };

    DIV_ROUND_CLOSEST_ULL((user_input as u64) * max, (1u64 << 32) - 1)
}

extern "C" {
    pub fn drm_color_ctm_s31_32_to_qm_n(user_input: u64, m: u32, n: u32) -> u64;

    pub fn drm_crtc_enable_color_mgmt(
        crtc: *mut drm_crtc,
        degamma_lut_size: u32,
        has_ctm: bool,
        gamma_lut_size: u32,
    );

    pub fn drm_mode_crtc_set_gamma_size(crtc: *mut drm_crtc, gamma_size: i32) -> i32;
}

#[inline]
pub unsafe fn drm_color_lut_size(blob: *const drm_property_blob) -> i32 {
    ((*blob).length / core::mem::size_of::<drm_color_lut>()) as i32
}

#[inline]
pub unsafe fn drm_color_lut32_size(blob: *const drm_property_blob) -> i32 {
    ((*blob).length / core::mem::size_of::<drm_color_lut32>()) as i32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_color_encoding {
    DRM_COLOR_YCBCR_BT601,
    DRM_COLOR_YCBCR_BT709,
    DRM_COLOR_YCBCR_BT2020,
    DRM_COLOR_ENCODING_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_color_range {
    DRM_COLOR_YCBCR_LIMITED_RANGE,
    DRM_COLOR_YCBCR_FULL_RANGE,
    DRM_COLOR_RANGE_MAX,
}

extern "C" {
    pub fn drm_plane_create_color_properties(
        plane: *mut drm_plane,
        supported_encodings: u32,
        supported_ranges: u32,
        default_encoding: drm_color_encoding,
        default_range: drm_color_range,
    ) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_color_lut_tests {
    DRM_COLOR_LUT_EQUAL_CHANNELS = 1 << 0,
    DRM_COLOR_LUT_NON_DECREASING = 1 << 1,
}

extern "C" {
    pub fn drm_color_lut_check(lut: *const drm_property_blob, tests: u32) -> i32;
}

pub type drm_crtc_set_lut_func =
    Option<unsafe extern "C" fn(*mut drm_crtc, u32, u16, u16, u16)>;

extern "C" {
    pub fn drm_crtc_load_gamma_888(
        crtc: *mut drm_crtc,
        lut: *const drm_color_lut,
        set_gamma: drm_crtc_set_lut_func,
    );
    pub fn drm_crtc_load_gamma_565_from_888(
        crtc: *mut drm_crtc,
        lut: *const drm_color_lut,
        set_gamma: drm_crtc_set_lut_func,
    );
    pub fn drm_crtc_load_gamma_555_from_888(
        crtc: *mut drm_crtc,
        lut: *const drm_color_lut,
        set_gamma: drm_crtc_set_lut_func,
    );

    pub fn drm_crtc_fill_gamma_888(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);
    pub fn drm_crtc_fill_gamma_565(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);
    pub fn drm_crtc_fill_gamma_555(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);

    pub fn drm_crtc_load_palette_8(
        crtc: *mut drm_crtc,
        lut: *const drm_color_lut,
        set_palette: drm_crtc_set_lut_func,
    );
    pub fn drm_crtc_fill_palette_332(crtc: *mut drm_crtc, set_palette: drm_crtc_set_lut_func);
    pub fn drm_crtc_fill_palette_8(crtc: *mut drm_crtc, set_palette: drm_crtc_set_lut_func);

    pub fn drm_color_lut32_check(lut: *const drm_property_blob, tests: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
