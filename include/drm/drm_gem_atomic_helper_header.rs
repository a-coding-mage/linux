/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Rust translation of drm_gem_atomic_helper.h. */

use core::ffi::c_uint;

/* Types supplied by the surrounding DRM headers. */
#[repr(C)]
pub struct drm_plane_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_plane {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_format_conv_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iosys_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_simple_display_pipe {
    _private: [u8; 0],
}

/* DRM_FORMAT_MAX_PLANES is supplied by drm_fourcc.h. */
pub const DRM_SHADOW_PLANE_MAX_WIDTH: c_uint = 4096u32;
pub const DRM_SHADOW_PLANE_MAX_HEIGHT: c_uint = 4096u32;

#[repr(C)]
pub struct drm_shadow_plane_state {
    /** @base: plane state */
    pub base: drm_plane_state,
    /** @fmtcnv_state: Format-conversion state */
    pub fmtcnv_state: drm_format_conv_state,
    /* Transitional state - do not export or duplicate */
    /** @map: Mappings of the plane's framebuffer BOs into kernel address space */
    pub map: [iosys_map; DRM_FORMAT_MAX_PLANES],
    /** @data: Address of each framebuffer BO's data */
    pub data: [iosys_map; DRM_FORMAT_MAX_PLANES],
}

/** Upcast from struct drm_plane_state. */
#[inline]
pub unsafe fn to_drm_shadow_plane_state(
    state: *mut drm_plane_state,
) -> *mut drm_shadow_plane_state {
    /* `base` is the first member, as required by the C container_of use. */
    state as *mut drm_shadow_plane_state
}

extern "C" {
    pub fn drm_gem_plane_helper_prepare_fb(
        plane: *mut drm_plane,
        state: *mut drm_plane_state,
    ) -> i32;

    pub fn __drm_gem_duplicate_shadow_plane_state(
        plane: *mut drm_plane,
        new_shadow_plane_state: *mut drm_shadow_plane_state,
    );
    pub fn __drm_gem_destroy_shadow_plane_state(
        shadow_plane_state: *mut drm_shadow_plane_state,
    );
    pub fn __drm_gem_reset_shadow_plane(
        plane: *mut drm_plane,
        shadow_plane_state: *mut drm_shadow_plane_state,
    );

    pub fn drm_gem_reset_shadow_plane(plane: *mut drm_plane);
    pub fn drm_gem_duplicate_shadow_plane_state(
        plane: *mut drm_plane,
    ) -> *mut drm_plane_state;
    pub fn drm_gem_destroy_shadow_plane_state(
        plane: *mut drm_plane,
        plane_state: *mut drm_plane_state,
    );

    pub fn drm_gem_begin_shadow_fb_access(
        plane: *mut drm_plane,
        plane_state: *mut drm_plane_state,
    ) -> i32;
    pub fn drm_gem_end_shadow_fb_access(
        plane: *mut drm_plane,
        plane_state: *mut drm_plane_state,
    );

    pub fn drm_gem_simple_kms_begin_shadow_fb_access(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    ) -> i32;
    pub fn drm_gem_simple_kms_end_shadow_fb_access(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    );
    pub fn drm_gem_simple_kms_reset_shadow_plane(pipe: *mut drm_simple_display_pipe);
    pub fn drm_gem_simple_kms_duplicate_shadow_plane_state(
        pipe: *mut drm_simple_display_pipe,
    ) -> *mut drm_plane_state;
    pub fn drm_gem_simple_kms_destroy_shadow_plane_state(
        pipe: *mut drm_simple_display_pipe,
        plane_state: *mut drm_plane_state,
    );
}

/*
 * C initializer-fragment macros, preserved as Rust documentation because Rust
 * has no direct equivalent of structure initializer member fragments:
 *
 * DRM_GEM_SHADOW_PLANE_FUNCS:
 *   reset = drm_gem_reset_shadow_plane,
 *   atomic_duplicate_state = drm_gem_duplicate_shadow_plane_state,
 *   atomic_destroy_state = drm_gem_destroy_shadow_plane_state
 *
 * DRM_GEM_SHADOW_PLANE_HELPER_FUNCS:
 *   begin_fb_access = drm_gem_begin_shadow_fb_access,
 *   end_fb_access = drm_gem_end_shadow_fb_access
 *
 * DRM_GEM_SIMPLE_DISPLAY_PIPE_SHADOW_PLANE_FUNCS:
 *   begin_fb_access = drm_gem_simple_kms_begin_shadow_fb_access,
 *   end_fb_access = drm_gem_simple_kms_end_shadow_fb_access,
 *   reset_plane = drm_gem_simple_kms_reset_shadow_plane,
 *   duplicate_plane_state = drm_gem_simple_kms_duplicate_shadow_plane_state,
 *   destroy_plane_state = drm_gem_simple_kms_destroy_shadow_plane_state
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
