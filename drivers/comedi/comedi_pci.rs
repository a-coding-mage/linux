// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_pci.c
 * Comedi PCI driver specific functions.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel and Comedi declarations are supplied by the surrounding crate.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}
#[repr(C)]
pub struct comedi_device {
    pub hw_dev: *mut device,
    pub ioenabled: bool,
    pub board_name: *const c_char,
    pub irq: c_int,
    pub mmio: *mut c_void,
}
#[repr(C)]
pub struct comedi_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_driver {
    _private: [u8; 0],
}

extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_enable_device(dev: *mut pci_dev) -> c_int;
    fn pci_request_regions(dev: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_disable_device(dev: *mut pci_dev);
    fn pci_release_regions(dev: *mut pci_dev);
    fn free_irq(irq: c_int, dev: *mut comedi_device);
    fn iounmap(addr: *mut c_void);
    fn comedi_auto_config(dev: *mut device, driver: *mut comedi_driver, context: usize) -> c_int;
    fn comedi_auto_unconfig(dev: *mut device);
    fn comedi_driver_register(driver: *mut comedi_driver) -> c_int;
    fn comedi_driver_unregister(driver: *mut comedi_driver);
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
}

/// Return PCI device attached to COMEDI device.
#[no_mangle]
pub unsafe extern "C" fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev {
    if !(*dev).hw_dev.is_null() {
        to_pci_dev((*dev).hw_dev)
    } else {
        core::ptr::null_mut()
    }
}

/// Enable the PCI device and request the regions.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_enable(dev: *mut comedi_device) -> c_int {
    let pcidev = comedi_to_pci_dev(dev);
    if pcidev.is_null() {
        return -19; // -ENODEV
    }

    let mut rc = pci_enable_device(pcidev);
    if rc < 0 {
        return rc;
    }

    rc = pci_request_regions(pcidev, (*dev).board_name);
    if rc < 0 {
        pci_disable_device(pcidev);
    } else {
        (*dev).ioenabled = true;
    }
    rc
}

/// Release the regions and disable the PCI device.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_disable(dev: *mut comedi_device) {
    let pcidev = comedi_to_pci_dev(dev);
    if !pcidev.is_null() && (*dev).ioenabled {
        pci_release_regions(pcidev);
        pci_disable_device(pcidev);
    }
    (*dev).ioenabled = false;
}

/// A generic detach handler for PCI COMEDI drivers.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_detach(dev: *mut comedi_device) {
    let pcidev = comedi_to_pci_dev(dev);
    if pcidev.is_null() || !(*dev).ioenabled {
        return;
    }
    if (*dev).irq != 0 {
        free_irq((*dev).irq, dev);
        (*dev).irq = 0;
    }
    if !(*dev).mmio.is_null() {
        iounmap((*dev).mmio);
        (*dev).mmio = core::ptr::null_mut();
    }
    comedi_pci_disable(dev);
}

/// Configure/probe a PCI COMEDI device.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_auto_config(
    pcidev: *mut pci_dev,
    driver: *mut comedi_driver,
    context: usize,
) -> c_int {
    comedi_auto_config(&mut (*pcidev).dev, driver, context)
}

/// Unconfigure/remove a PCI COMEDI device.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_auto_unconfig(pcidev: *mut pci_dev) {
    comedi_auto_unconfig(&mut (*pcidev).dev);
}

/// Register a PCI COMEDI driver.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_driver_register(
    comedi_driver: *mut comedi_driver,
    pci_driver: *mut pci_driver,
) -> c_int {
    let mut ret = comedi_driver_register(comedi_driver);
    if ret < 0 {
        return ret;
    }
    ret = pci_register_driver(pci_driver);
    if ret < 0 {
        comedi_driver_unregister(comedi_driver);
        return ret;
    }
    0
}

/// Unregister a PCI COMEDI driver.
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_driver_unregister(
    comedi_driver: *mut comedi_driver,
    pci_driver: *mut pci_driver,
) {
    pci_unregister_driver(pci_driver);
    comedi_driver_unregister(comedi_driver);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
