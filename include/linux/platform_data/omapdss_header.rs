/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Texas Instruments, Inc.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum omapdss_version {
    OMAPDSS_VER_UNKNOWN = 0,
    OMAPDSS_VER_OMAP24xx,
    OMAPDSS_VER_OMAP34xx_ES1, // OMAP3430 ES1.0, 2.0
    OMAPDSS_VER_OMAP34xx_ES3, // OMAP3430 ES3.0+
    OMAPDSS_VER_OMAP3630,
    OMAPDSS_VER_AM35xx,
    OMAPDSS_VER_OMAP4430_ES1, // OMAP4430 ES1.0
    OMAPDSS_VER_OMAP4430_ES2, // OMAP4430 ES2.0, 2.1, 2.2
    OMAPDSS_VER_OMAP4,         // All other OMAP4s
    OMAPDSS_VER_OMAP5,
    OMAPDSS_VER_AM43xx,
    OMAPDSS_VER_DRA7xx,
}

// External dependency supplied by another translation unit.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* Board specific data */
#[repr(C)]
pub struct omap_dss_board_info {
    pub dsi_enable_pads:
        Option<unsafe extern "C" fn(dsi_id: ::std::os::raw::c_int, lane_mask: ::std::os::raw::c_uint) -> ::std::os::raw::c_int>,
    pub dsi_disable_pads:
        Option<unsafe extern "C" fn(dsi_id: ::std::os::raw::c_int, lane_mask: ::std::os::raw::c_uint)>,
    pub set_min_bus_tput:
        Option<unsafe extern "C" fn(dev: *mut device, r: ::std::os::raw::c_ulong) -> ::std::os::raw::c_int>,
    pub version: omapdss_version,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
