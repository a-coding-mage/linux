/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Noralf Trønnes
 */

/* C header dependency: linux/types.h */

use core::ffi::c_void;

pub enum drm_device {}
pub enum drm_format_info {}
pub enum drm_framebuffer {}
pub enum drm_rect {}
pub enum iosys_map {}

/* The definition is supplied by the translated linux/types.h dependency. */
pub type gfp_t = u32;

/**
 * struct drm_format_conv_state - Stores format-conversion state
 *
 * DRM helpers for format conversion store temporary state in
 * struct drm_xfrm_buf. The buffer's resources can be reused
 * among multiple conversion operations.
 *
 * All fields are considered private.
 */
#[repr(C)]
pub struct drm_format_conv_state {
    /* private: */
    pub tmp: drm_format_conv_state_tmp,
}

#[repr(C)]
pub struct drm_format_conv_state_tmp {
    pub mem: *mut c_void,
    pub size: usize,
    pub preallocated: bool,
}

#[inline]
pub const fn __DRM_FORMAT_CONV_STATE_INIT(
    mem: *mut c_void,
    size: usize,
    preallocated: bool,
) -> drm_format_conv_state {
    drm_format_conv_state {
        tmp: drm_format_conv_state_tmp {
            mem,
            size,
            preallocated,
        },
    }
}

/**
 * DRM_FORMAT_CONV_STATE_INIT - Initializer for struct drm_format_conv_state
 *
 * Initializes an instance of struct drm_format_conv_state to default values.
 */
pub const DRM_FORMAT_CONV_STATE_INIT: drm_format_conv_state =
    drm_format_conv_state {
        tmp: drm_format_conv_state_tmp {
            mem: core::ptr::null_mut(),
            size: 0,
            preallocated: false,
        },
    };

/**
 * DRM_FORMAT_CONV_STATE_INIT_PREALLOCATED - Initializer for struct drm_format_conv_state
 * @_mem: The preallocated memory area
 * @_size: The number of bytes in _mem
 *
 * Initializes an instance of struct drm_format_conv_state to preallocated
 * storage. The caller is responsible for releasing the provided memory range.
 */
#[inline]
pub const fn DRM_FORMAT_CONV_STATE_INIT_PREALLOCATED(
    mem: *mut c_void,
    size: usize,
) -> drm_format_conv_state {
    __DRM_FORMAT_CONV_STATE_INIT(mem, size, true)
}

extern "C" {
    pub fn drm_format_conv_state_init(state: *mut drm_format_conv_state);
    pub fn drm_format_conv_state_copy(
        state: *mut drm_format_conv_state,
        old_state: *const drm_format_conv_state,
    );
    pub fn drm_format_conv_state_reserve(
        state: *mut drm_format_conv_state,
        new_size: usize,
        flags: gfp_t,
    ) -> *mut c_void;
    pub fn drm_format_conv_state_release(state: *mut drm_format_conv_state);

    pub fn drm_fb_clip_offset(
        pitch: u32,
        format: *const drm_format_info,
        clip: *const drm_rect,
    ) -> u32;

    pub fn drm_fb_memcpy(
        dst: *mut iosys_map,
        dst_pitch: *const u32,
        src: *const iosys_map,
        fb: *const drm_framebuffer,
        clip: *const drm_rect,
    );
    pub fn drm_fb_swab(
        dst: *mut iosys_map,
        dst_pitch: *const u32,
        src: *const iosys_map,
        fb: *const drm_framebuffer,
        clip: *const drm_rect,
        cached: bool,
        state: *mut drm_format_conv_state,
    );

    pub fn drm_fb_xrgb8888_to_rgb332(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_rgb565(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_rgb565be(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_xrgb1555(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_argb1555(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_rgba5551(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_rgb888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_bgr888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_argb8888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_abgr8888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_xbgr8888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_bgrx8888(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_xrgb2101010(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_argb2101010(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_gray8(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_argb8888_to_argb4444(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_mono(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
    pub fn drm_fb_xrgb8888_to_gray2(dst: *mut iosys_map, dst_pitch: *const u32, src: *const iosys_map, fb: *const drm_framebuffer, clip: *const drm_rect, state: *mut drm_format_conv_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
