/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * PCI Endpoint ConfigFS header file
 *
 * Copyright (C) 2017 Texas Instruments
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 */

// Original dependency: <linux/configfs.h>
use core::ffi::c_char;

// Opaque type supplied by the configfs dependency.
#[repr(C)]
pub struct config_group {
    _private: [u8; 0],
}

// CONFIG_PCI_ENDPOINT_CONFIGFS is a build-time configuration condition.
#[cfg(CONFIG_PCI_ENDPOINT_CONFIGFS)]
extern "C" {
    pub fn pci_ep_cfs_add_epc_group(name: *const c_char) -> *mut config_group;
    pub fn pci_ep_cfs_remove_epc_group(group: *mut config_group);
    pub fn pci_ep_cfs_add_epf_group(name: *const c_char) -> *mut config_group;
    pub fn pci_ep_cfs_remove_epf_group(group: *mut config_group);
}

#[cfg(not(CONFIG_PCI_ENDPOINT_CONFIGFS))]
pub unsafe fn pci_ep_cfs_add_epc_group(_name: *const c_char) -> *mut config_group {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_PCI_ENDPOINT_CONFIGFS))]
pub unsafe fn pci_ep_cfs_remove_epc_group(_group: *mut config_group) {}

#[cfg(not(CONFIG_PCI_ENDPOINT_CONFIGFS))]
pub unsafe fn pci_ep_cfs_add_epf_group(_name: *const c_char) -> *mut config_group {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_PCI_ENDPOINT_CONFIGFS))]
pub unsafe fn pci_ep_cfs_remove_epf_group(_group: *mut config_group) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
