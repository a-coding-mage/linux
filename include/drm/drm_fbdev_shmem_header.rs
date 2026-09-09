/* SPDX-License-Identifier: MIT */

// C dependency declarations:
// struct drm_fb_helper;
// struct drm_fb_helper_surface_size;

#[repr(C)]
pub struct drm_fb_helper {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_fb_helper_surface_size {
    _private: [u8; 0],
}

// CONFIG_DRM_FBDEV_EMULATION is a build-time C configuration condition.
// When enabled, this declares the external fbdev probe function.
#[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
extern "C" {
    pub fn drm_fbdev_shmem_driver_fbdev_probe(
        fb_helper: *mut drm_fb_helper,
        sizes: *mut drm_fb_helper_surface_size,
    ) -> ::core::ffi::c_int;
}

// C macro equivalent when CONFIG_DRM_FBDEV_EMULATION is enabled:
// DRM_FBDEV_SHMEM_DRIVER_OPS expands to
//     .fbdev_probe = drm_fbdev_shmem_driver_fbdev_probe
// When disabled, it expands to
//     .fbdev_probe = NULL

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
