/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Noralf Trønnes
 */

/*
 * Simple KMS helpers are deprected in favor of regular atomic helpers. Do not
 * use the min new code.
 */

// C header dependencies:
// #include <drm/drm_crtc.h>
// #include <drm/drm_encoder.h>
// #include <drm/drm_plane.h>

// Types declared by the included DRM headers are referenced below.

#[repr(C)]
pub struct drm_simple_display_pipe_funcs {
    pub mode_valid: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        mode: *const drm_display_mode,
    ) -> drm_mode_status>,
    pub enable: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        crtc_state: *mut drm_crtc_state,
        plane_state: *mut drm_plane_state,
    )>,
    pub disable: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe)>,
    pub check: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
        crtc_state: *mut drm_crtc_state,
    ) -> ::core::ffi::c_int>,
    pub update: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        old_plane_state: *mut drm_plane_state,
    )>,
    pub prepare_fb: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    ) -> ::core::ffi::c_int>,
    pub cleanup_fb: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    )>,
    pub begin_fb_access: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        new_plane_state: *mut drm_plane_state,
    ) -> ::core::ffi::c_int>,
    pub end_fb_access: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    )>,
    pub enable_vblank: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe) -> ::core::ffi::c_int>,
    pub disable_vblank: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe)>,
    pub reset_crtc: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe)>,
    pub duplicate_crtc_state: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe) -> *mut drm_crtc_state>,
    pub destroy_crtc_state: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        crtc_state: *mut drm_crtc_state,
    )>,
    pub reset_plane: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe)>,
    pub duplicate_plane_state: Option<unsafe extern "C" fn(pipe: *mut drm_simple_display_pipe) -> *mut drm_plane_state>,
    pub destroy_plane_state: Option<unsafe extern "C" fn(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    )>,
}

#[repr(C)]
pub struct drm_simple_display_pipe {
    pub crtc: drm_crtc,
    pub plane: drm_plane,
    pub encoder: drm_encoder,
    pub connector: *mut drm_connector,
    pub funcs: *const drm_simple_display_pipe_funcs,
}

extern "C" {
    pub fn drm_simple_display_pipe_attach_bridge(
        pipe: *mut drm_simple_display_pipe,
        bridge: *mut drm_bridge,
    ) -> ::core::ffi::c_int;

    pub fn drm_simple_display_pipe_init(
        dev: *mut drm_device,
        pipe: *mut drm_simple_display_pipe,
        funcs: *const drm_simple_display_pipe_funcs,
        formats: *const u32,
        format_count: ::core::ffi::c_uint,
        format_modifiers: *const u64,
        connector: *mut drm_connector,
    ) -> ::core::ffi::c_int;

    pub fn drm_simple_encoder_init(
        dev: *mut drm_device,
        encoder: *mut drm_encoder,
        encoder_type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn __drmm_simple_encoder_alloc(
        dev: *mut drm_device,
        size: usize,
        offset: usize,
        encoder_type: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
}

// C macro equivalent; `offset` must be supplied by the caller as the member offset.
#[macro_export]
macro_rules! drmm_simple_encoder_alloc {
    ($dev:expr, $type:ty, $member_offset:expr, $encoder_type:expr) => {
        __drmm_simple_encoder_alloc(
            $dev,
            ::core::mem::size_of::<$type>(),
            $member_offset,
            $encoder_type,
        ) as *mut $type
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
