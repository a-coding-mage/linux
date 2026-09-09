/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2023 Linaro Ltd.
 *
 * Author: Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
 */

// Dependency: declarations supplied by the DRM and device subsystems.

#[repr(C)]
pub struct auxiliary_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// Dependency: enum drm_connector_status supplied by drm_connector.h.
pub type drm_connector_status = i32;

#[cfg(feature = "CONFIG_DRM_AUX_BRIDGE")]
unsafe extern "C" {
    pub fn drm_aux_bridge_register(parent: *mut device) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_DRM_AUX_BRIDGE"))]
#[inline]
pub unsafe fn drm_aux_bridge_register(_parent: *mut device) -> ::core::ffi::c_int {
    0
}

#[cfg(feature = "CONFIG_DRM_AUX_HPD_BRIDGE")]
unsafe extern "C" {
    pub fn devm_drm_dp_hpd_bridge_alloc(
        parent: *mut device,
        np: *mut device_node,
    ) -> *mut auxiliary_device;
    pub fn devm_drm_dp_hpd_bridge_add(
        dev: *mut device,
        adev: *mut auxiliary_device,
    ) -> ::core::ffi::c_int;
    pub fn drm_dp_hpd_bridge_register(
        parent: *mut device,
        np: *mut device_node,
    ) -> *mut device;
    pub fn drm_aux_hpd_bridge_notify(
        dev: *mut device,
        status: drm_connector_status,
    );
}

#[cfg(not(feature = "CONFIG_DRM_AUX_HPD_BRIDGE"))]
#[inline]
pub unsafe fn devm_drm_dp_hpd_bridge_alloc(
    _parent: *mut device,
    _np: *mut device_node,
) -> *mut auxiliary_device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_DRM_AUX_HPD_BRIDGE"))]
#[inline]
pub unsafe fn devm_drm_dp_hpd_bridge_add(
    _dev: *mut device,
    _adev: *mut auxiliary_device,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DRM_AUX_HPD_BRIDGE"))]
#[inline]
pub unsafe fn drm_dp_hpd_bridge_register(
    _parent: *mut device,
    _np: *mut device_node,
) -> *mut device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_DRM_AUX_HPD_BRIDGE"))]
#[inline]
pub unsafe fn drm_aux_hpd_bridge_notify(
    _dev: *mut device,
    _status: drm_connector_status,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
