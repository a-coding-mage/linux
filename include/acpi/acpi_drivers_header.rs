/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  acpi_drivers.h  ($Revision: 31 $)
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 */

pub const ACPI_MAX_STRING: usize = 80;

/*
 * _HID definitions
 * HIDs must conform to ACPI spec(6.1.4)
 * Linux specific HIDs do not apply to this and begin with LNX:
 */

pub const ACPI_POWER_HID: &str = "LNXPOWER";
pub const ACPI_PROCESSOR_OBJECT_HID: &str = "LNXCPU";
pub const ACPI_SYSTEM_HID: &str = "LNXSYSTM";
pub const ACPI_THERMAL_HID: &str = "LNXTHERM";
pub const ACPI_BUTTON_HID_POWERF: &str = "LNXPWRBN";
pub const ACPI_BUTTON_HID_SLEEPF: &str = "LNXSLPBN";
pub const ACPI_VIDEO_HID: &str = "LNXVIDEO";
pub const ACPI_BAY_HID: &str = "LNXIOBAY";
pub const ACPI_DOCK_HID: &str = "LNXDOCK";
pub const ACPI_ECDT_HID: &str = "LNXEC";
/* SMBUS HID definition as supported by Microsoft Windows */
pub const ACPI_SMBUS_MS_HID: &str = "SMB0001";
/* Quirk for broken IBM BIOSes */
pub const ACPI_SMBUS_IBM_HID: &str = "SMBUSIBM";

/*
 * For fixed hardware buttons, we fabricate acpi_devices with HID
 * ACPI_BUTTON_HID_POWERF or ACPI_BUTTON_HID_SLEEPF.  Fixed hardware
 * signals only an event; it doesn't supply a notification value.
 * To allow drivers to treat notifications from fixed hardware the
 * same as those from real devices, we turn the events into this
 * notification value.
 */
pub const ACPI_FIXED_HARDWARE_EVENT: u32 = 0x100;

/* --------------------------------------------------------------------------
                                       PCI
   -------------------------------------------------------------------------- */

/* ACPI PCI Interrupt Link */

extern "C" {
    pub fn acpi_irq_penalty_init() -> ::core::ffi::c_int;
    pub fn acpi_pci_link_allocate_irq(
        handle: acpi_handle,
        index: ::core::ffi::c_int,
        triggering: *mut ::core::ffi::c_int,
        polarity: *mut ::core::ffi::c_int,
        name: *mut *mut ::core::ffi::c_char,
        gsi: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn acpi_pci_link_free_irq(handle: acpi_handle) -> ::core::ffi::c_int;
}

/* ACPI PCI Device Binding */

#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[cfg(CONFIG_PCI)]
extern "C" {
    pub fn acpi_dev_get_pci_dev(adev: *mut acpi_device) -> *mut pci_dev;
}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub unsafe fn acpi_dev_get_pci_dev(_adev: *mut acpi_device) -> *mut pci_dev {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn acpi_get_pci_dev(handle: acpi_handle) -> *mut pci_dev {
    acpi_dev_get_pci_dev(acpi_fetch_acpi_dev(handle))
}

/* Arch-defined function to add a bus to the system */

extern "C" {
    pub fn pci_acpi_scan_root(root: *mut acpi_pci_root) -> *mut pci_bus;
}

#[cfg(CONFIG_X86)]
extern "C" {
    pub fn pci_acpi_crs_quirks();
}

#[cfg(not(CONFIG_X86))]
#[inline]
pub fn pci_acpi_crs_quirks() {}

/*--------------------------------------------------------------------------
                                  Dock Station
  -------------------------------------------------------------------------- */

#[cfg(CONFIG_ACPI_DOCK)]
extern "C" {
    pub fn is_dock_device(adev: *mut acpi_device) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_ACPI_DOCK))]
#[inline]
pub fn is_dock_device(_adev: *mut acpi_device) -> ::core::ffi::c_int {
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
