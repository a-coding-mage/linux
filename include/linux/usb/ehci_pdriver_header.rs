// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2012 Hauke Mehrtens <hauke@hauke-m.de>
 */

// Forward declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_hcd {
    _private: [u8; 0],
}

/**
 * struct usb_ehci_pdata - platform_data for generic ehci driver
 *
 * @caps_offset: offset of the EHCI Capability Registers to the start of
 *               the io memory region provided to the driver.
 * @has_tt: set to 1 if TT is integrated in root hub.
 * @port_power_on: set to 1 if the controller needs a power up after
 *                 initialization.
 * @port_power_off: set to 1 if the controller needs to be powered down
 *                  after initialization.
 * @no_io_watchdog: set to 1 if the controller does not need the I/O
 *                  watchdog to run.
 * @reset_on_resume: set to 1 if the controller needs to be reset after
 *                   a suspend / resume cycle (but can't detect that itself).
 *
 * These are general configuration options for the EHCI controller. All of
 * these options are activating more or less workarounds for some hardware.
 */
#[repr(C)]
pub struct usb_ehci_pdata {
    pub caps_offset: core::ffi::c_int,

    // C bit-fields, each one bit wide, represented in their containing word.
    pub has_tt: u32,
    pub has_synopsys_hc_bug: u32,
    pub big_endian_desc: u32,
    pub big_endian_mmio: u32,
    pub no_io_watchdog: u32,
    pub reset_on_resume: u32,
    pub dma_mask_64: u32,
    pub spurious_oc: u32,

    /* Turn on all power and clocks */
    pub power_on: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> core::ffi::c_int>,
    /* Turn off all power and clocks */
    pub power_off: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    /* Turn on only VBUS suspend power and hotplug detection,
     * turn off everything else */
    pub power_suspend: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    pub pre_setup: Option<unsafe extern "C" fn(hcd: *mut usb_hcd) -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
