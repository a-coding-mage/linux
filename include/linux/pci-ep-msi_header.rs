/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PCI Endpoint *Function* side MSI header file
 *
 * Copyright (C) 2024 NXP
 * Author: Frank Li <Frank.Li@nxp.com>
 */

// C header guard: __PCI_EP_MSI__

#[repr(C)]
pub struct pci_epf {
    _private: [u8; 0],
}

// CONFIG_PCI_ENDPOINT_MSI_DOORBELL is a build-time C configuration option.
#[cfg(feature = "CONFIG_PCI_ENDPOINT_MSI_DOORBELL")]
extern "C" {
    pub fn pci_epf_alloc_doorbell(epf: *mut pci_epf, nums: u16) -> i32;
    pub fn pci_epf_free_doorbell(epf: *mut pci_epf);
}

// CONFIG_PCI_ENDPOINT_MSI_DOORBELL disabled: preserve the C static inline fallback.
#[cfg(not(feature = "CONFIG_PCI_ENDPOINT_MSI_DOORBELL"))]
pub unsafe fn pci_epf_alloc_doorbell(_epf: *mut pci_epf, _nums: u16) -> i32 {
    -ENODATA
}

#[cfg(not(feature = "CONFIG_PCI_ENDPOINT_MSI_DOORBELL"))]
pub unsafe fn pci_epf_free_doorbell(_epf: *mut pci_epf) {
}

// ENODATA is supplied by the corresponding platform/kernel dependencies.
extern "C" {
    pub static ENODATA: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
