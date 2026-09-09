/*
 * Copyright © 2006 Keith Packard
 * Copyright © 2007-2008 Dave Airlie
 * Copyright © 2007-2008 Intel Corporation
 *   Jesse Barnes <jesse.barnes@intel.com>
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

/*
 * The DRM mode setting helper functions are common code for drivers to use if
 * they wish. Drivers are not forced to use this code in their
 * implementations but it would be useful if they code they do use at least
 * provides a consistent interface and operation to userspace.
 */

// The original header guard and Linux type include are C-only constructs.

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_connector {
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
pub struct drm_display_mode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_mode_set {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_helper_disable_unused_functions(dev: *mut drm_device);
    pub fn drm_crtc_helper_set_config(
        set: *mut drm_mode_set,
        ctx: *mut drm_modeset_acquire_ctx,
    ) -> i32;
    pub fn drm_crtc_helper_set_mode(
        crtc: *mut drm_crtc,
        mode: *mut drm_display_mode,
        x: i32,
        y: i32,
        old_fb: *mut drm_framebuffer,
    ) -> bool;
    pub fn drm_crtc_helper_atomic_check(
        crtc: *mut drm_crtc,
        state: *mut drm_atomic_commit,
    ) -> i32;
    pub fn drm_helper_crtc_in_use(crtc: *mut drm_crtc) -> bool;
    pub fn drm_helper_encoder_in_use(encoder: *mut drm_encoder) -> bool;

    pub fn drm_helper_connector_dpms(connector: *mut drm_connector, mode: i32) -> i32;

    pub fn drm_helper_resume_force_mode(dev: *mut drm_device);
    pub fn drm_helper_force_disable_all(dev: *mut drm_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
