// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Tascam US-X2Y USB soundcards
//
// Copyright (c) 2003 by Karsten Wiese <annabellesgarden@yahoo.de>

pub const USX2Y_DRIVER_VERSION: u32 = 0x0100; // 0.1.0

// hwdep id string
pub const SND_USX2Y_LOADER_ID: &str = "USX2Y Loader";
pub const SND_USX2Y_USBPCM_ID: &str = "USX2Y USBPCM";

// hardware type
pub const USX2Y_TYPE_122: u32 = 0;
pub const USX2Y_TYPE_224: u32 = 1;
pub const USX2Y_TYPE_428: u32 = 2;
pub const USX2Y_TYPE_NUMS: u32 = 3;

pub const USB_ID_US122: u16 = 0x8007;
pub const USB_ID_US224: u16 = 0x8005;
pub const USB_ID_US428: u16 = 0x8001;

// chip status
pub const USX2Y_STAT_CHIP_INIT: u32 = 1 << 0; // all operational
pub const USX2Y_STAT_CHIP_MMAP_PCM_URBS: u32 = 1 << 1; // pcm transport over mmaped urbs
pub const USX2Y_STAT_CHIP_HUP: u32 = 1 << 31; // all operational

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
