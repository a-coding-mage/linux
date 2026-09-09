/*
 * Copyright (c) 2016 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
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

// Dependencies supplied by the corresponding Linux DRM headers.

/// DRM_FORMAT_MAX_PLANES - maximum number of planes a DRM format can have
pub const DRM_FORMAT_MAX_PLANES: usize = 4;

#[cfg(target_endian = "big")]
pub const DRM_FORMAT_HOST_XRGB1555: u32 = DRM_FORMAT_XRGB1555 | DRM_FORMAT_BIG_ENDIAN;
#[cfg(target_endian = "big")]
pub const DRM_FORMAT_HOST_RGB565: u32 = DRM_FORMAT_RGB565 | DRM_FORMAT_BIG_ENDIAN;
#[cfg(target_endian = "big")]
pub const DRM_FORMAT_HOST_XRGB8888: u32 = DRM_FORMAT_BGRX8888;
#[cfg(target_endian = "big")]
pub const DRM_FORMAT_HOST_ARGB8888: u32 = DRM_FORMAT_BGRA8888;

#[cfg(not(target_endian = "big"))]
pub const DRM_FORMAT_HOST_XRGB1555: u32 = DRM_FORMAT_XRGB1555;
#[cfg(not(target_endian = "big"))]
pub const DRM_FORMAT_HOST_RGB565: u32 = DRM_FORMAT_RGB565;
#[cfg(not(target_endian = "big"))]
pub const DRM_FORMAT_HOST_XRGB8888: u32 = DRM_FORMAT_XRGB8888;
#[cfg(not(target_endian = "big"))]
pub const DRM_FORMAT_HOST_ARGB8888: u32 = DRM_FORMAT_ARGB8888;

#[repr(C)]
pub union DrmFormatInfoCpp {
    pub cpp: [u8; DRM_FORMAT_MAX_PLANES],
    pub char_per_block: [u8; DRM_FORMAT_MAX_PLANES],
}

/// struct drm_format_info - information about a DRM format
#[repr(C)]
pub struct drm_format_info {
    /// 4CC format identifier (DRM_FORMAT_*)
    pub format: u32,
    /// Color depth, valid for a subset of RGB formats only.
    pub depth: u8,
    /// Number of color planes (1 to 3)
    pub num_planes: u8,
    pub cpp: DrmFormatInfoCpp,
    /// Block width in pixels.
    pub block_w: [u8; DRM_FORMAT_MAX_PLANES],
    /// Block height in pixels.
    pub block_h: [u8; DRM_FORMAT_MAX_PLANES],
    /// Horizontal chroma subsampling factor
    pub hsub: u8,
    /// Vertical chroma subsampling factor
    pub vsub: u8,
    /// Does the format embed an alpha component?
    pub has_alpha: bool,
    /// Is it a YUV format?
    pub is_yuv: bool,
    /// Is it a color-indexed format?
    pub is_color_indexed: bool,
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_packed(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).num_planes == 1
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_semiplanar(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).num_planes == 2
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_planar(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).num_planes == 3
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_sampling_410(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).hsub == 4 && (*info).vsub == 4
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_sampling_411(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).hsub == 4 && (*info).vsub == 1
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_sampling_420(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).hsub == 2 && (*info).vsub == 2
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_sampling_422(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).hsub == 2 && (*info).vsub == 1
}

#[inline]
pub unsafe fn drm_format_info_is_yuv_sampling_444(info: *const drm_format_info) -> bool {
    (*info).is_yuv && (*info).hsub == 1 && (*info).vsub == 1
}

#[inline]
pub unsafe fn drm_format_info_plane_width(info: *const drm_format_info, width: i32, plane: i32) -> i32 {
    if info.is_null() || plane >= (*info).num_planes as i32 { return 0; }
    if plane == 0 { return width; }
    (width + (*info).hsub as i32 - 1) / (*info).hsub as i32
}

#[inline]
pub unsafe fn drm_format_info_plane_height(info: *const drm_format_info, height: i32, plane: i32) -> i32 {
    if info.is_null() || plane >= (*info).num_planes as i32 { return 0; }
    if plane == 0 { return height; }
    (height + (*info).vsub as i32 - 1) / (*info).vsub as i32
}

extern "C" {
    pub fn __drm_format_info(format: u32) -> *const drm_format_info;
    pub fn drm_format_info(format: u32) -> *const drm_format_info;
    pub fn drm_get_format_info(dev: *mut drm_device, pixel_format: u32, modifier: u64) -> *const drm_format_info;
    pub fn drm_mode_legacy_fb_format(bpp: u32, depth: u32) -> u32;
    pub fn drm_driver_legacy_fb_format(dev: *mut drm_device, bpp: u32, depth: u32) -> u32;
    pub fn drm_driver_color_mode_format(dev: *mut drm_device, color_mode: c_uint) -> u32;
    pub fn drm_format_info_block_width(info: *const drm_format_info, plane: i32) -> c_uint;
    pub fn drm_format_info_block_height(info: *const drm_format_info, plane: i32) -> c_uint;
    pub fn drm_format_info_bpp(info: *const drm_format_info, plane: i32) -> c_uint;
    pub fn drm_format_info_min_pitch(info: *const drm_format_info, plane: i32, buffer_width: c_uint) -> u64;
}

pub type c_uint = u32;

pub enum drm_device {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
