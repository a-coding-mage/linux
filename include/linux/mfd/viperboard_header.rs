/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/linux/mfd/viperboard.h
 *
 *  Nano River Technologies viperboard definitions
 *
 *  (C) 2012 by Lemonage GmbH
 *  Author: Lars Poeschel <poeschel@lemonage.de>
 *  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const VPRBRD_EP_OUT: u8 = 0x02;
pub const VPRBRD_EP_IN: u8 = 0x86;

pub const VPRBRD_I2C_MSG_LEN: usize = 512; /* max length of a msg on USB level */

pub const VPRBRD_I2C_FREQ_6MHZ: u8 = 1; /*   6 MBit/s */
pub const VPRBRD_I2C_FREQ_3MHZ: u8 = 2; /*   3 MBit/s */
pub const VPRBRD_I2C_FREQ_1MHZ: u8 = 3; /*   1 MBit/s */
pub const VPRBRD_I2C_FREQ_FAST: u8 = 4; /* 400 kbit/s */
pub const VPRBRD_I2C_FREQ_400KHZ: u8 = VPRBRD_I2C_FREQ_FAST;
pub const VPRBRD_I2C_FREQ_200KHZ: u8 = 5; /* 200 kbit/s */
pub const VPRBRD_I2C_FREQ_STD: u8 = 6; /* 100 kbit/s */
pub const VPRBRD_I2C_FREQ_100KHZ: u8 = VPRBRD_I2C_FREQ_STD;
pub const VPRBRD_I2C_FREQ_10KHZ: u8 = 7; /*  10 kbit/s */

pub const VPRBRD_I2C_CMD_WRITE: u8 = 0x00;
pub const VPRBRD_I2C_CMD_READ: u8 = 0x01;
pub const VPRBRD_I2C_CMD_ADDR: u8 = 0x02;

pub const VPRBRD_USB_TYPE_OUT: u8 = 0x40;
pub const VPRBRD_USB_TYPE_IN: u8 = 0xc0;
pub const VPRBRD_USB_TIMEOUT_MS: u32 = 100;
pub const VPRBRD_USB_REQUEST_I2C_FREQ: u8 = 0xe6;
pub const VPRBRD_USB_REQUEST_I2C: u8 = 0xe9;
pub const VPRBRD_USB_REQUEST_MAJOR: u8 = 0xea;
pub const VPRBRD_USB_REQUEST_MINOR: u8 = 0xeb;
pub const VPRBRD_USB_REQUEST_ADC: u8 = 0xec;
pub const VPRBRD_USB_REQUEST_GPIOA: u8 = 0xed;
pub const VPRBRD_USB_REQUEST_GPIOB: u8 = 0xdd;

#[repr(C, packed)]
pub struct vprbrd_i2c_write_hdr {
    pub cmd: u8,
    pub addr: u16, // __le16
    pub len1: u8,
    pub len2: u8,
    pub last: u8,
    pub chan: u8,
    pub spi: u16,
}

#[repr(C, packed)]
pub struct vprbrd_i2c_read_hdr {
    pub cmd: u8,
    pub addr: u16, // __le16
    pub len0: u8,
    pub len1: u8,
    pub len2: u8,
    pub len3: u8,
    pub len4: u8,
    pub len5: u8,
    pub tf1: u16, // __le16, transfer 1 length
    pub tf2: u16, // __le16, transfer 2 length
}

#[repr(C, packed)]
pub struct vprbrd_i2c_status {
    pub unknown: [u8; 11],
    pub status: u8,
}

#[repr(C, packed)]
pub struct vprbrd_i2c_write_msg {
    pub header: vprbrd_i2c_write_hdr,
    pub data: [u8; VPRBRD_I2C_MSG_LEN - core::mem::size_of::<vprbrd_i2c_write_hdr>()],
}

#[repr(C, packed)]
pub struct vprbrd_i2c_read_msg {
    pub header: vprbrd_i2c_read_hdr,
    pub data: [u8; VPRBRD_I2C_MSG_LEN - core::mem::size_of::<vprbrd_i2c_read_hdr>()],
}

#[repr(C, packed)]
pub struct vprbrd_i2c_addr_msg {
    pub cmd: u8,
    pub addr: u8,
    pub unknown1: u8,
    pub len: u16, // __le16
    pub unknown2: u8,
    pub unknown3: u8,
}

/* Structure to hold all device specific stuff */
#[repr(C)]
pub struct vprbrd {
    pub usb_dev: *mut usb_device, /* the usb device for this device */
    pub lock: mutex,
    pub buf: [u8; core::mem::size_of::<vprbrd_i2c_write_msg>()],
    pub pdev: platform_device,
}

// Opaque declarations supplied by included kernel headers.
pub struct usb_device;
pub struct mutex;
pub struct platform_device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
