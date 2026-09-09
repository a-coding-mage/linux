/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024, Fuzhou Rockchip Electronics Co., Ltd
 *
 * Authors: Guochun Huang <hero.huang@rock-chips.com>
 *          Heiko Stuebner <heiko.stuebner@cherry.de>
 */

// Dependencies supplied by the surrounding kernel/Rust environment.

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dw_mipi_dsi2 {
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
pub enum drm_mode_status {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dw_mipi_dsi2_phy_type {
    DW_MIPI_DSI2_DPHY,
    DW_MIPI_DSI2_CPHY,
}

#[repr(C)]
pub struct dw_mipi_dsi2_phy_iface {
    pub ppi_width: ::core::ffi::c_int,
    pub phy_type: dw_mipi_dsi2_phy_type,
}

#[repr(C)]
pub struct dw_mipi_dsi2_phy_timing {
    pub data_hs2lp: u32,
    pub data_lp2hs: u32,
}

#[repr(C)]
pub struct dw_mipi_dsi2_phy_ops {
    pub init: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub power_on: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void)>,
    pub power_off: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void)>,
    pub get_interface: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, iface: *mut dw_mipi_dsi2_phy_iface)>,
    pub get_lane_mbps: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, mode: *const drm_display_mode, mode_flags: ::core::ffi::c_ulong, lanes: u32, format: u32, lane_mbps: *mut u32) -> ::core::ffi::c_int>,
    pub get_timing: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, lane_mbps: u32, timing: *mut dw_mipi_dsi2_phy_timing) -> ::core::ffi::c_int>,
    pub get_esc_clk_rate: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, esc_clk_rate: *mut u32) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct dw_mipi_dsi2_host_ops {
    pub attach: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, dsi: *mut mipi_dsi_device) -> ::core::ffi::c_int>,
    pub detach: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, dsi: *mut mipi_dsi_device) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct dw_mipi_dsi2_plat_data {
    pub regmap: *mut regmap,
    pub max_data_lanes: ::core::ffi::c_uint,
    pub mode_valid: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, mode: *const drm_display_mode, mode_flags: ::core::ffi::c_ulong, lanes: u32, format: u32) -> drm_mode_status>,
    pub mode_fixup: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, mode: *const drm_display_mode, adjusted_mode: *mut drm_display_mode) -> bool>,
    pub get_input_bus_fmts: Option<unsafe extern "C" fn(priv_data: *mut ::core::ffi::c_void, bridge: *mut drm_bridge, bridge_state: *mut drm_bridge_state, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state, output_fmt: u32, num_input_fmts: *mut ::core::ffi::c_uint) -> *mut u32>,
    pub phy_ops: *const dw_mipi_dsi2_phy_ops,
    pub host_ops: *const dw_mipi_dsi2_host_ops,
    pub priv_data: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    pub fn dw_mipi_dsi2_probe(pdev: *mut platform_device, plat_data: *const dw_mipi_dsi2_plat_data) -> *mut dw_mipi_dsi2;
    pub fn dw_mipi_dsi2_remove(dsi2: *mut dw_mipi_dsi2);
    pub fn dw_mipi_dsi2_bind(dsi2: *mut dw_mipi_dsi2, encoder: *mut drm_encoder) -> ::core::ffi::c_int;
    pub fn dw_mipi_dsi2_unbind(dsi2: *mut dw_mipi_dsi2);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
