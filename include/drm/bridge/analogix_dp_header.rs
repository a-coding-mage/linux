/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Analogix DP (Display Port) Core interface driver.
 *
 * Copyright (C) 2015 Rockchip Electronics Co., Ltd.
 */

use core::ffi::c_int;

// Supplied by the corresponding DRM and platform dependencies.
#[repr(C)]
pub struct analogix_dp_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_aux {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_panel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_bridge {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct component_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum analogix_dp_devtype {
    EXYNOS_DP,
    RK3288_DP,
    RK3399_EDP,
    RK3576_EDP,
    RK3588_EDP,
}

#[inline]
pub fn analogix_dp_is_rockchip(type_: analogix_dp_devtype) -> bool {
    match type_ {
        analogix_dp_devtype::RK3288_DP
        | analogix_dp_devtype::RK3399_EDP
        | analogix_dp_devtype::RK3576_EDP
        | analogix_dp_devtype::RK3588_EDP => true,
        _ => false,
    }
}

#[repr(C)]
pub struct analogix_dp_plat_data {
    pub dev_type: analogix_dp_devtype,
    pub panel: *mut drm_panel,
    pub next_bridge: *mut drm_bridge,
    pub encoder: *mut drm_encoder,
    pub connector: *mut drm_connector,
    pub ops: *const component_ops,
    pub power_on:
        Option<unsafe extern "C" fn(plat_data: *mut analogix_dp_plat_data) -> c_int>,
    pub power_off:
        Option<unsafe extern "C" fn(plat_data: *mut analogix_dp_plat_data) -> c_int>,
}

extern "C" {
    pub fn analogix_dp_resume(dp: *mut analogix_dp_device) -> c_int;
    pub fn analogix_dp_suspend(dp: *mut analogix_dp_device) -> c_int;

    pub fn analogix_dp_probe(
        dev: *mut device,
        plat_data: *mut analogix_dp_plat_data,
    ) -> *mut analogix_dp_device;
    pub fn analogix_dp_bind(
        dp: *mut analogix_dp_device,
        drm_dev: *mut drm_device,
    ) -> c_int;
    pub fn analogix_dp_unbind(dp: *mut analogix_dp_device);

    pub fn analogix_dp_start_crc(connector: *mut drm_connector) -> c_int;
    pub fn analogix_dp_stop_crc(connector: *mut drm_connector) -> c_int;

    pub fn analogix_dp_aux_to_plat_data(
        aux: *mut drm_dp_aux,
    ) -> *mut analogix_dp_plat_data;
    pub fn analogix_dp_get_aux(dp: *mut analogix_dp_device) -> *mut drm_dp_aux;
    pub fn analogix_dp_finish_probe(dp: *mut analogix_dp_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
