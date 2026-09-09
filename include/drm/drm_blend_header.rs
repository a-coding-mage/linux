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
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

// Dependencies supplied by the surrounding DRM/Linux translation.

pub const DRM_MODE_BLEND_PREMULTI: u32 = 0;
pub const DRM_MODE_BLEND_COVERAGE: u32 = 1;
pub const DRM_MODE_BLEND_PIXEL_NONE: u32 = 2;

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_plane {
    _private: [u8; 0],
}

#[inline]
pub fn drm_rotation_90_or_270(rotation: u32) -> bool {
    (rotation & (DRM_MODE_ROTATE_90 | DRM_MODE_ROTATE_270)) != 0
}

pub const DRM_BLEND_ALPHA_OPAQUE: u32 = 0xffff;

unsafe extern "C" {
    pub fn drm_plane_create_alpha_property(plane: *mut drm_plane) -> i32;
    pub fn drm_plane_create_rotation_property(
        plane: *mut drm_plane,
        rotation: u32,
        supported_rotations: u32,
    ) -> i32;
    pub fn drm_rotation_simplify(rotation: u32, supported_rotations: u32) -> u32;

    pub fn drm_plane_create_zpos_property(
        plane: *mut drm_plane,
        zpos: u32,
        min: u32,
        max: u32,
    ) -> i32;
    pub fn drm_plane_create_zpos_immutable_property(plane: *mut drm_plane, zpos: u32) -> i32;
    pub fn drm_atomic_normalize_zpos(
        dev: *mut drm_device,
        state: *mut drm_atomic_commit,
    ) -> i32;
    pub fn drm_plane_create_blend_mode_property(plane: *mut drm_plane, supported_modes: u32) -> i32;
    pub fn drm_crtc_attach_background_color_property(crtc: *mut drm_crtc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
