/* SPDX-License-Identifier: MIT */

// Translation of drm_fbdev_ttm.h.
// The C header includes linux/stddef.h; its dependency is supplied externally.

#[repr(C)]
pub struct drm_fb_helper {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_fb_helper_surface_size {
    _private: [u8; 0],
}

// Preserves the build-time CONFIG_DRM_FBDEV_EMULATION condition from the C header.
#[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
unsafe extern "C" {
    pub fn drm_fbdev_ttm_driver_fbdev_probe(
        fb_helper: *mut drm_fb_helper,
        sizes: *mut drm_fb_helper_surface_size,
    ) -> ::core::ffi::c_int;
}

// C macro equivalent when CONFIG_DRM_FBDEV_EMULATION is enabled:
// .fbdev_probe = drm_fbdev_ttm_driver_fbdev_probe
#[cfg(feature = "CONFIG_DRM_FBDEV_EMULATION")]
#[macro_export]
macro_rules! DRM_FBDEV_TTM_DRIVER_OPS {
    () => {
        fbdev_probe: Some($crate::drm_fbdev_ttm_driver_fbdev_probe)
    };
}

// C macro equivalent when CONFIG_DRM_FBDEV_EMULATION is disabled:
// .fbdev_probe = NULL
#[cfg(not(feature = "CONFIG_DRM_FBDEV_EMULATION"))]
#[macro_export]
macro_rules! DRM_FBDEV_TTM_DRIVER_OPS {
    () => {
        fbdev_probe: None
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
