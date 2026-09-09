// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2012 Hauke Mehrtens <hauke@hauke-m.de>
 */

// Translated from the C header __USB_CORE_OHCI_PDRIVER_H.

/**
 * struct usb_ohci_pdata - platform_data for generic ohci driver
 *
 * @big_endian_desc:  BE descriptors
 * @big_endian_mmio:  BE registers
 * @no_big_frame_no:  no big endian frame_no shift
 * @num_ports:        number of ports
 *
 * These are general configuration options for the OHCI controller. All of
 * these options are activating more or less workarounds for some hardware.
 */
#[repr(C)]
pub struct usb_ohci_pdata {
    // C bit-fields: each occupies one bit of the first unsigned storage unit.
    pub big_endian_desc: u32,
    pub big_endian_mmio: u32,
    pub no_big_frame_no: u32,
    pub num_ports: u32,

    // Turn on all power and clocks
    pub power_on: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    // Turn off all power and clocks
    pub power_off: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    // Turn on only VBUS suspend power and hotplug detection,
    // turn off everything else
    pub power_suspend: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
}

// Forward declaration supplied by the platform-device dependency.
pub enum platform_device {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
