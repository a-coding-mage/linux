/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2019 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

// __DRM_BRIDGE_CONNECTOR_H__ include guard from the C header.

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_bridge_connector_init(
        drm: *mut drm_device,
        encoder: *mut drm_encoder,
    ) -> *mut drm_connector;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
