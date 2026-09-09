/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Marvell International Ltd. All rights reserved.
 */

use core::ffi::c_void;

pub const MV_USB_MODE_OTG: i32 = 0;
pub const MV_USB_MODE_HOST: i32 = 1;

pub const VBUS_LOW: i32 = 0;
pub const VBUS_HIGH: i32 = 1 << 0;

#[repr(C)]
pub struct mv_usb_addon_irq {
    pub irq: core::ffi::c_uint,
    pub poll: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

#[repr(C)]
pub struct mv_usb_platform_data {
    /// Only valid for OTG. ID pin change.
    pub id: *mut mv_usb_addon_irq,
    /// Valid for OTG/UDC. VBUS change.
    pub vbus: *mut mv_usb_addon_irq,

    /// Only valid for HCD. OTG or Host only.
    pub mode: core::ffi::c_uint,

    /// This flag is used for that needs id pin checked by otg.
    /// C bitfield width: 1 bit.
    pub disable_otg_clock_gating: core::ffi::c_uint,
    /// Force a_bus_req to be asserted.
    /// C bitfield width: 1 bit.
    pub otg_force_a_bus_req: core::ffi::c_uint,

    pub phy_init: Option<unsafe extern "C" fn(regbase: *mut c_void) -> core::ffi::c_int>,
    pub phy_deinit: Option<unsafe extern "C" fn(regbase: *mut c_void)>,
    pub set_vbus: Option<unsafe extern "C" fn(vbus: core::ffi::c_uint) -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
