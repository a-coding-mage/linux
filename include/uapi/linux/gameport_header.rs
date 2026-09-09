/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  Copyright (c) 1999-2002 Vojtech Pavlik
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 */

pub const GAMEPORT_MODE_DISABLED: u32 = 0;
pub const GAMEPORT_MODE_RAW: u32 = 1;
pub const GAMEPORT_MODE_COOKED: u32 = 2;

pub const GAMEPORT_ID_VENDOR_ANALOG: u32 = 0x0001;
pub const GAMEPORT_ID_VENDOR_MADCATZ: u32 = 0x0002;
pub const GAMEPORT_ID_VENDOR_LOGITECH: u32 = 0x0003;
pub const GAMEPORT_ID_VENDOR_CREATIVE: u32 = 0x0004;
pub const GAMEPORT_ID_VENDOR_GENIUS: u32 = 0x0005;
pub const GAMEPORT_ID_VENDOR_INTERACT: u32 = 0x0006;
pub const GAMEPORT_ID_VENDOR_MICROSOFT: u32 = 0x0007;
pub const GAMEPORT_ID_VENDOR_THRUSTMASTER: u32 = 0x0008;
pub const GAMEPORT_ID_VENDOR_GRAVIS: u32 = 0x0009;
pub const GAMEPORT_ID_VENDOR_GUILLEMOT: u32 = 0x000a;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
