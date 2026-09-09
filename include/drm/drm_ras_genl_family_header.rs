/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2026 Intel Corporation
 */

// C conditional: #if IS_ENABLED(CONFIG_DRM_RAS)
// The CONFIG_DRM_RAS build-time condition is preserved through this cfg.
#[cfg(feature = "CONFIG_DRM_RAS")]
extern "C" {
    pub fn drm_ras_genl_family_register() -> ::core::ffi::c_int;
    pub fn drm_ras_genl_family_unregister();
}

#[cfg(not(feature = "CONFIG_DRM_RAS"))]
#[inline]
pub fn drm_ras_genl_family_register() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DRM_RAS"))]
#[inline]
pub fn drm_ras_genl_family_unregister() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
