/*
 * Copyright (C) 2014 Red Hat
 * Copyright (C) 2014 Intel Corp.
 * Copyright (C) 2018 Intel Corp.
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
 * Authors:
 * Rob Clark <robdclark@gmail.com>
 * Daniel Vetter <daniel.vetter@ffwll.ch>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct drm_crtc_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_property_blob {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_plane_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_connector_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_colorop {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_atomic_set_mode_for_crtc(
        state: *mut drm_crtc_state,
        mode: *const drm_display_mode,
    ) -> c_int;

    pub fn drm_atomic_set_mode_prop_for_crtc(
        state: *mut drm_crtc_state,
        blob: *mut drm_property_blob,
    ) -> c_int;

    pub fn drm_atomic_set_crtc_for_plane(
        plane_state: *mut drm_plane_state,
        crtc: *mut drm_crtc,
    ) -> c_int;

    pub fn drm_atomic_set_fb_for_plane(
        plane_state: *mut drm_plane_state,
        fb: *mut drm_framebuffer,
    );

    pub fn drm_atomic_set_colorop_for_plane(
        plane_state: *mut drm_plane_state,
        colorop: *mut drm_colorop,
    ) -> bool;

    pub fn drm_atomic_set_crtc_for_connector(
        conn_state: *mut drm_connector_state,
        crtc: *mut drm_crtc,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
