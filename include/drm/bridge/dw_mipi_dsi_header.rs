/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) STMicroelectronics SA 2017
 *
 * Authors: Philippe Cornu <philippe.cornu@st.com>
 *          Yannick Fertre <yannick.fertre@st.com>
 */

// Dependencies supplied by the corresponding kernel DRM and Linux type headers
// are intentionally referenced but not implemented here.

#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dw_mipi_dsi {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mipi_dsi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_bridge {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_bridge_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dw_mipi_dsi_dphy_timing {
    pub data_hs2lp: u16,
    pub data_lp2hs: u16,
    pub clk_hs2lp: u16,
    pub clk_lp2hs: u16,
}

#[repr(C)]
pub struct dw_mipi_dsi_phy_ops {
    pub init: Option<unsafe extern "C" fn(priv_data: *mut core::ffi::c_void) -> i32>,
    pub power_on: Option<unsafe extern "C" fn(priv_data: *mut core::ffi::c_void)>,
    pub power_off: Option<unsafe extern "C" fn(priv_data: *mut core::ffi::c_void)>,
    pub get_lane_mbps: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        mode: *const drm_display_mode,
        mode_flags: usize,
        lanes: u32,
        format: u32,
        lane_mbps: *mut u32,
    ) -> i32>,
    pub get_timing: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        lane_mbps: u32,
        timing: *mut dw_mipi_dsi_dphy_timing,
    ) -> i32>,
    pub get_esc_clk_rate: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        esc_clk_rate: *mut u32,
    ) -> i32>,
}

#[repr(C)]
pub struct dw_mipi_dsi_host_ops {
    pub attach: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        dsi: *mut mipi_dsi_device,
    ) -> i32>,
    pub detach: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        dsi: *mut mipi_dsi_device,
    ) -> i32>,
}

#[repr(C)]
pub struct dw_mipi_dsi_plat_data {
    pub base: *mut core::ffi::c_void,
    pub max_data_lanes: u32,
    pub mode_valid: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        mode: *const drm_display_mode,
        mode_flags: usize,
        lanes: u32,
        format: u32,
    ) -> i32>,
    pub mode_fixup: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        mode: *const drm_display_mode,
        adjusted_mode: *mut drm_display_mode,
    ) -> bool>,
    pub get_input_bus_fmts: Option<unsafe extern "C" fn(
        priv_data: *mut core::ffi::c_void,
        bridge: *mut drm_bridge,
        bridge_state: *mut drm_bridge_state,
        crtc_state: *mut drm_crtc_state,
        conn_state: *mut drm_connector_state,
        output_fmt: u32,
        num_input_fmts: *mut u32,
    ) -> *mut u32>,
    pub phy_ops: *const dw_mipi_dsi_phy_ops,
    pub host_ops: *const dw_mipi_dsi_host_ops,
    pub priv_data: *mut core::ffi::c_void,
}

extern "C" {
    pub fn dw_mipi_dsi_probe(
        pdev: *mut platform_device,
        plat_data: *const dw_mipi_dsi_plat_data,
    ) -> *mut dw_mipi_dsi;
    pub fn dw_mipi_dsi_remove(dsi: *mut dw_mipi_dsi);
    pub fn dw_mipi_dsi_bind(dsi: *mut dw_mipi_dsi, encoder: *mut drm_encoder) -> i32;
    pub fn dw_mipi_dsi_unbind(dsi: *mut dw_mipi_dsi);
    pub fn dw_mipi_dsi_set_slave(dsi: *mut dw_mipi_dsi, slave: *mut dw_mipi_dsi);
    pub fn dw_mipi_dsi_get_bridge(dsi: *mut dw_mipi_dsi) -> *mut drm_bridge;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
