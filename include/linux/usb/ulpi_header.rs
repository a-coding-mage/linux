// SPDX-License-Identifier: GPL-2.0
/*
 * ulpi.h -- ULPI defines and function prorotypes
 *
 * Copyright (C) 2010 Nokia Corporation
 */

// Dependencies supplied by the corresponding Linux USB OTG and ULPI register
// definitions are intentionally not implemented here.

/*-------------------------------------------------------------------------*/

/*
 * ULPI Flags
 */
pub const ULPI_OTG_ID_PULLUP: u32 = 1 << 0;
pub const ULPI_OTG_DP_PULLDOWN_DIS: u32 = 1 << 1;
pub const ULPI_OTG_DM_PULLDOWN_DIS: u32 = 1 << 2;
pub const ULPI_OTG_DISCHRGVBUS: u32 = 1 << 3;
pub const ULPI_OTG_CHRGVBUS: u32 = 1 << 4;
pub const ULPI_OTG_DRVVBUS: u32 = 1 << 5;
pub const ULPI_OTG_DRVVBUS_EXT: u32 = 1 << 6;
pub const ULPI_OTG_EXTVBUSIND: u32 = 1 << 7;

pub const ULPI_IC_6PIN_SERIAL: u32 = 1 << 8;
pub const ULPI_IC_3PIN_SERIAL: u32 = 1 << 9;
pub const ULPI_IC_CARKIT: u32 = 1 << 10;
pub const ULPI_IC_CLKSUSPM: u32 = 1 << 11;
pub const ULPI_IC_AUTORESUME: u32 = 1 << 12;
pub const ULPI_IC_EXTVBUS_INDINV: u32 = 1 << 13;
pub const ULPI_IC_IND_PASSTHRU: u32 = 1 << 14;
pub const ULPI_IC_PROTECT_DIS: u32 = 1 << 15;

pub const ULPI_FC_HS: u32 = 1 << 16;
pub const ULPI_FC_FS: u32 = 1 << 17;
pub const ULPI_FC_LS: u32 = 1 << 18;
pub const ULPI_FC_FS4LS: u32 = 1 << 19;
pub const ULPI_FC_TERMSEL: u32 = 1 << 20;
pub const ULPI_FC_OP_NORM: u32 = 1 << 21;
pub const ULPI_FC_OP_NODRV: u32 = 1 << 22;
pub const ULPI_FC_OP_DIS_NRZI: u32 = 1 << 23;
pub const ULPI_FC_OP_NSYNC_NEOP: u32 = 1 << 24;
pub const ULPI_FC_RST: u32 = 1 << 25;
pub const ULPI_FC_SUSPM: u32 = 1 << 26;

/*-------------------------------------------------------------------------*/

// The CONFIG_USB_ULPI condition is a build-time configuration supplied by
// the surrounding kernel translation.
#[cfg(CONFIG_USB_ULPI)]
pub unsafe extern "C" {
    pub fn devm_otg_ulpi_create(
        dev: *mut device,
        ops: *mut usb_phy_io_ops,
        flags: u32,
    ) -> *mut usb_phy;
}

#[cfg(not(CONFIG_USB_ULPI))]
#[inline]
pub unsafe fn devm_otg_ulpi_create(
    _dev: *mut device,
    _ops: *mut usb_phy_io_ops,
    _flags: u32,
) -> *mut usb_phy {
    core::ptr::null_mut()
}

// Access operations for controllers with a viewport register.
#[cfg(CONFIG_USB_ULPI_VIEWPORT)]
pub unsafe extern "C" {
    pub static mut ulpi_viewport_access_ops: usb_phy_io_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
