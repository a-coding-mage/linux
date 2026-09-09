/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * omap_usb.h -- omap usb2 phy header file
 *
 * Copyright (C) 2012-2020 Texas Instruments Incorporated - http://www.ti.com
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 */

// Dependency supplied by the Linux USB PHY companion header.

/// Equivalent of the C `phy_to_omapusb(x)` container-of macro.
#[macro_export]
macro_rules! phy_to_omapusb {
    ($x:expr) => {
        container_of!($x, omap_usb, phy)
    };
}

#[cfg(any(feature = "CONFIG_OMAP_USB2", feature = "CONFIG_OMAP_USB2_MODULE"))]
extern "C" {
    pub fn omap_usb2_set_comparator(
        comparator: *mut phy_companion,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(any(feature = "CONFIG_OMAP_USB2", feature = "CONFIG_OMAP_USB2_MODULE")))]
#[inline]
pub unsafe fn omap_usb2_set_comparator(
    _comparator: *mut phy_companion,
) -> ::core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
