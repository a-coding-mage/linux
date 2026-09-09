/*
 * Copyright (C) 2011-2013 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_plane {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_plane_helper_update_primary(
        plane: *mut drm_plane,
        crtc: *mut drm_crtc,
        fb: *mut drm_framebuffer,
        crtc_x: i32,
        crtc_y: i32,
        crtc_w: u32,
        crtc_h: u32,
        src_x: u32,
        src_y: u32,
        src_w: u32,
        src_h: u32,
        ctx: *mut drm_modeset_acquire_ctx,
    ) -> i32;

    pub fn drm_plane_helper_disable_primary(
        plane: *mut drm_plane,
        ctx: *mut drm_modeset_acquire_ctx,
    ) -> i32;

    pub fn drm_plane_helper_destroy(plane: *mut drm_plane);
}

/**
 * DRM_PLANE_NON_ATOMIC_FUNCS - Default plane functions for non-atomic drivers
 *
 * This macro initializes plane functions for non-atomic drivers to default
 * values. Non-atomic interfaces are deprecated and should not be used in new
 * drivers.
 */
#[macro_export]
macro_rules! DRM_PLANE_NON_ATOMIC_FUNCS {
    ($plane:expr) => {
        $plane.update_plane = drm_plane_helper_update_primary;
        $plane.disable_plane = drm_plane_helper_disable_primary;
        $plane.destroy = drm_plane_helper_destroy;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
