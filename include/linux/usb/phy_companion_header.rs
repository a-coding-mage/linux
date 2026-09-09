// SPDX-License-Identifier: GPL-2.0+
/*
 * phy-companion.h -- phy companion to indicate the comparator part of PHY
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 */

// Dependency intent: declarations from <linux/usb/otg.h> are supplied externally.

/* phy_companion to take care of VBUS, ID and srp capabilities */
#[repr(C)]
pub struct phy_companion {
    /* effective for A-peripheral, ignored for B devices */
    pub set_vbus:
        Option<unsafe extern "C" fn(x: *mut phy_companion, enabled: bool) -> ::core::ffi::c_int>,

    /* for B devices only:  start session with A-Host */
    pub start_srp:
        Option<unsafe extern "C" fn(x: *mut phy_companion) -> ::core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
