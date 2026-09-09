/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2020
 *
 * Author(s):
 *   Niklas Schnelle <schnelle@linux.ibm.com>
 *
 */

// The C header includes <linux/pci.h>; the corresponding types are supplied
// by external dependencies.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zpci_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zpci_dev {
    _private: [u8; 0],
}

// CONFIG_PCI_IOV
#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn zpci_iov_remove_virtfn(pdev: *mut pci_dev, vfn: ::core::ffi::c_int);

    pub fn zpci_iov_map_resources(pdev: *mut pci_dev);

    pub fn zpci_iov_setup_virtfn(
        zbus: *mut zpci_bus,
        virtfn: *mut pci_dev,
        vfn: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn zpci_iov_find_parent_pf(
        zbus: *mut zpci_bus,
        zdev: *mut zpci_dev,
    ) -> *mut pci_dev;
}

// !CONFIG_PCI_IOV
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn zpci_iov_remove_virtfn(_pdev: *mut pci_dev, _vfn: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn zpci_iov_map_resources(_pdev: *mut pci_dev) {}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn zpci_iov_setup_virtfn(
    _zbus: *mut zpci_bus,
    _virtfn: *mut pci_dev,
    _vfn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn zpci_iov_find_parent_pf(
    _zbus: *mut zpci_bus,
    _zdev: *mut zpci_dev,
) -> *mut pci_dev {
    ::core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
