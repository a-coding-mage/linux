/* SPDX-License-Identifier: GPL-2.0 or MIT */

/*
 * Copyright (c) 2024 Intel
 * Copyright (c) 2024 Red Hat
 */

//! Rust translation of `drm_panic.h`.
//! C header dependencies are intentionally left as external symbols/types.

/// DRM scanout buffer.
///
/// This structure holds the information necessary for drm_panic to draw the
/// panic screen, and display it.
#[repr(C)]
pub struct drm_scanout_buffer {
    /// DRM format of the scanout buffer.
    pub format: *const drm_format_info,

    /// Virtual address of the scanout buffer, either in memory or iomem.
    /// The scanout buffer should be in linear format.
    pub map: [iosys_map; DRM_FORMAT_MAX_PLANES],

    /// Optional array of pages when the scanout buffer is not mapped.
    pub pages: *mut *mut page,

    /// Width of the scanout buffer, in pixels.
    pub width: core::ffi::c_uint,

    /// Height of the scanout buffer, in pixels.
    pub height: core::ffi::c_uint,

    /// Length in bytes between the start of two consecutive lines.
    pub pitch: [core::ffi::c_uint; DRM_FORMAT_MAX_PLANES],

    /// Optional callback to set a pixel color on the framebuffer.
    pub set_pixel: Option<unsafe extern "C" fn(
        sb: *mut drm_scanout_buffer,
        x: core::ffi::c_uint,
        y: core::ffi::c_uint,
        color: u32,
    )>,

    /// Private pointer available to callbacks.
    pub private: *mut core::ffi::c_void,
}

// Under CONFIG_DRM_PANIC these operations access
// dev->mode_config.panic_lock through the corresponding raw-spinlock helpers.
#[cfg(feature = "CONFIG_DRM_PANIC")]
#[macro_export]
macro_rules! drm_panic_trylock {
    ($dev:expr, $flags:expr) => {
        raw_spin_trylock_irqsave(unsafe { &mut (*$dev).mode_config.panic_lock }, $flags)
    };
}

#[cfg(feature = "CONFIG_DRM_PANIC")]
#[macro_export]
macro_rules! drm_panic_lock {
    ($dev:expr, $flags:expr) => {
        raw_spin_lock_irqsave(unsafe { &mut (*$dev).mode_config.panic_lock }, $flags)
    };
}

#[cfg(feature = "CONFIG_DRM_PANIC")]
#[macro_export]
macro_rules! drm_panic_unlock {
    ($dev:expr, $flags:expr) => {
        raw_spin_unlock_irqrestore(unsafe { &mut (*$dev).mode_config.panic_lock }, $flags)
    };
}

#[cfg(not(feature = "CONFIG_DRM_PANIC"))]
#[inline]
pub unsafe fn drm_panic_trylock(_dev: *mut drm_device, _flags: core::ffi::c_ulong) -> bool {
    true
}

#[cfg(not(feature = "CONFIG_DRM_PANIC"))]
#[inline]
pub unsafe fn drm_panic_lock(_dev: *mut drm_device, _flags: core::ffi::c_ulong) {}

#[cfg(not(feature = "CONFIG_DRM_PANIC"))]
#[inline]
pub unsafe fn drm_panic_unlock(_dev: *mut drm_device, _flags: core::ffi::c_ulong) {}

#[cfg(feature = "CONFIG_DRM_PANIC_SCREEN_QR_CODE")]
extern "C" {
    pub fn drm_panic_qr_max_data_size(version: u8, url_len: usize) -> usize;

    pub fn drm_panic_qr_generate(
        url: *const core::ffi::c_char,
        data: *mut u8,
        data_len: usize,
        data_size: usize,
        tmp: *mut u8,
        tmp_size: usize,
    ) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
