/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * WebUSB descriptors and constants
 *
 * Copyright (C) 2023 Jó Ágila Bitsch <jgilab@gmail.com>
 */

// Dependency intent from the C header: `uapi/linux/usb/ch9.h`.

/*
 * Little Endian PlatformCapabilityUUID for WebUSB
 * 3408b638-09a9-47a0-8bfd-a0768815b665
 * to identify Platform Device Capability descriptors as referring to WebUSB.
 */
pub const WEBUSB_UUID: [u8; 16] = [
    0x38, 0xb6, 0x08, 0x34, 0xa9, 0x09, 0xa0, 0x47,
    0x8b, 0xfd, 0xa0, 0x76, 0x88, 0x15, 0xb6, 0x65,
];

/*
 * WebUSB Platform Capability data
 *
 * A device announces support for the
 * WebUSB command set by including the following Platform Descriptor Data in its
 * Binary Object Store associated with the WebUSB_UUID above.
 * See: https://wicg.github.io/webusb/#webusb-platform-capability-descriptor
 */
#[repr(C, packed)]
pub struct UsbWebusbCapData {
    pub bcdVersion: u16,
    pub bVendorCode: u8,
    pub iLandingPage: u8,
}

pub const WEBUSB_VERSION_1_00: u16 = 0x0100;
pub const WEBUSB_LANDING_PAGE_NOT_PRESENT: u8 = 0;
pub const WEBUSB_LANDING_PAGE_PRESENT: u8 = 1;

pub const USB_WEBUSB_CAP_DATA_SIZE: usize = 4;

/*
 * Get URL Request
 *
 * The request to fetch an URL is defined in https://wicg.github.io/webusb/#get-url as:
 * bmRequestType: (USB_DIR_IN | USB_TYPE_VENDOR) = 11000000B
 * bRequest: bVendorCode
 * wValue: iLandingPage
 * wIndex: GET_URL = 2
 * wLength: Descriptor Length (typically U8_MAX = 255)
 * Data: URL Descriptor
 */
pub const WEBUSB_GET_URL: u8 = 2;

/*
 * This descriptor contains a single URL and is returned by the Get URL request.
 *
 * See: https://wicg.github.io/webusb/#url-descriptor
 */
#[repr(C, packed)]
pub struct WebusbUrlDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bScheme: u8,
    pub URL: [u8; u8::MAX as usize - WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH],
}

pub const WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH: usize = 3;
pub const WEBUSB_URL_DESCRIPTOR_TYPE: u8 = 3;
pub const WEBUSB_URL_SCHEME_HTTP: u8 = 0;
pub const WEBUSB_URL_SCHEME_HTTPS: u8 = 1;
pub const WEBUSB_URL_SCHEME_NONE: u8 = 255;

/*
 * Buffer size to hold the longest URL that can be in an URL descriptor
 *
 * The descriptor can be U8_MAX  bytes long.
 * WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH bytes are used for a header.
 * Since the longest prefix that might be stripped is "https://", we may accommodate an additional
 * 8 bytes.
 */
pub const WEBUSB_URL_RAW_MAX_LENGTH: usize =
    u8::MAX as usize - WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH + 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
