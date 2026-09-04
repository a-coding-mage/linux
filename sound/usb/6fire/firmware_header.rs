// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Author: Torsten Schenk
 * Created: Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// Requires: common.h provides usb_interface type definition
// (Corresponds to #include "common.h")

// Forward declare external type from common.h
#[repr(C)]
pub struct usb_interface;

// Firmware state of device
pub const FW_READY: i32 = 0;
pub const FW_NOT_READY: i32 = 1;

extern "C" {
    pub fn usb6fire_fw_init(intf: *mut usb_interface) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
