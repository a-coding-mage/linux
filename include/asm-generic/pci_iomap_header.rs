/* SPDX-License-Identifier: GPL-2.0+ */
/* Generic I/O port emulation.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[cfg(CONFIG_PCI)]
extern "C" {
    /* Create a virtual mapping cookie for a PCI BAR (memory or IO) */
    pub fn pci_iomap(dev: *mut pci_dev, bar: c_int, max: c_ulong) -> *mut c_void;
    pub fn pci_iomap_wc(dev: *mut pci_dev, bar: c_int, max: c_ulong) -> *mut c_void;
    pub fn pci_iomap_range(
        dev: *mut pci_dev,
        bar: c_int,
        offset: c_ulong,
        maxlen: c_ulong,
    ) -> *mut c_void;
    pub fn pci_iomap_wc_range(
        dev: *mut pci_dev,
        bar: c_int,
        offset: c_ulong,
        maxlen: c_ulong,
    ) -> *mut c_void;
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut c_void);

    /* Create a virtual mapping cookie for a port on a given PCI device.
     * Do not call this directly, it exists to make it easier for architectures
     * to override
     */
    #[cfg(CONFIG_NO_GENERIC_PCI_IOPORT_MAP)]
    pub fn __pci_ioport_map(
        dev: *mut pci_dev,
        port: c_ulong,
        nr: c_uint,
    ) -> *mut c_void;
}

#[cfg(all(CONFIG_PCI, not(CONFIG_NO_GENERIC_PCI_IOPORT_MAP), not(CONFIG_HAS_IOPORT_MAP)))]
#[inline]
pub unsafe fn __pci_ioport_map(
    _dev: *mut pci_dev,
    _port: c_ulong,
    _nr: c_uint,
) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(all(CONFIG_PCI, not(CONFIG_NO_GENERIC_PCI_IOPORT_MAP), CONFIG_HAS_IOPORT_MAP))]
extern "C" {
    pub fn ioport_map(port: c_ulong, nr: c_uint) -> *mut c_void;
}

#[cfg(all(CONFIG_PCI, not(CONFIG_NO_GENERIC_PCI_IOPORT_MAP), CONFIG_HAS_IOPORT_MAP))]
#[inline]
pub unsafe fn __pci_ioport_map(
    _dev: *mut pci_dev,
    port: c_ulong,
    nr: c_uint,
) -> *mut c_void {
    ioport_map(port, nr)
}

#[cfg(all(not(CONFIG_PCI), CONFIG_GENERIC_PCI_IOMAP))]
#[inline]
pub unsafe fn pci_iomap(_dev: *mut pci_dev, _bar: c_int, _max: c_ulong) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(all(not(CONFIG_PCI), CONFIG_GENERIC_PCI_IOMAP))]
#[inline]
pub unsafe fn pci_iomap_wc(_dev: *mut pci_dev, _bar: c_int, _max: c_ulong) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(all(not(CONFIG_PCI), CONFIG_GENERIC_PCI_IOMAP))]
#[inline]
pub unsafe fn pci_iomap_range(
    _dev: *mut pci_dev,
    _bar: c_int,
    _offset: c_ulong,
    _maxlen: c_ulong,
) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(all(not(CONFIG_PCI), CONFIG_GENERIC_PCI_IOMAP))]
#[inline]
pub unsafe fn pci_iomap_wc_range(
    _dev: *mut pci_dev,
    _bar: c_int,
    _offset: c_ulong,
    _maxlen: c_ulong,
) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(all(not(CONFIG_PCI), CONFIG_GENERIC_PCI_IOMAP))]
#[inline]
pub unsafe fn pci_iounmap(_dev: *mut pci_dev, _addr: *mut c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
