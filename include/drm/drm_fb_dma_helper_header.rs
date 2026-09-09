/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux types headers in the original source.
// The concrete Rust definition is provided by the surrounding translation.
use crate::dma_addr_t;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_framebuffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_plane {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_plane_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_scanout_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_gem_dma_object {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_fb_dma_get_gem_obj(
        fb: *mut drm_framebuffer,
        plane: ::core::ffi::c_uint,
    ) -> *mut drm_gem_dma_object;

    pub fn drm_fb_dma_get_gem_addr(
        fb: *mut drm_framebuffer,
        state: *mut drm_plane_state,
        plane: ::core::ffi::c_uint,
    ) -> dma_addr_t;

    pub fn drm_fb_dma_sync_non_coherent(
        drm: *mut drm_device,
        old_state: *mut drm_plane_state,
        state: *mut drm_plane_state,
    );

    pub fn drm_fb_dma_get_scanout_buffer(
        plane: *mut drm_plane,
        sb: *mut drm_scanout_buffer,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
