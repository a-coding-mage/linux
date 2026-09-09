/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi_pci.h
 * header file for Comedi PCI drivers
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Dependencies supplied by the Linux PCI and Comedi device headers.

/*
 * PCI Vendor IDs not in <linux/pci_ids.h>
 */
pub const PCI_VENDOR_ID_KOLTER: u16 = 0x1001;
pub const PCI_VENDOR_ID_ICP: u16 = 0x104c;
pub const PCI_VENDOR_ID_DT: u16 = 0x1116;
pub const PCI_VENDOR_ID_IOTECH: u16 = 0x1616;
pub const PCI_VENDOR_ID_CONTEC: u16 = 0x1221;
pub const PCI_VENDOR_ID_RTD: u16 = 0x1435;
pub const PCI_VENDOR_ID_HUMUSOFT: u16 = 0x186c;

extern "C" {
    pub fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;

    pub fn comedi_pci_enable(dev: *mut comedi_device) -> ::std::os::raw::c_int;
    pub fn comedi_pci_disable(dev: *mut comedi_device);
    pub fn comedi_pci_detach(dev: *mut comedi_device);

    pub fn comedi_pci_auto_config(
        pcidev: *mut pci_dev,
        driver: *mut comedi_driver,
        context: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn comedi_pci_auto_unconfig(pcidev: *mut pci_dev);

    pub fn comedi_pci_driver_register(
        comedi_driver: *mut comedi_driver,
        pci_driver: *mut pci_driver,
    ) -> ::std::os::raw::c_int;
    pub fn comedi_pci_driver_unregister(
        comedi_driver: *mut comedi_driver,
        pci_driver: *mut pci_driver,
    );
}

/**
 * module_comedi_pci_driver() - Helper macro for registering a comedi PCI driver
 * @__comedi_driver: comedi_driver struct
 * @__pci_driver: pci_driver struct
 *
 * Helper macro for comedi PCI drivers which do not do anything special
 * in module init/exit. This eliminates a lot of boilerplate. Each
 * module may only use this macro once, and calling it replaces
 * module_init() and module_exit()
 */
#[macro_export]
macro_rules! module_comedi_pci_driver {
    ($__comedi_driver:expr, $__pci_driver:expr) => {
        module_driver!(
            $__comedi_driver,
            comedi_pci_driver_register,
            comedi_pci_driver_unregister,
            &mut ($__pci_driver)
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
