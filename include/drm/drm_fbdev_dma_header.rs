/* SPDX-License-Identifier: MIT */

// C forward declarations.
#[repr(C)]
pub struct drm_fb_helper {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_fb_helper_surface_size {
    _opaque: [u8; 0],
}

#[cfg(CONFIG_DRM_FBDEV_EMULATION)]
unsafe extern "C" {
    pub fn drm_fbdev_dma_driver_fbdev_probe(
        fb_helper: *mut drm_fb_helper,
        sizes: *mut drm_fb_helper_surface_size,
    ) -> ::core::ffi::c_int;
}

// C macro:
// #define DRM_FBDEV_DMA_DRIVER_OPS \
//     .fbdev_probe = drm_fbdev_dma_driver_fbdev_probe
#[cfg(CONFIG_DRM_FBDEV_EMULATION)]
#[macro_export]
macro_rules! DRM_FBDEV_DMA_DRIVER_OPS {
    () => {
        fbdev_probe: Some(drm_fbdev_dma_driver_fbdev_probe)
    };
}

// C macro:
// #define DRM_FBDEV_DMA_DRIVER_OPS \
//     .fbdev_probe = NULL
#[cfg(not(CONFIG_DRM_FBDEV_EMULATION))]
#[macro_export]
macro_rules! DRM_FBDEV_DMA_DRIVER_OPS {
    () => {
        fbdev_probe: None
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
