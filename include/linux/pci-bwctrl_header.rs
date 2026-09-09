/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PCIe bandwidth controller
 *
 * Copyright (C) 2023-2024 Intel Corporation
 */

// Dependency supplied by the Linux PCI headers.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thermal_cooling_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PCIE_THERMAL")]
extern "C" {
    pub fn pcie_cooling_device_register(
        port: *mut pci_dev,
    ) -> *mut thermal_cooling_device;
    pub fn pcie_cooling_device_unregister(cdev: *mut thermal_cooling_device);
}

#[cfg(not(feature = "CONFIG_PCIE_THERMAL"))]
#[inline]
pub unsafe fn pcie_cooling_device_register(
    _port: *mut pci_dev,
) -> *mut thermal_cooling_device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_PCIE_THERMAL"))]
#[inline]
pub unsafe fn pcie_cooling_device_unregister(_cdev: *mut thermal_cooling_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
