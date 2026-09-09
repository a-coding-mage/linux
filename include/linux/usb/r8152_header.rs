/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (c) 2020 Realtek Semiconductor Corp. All rights reserved.
 */

pub const RTL8152_REQT_READ: u8 = 0xc0;
pub const RTL8152_REQT_WRITE: u8 = 0x40;
pub const RTL8152_REQ_GET_REGS: u8 = 0x05;
pub const RTL8152_REQ_SET_REGS: u8 = 0x05;

pub const BYTE_EN_DWORD: u8 = 0xff;
pub const BYTE_EN_WORD: u8 = 0x33;
pub const BYTE_EN_BYTE: u8 = 0x11;
pub const BYTE_EN_SIX_BYTES: u8 = 0x3f;
pub const BYTE_EN_START_MASK: u8 = 0x0f;
pub const BYTE_EN_END_MASK: u8 = 0xf0;

pub const MCU_TYPE_PLA: u16 = 0x0100;
pub const MCU_TYPE_USB: u16 = 0x0000;

/* Define these values to match your device */
pub const VENDOR_ID_REALTEK: u16 = 0x0bda;
pub const VENDOR_ID_MICROSOFT: u16 = 0x045e;
pub const VENDOR_ID_SAMSUNG: u16 = 0x04e8;
pub const VENDOR_ID_LENOVO: u16 = 0x17ef;
pub const VENDOR_ID_LINKSYS: u16 = 0x13b1;
pub const VENDOR_ID_NVIDIA: u16 = 0x0955;
pub const VENDOR_ID_TPLINK: u16 = 0x2357;
pub const VENDOR_ID_DLINK: u16 = 0x2001;
pub const VENDOR_ID_DELL: u16 = 0x413c;
pub const VENDOR_ID_ASUS: u16 = 0x0b05;
pub const VENDOR_ID_TRENDNET: u16 = 0x20f4;

/* Preserved from the build-time condition IS_REACHABLE(CONFIG_USB_RTL8152). */
#[cfg(feature = "CONFIG_USB_RTL8152")]
extern "C" {
    pub fn rtl8152_get_version(intf: *mut usb_interface) -> u8;
}

/* External type supplied by the USB subsystem. */
#[repr(C)]
pub struct usb_interface {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
