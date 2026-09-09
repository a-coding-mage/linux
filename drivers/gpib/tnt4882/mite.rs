// SPDX-License-Identifier: GPL-2.0-only

/*
 * Hardware driver for NI Mite PCI interface chip, adapted from COMEDI.
 *
 * The Linux kernel headers and mite.h declarations used by this translation
 * are supplied by the surrounding crate.
 */

use core::ffi::c_void;

// C preprocessor constants from the original translation unit.
pub const PCI_MITE_SIZE: usize = 4096;
pub const PCI_DAQ_SIZE: usize = 4096;

// Supplied by the kernel/mite header dependency.
extern "C" {
    static mut mite_devices: *mut mite_struct;

    fn pci_get_device(
        vendor: u32,
        device: u32,
        from: *mut pci_dev,
    ) -> *mut pci_dev;
    fn pci_dev_get(dev: *mut pci_dev);
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_enable_device(dev: *mut pci_dev) -> i32;
    fn pci_set_master(dev: *mut pci_dev);
    fn pci_request_regions(dev: *mut pci_dev, name: *const i8) -> i32;
    fn pci_release_regions(dev: *mut pci_dev);
    fn pci_disable_device(dev: *mut pci_dev);
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> u32;
    fn pci_resource_len(dev: *mut pci_dev, bar: u32) -> usize;
    fn ioremap(addr: u32, len: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn writel(value: u32, addr: *mut u8);
    fn kzalloc_mite_struct() -> *mut mite_struct;
    fn kfree(ptr: *mut mite_struct);
}

// These types and constants are defined by mite.h and the Linux PCI headers.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mite_struct {
    pub pcidev: *mut pci_dev,
    pub next: *mut mite_struct,
    pub mite_phys_addr: u32,
    pub mite_io_addr: *mut u8,
    pub daq_phys_addr: u32,
    pub daq_io_addr: *mut u8,
    pub used: i32,
}

pub const PCI_VENDOR_ID_NATINST: u32 = 0x1093;
pub const PCI_ANY_ID: u32 = 0xffff;
pub const MITE_IODWBSR: usize = 0xc0;
pub const WENAB: u32 = 0x8000_0000;
pub const EIO: i32 = 5;
pub const ENOMEM: i32 = 12;

#[inline]
unsafe fn top_of_page(x: usize) -> usize {
    x | !PAGE_MASK
}

// PAGE_MASK is provided by the kernel headers.
extern "C" {
    static PAGE_MASK: usize;
}

pub unsafe fn mite_init() {
    let mut pcidev: *mut pci_dev;
    let mut mite: *mut mite_struct;

    pcidev = pci_get_device(PCI_VENDOR_ID_NATINST, PCI_ANY_ID, core::ptr::null_mut());
    while !pcidev.is_null() {
        mite = kzalloc_mite_struct();
        if mite.is_null() {
            return;
        }

        (*mite).pcidev = pcidev;
        pci_dev_get((*mite).pcidev);
        (*mite).next = mite_devices;
        mite_devices = mite;

        pcidev = pci_get_device(PCI_VENDOR_ID_NATINST, PCI_ANY_ID, pcidev);
    }
}

pub unsafe fn mite_setup(mite: *mut mite_struct) -> i32 {
    let mut addr: u32;

    if pci_enable_device((*mite).pcidev) != 0 {
        return -EIO;
    }
    pci_set_master((*mite).pcidev);
    if pci_request_regions((*mite).pcidev, b"mite\0".as_ptr() as *const i8) != 0 {
        return -EIO;
    }
    addr = pci_resource_start((*mite).pcidev, 0);
    (*mite).mite_phys_addr = addr;
    (*mite).mite_io_addr = ioremap(addr, pci_resource_len((*mite).pcidev, 0));
    if (*mite).mite_io_addr.is_null() {
        return -ENOMEM;
    }
    addr = pci_resource_start((*mite).pcidev, 1);
    (*mite).daq_phys_addr = addr;
    (*mite).daq_io_addr = ioremap((*mite).daq_phys_addr, pci_resource_len((*mite).pcidev, 1));
    if (*mite).daq_io_addr.is_null() {
        return -ENOMEM;
    }
    writel(
        (*mite).daq_phys_addr | WENAB,
        (*mite).mite_io_addr.add(MITE_IODWBSR),
    );
    (*mite).used = 1;
    0
}

pub unsafe fn mite_cleanup() {
    let mut mite: *mut mite_struct = mite_devices;
    let mut next: *mut mite_struct;

    while !mite.is_null() {
        next = (*mite).next;
        if !(*mite).pcidev.is_null() {
            pci_dev_put((*mite).pcidev);
        }
        kfree(mite);
        mite = next;
    }
}

pub unsafe fn mite_unsetup(mite: *mut mite_struct) {
    if mite.is_null() {
        return;
    }
    if !(*mite).mite_io_addr.is_null() {
        iounmap((*mite).mite_io_addr);
        (*mite).mite_io_addr = core::ptr::null_mut();
    }
    if !(*mite).daq_io_addr.is_null() {
        iounmap((*mite).daq_io_addr);
        (*mite).daq_io_addr = core::ptr::null_mut();
    }
    if (*mite).mite_phys_addr != 0 {
        pci_release_regions((*mite).pcidev);
        pci_disable_device((*mite).pcidev);
        (*mite).mite_phys_addr = 0;
    }
    (*mite).used = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
