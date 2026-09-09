/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025 Rockchip Electronics Co., Ltd.
 */

// Translated from the C header. The Linux type dependency is represented by
// the corresponding Rust primitive types used below.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inno_hdmi {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inno_hdmi_plat_ops {
    pub enable: Option<unsafe extern "C" fn(
        pdev: *mut device,
        mode: *mut drm_display_mode,
    )>,
}

#[repr(C)]
pub struct inno_hdmi_phy_config {
    pub pixelclock: usize,
    pub pre_emphasis: u8,
    pub voltage_level_control: u8,
}

#[repr(C)]
pub struct inno_hdmi_plat_data {
    pub ops: *const inno_hdmi_plat_ops,
    pub phy_configs: *mut inno_hdmi_phy_config,
    pub default_phy_config: *mut inno_hdmi_phy_config,
}

unsafe extern "C" {
    pub fn inno_hdmi_bind(
        pdev: *mut device,
        encoder: *mut drm_encoder,
        plat_data: *const inno_hdmi_plat_data,
    ) -> *mut inno_hdmi;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
