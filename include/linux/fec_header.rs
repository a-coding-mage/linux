/* SPDX-License-Identifier: GPL-2.0-only */
/* include/linux/fec.h
 *
 * Copyright (c) 2009 Orex Computed Radiography
 *   Baruch Siach <baruch@tkos.co.il>
 *
 * Copyright (C) 2010 Freescale Semiconductor, Inc.
 *
 * Header file for the FEC platform data
 */

// Dependency supplied by the corresponding Linux PHY definitions.
// `ETH_ALEN` is likewise supplied by the surrounding Linux definitions.

#[repr(C)]
pub struct fec_platform_data {
    pub phy: phy_interface_t,
    pub mac: [::core::ffi::c_uchar; ETH_ALEN],
    pub sleep_mode_enable: Option<unsafe extern "C" fn(enabled: ::core::ffi::c_int)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
