/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * shmob_drm.h  --  SH Mobile DRM driver
 *
 * Copyright (C) 2012 Renesas Corporation
 *
 * Laurent Pinchart (laurent.pinchart@ideasonboard.com)
 */

// Dependency corresponding to <video/videomode.h> is supplied externally.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum shmob_drm_clk_source {
    SHMOB_DRM_CLK_BUS,
    SHMOB_DRM_CLK_PERIPHERAL,
    SHMOB_DRM_CLK_EXTERNAL,
}

#[repr(C)]
pub struct shmob_drm_panel_data {
    pub width_mm: ::core::ffi::c_uint, // Panel width in mm
    pub height_mm: ::core::ffi::c_uint, // Panel height in mm
    pub mode: videomode,
}

#[repr(C)]
pub struct shmob_drm_interface_data {
    pub bus_fmt: ::core::ffi::c_uint, // MEDIA_BUS_FMT_*
    pub clk_div: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct shmob_drm_platform_data {
    pub clk_source: shmob_drm_clk_source,
    pub iface: shmob_drm_interface_data,
    pub panel: shmob_drm_panel_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
