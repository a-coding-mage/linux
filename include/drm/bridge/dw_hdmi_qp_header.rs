/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021-2022 Rockchip Electronics Co., Ltd.
 * Copyright (c) 2024 Collabora Ltd.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dw_hdmi_qp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

/* External enum drm_connector_status, represented by its C integer ABI type. */
pub type drm_connector_status = i32;

#[repr(C)]
pub struct dw_hdmi_qp_phy_ops {
    pub init: Option<unsafe extern "C" fn(hdmi: *mut dw_hdmi_qp, data: *mut c_void) -> i32>,
    pub disable: Option<unsafe extern "C" fn(hdmi: *mut dw_hdmi_qp, data: *mut c_void)>,
    pub read_hpd: Option<
        unsafe extern "C" fn(hdmi: *mut dw_hdmi_qp, data: *mut c_void) -> drm_connector_status,
    >,
    pub setup_hpd: Option<unsafe extern "C" fn(hdmi: *mut dw_hdmi_qp, data: *mut c_void)>,
}

#[repr(C)]
pub struct dw_hdmi_qp_plat_data {
    pub phy_ops: *const dw_hdmi_qp_phy_ops,
    pub phy_data: *mut c_void,
    pub main_irq: i32,
    pub cec_irq: i32,
    pub ref_clk_rate: core::ffi::c_ulong,
    /* Supported output formats: bitmask of @drm_output_color_format */
    pub supported_formats: u32,
    /* Maximum bits per color channel: 8, 10 or 12 */
    pub max_bpc: u32,
}

unsafe extern "C" {
    pub fn dw_hdmi_qp_bind(
        pdev: *mut platform_device,
        encoder: *mut drm_encoder,
        plat_data: *const dw_hdmi_qp_plat_data,
    ) -> *mut dw_hdmi_qp;
    pub fn dw_hdmi_qp_suspend(dev: *mut device, hdmi: *mut dw_hdmi_qp);
    pub fn dw_hdmi_qp_resume(dev: *mut device, hdmi: *mut dw_hdmi_qp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
