/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025 Rockchip Electronics Co., Ltd.
 */

// Dependency supplied externally: Linux device types and kernel integer aliases.

use core::ffi::c_void;

pub type u32 = ::core::primitive::u32;
pub type u8 = ::core::primitive::u8;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dw_dp {
    _private: [u8; 0],
}

#[repr(C)]
pub enum dw_dp_mp {
    DW_DP_MP_SINGLE_PIXEL = 0,
    DW_DP_MP_DUAL_PIXEL,
    DW_DP_MP_QUAD_PIXEL,
}

#[repr(C)]
pub struct dw_dp_plat_data {
    pub max_link_rate: u32,
    pub pixel_mode: u8,
}

unsafe extern "C" {
    pub fn dw_dp_bind(
        dev: *mut device,
        encoder: *mut drm_encoder,
        plat_data: *const dw_dp_plat_data,
    ) -> *mut dw_dp;
    pub fn dw_dp_unbind(dp: *mut dw_dp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
