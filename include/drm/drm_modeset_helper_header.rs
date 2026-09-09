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
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT,
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

// Forward declarations corresponding to the C header's incomplete structs.
#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc_funcs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_format_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_mode_fb_cmd2 {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_helper_move_panel_connectors_to_head(dev: *mut drm_device);

    pub fn drm_helper_mode_fill_fb_struct(
        dev: *mut drm_device,
        fb: *mut drm_framebuffer,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
    );

    pub fn drm_crtc_init(
        dev: *mut drm_device,
        crtc: *mut drm_crtc,
        funcs: *const drm_crtc_funcs,
    ) -> ::std::os::raw::c_int;

    pub fn drm_mode_config_helper_suspend(dev: *mut drm_device) -> ::std::os::raw::c_int;
    pub fn drm_mode_config_helper_resume(dev: *mut drm_device) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
